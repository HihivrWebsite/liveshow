use axum::http::{header, StatusCode};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
    routing::{get, get_service, post},
    Json, Router,
};
use chrono::NaiveDateTime;
use reqwest::Client;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

mod avatar_cache;
mod gift_response_cache;
mod upstream_cache;
use upstream_cache::{Endpoint, Key, Source, UpstreamCache};

use avatar_cache::{
    AvatarCacheResult, AvatarStore, ConditionalHeaders, RefreshOutcome, RemoteAvatarResponse,
};

use crate::gift_response_cache::{
    clear_gift_response_cache, gift_response_cache_middleware, gift_response_cache_stats,
    GiftResponseCacheStats,
};
use base64::Engine;
use tokio::sync::RwLock;
use tokio::time::timeout;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer,
};
use tracing_subscriber;

// ==================== 缓存相关定义 ====================

#[derive(Clone)]
struct AvatarCacheEntry {
    face_url: String,
    image_bytes: Option<Vec<u8>>,
    fetched_at: std::time::SystemTime,
}

// 全局 face URL 缓存（48 小时），压缩 WebP 的唯一索引是 avatar_cache 清单。
lazy_static::lazy_static! {
    static ref AVATAR_CACHE: Arc<RwLock<HashMap<String, AvatarCacheEntry>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

lazy_static::lazy_static! {
    static ref UPSTREAM_CACHE: UpstreamCache = UpstreamCache::new("cache_data/upstream_v1".into());
}

lazy_static::lazy_static! {
    static ref AVATAR_STORE: AvatarStore =
        AvatarStore::open("cache_data/avatars_compressed")
            .expect("failed to open compressed avatar manifest");
}

fn is_avatar_cache_valid(fetched_at: std::time::SystemTime) -> bool {
    match std::time::SystemTime::now().duration_since(fetched_at) {
        Ok(duration) => duration.as_secs() < 48 * 3600,
        Err(_) => false,
    }
}

async fn get_face_url_from_bilibili(room_id: &str, force_refresh: bool) -> Option<String> {
    if !force_refresh {
        let cache = AVATAR_CACHE.read().await;
        if let Some(entry) = cache.get(room_id) {
            if is_avatar_cache_valid(entry.fetched_at) {
                return Some(entry.face_url.clone());
            }
        }
    }

    let api_url = format!(
        "https://api.live.bilibili.com/live_user/v1/UserInfo/get_anchor_in_room?roomid={}",
        room_id
    );
    let resp = HTTP_CLIENT
        .get(&api_url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .ok()?;
    let json: serde_json::Value = resp.json().await.ok()?;
    let face = json.get("data")?.get("info")?.get("face")?.as_str()?;
    let face = face.to_string();
    let mut cache = AVATAR_CACHE.write().await;
    let image_bytes = cache
        .get(room_id)
        .and_then(|entry| entry.image_bytes.clone());
    cache.insert(
        room_id.to_string(),
        AvatarCacheEntry {
            face_url: face.clone(),
            image_bytes,
            fetched_at: std::time::SystemTime::now(),
        },
    );
    Some(face)
}

async fn fetch_all_anchors_for_avatars(
    force_refresh: bool,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error + Send + Sync>> {
    let mut anchors = Vec::new();

    if let Ok(vr_anchors) =
        fetch_anchor_data_by_url("https://vr.qianqiuzy.cn/gift", "avatars").await
    {
        for anchor in vr_anchors {
            let room_id = anchor.room_id.to_string();
            let face_url = get_face_url_from_bilibili(&room_id, force_refresh).await;
            if let Some(url) = face_url {
                anchors.push((room_id, url));
            }
        }
    }

    if let Ok(psp_anchors) =
        fetch_anchor_data_by_url("https://psp.qianqiuzy.cn/gift", "avatars").await
    {
        for anchor in psp_anchors {
            let room_id = anchor.room_id.to_string();
            let face_url = get_face_url_from_bilibili(&room_id, force_refresh).await;
            if let Some(url) = face_url {
                anchors.push((room_id, url));
            }
        }
    }

    Ok(anchors)
}

async fn force_refresh_face_url_for_cache_key(key: &str) -> Option<String> {
    if let Some(uid) = key.strip_prefix("uid_") {
        if !uid.chars().all(|character| character.is_ascii_digit()) {
            return None;
        }
        let api_url = format!("https://api.bilibili.com/x/web-interface/card?mid={}", uid);
        let response = HTTP_CLIENT
            .get(&api_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Accept", "application/json")
            .send()
            .await
            .ok()?;
        let raw_data: serde_json::Value = response.json().await.ok()?;
        let face = raw_data
            .get("data")
            .and_then(|data| data.get("card"))
            .and_then(|card| card.get("face"))
            .and_then(|face| face.as_str())?;
        if face.is_empty() {
            return None;
        }
        Some(face.to_string())
    } else if key.chars().all(|character| character.is_ascii_digit()) {
        get_face_url_from_bilibili(key, true).await
    } else {
        None
    }
}

async fn fetch_remote_avatar_response(
    face_url: &str,
    conditional: ConditionalHeaders,
) -> AvatarCacheResult<RemoteAvatarResponse> {
    let mut request = HTTP_CLIENT
        .get(face_url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .header("Referer", "https://www.bilibili.com")
        .header("Accept", "image/webp,image/*;q=0.8,*/*;q=0.5");
    if let Some(etag) = conditional.etag {
        request = request.header(reqwest::header::IF_NONE_MATCH, etag);
    }
    if let Some(last_modified) = conditional.last_modified {
        request = request.header(reqwest::header::IF_MODIFIED_SINCE, last_modified);
    }

    let response = timeout(Duration::from_secs(10), request.send())
        .await
        .map_err(|_| "remote avatar request timed out".to_string())?
        .map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let last_modified = response
        .headers()
        .get(reqwest::header::LAST_MODIFIED)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);

    if status == 304 {
        return Ok(RemoteAvatarResponse {
            status,
            etag,
            last_modified,
            body: None,
        });
    }
    if !(200..300).contains(&status) {
        return Err(format!("remote avatar returned HTTP {status}").into());
    }
    let body = timeout(Duration::from_secs(10), response.bytes())
        .await
        .map_err(|_| "remote avatar body timed out".to_string())?
        .map_err(|error| error.to_string())?
        .to_vec();
    Ok(RemoteAvatarResponse {
        status,
        etag,
        last_modified,
        body: Some(body),
    })
}

async fn batch_download_avatars() {
    println!("🔄 [头像批量更新] 开始...");

    let mut anchors = match fetch_all_anchors_for_avatars(true).await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("❌ [头像批量更新] 获取主播列表失败: {}", e);
            Vec::new()
        }
    };

    // 添加工会官方账号 UID
    let guild_uids = vec![
        ("uid_413748120".to_string(), "VirtuaReal".to_string()),
        ("uid_454673997".to_string(), "PSPlive".to_string()),
    ];
    for (key, name) in &guild_uids {
        let api_url = format!(
            "https://api.bilibili.com/x/web-interface/card?mid={}",
            key.replace("uid_", "")
        );
        if let Ok(resp) = HTTP_CLIENT
            .get(&api_url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
        {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(face) = json
                    .get("data")
                    .and_then(|d| d.get("card"))
                    .and_then(|c| c.get("face"))
                    .and_then(|f| f.as_str())
                {
                    anchors.push((key.clone(), face.to_string()));
                    println!("✅ [工会头像] {} -> {}", name, face);
                }
            }
        }
    }

    let manifest_keys = AVATAR_STORE.entries().await;
    for key in manifest_keys.keys() {
        if anchors.iter().any(|(anchor_key, _)| anchor_key == key) {
            continue;
        }
        // 即使主播列表短时失败，也逐键强制重新请求 face URL，
        // 不复用旧的 48 小时 AVATAR_CACHE URL。
        if let Some(face_url) = force_refresh_face_url_for_cache_key(key).await {
            anchors.push((key.clone(), face_url));
        }
    }

    let mut updated = 0;
    let mut unchanged = 0;
    let mut not_modified = 0;
    let mut fail = 0;

    for (cache_key, face_url) in anchors {
        let outcome = AVATAR_STORE
            .refresh_from_remote(&cache_key, &face_url, |conditional| {
                fetch_remote_avatar_response(&face_url, conditional)
            })
            .await;
        match outcome {
            Ok(RefreshOutcome::Updated) => updated += 1,
            Ok(RefreshOutcome::Unchanged) => unchanged += 1,
            Ok(RefreshOutcome::NotModified) => not_modified += 1,
            Err(error) => {
                fail += 1;
                eprintln!("❌ [头像更新失败] {}: {}", cache_key, error);
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    println!(
        "✅ [头像批量更新] 完成: 更新 {}, 字节不变 {}, 304 {}, 失败 {}",
        updated, unchanged, not_modified, fail
    );
}

/// 批量获取单月份所有日期的attention数据
async fn get_attention_cache_data(
    client: &Client,
    room_id: &str,
    month: &str,
    union: &str,
) -> Result<serde_json::Value, upstream_cache::Error> {
    let key = Key::new(
        Source::from_union(union),
        Endpoint::Attention,
        Some(room_id),
        month,
    )?;
    UPSTREAM_CACHE.get(client, key, "attention").await
}

async fn clear_avatar_cache(room_id: &str) {
    AVATAR_CACHE.write().await.remove(room_id);
}

// 计算两个时间字符串之间的持续时间（分钟）
fn calculate_duration_minutes(start_time: &str, end_time: &str) -> i32 {
    if start_time.is_empty() || end_time.is_empty() {
        return 0;
    }

    // 解析时间字符串，格式如 "2026-01-01 19:55:04"
    let start_naive = NaiveDateTime::parse_from_str(start_time, "%Y-%m-%d %H:%M:%S").ok();
    let end_naive = NaiveDateTime::parse_from_str(end_time, "%Y-%m-%d %H:%M:%S").ok();

    if let (Some(start), Some(end)) = (start_naive, end_naive) {
        let duration = end.signed_duration_since(start);
        duration.num_minutes() as i32
    } else {
        0 // 解析失败时返回0
    }
}

// 将JSON值解析为i64，处理整数和浮点数
fn parse_to_i64(value: Option<&serde_json::Value>) -> i64 {
    match value {
        Some(v) => {
            if let Some(i) = v.as_i64() {
                i
            } else if let Some(f) = v.as_f64() {
                f as i64 // 将浮点数转换为整数（截断小数部分）
            } else if let Some(s) = v.as_str() {
                s.parse::<i64>().unwrap_or(0) // 尝试解析字符串为整数
            } else {
                0
            }
        }
        None => 0,
    }
}

// 安全解析为i64的辅助函数，防止类型错误导致崩溃
fn safe_parse_to_i64(value: Option<&serde_json::Value>) -> i64 {
    match value {
        Some(v) => {
            if v.is_null() {
                0
            } else if let Some(i) = v.as_i64() {
                i
            } else if let Some(u) = v.as_u64() {
                u as i64
            } else if let Some(f) = v.as_f64() {
                // 检查浮点数是否在i64范围内
                if f >= i64::MIN as f64 && f <= i64::MAX as f64 {
                    f as i64
                } else {
                    0 // 超出范围则返回0
                }
            } else if let Some(s) = v.as_str() {
                // 尝试解析字符串为整数，先移除可能的千位分隔符
                let cleaned_str = s.replace(",", "");
                cleaned_str.parse::<i64>().unwrap_or(0)
            } else {
                0 // 无法解析则返回0
            }
        }
        None => 0,
    }
}

// 安全解析为f64的辅助函数
fn safe_parse_to_f64(value: Option<&serde_json::Value>) -> f64 {
    match value {
        Some(v) => {
            if v.is_null() {
                0.0
            } else if let Some(i) = v.as_i64() {
                i as f64
            } else if let Some(u) = v.as_u64() {
                u as f64
            } else if let Some(f) = v.as_f64() {
                f
            } else if let Some(s) = v.as_str() {
                // 尝试解析字符串为浮点数，先移除可能的千位分隔符
                let cleaned_str = s.replace(",", "");
                cleaned_str.parse::<f64>().unwrap_or(0.0)
            } else {
                0.0 // 无法解析则返回0
            }
        }
        None => 0.0,
    }
}

// 安全解析可选f64的辅助函数
fn safe_parse_optional_f64(value: Option<&serde_json::Value>) -> Option<f64> {
    match value {
        Some(v) => {
            if v.is_null() {
                None
            } else if let Some(i) = v.as_i64() {
                Some(i as f64)
            } else if let Some(u) = v.as_u64() {
                Some(u as f64)
            } else if let Some(f) = v.as_f64() {
                Some(f)
            } else if let Some(s) = v.as_str() {
                let cleaned_str = s.replace(",", "");
                cleaned_str.parse::<f64>().ok()
            } else {
                None // 无法解析则返回None
            }
        }
        None => None,
    }
}

// 安全解析可选i64的辅助函数
fn safe_parse_optional_i64(value: Option<&serde_json::Value>) -> Option<i64> {
    match value {
        Some(v) => {
            if v.is_null() {
                None
            } else if let Some(i) = v.as_i64() {
                Some(i)
            } else if let Some(u) = v.as_u64() {
                Some(u as i64)
            } else if let Some(f) = v.as_f64() {
                if f >= i64::MIN as f64 && f <= i64::MAX as f64 {
                    Some(f as i64)
                } else {
                    None // 超出范围则返回None
                }
            } else if let Some(s) = v.as_str() {
                let cleaned_str = s.replace(",", "");
                cleaned_str.parse::<i64>().ok()
            } else {
                None // 无法解析则返回None
            }
        }
        None => None,
    }
}

// 安全格式化月份标题，避免短字符串 panic
fn format_month_title(month: &str) -> String {
    if month.len() >= 6 {
        format!("{}年{}月直播数据", &month[..4], &month[4..])
    } else if month.len() >= 4 {
        format!("{}年直播数据", &month[..4])
    } else {
        format!("{}月直播数据", month)
    }
}

// ==================== 数据模型定义 ====================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub anchor_name: String,
    pub attention: i64,
    pub effective_days: i64,
    pub fans_count: i64,
    pub gift: f64,
    pub guard: f64,
    pub guard_1: i64,
    pub guard_2: i64,
    pub guard_3: i64,
    pub live_duration: String,
    pub live_time: String,
    pub month: String,
    pub room_id: u64,
    pub status: i64,
    pub super_chat: f64,
    pub title: String,
    pub total_revenue: f64,
    pub union: String,
    pub current_concurrency: Option<i64>, // 即时同接人数，开播时显示具体数值，未开播时为null
    pub top1: Option<f64>,
    pub top5: Option<f64>,
    pub top10: Option<f64>,
    pub top1_percent: Option<f64>,
    pub danmaku_total: Option<i64>,
    pub danmaku_captain: Option<i64>,
    pub danmaku_admiral: Option<i64>,
    pub danmaku_governor: Option<i64>,
    pub danmaku_normal: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSession {
    pub start_time: String,
    pub end_time: String,
    pub duration_minutes: i32,
    pub start_guard_1: i64,
    pub start_guard_2: i64,
    pub start_guard_3: i64,
    pub end_guard_1: i64,
    pub end_guard_2: i64,
    pub end_guard_3: i64,
    pub start_fans_count: i64,
    pub end_fans_count: i64,
    pub danmaku_count: i64,
    pub captain_danmaku_count: Option<i64>,
    pub admiral_danmaku_count: Option<i64>,
    pub governor_danmaku_count: Option<i64>,
    pub normal_danmaku_count: Option<i64>,
    pub gift: f64,
    pub guard: f64,
    pub super_chat: f64,
    pub total_revenue: f64,
    pub title: String,
    pub avg_concurrency: Option<f64>,
    pub current_concurrency: Option<i64>,
    pub max_concurrency: Option<i64>,
    pub new_fans_count: i64,
    pub start_attention: Option<i64>,
    pub end_attention: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperChat {
    pub send_time: String,
    pub uname: String,
    pub uid: u64,
    pub price: f64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCResponse {
    pub room_id: u64,
    pub month: String,
    pub list: Vec<SuperChat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub start_time: String,
    pub end_time: String,
    pub duration_minutes: i32,
    pub gift: f64,
    pub guard: f64,
    pub super_chat: f64,
    pub total_revenue: f64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub anchors: Vec<Anchor>,
    pub refresh_time: String,
    pub filter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByMonthResponse {
    pub anchors: Vec<Anchor>,
    pub refresh_time: String,
    pub filter: String,
    pub month: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSessionResponse {
    pub sessions: Vec<LiveSession>,
    pub room_id: String,
    pub queried_user: String,
    pub union: String,
    pub title: String,
    pub refresh_time: String,
}

#[derive(Debug, Deserialize)]
struct FilterQuery {
    filter: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MonthQuery {
    month: Option<String>,
    filter: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LiveSessionQuery {
    room_id: Option<String>,
    union: Option<String>,
    month: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SCQuery {
    room_id: Option<String>,
    union: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AttentionQuery {
    room_id: Option<String>,
    month: Option<String>,
    union: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AvatarQuery {
    room_id: Option<String>,
    uid: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FansOverlapQuery {
    room_id_a: Option<String>,
    room_id_b: Option<String>,
    room_ids: Option<String>, // 逗号分隔的多个房间号，支持批量查询
}

#[derive(Debug, Serialize)]
struct AvatarResponse {
    room_id: u64,
    face: String,
}

type SharedData = Arc<RwLock<HashMap<String, Vec<Anchor>>>>;

// 全局HTTP客户端，复用连接
lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .tcp_keepalive(Duration::from_secs(60))
        .build()
        .expect("Failed to create HTTP client");
}

// 全局请求计数器，用于监控
static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt::init();
    lazy_static::initialize(&UPSTREAM_CACHE);
    // 启动时先完成旧 JPEG → WebP 清单迁移，使批量接口在首个刷新任务完成前
    // 就能从 manifest 恢复数据，而不是返回空列表。
    lazy_static::initialize(&AVATAR_STORE);

    // 创建共享数据存储
    let shared_data: SharedData = Arc::new(RwLock::new(HashMap::new()));

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        pid = std::process::id(),
        "liveshow backend started"
    );

    // 创建安全的CORS中间件层
    let cors_layer = CorsLayer::new()
        .allow_origin([
            "http://localhost:2992"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
            "http://127.0.0.1:2992"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
            "https://localhost:2992"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
            "https://127.0.0.1:2992"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::USER_AGENT,
        ])
        .max_age(Duration::from_secs(86400)); // 24小时缓存预检请求

    // GET /gift* 路由共享同一层10秒HTTP响应缓存。
    let gift_router = Router::new()
        .route("/gift", get(get_anchors))
        .route("/gift/by_month", get(get_anchors_by_month))
        .route("/gift/live_sessions", get(get_live_sessions))
        .route(
            "/gift/live_sessions_with_fans",
            get(get_live_sessions_with_fans),
        )
        .route("/gift/sc", get(get_sc_history))
        .route("/gift/attention", get(get_attention_data))
        .route("/gift/avatar", get(get_avatar))
        .route("/gift/avatar_proxy", get(get_avatar_proxy))
        .route("/gift/avatars/batch", get(get_avatars_batch))
        .route("/gift/fans_overlap", get(get_fans_overlap));

    // 构建Axum应用程序路由器；缓存先挂载为最内层，日志、压缩和CORS随后包裹在外侧。
    let app = Router::new()
        .merge(gift_router)
        .route("/cache/stats", get(get_cache_stats_endpoint))
        .route("/cache/clear", post(clear_cache_endpoint))
        .nest_service(
            "/assets",
            get_service(ServeDir::new("../rust_backend/dist/assets")),
        )
        .route("/logo1.png", get(serve_logo1))
        .route("/favicon.ico", get(favicon))
        .fallback(index_handler)
        // 添加中间件
        .layer(axum::middleware::from_fn(gift_response_cache_middleware))
        .layer(TraceLayer::new_for_http()) // 请求跟踪
        .layer(CompressionLayer::new()) // 响应压缩
        .layer(cors_layer) // CORS
        .with_state(shared_data);

    // 绑定TCP监听器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2992").await.unwrap();
    println!("Server running on http://0.0.0.0:2992");

    tokio::spawn(async {
        let period = Duration::from_secs(4 * 3600);
        let mut interval = tokio::time::interval_at(tokio::time::Instant::now() + period, period);
        batch_download_avatars().await;
        loop {
            interval.tick().await;
            batch_download_avatars().await;
        }
    });

    axum::serve(listener, app).await.unwrap();
}

async fn get_anchors(
    Query(query): Query<FilterQuery>,
    _data: State<SharedData>,
) -> Result<Json<ApiResponse>, StatusCode> {
    // 增加请求计数
    REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);

    let filter = query.filter.unwrap_or_else(|| "all".to_string());
    let anchors = fetch_anchor_data(&filter, None).await?;

    Ok(Json(ApiResponse {
        anchors,
        refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        filter,
    }))
}

async fn get_anchors_by_month(
    Query(query): Query<MonthQuery>,
    _data: State<SharedData>,
) -> Result<Json<ByMonthResponse>, StatusCode> {
    // 增加请求计数
    REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);

    let month = query
        .month
        .unwrap_or_else(|| chrono::Local::now().format("%Y%m").to_string());
    let filter = query.filter.unwrap_or_else(|| "all".to_string());

    let anchors = fetch_anchor_data(&filter, Some(&month)).await?;

    Ok(Json(ByMonthResponse {
        anchors,
        refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        filter,
        month,
    }))
}

async fn get_live_sessions(
    Query(query): Query<LiveSessionQuery>,
    _data: State<SharedData>,
) -> Result<Json<LiveSessionResponse>, StatusCode> {
    // 增加请求计数
    REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);

    let room_id = query.room_id.unwrap_or_default();
    let union = query.union.unwrap_or_else(|| "VirtuaReal".to_string());
    let month = query
        .month
        .unwrap_or_else(|| chrono::Local::now().format("%Y%m").to_string());

    println!(
        "📊 [/gift/live_sessions] room_id={}, union={}, month={}",
        room_id, union, month
    );

    if room_id.is_empty() {
        return Ok(Json(LiveSessionResponse {
            sessions: vec![],
            room_id,
            queried_user: "未知主播".to_string(),
            union,
            title: format_month_title(&month),
            refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }));
    }

    Key::new(
        Source::from_union(&union),
        Endpoint::Sessions,
        Some(&room_id),
        &month,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let base_api_url = if Source::from_union(&union) == Source::Psp {
        "https://psp.qianqiuzy.cn/gift"
    } else {
        "https://vr.qianqiuzy.cn/gift"
    };

    let all_data = match fetch_anchor_data_by_url(base_api_url, "session_name").await {
        Ok(data) => {
            let mut data_with_union = data;
            for item in &mut data_with_union {
                item.union = union.clone();
            }
            data_with_union
        }
        Err(_) => vec![],
    };

    let anchor_data = all_data
        .iter()
        .find(|item| item.room_id.to_string() == room_id);
    let queried_user = if let Some(anchor) = anchor_data {
        anchor.anchor_name.clone()
    } else {
        "未知主播".to_string()
    };

    println!("👤 主播: {}", queried_user);

    let sessions = fetch_live_session_data(&HTTP_CLIENT, &room_id, &union, &month).await?;

    Ok(Json(LiveSessionResponse {
        sessions,
        room_id,
        queried_user,
        union,
        title: format_month_title(&month),
        refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }))
}

async fn get_sc_history(
    Query(query): Query<SCQuery>,
) -> Result<Json<SCResponse>, (StatusCode, String)> {
    // 增加请求计数
    REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);

    let room_id = query.room_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "Missing room_id parameter".to_string(),
        )
    })?;

    // 验证room_id格式
    if !room_id.chars().all(|c| c.is_ascii_digit()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid room_id format".to_string(),
        ));
    }

    let key = Key::new(
        Source::from_union(query.union.as_deref().unwrap_or("VirtuaReal")),
        Endpoint::Sc,
        Some(&room_id),
        &chrono::Local::now().format("%Y%m").to_string(),
    )
    .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid room_id".to_owned()))?;
    let raw_data = UPSTREAM_CACHE
        .uncached(&HTTP_CLIENT, key, "sc")
        .await
        .map_err(|_| (StatusCode::BAD_GATEWAY, "上游SC接口请求失败".to_owned()))?;

    let mut sc_list = Vec::new();

    if let Some(sc_array) = raw_data.as_array() {
        for item in sc_array {
            let send_time = item
                .get("send_time")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uname = item
                .get("uname")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uid = item.get("uid").and_then(|v| v.as_u64()).unwrap_or(0);
            let price = item.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let message = item
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if !message.is_empty() {
                println!("解析到SC消息: {}", message);
            }

            sc_list.push(SuperChat {
                send_time,
                uname,
                uid,
                price,
                message,
            });
        }
    } else if let Some(sc_array) = raw_data.get("list").and_then(|v| v.as_array()) {
        for item in sc_array {
            let send_time = item
                .get("send_time")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uname = item
                .get("uname")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uid = item.get("uid").and_then(|v| v.as_u64()).unwrap_or(0);
            let price = item.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let message = item
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if !message.is_empty() {
                println!("解析到SC消息: {}", message);
            }

            sc_list.push(SuperChat {
                send_time,
                uname,
                uid,
                price,
                message,
            });
        }
    }

    Ok(Json(SCResponse {
        room_id: room_id.parse::<u64>().unwrap_or(0),
        month: chrono::Local::now().format("%Y%m").to_string(),
        list: sc_list,
    }))
}

async fn fetch_anchor_data(filter: &str, month: Option<&str>) -> Result<Vec<Anchor>, StatusCode> {
    if let Some(month) = month {
        upstream_cache::normalize_month(month).ok_or(StatusCode::BAD_REQUEST)?;
    }
    let mut all_data = Vec::new();
    for (name, base) in [
        ("vr", "https://vr.qianqiuzy.cn/gift"),
        ("psp", "https://psp.qianqiuzy.cn/gift"),
    ] {
        if filter != "all" && filter != name {
            continue;
        }
        let url = month
            .map(|month| format!("{base}/by_month?month={month}"))
            .unwrap_or_else(|| base.to_owned());
        let mut data = fetch_anchor_data_by_url(&url, "anchor_list")
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
        for item in &mut data {
            item.union = if name == "vr" {
                "VirtuaReal"
            } else {
                "PSPlive"
            }
            .to_owned();
        }
        all_data.extend(data);
    }
    all_data.iter_mut().for_each(|item| {
        let gift = Decimal::from_str(&item.gift.to_string()).unwrap_or(Decimal::zero());
        let guard = Decimal::from_str(&item.guard.to_string()).unwrap_or(Decimal::zero());
        let sc = Decimal::from_str(&item.super_chat.to_string()).unwrap_or(Decimal::zero());
        item.total_revenue = (gift + guard + sc).to_f64().unwrap_or(0.0);
    });
    all_data.sort_by(|a, b| {
        b.total_revenue
            .partial_cmp(&a.total_revenue)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(all_data)
}

// 缓存统计信息响应结构 - 增强版，包含更详细的信息
#[derive(Serialize)]
struct CacheStatsResponse {
    // 缓存命中率统计
    hits: usize,                 // 缓存命中次数
    misses: usize,               // 缓存未命中次数
    hit_rate: f64,               // 缓存命中率 (0-1)
    hit_rate_percentage: String, // 缓存命中率百分比字符串

    // 内存缓存统计
    current_size: usize,     // 当前缓存大小（字节）
    current_size_mb: String, // 当前缓存大小（MB格式）
    entry_count: usize,      // 缓存条目数
    max_size: usize,         // 最大缓存大小（字节）
    max_size_mb: String,     // 最大缓存大小（MB格式）

    // Attention缓存统计
    attention_entries: usize,     // Attention缓存条目数
    live_sessions_entries: usize, // LiveSessions缓存条目数

    // GET /gift* 最外层10秒HTTP响应缓存统计
    gift_response_cache: GiftResponseCacheStats,
    upstream_cache: upstream_cache::Stats,
}

// 计算大小格式字符串（字节转MB）
fn format_size_mb(bytes: usize) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    format!("{:.2} MB", mb)
}

// 获取缓存统计信息的API端点 - 增强版
async fn get_cache_stats_endpoint() -> Result<Json<CacheStatsResponse>, StatusCode> {
    let upstream_cache = UPSTREAM_CACHE.stats();
    let hits = (upstream_cache.totals.hits
        + upstream_cache.totals.disk_hits
        + upstream_cache.totals.coalesced_hits) as usize;
    let misses = upstream_cache.totals.upstream_requests as usize;
    let hit_rate = if hits + misses == 0 {
        0.0
    } else {
        hits as f64 / (hits + misses) as f64
    };
    let hit_rate_percentage = format!("{:.2}%", hit_rate * 100.0);
    let current_size = upstream_cache.bytes;
    let current_size_mb = format_size_mb(current_size);
    let max_size_mb = format_size_mb(upstream_cache.max_bytes);
    let attention_entries = upstream_cache.attention_entries;
    let live_sessions_entries = upstream_cache.live_sessions_entries;
    let entry_count = upstream_cache.entries;

    Ok(Json(CacheStatsResponse {
        hits,
        misses,
        hit_rate,
        hit_rate_percentage,
        current_size,
        current_size_mb,
        entry_count,
        max_size: upstream_cache.max_bytes,
        max_size_mb,
        attention_entries,
        live_sessions_entries,
        gift_response_cache: gift_response_cache_stats(),
        upstream_cache,
    }))
}

#[derive(Deserialize)]
struct ClearCacheQuery {
    room_id: Option<String>,
    month: Option<String>,
    union: Option<String>,
}

async fn clear_cache_endpoint(
    Query(query): Query<ClearCacheQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let (Some(room), Some(month)) = (query.room_id, query.month) else {
        return Ok(Json(
            serde_json::json!({"ok": false, "message": "需要 room_id 和 month 参数"}),
        ));
    };
    let room = match room.parse::<u64>() {
        Ok(id) if id > 0 && id.to_string() == room => id.to_string(),
        _ => {
            return Ok(Json(
                serde_json::json!({"ok": false, "message": "room_id 必须为正整数"}),
            ))
        }
    };
    if upstream_cache::normalize_month(&month).is_none() {
        return Ok(Json(
            serde_json::json!({"ok": false, "message": "month 必须为有效 YYYYMM"}),
        ));
    }
    let sources = query
        .union
        .as_deref()
        .map(|union| vec![Source::from_union(union)])
        .unwrap_or_else(|| vec![Source::Vr, Source::Psp]);
    UPSTREAM_CACHE
        .clear(&sources, &room, &month)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    clear_avatar_cache(&room).await;
    clear_gift_response_cache();
    Ok(Json(
        serde_json::json!({"ok": true, "message": format!("已清除 {} {} 的内存、磁盘及响应缓存", room, month)}),
    ))
}

async fn fetch_anchor_data_by_url(
    url: &str,
    caller: &str,
) -> Result<Vec<Anchor>, upstream_cache::Error> {
    let source = if url.starts_with("https://psp.qianqiuzy.cn/") {
        Source::Psp
    } else {
        Source::Vr
    };
    let parsed = reqwest::Url::parse(url)?;
    let endpoint = if parsed.path() == "/gift/by_month" {
        Endpoint::Month
    } else {
        Endpoint::Gift
    };
    let month = parsed
        .query_pairs()
        .find(|(key, _)| key == "month")
        .map(|(_, value)| value.into_owned())
        .unwrap_or_else(|| chrono::Local::now().format("%Y%m").to_string());
    let key = Key::new(source, endpoint, None, &month)?;
    let raw_data = UPSTREAM_CACHE.get(&HTTP_CLIENT, key, caller).await?;
    Ok(parse_anchor_data(raw_data))
}

fn parse_anchor_data(raw_data: serde_json::Value) -> Vec<Anchor> {
    let mut anchors = Vec::new();

    if let serde_json::Value::Array(items) = raw_data {
        for item in items {
            let anchor_name = item
                .get("anchor_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let attention = item.get("attention").and_then(|v| v.as_i64()).unwrap_or(0);
            let effective_days = parse_to_i64(item.get("effective_days"));
            let fans_count = parse_to_i64(item.get("fans_count"));
            let gift = item.get("gift").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let guard = item.get("guard").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let guard_1 = parse_to_i64(item.get("guard_1"));
            let guard_2 = parse_to_i64(item.get("guard_2"));
            let guard_3 = parse_to_i64(item.get("guard_3"));
            let live_duration = item
                .get("live_duration")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let live_time = item
                .get("live_time")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let month = item
                .get("month")
                .and_then(|v| v.as_str())
                .unwrap_or(&chrono::Local::now().format("%Y%m").to_string())
                .to_string();
            let room_id = item.get("room_id").and_then(|v| v.as_u64()).unwrap_or(0);
            let status = parse_to_i64(item.get("status"));
            let super_chat = item
                .get("super_chat")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let title = item
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let whale = item.get("whale_dependency");
            let top1 = whale.and_then(|w| w.get("top1")).and_then(|v| v.as_f64());
            let top5 = whale.and_then(|w| w.get("top5")).and_then(|v| v.as_f64());
            let top10 = whale.and_then(|w| w.get("top10")).and_then(|v| v.as_f64());
            let top1_percent = whale
                .and_then(|w| w.get("top1_percent"))
                .and_then(|v| v.as_f64());

            let danmaku = item.get("danmaku");
            let danmaku_total = danmaku
                .and_then(|d| d.get("total"))
                .and_then(|v| v.as_i64());
            let danmaku_captain = danmaku
                .and_then(|d| d.get("captain"))
                .and_then(|v| v.as_i64());
            let danmaku_admiral = danmaku
                .and_then(|d| d.get("admiral"))
                .and_then(|v| v.as_i64());
            let danmaku_governor = danmaku
                .and_then(|d| d.get("governor"))
                .and_then(|v| v.as_i64());
            let danmaku_normal = danmaku
                .and_then(|d| d.get("normal"))
                .and_then(|v| v.as_i64());

            anchors.push(Anchor {
                anchor_name,
                attention,
                effective_days,
                fans_count,
                gift,
                guard,
                guard_1,
                guard_2,
                guard_3,
                live_duration,
                live_time,
                month,
                room_id,
                status,
                super_chat,
                title,
                total_revenue: 0.0,
                union: "".to_string(),
                current_concurrency: item.get("current_concurrency").and_then(|v| v.as_i64()),
                top1,
                top5,
                top10,
                top1_percent,
                danmaku_total,
                danmaku_captain,
                danmaku_admiral,
                danmaku_governor,
                danmaku_normal,
            });
        }
    }

    anchors
}

async fn fetch_live_session_data(
    client: &Client,
    room_id: &str,
    union: &str,
    month: &str,
) -> Result<Vec<LiveSession>, StatusCode> {
    fetch_live_session_data_with_fans_calc(client, room_id, union, month).await
}

async fn fetch_live_session_data_with_fans_calc(
    client: &Client,
    room_id: &str,
    union: &str,
    month: &str,
) -> Result<Vec<LiveSession>, StatusCode> {
    let key = Key::new(
        Source::from_union(union),
        Endpoint::Sessions,
        Some(room_id),
        month,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;
    let raw_data = UPSTREAM_CACHE
        .get(client, key, "live_sessions")
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    parse_live_session_data(raw_data).map_err(|_| StatusCode::BAD_GATEWAY)
}

fn parse_live_session_data(
    raw_data: serde_json::Value,
) -> Result<Vec<LiveSession>, upstream_cache::Error> {
    let sessions_array = if let Some(array) = raw_data.get("sessions").and_then(|v| v.as_array()) {
        array
    } else {
        raw_data
            .as_array()
            .ok_or("Response is not an array or object with sessions field")?
    };

    let mut sessions = Vec::new();

    for (_index, item) in sessions_array.iter().enumerate() {
        // 安全地获取字符串值
        let start_time = item
            .get("start_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let end_time = item
            .get("end_time")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // 计算持续时间（分钟）
        let duration_minutes = calculate_duration_minutes(&start_time, &end_time);

        // 安全地解析数值，防止类型错误导致崩溃
        let start_guard_1 = safe_parse_to_i64(item.get("start_guard_1"));
        let start_guard_2 = safe_parse_to_i64(item.get("start_guard_2"));
        let start_guard_3 = safe_parse_to_i64(item.get("start_guard_3"));
        let end_guard_1 = safe_parse_to_i64(item.get("end_guard_1"));
        let end_guard_2 = safe_parse_to_i64(item.get("end_guard_2"));
        let end_guard_3 = safe_parse_to_i64(item.get("end_guard_3"));
        let start_fans_count = safe_parse_to_i64(item.get("start_fans_count"));
        let end_fans_count = safe_parse_to_i64(item.get("end_fans_count"));
        let danmaku_count = safe_parse_to_i64(item.get("danmaku_count"));
        let captain_danmaku_count = item.get("captain_danmaku_count").and_then(|v| v.as_i64());
        let admiral_danmaku_count = item.get("admiral_danmaku_count").and_then(|v| v.as_i64());
        let governor_danmaku_count = item.get("governor_danmaku_count").and_then(|v| v.as_i64());
        let normal_danmaku_count = item.get("normal_danmaku_count").and_then(|v| v.as_i64());
        let gift = safe_parse_to_f64(item.get("gift"));
        let guard = safe_parse_to_f64(item.get("guard"));
        let super_chat = safe_parse_to_f64(item.get("super_chat"));
        let title = item
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let total_revenue = gift + guard + super_chat;

        // 安全地处理可选字段
        let avg_concurrency = safe_parse_optional_f64(item.get("avg_concurrency"));
        let current_concurrency = safe_parse_optional_i64(item.get("current_concurrency"));
        let max_concurrency = safe_parse_optional_i64(item.get("max_concurrency"));
        let start_attention = item.get("start_attention").and_then(|v| v.as_i64());
        let end_attention = item.get("end_attention").and_then(|v| v.as_i64());

        let live_session = LiveSession {
            start_time,
            end_time,
            duration_minutes,
            start_guard_1,
            start_guard_2,
            start_guard_3,
            end_guard_1,
            end_guard_2,
            end_guard_3,
            start_fans_count,
            end_fans_count,
            danmaku_count,
            captain_danmaku_count,
            admiral_danmaku_count,
            governor_danmaku_count,
            normal_danmaku_count,
            gift,
            guard,
            super_chat,
            total_revenue,
            title,
            avg_concurrency,
            current_concurrency,
            max_concurrency,
            new_fans_count: match (start_attention, end_attention) {
                (Some(start), Some(end)) => end - start,
                _ => -1,
            },
            start_attention,
            end_attention,
        };

        sessions.push(live_session);
    }

    Ok(sessions)
}

/// 获取粉丝数快照数据 - 使用缓存机制
async fn get_attention_data(
    Query(query): Query<AttentionQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let room_id = match query.room_id {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    // 验证 room_id 为正整数
    if !room_id.chars().all(|c| c.is_ascii_digit()) || room_id.parse::<u64>().unwrap_or(0) == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let month = query
        .month
        .unwrap_or_else(|| chrono::Local::now().format("%Y%m").to_string());

    let union = query.union.unwrap_or_else(|| "VirtuaReal".to_string());

    println!(
        "📊 [/gift/attention] room_id={}, month={}, union={}",
        room_id, month, union
    );

    // 使用缓存机制获取数据
    Key::new(
        Source::from_union(&union),
        Endpoint::Attention,
        Some(&room_id),
        &month,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;
    match get_attention_cache_data(&HTTP_CLIENT, &room_id, &month, &union).await {
        Ok(data) => {
            println!(
                "✅ [/gift/attention] 缓存/API调用成功 - room_id={}, month={}",
                room_id, month
            );
            Ok(Json(data))
        }
        Err(e) => {
            eprintln!(
                "❌ [/gift/attention] 获取失败 - room_id={}, month={}, error={}",
                room_id, month, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 获取包含粉丝计算的直播会话（完整模式）
async fn get_live_sessions_with_fans(
    Query(query): Query<LiveSessionQuery>,
    _data: State<SharedData>,
) -> Result<Json<LiveSessionResponse>, StatusCode> {
    // 增加请求计数
    REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);

    let room_id = query.room_id.unwrap_or_default();
    let union = query.union.unwrap_or_else(|| "VirtuaReal".to_string());
    let month = query
        .month
        .unwrap_or_else(|| chrono::Local::now().format("%Y%m").to_string());

    println!(
        "📊 [/gift/live_sessions_with_fans] room_id={}, union={}, month={}",
        room_id, union, month
    );

    if room_id.is_empty() {
        return Ok(Json(LiveSessionResponse {
            sessions: vec![],
            room_id,
            queried_user: "未知主播".to_string(),
            union,
            title: format_month_title(&month),
            refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }));
    }

    Key::new(
        Source::from_union(&union),
        Endpoint::Sessions,
        Some(&room_id),
        &month,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let base_api_url = if Source::from_union(&union) == Source::Psp {
        "https://psp.qianqiuzy.cn/gift"
    } else {
        "https://vr.qianqiuzy.cn/gift"
    };

    let all_data = match fetch_anchor_data_by_url(base_api_url, "session_name").await {
        Ok(data) => {
            let mut data_with_union = data;
            for item in &mut data_with_union {
                item.union = union.clone();
            }
            data_with_union
        }
        Err(_) => vec![],
    };

    let anchor_data = all_data
        .iter()
        .find(|item| item.room_id.to_string() == room_id);
    let queried_user = if let Some(anchor) = anchor_data {
        anchor.anchor_name.clone()
    } else {
        "未知主播".to_string()
    };

    println!("👤 主播: {}", queried_user);

    // 使用完整计算模式
    let sessions =
        fetch_live_session_data_with_fans_calc(&HTTP_CLIENT, &room_id, &union, &month).await?;

    Ok(Json(LiveSessionResponse {
        sessions,
        room_id,
        queried_user,
        union,
        title: format_month_title(&month),
        refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }))
}

async fn get_avatar(Query(query): Query<AvatarQuery>) -> Result<Json<AvatarResponse>, StatusCode> {
    let room_id_str = match query.room_id {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    if !room_id_str.chars().all(|c| c.is_ascii_digit())
        || room_id_str.parse::<u64>().unwrap_or(0) == 0
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    {
        let cache = AVATAR_CACHE.read().await;
        if let Some(entry) = cache.get(&room_id_str) {
            if is_avatar_cache_valid(entry.fetched_at) {
                return Ok(Json(AvatarResponse {
                    room_id: room_id_str.parse::<u64>().unwrap_or(0),
                    face: entry.face_url.clone(),
                }));
            }
        }
    }

    let api_url = format!(
        "https://api.live.bilibili.com/live_user/v1/UserInfo/get_anchor_in_room?roomid={}",
        room_id_str
    );

    let response = match timeout(
        Duration::from_secs(10),
        HTTP_CLIENT
            .get(&api_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Accept", "application/json")
            .send(),
    )
    .await
    {
        Ok(Ok(resp)) => resp,
        _ => return Err(StatusCode::BAD_GATEWAY),
    };

    let json_text = match timeout(Duration::from_secs(10), response.text()).await {
        Ok(Ok(text)) => text,
        _ => return Err(StatusCode::BAD_GATEWAY),
    };

    let raw_data: serde_json::Value = match serde_json::from_str(&json_text) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::BAD_GATEWAY),
    };

    let face = raw_data
        .get("data")
        .and_then(|d| d.get("info"))
        .and_then(|i| i.get("face"))
        .and_then(|f| f.as_str())
        .unwrap_or("")
        .to_string();

    if !face.is_empty() {
        let mut cache = AVATAR_CACHE.write().await;
        cache.insert(
            room_id_str.clone(),
            AvatarCacheEntry {
                face_url: face.clone(),
                image_bytes: None,
                fetched_at: std::time::SystemTime::now(),
            },
        );
    }

    Ok(Json(AvatarResponse {
        room_id: room_id_str.parse::<u64>().unwrap_or(0),
        face,
    }))
}

async fn get_avatar_proxy(
    Query(query): Query<AvatarQuery>,
) -> Result<([(header::HeaderName, String); 2], Vec<u8>), StatusCode> {
    avatar_proxy_from_store(query, &AVATAR_STORE).await
}

async fn avatar_proxy_from_store(
    query: AvatarQuery,
    store: &AvatarStore,
) -> Result<([(header::HeaderName, String); 2], Vec<u8>), StatusCode> {
    let (_cache_key, face_url) = if let Some(uid) = &query.uid {
        if !uid.chars().all(|c| c.is_ascii_digit()) || uid.parse::<u64>().unwrap_or(0) == 0 {
            return Err(StatusCode::BAD_REQUEST);
        }
        let cache_key = format!("uid_{}", uid);
        if let Some(bytes) = store
            .get(&cache_key)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            return Ok(compressed_avatar_response(bytes));
        }
        let cached = {
            let cache = AVATAR_CACHE.read().await;
            cache
                .get(&cache_key)
                .filter(|entry| is_avatar_cache_valid(entry.fetched_at))
                .map(|entry| entry.face_url.clone())
        };
        let face_url = match cached {
            Some(url) => url,
            None => {
                let api_url = format!("https://api.bilibili.com/x/web-interface/card?mid={}", uid);
                let response = match timeout(
                    Duration::from_secs(10),
                    HTTP_CLIENT
                        .get(&api_url)
                        .header(
                            "User-Agent",
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
                        )
                        .header("Accept", "application/json")
                        .send(),
                )
                .await
                {
                    Ok(Ok(resp)) => resp,
                    _ => return Err(StatusCode::BAD_GATEWAY),
                };
                let json_text = match timeout(Duration::from_secs(10), response.text()).await {
                    Ok(Ok(text)) => text,
                    _ => return Err(StatusCode::BAD_GATEWAY),
                };
                let raw_data: serde_json::Value = match serde_json::from_str(&json_text) {
                    Ok(data) => data,
                    Err(_) => return Err(StatusCode::BAD_GATEWAY),
                };
                let face = raw_data
                    .get("data")
                    .and_then(|d| d.get("card"))
                    .and_then(|c| c.get("face"))
                    .and_then(|f| f.as_str())
                    .unwrap_or("")
                    .to_string();
                if face.is_empty() {
                    return Err(StatusCode::NOT_FOUND);
                }
                {
                    let mut cache = AVATAR_CACHE.write().await;
                    cache.insert(
                        cache_key.clone(),
                        AvatarCacheEntry {
                            face_url: face.clone(),
                            image_bytes: None,
                            fetched_at: std::time::SystemTime::now(),
                        },
                    );
                }
                face
            }
        };
        (cache_key, face_url)
    } else if let Some(room_id) = &query.room_id {
        if !room_id.chars().all(|c| c.is_ascii_digit()) || room_id.parse::<u64>().unwrap_or(0) == 0
        {
            return Err(StatusCode::BAD_REQUEST);
        }
        if let Some(bytes) = store
            .get(room_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            return Ok(compressed_avatar_response(bytes));
        }
        let cached = {
            let cache = AVATAR_CACHE.read().await;
            cache
                .get(room_id)
                .filter(|entry| is_avatar_cache_valid(entry.fetched_at))
                .map(|entry| entry.face_url.clone())
        };
        let face_url = match cached {
            Some(url) => url,
            None => {
                let api_url = format!(
                    "https://api.live.bilibili.com/live_user/v1/UserInfo/get_anchor_in_room?roomid={}",
                    room_id
                );
                let response = match timeout(
                    Duration::from_secs(10),
                    HTTP_CLIENT
                        .get(&api_url)
                        .header(
                            "User-Agent",
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
                        )
                        .header("Accept", "application/json")
                        .send(),
                )
                .await
                {
                    Ok(Ok(resp)) => resp,
                    _ => return Err(StatusCode::BAD_GATEWAY),
                };
                let json_text = match timeout(Duration::from_secs(10), response.text()).await {
                    Ok(Ok(text)) => text,
                    _ => return Err(StatusCode::BAD_GATEWAY),
                };
                let raw_data: serde_json::Value = match serde_json::from_str(&json_text) {
                    Ok(data) => data,
                    Err(_) => return Err(StatusCode::BAD_GATEWAY),
                };
                let face = raw_data
                    .get("data")
                    .and_then(|d| d.get("info"))
                    .and_then(|i| i.get("face"))
                    .and_then(|f| f.as_str())
                    .unwrap_or("")
                    .to_string();
                if face.is_empty() {
                    return Err(StatusCode::NOT_FOUND);
                }
                {
                    let mut cache = AVATAR_CACHE.write().await;
                    cache.insert(
                        room_id.clone(),
                        AvatarCacheEntry {
                            face_url: face.clone(),
                            image_bytes: None,
                            fetched_at: std::time::SystemTime::now(),
                        },
                    );
                }
                face
            }
        };
        (room_id.clone(), face_url)
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let _outcome = store
        .refresh_from_remote(&_cache_key, &face_url, |conditional| {
            fetch_remote_avatar_response(&face_url, conditional)
        })
        .await
        .map_err(|error| {
            eprintln!("❌ [头像代理下载压缩失败] {}: {}", _cache_key, error);
            StatusCode::BAD_GATEWAY
        })?;
    let bytes = store
        .get(&_cache_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_GATEWAY)?;
    Ok(compressed_avatar_response(bytes))
}

async fn get_avatars_batch() -> Json<serde_json::Value> {
    let compressed = match AVATAR_STORE.snapshot().await {
        Ok(compressed) => compressed,
        Err(error) => {
            eprintln!("❌ [头像批量清单读取失败] {}", error);
            BTreeMap::new()
        }
    };
    let avatars = compressed
        .into_iter()
        .map(|(key, bytes)| {
            let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
            (
                key,
                serde_json::Value::String(format!("data:image/webp;base64,{}", b64)),
            )
        })
        .collect::<serde_json::Map<_, _>>();

    Json(serde_json::json!({
        "avatars": avatars,
        "count": avatars.len()
    }))
}

fn compressed_avatar_response(bytes: Vec<u8>) -> ([(header::HeaderName, String); 2], Vec<u8>) {
    (
        [
            (header::CONTENT_TYPE, "image/webp".to_string()),
            (header::CACHE_CONTROL, "max-age=86400".to_string()),
        ],
        bytes,
    )
}

async fn serve_logo1() -> impl IntoResponse {
    match std::fs::read("../rust_backend/dist/logo1.png") {
        Ok(data) => ([(header::CONTENT_TYPE, "image/png".to_string())], data).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn index_handler() -> Html<String> {
    match std::fs::read_to_string("../rust_backend/dist/index.html") {
        Ok(content) => Html(content),
        Err(e) => {
            eprintln!("警告: 无法读取 index.html: {}", e);
            Html(format!(
                "<h1>500 - 页面未找到</h1><p>index.html 不存在: {}</p>",
                e
            ))
        }
    }
}

async fn favicon() -> impl axum::response::IntoResponse {
    match std::fs::read("../rust_backend/dist/favicon.ico") {
        Ok(data) => axum::response::Response::builder()
            .header("content-type", "image/x-icon")
            .body(axum::body::Body::from(data))
            .unwrap_or_else(|e| {
                eprintln!("警告: 构建 favicon 响应失败: {}", e);
                axum::response::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(axum::body::Body::empty())
                    .unwrap()
            }),
        Err(e) => {
            eprintln!("警告: 无法读取 favicon.ico: {}", e);
            axum::response::Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(axum::body::Body::empty())
                .unwrap()
        }
    }
}

async fn get_fans_overlap(
    Query(query): Query<FansOverlapQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // 解析房间号列表：优先使用 room_ids（逗号分隔），兼容旧的 room_id_a + room_id_b
    let room_ids: Vec<String> = if let Some(ids_str) = &query.room_ids {
        ids_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        let mut ids = Vec::new();
        if let Some(a) = &query.room_id_a {
            if !a.is_empty() {
                ids.push(a.clone());
            }
        }
        if let Some(b) = &query.room_id_b {
            if !b.is_empty() {
                ids.push(b.clone());
            }
        }
        ids
    };

    if room_ids.len() < 2 {
        eprintln!("⚠️ [fans_overlap] 至少需要2个房间号，当前: {:?}", room_ids);
        return Err(StatusCode::BAD_REQUEST);
    }

    // 为每个唯一房间号获取粉丝数据（每个房间只请求一次）
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
    let mut fans_cache: HashMap<String, serde_json::Value> = HashMap::new();

    // 去重并并发请求所有房间
    let unique_ids: Vec<String> = {
        let mut seen = std::collections::HashSet::new();
        room_ids
            .iter()
            .filter(|id| seen.insert((*id).clone()))
            .cloned()
            .collect()
    };

    // 并发获取所有房间的粉丝数据（使用 JoinSet）
    let mut join_set = tokio::task::JoinSet::new();
    for room_id in unique_ids.iter() {
        let url = format!("https://fans.harei.cn/fans?room_id={}", room_id);
        let rid = room_id.clone();
        join_set.spawn(async move {
            let resp = HTTP_CLIENT.get(&url).header("User-Agent", ua).send().await;
            match resp {
                Ok(r) => {
                    let json: Result<serde_json::Value, _> = r.json().await;
                    (rid, json.ok())
                }
                Err(e) => {
                    eprintln!("⚠️ [fans_overlap] 请求房间{}失败: {}", rid, e);
                    (rid, None)
                }
            }
        });
    }

    while let Some(join_result) = join_set.join_next().await {
        match join_result {
            Ok((room_id, json_opt)) => {
                if let Some(json) = json_opt {
                    if json.get("code").and_then(|c| c.as_i64()).unwrap_or(-1) != 0 {
                        eprintln!("⚠️ [fans_overlap] 跳过房间{}（上游返回错误）", room_id);
                        continue;
                    }
                    fans_cache.insert(room_id, json);
                } else {
                    eprintln!("⚠️ [fans_overlap] 跳过房间{}（获取失败）", room_id);
                    continue;
                }
            }
            Err(e) => {
                eprintln!("⚠️ [fans_overlap] 跳过任务（JoinSet错误: {}）", e);
                continue;
            }
        }
    }

    // 生成所有 C(n,2) 配对并计算重合度（仅使用有数据的房间）
    let mut pairs = Vec::new();
    let valid_ids: Vec<&String> = room_ids
        .iter()
        .filter(|id| fans_cache.contains_key(id.as_str()))
        .collect();
    for i in 0..valid_ids.len() {
        for j in (i + 1)..valid_ids.len() {
            let a_id = valid_ids[i];
            let b_id = valid_ids[j];
            let json_a = fans_cache.get(a_id.as_str()).unwrap();
            let json_b = fans_cache.get(b_id.as_str()).unwrap();
            let pair = compute_pair_overlap(a_id, b_id, json_a, json_b);
            pairs.push(pair);
        }
    }

    let skipped: Vec<&String> = room_ids
        .iter()
        .filter(|id| !fans_cache.contains_key(id.as_str()))
        .collect();
    Ok(Json(serde_json::json!({
        "success": true,
        "pairs": pairs,
        "skipped": skipped,
    })))
}

/// 计算两个房间之间的粉丝重合度（提取为独立函数以复用逻辑）
fn compute_pair_overlap(
    a_room_id: &str,
    b_room_id: &str,
    json_a: &serde_json::Value,
    json_b: &serde_json::Value,
) -> serde_json::Value {
    // 提取 medal 映射: uid_string -> medal_level
    let medal_a: HashMap<String, i64> = json_a
        .get("medal")
        .and_then(|m| serde_json::from_value(m.clone()).ok())
        .unwrap_or_default();
    let medal_b: HashMap<String, i64> = json_b
        .get("medal")
        .and_then(|m| serde_json::from_value(m.clone()).ok())
        .unwrap_or_default();

    // 提取 guard_level 映射
    let guard_a: HashMap<String, i64> = json_a
        .get("guard_level")
        .and_then(|m| serde_json::from_value(m.clone()).ok())
        .unwrap_or_default();
    let guard_b: HashMap<String, i64> = json_b
        .get("guard_level")
        .and_then(|m| serde_json::from_value(m.clone()).ok())
        .unwrap_or_default();

    let a_total = medal_a.len() as i64;
    let b_total = medal_b.len() as i64;

    // 计算交集
    let keys_a: std::collections::HashSet<&String> = medal_a.keys().collect();
    let keys_b: std::collections::HashSet<&String> = medal_b.keys().collect();
    let intersection: Vec<&String> = keys_a.intersection(&keys_b).cloned().collect();
    let intersection_count = intersection.len() as i64;

    let a_in_b_percent = if a_total > 0 {
        (intersection_count as f64 / a_total as f64 * 100.0 * 100.0).round() / 100.0
    } else {
        0.0
    };
    let b_in_a_percent = if b_total > 0 {
        (intersection_count as f64 / b_total as f64 * 100.0 * 100.0).round() / 100.0
    } else {
        0.0
    };

    // 等级分布桶: 只统计交集粉丝，用各自主播的牌子等级
    // 标准逻辑：A&B 1~10级 = A中拥有B牌子且A的牌子等级为1~10级的人数
    let buckets = [
        ("1~10", 1i64, 10i64),
        ("11~20", 11, 20),
        ("21~30", 21, 30),
        ("31~40", 31, 40),
        ("41~50", 41, 50),
        ("51~60", 51, 60),
    ];
    let a_levels: Vec<serde_json::Value> = buckets
        .iter()
        .map(|(label, lo, hi)| {
            let count = intersection
                .iter()
                .filter(|uid| {
                    medal_a
                        .get(uid.as_str())
                        .map_or(false, |&v| v >= *lo && v <= *hi)
                })
                .count() as i64;
            serde_json::json!({"range": label, "count": count})
        })
        .collect();
    let b_levels: Vec<serde_json::Value> = buckets
        .iter()
        .map(|(label, lo, hi)| {
            let count = intersection
                .iter()
                .filter(|uid| {
                    medal_b
                        .get(uid.as_str())
                        .map_or(false, |&v| v >= *lo && v <= *hi)
                })
                .count() as i64;
            serde_json::json!({"range": label, "count": count})
        })
        .collect();

    // 舰长统计: 只统计交集粉丝，用各自主播的guard_level
    // 标准逻辑：A&B 舰长 = A中拥有B牌子且为A的舰长的人数
    let guard_tiers = [("舰长", 3i64), ("提督", 2), ("总督", 1)];
    let a_guard: Vec<serde_json::Value> = guard_tiers
        .iter()
        .map(|(label, level)| {
            let count = intersection
                .iter()
                .filter(|uid| guard_a.get(uid.as_str()).copied() == Some(*level))
                .count() as i64;
            serde_json::json!({"tier": label, "count": count})
        })
        .collect();
    let b_guard: Vec<serde_json::Value> = guard_tiers
        .iter()
        .map(|(label, level)| {
            let count = intersection
                .iter()
                .filter(|uid| guard_b.get(uid.as_str()).copied() == Some(*level))
                .count() as i64;
            serde_json::json!({"tier": label, "count": count})
        })
        .collect();

    // 共同上舰: 交集UID中双方guard_level都不为0且相等
    // 标准逻辑：guardA(fan) == guardB(fan) != 0
    let shared_guard = intersection
        .iter()
        .filter(|uid| {
            let ga = guard_a.get(uid.as_str()).copied().unwrap_or(0);
            let gb = guard_b.get(uid.as_str()).copied().unwrap_or(0);
            ga > 0 && ga == gb
        })
        .count() as i64;

    serde_json::json!({
        "a_room_id": a_room_id,
        "b_room_id": b_room_id,
        "a_total": a_total,
        "b_total": b_total,
        "intersection": intersection_count,
        "a_in_b_percent": a_in_b_percent,
        "b_in_a_percent": b_in_a_percent,
        "a_levels": a_levels,
        "b_levels": b_levels,
        "a_guard": a_guard,
        "b_guard": b_guard,
        "shared_guard": shared_guard,
    })
}

#[cfg(test)]
mod cache_api_tests {
    use super::*;

    #[tokio::test]
    async fn avatar_proxy_manifest_hit_returns_webp_mime() {
        let directory = std::env::temp_dir().join(format!(
            "liveshow-avatar-proxy-test-{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&directory).unwrap();
        let store = AvatarStore::open(&directory).unwrap();
        let mut png = Vec::new();
        image::RgbaImage::new(40, 40)
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .unwrap();
        let webp = avatar_cache::compress_avatar(&png).unwrap();
        store.put_local("123", &webp, None).await.unwrap();

        let (headers, bytes) = avatar_proxy_from_store(
            AvatarQuery {
                room_id: Some("123".into()),
                uid: None,
            },
            &store,
        )
        .await
        .unwrap();
        assert_eq!(headers[0].0, header::CONTENT_TYPE);
        assert_eq!(headers[0].1, "image/webp");
        assert_eq!(headers[1].0, header::CACHE_CONTROL);
        assert_eq!(headers[1].1, "max-age=86400");
        assert_eq!(bytes, webp);
        avatar_cache::validate_webp(&bytes).unwrap();
        drop(store);
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[tokio::test]
    async fn invalid_refresh_and_month_do_not_invalidate_or_contact_upstream() {
        let before = UPSTREAM_CACHE.stats();
        for (room, month) in [("../escape", "202610"), ("1", "202613"), ("0", "202610")] {
            let result = clear_cache_endpoint(Query(ClearCacheQuery {
                room_id: Some(room.into()),
                month: Some(month.into()),
                union: None,
            }))
            .await
            .unwrap();
            assert_eq!(result.0["ok"], false);
        }
        let result = get_attention_data(Query(AttentionQuery {
            room_id: Some("1".into()),
            month: Some("202613".into()),
            union: None,
        }))
        .await;
        assert!(matches!(result, Err(StatusCode::BAD_REQUEST)));
        let after = UPSTREAM_CACHE.stats();
        assert_eq!(before.invalidations, after.invalidations);
        assert_eq!(
            before.totals.upstream_requests,
            after.totals.upstream_requests
        );
    }

    #[tokio::test]
    async fn stats_keep_numeric_compatibility_fields_and_expose_real_source_metrics() {
        let stats = serde_json::to_value(get_cache_stats_endpoint().await.unwrap().0).unwrap();
        for field in [
            "hits",
            "misses",
            "current_size",
            "entry_count",
            "max_size",
            "attention_entries",
            "live_sessions_entries",
        ] {
            assert!(stats[field].is_u64(), "{field}");
        }
        assert!(stats["hit_rate"].is_number());
        assert!(stats["gift_response_cache"].is_object());
        assert_eq!(
            stats["upstream_cache"]["version"],
            env!("CARGO_PKG_VERSION")
        );
        assert!(stats["upstream_cache"]["totals"]["upstream_requests"].is_u64());
    }
}
