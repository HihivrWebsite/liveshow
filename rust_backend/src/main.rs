use axum::{
    extract::{Query, State},
    response::Html,
    routing::{get, get_service},
    Json, Router,
};
use axum::http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    compression::CompressionLayer,
    trace::TraceLayer,
};
use tracing_subscriber;
use rust_decimal::prelude::*;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;
use std::sync::atomic::{AtomicUsize, Ordering};
use lru::LruCache;
use std::hash::Hash;
use std::num::NonZeroUsize;

// ==================== 缓存相关定义 ====================

// 缓存条目结构，包含数据和大小估算
#[derive(Clone)]
struct CacheEntry {
    data: Vec<Anchor>,
    size_estimate: usize,  // 大小估算（字节）
    timestamp: std::time::SystemTime,  // 缓存时间戳
}

// 全局缓存，使用LRU策略，限制为5GB (5 * 1024 * 1024 * 1024 bytes)
const MAX_CACHE_SIZE: usize = 5 * 1024 * 1024 * 1024; // 5GB

// 全局缓存实例
lazy_static::lazy_static! {
    static ref GLOBAL_CACHE: Arc<RwLock<CacheManager>> = Arc::new(RwLock::new(CacheManager::new()));
}

// 缓存管理器
struct CacheManager {
    cache: LruCache<String, CacheEntry>,
    current_size: usize,
    hit_count: usize,      // 缓存命中次数
    miss_count: usize,     // 缓存未命中次数
}

impl CacheManager {
    fn new() -> Self {
        // 限制缓存条目数量，配合大小限制
        let max_entries = 1000; // 合理的条目数量限制
        let capacity = std::num::NonZeroUsize::new(max_entries).unwrap();
        CacheManager {
            cache: LruCache::new(capacity),
            current_size: 0,
            hit_count: 0,
            miss_count: 0,
        }
    }

    fn get(&mut self, key: &str) -> Option<&CacheEntry> {
        if let Some(entry) = self.cache.get(key) {
            self.hit_count += 1;
            Some(entry)
        } else {
            self.miss_count += 1;
            None
        }
    }

    fn put(&mut self, key: String, entry: CacheEntry) -> bool {
        // 检查是否会超出大小限制
        if self.current_size + entry.size_estimate > MAX_CACHE_SIZE {
            // 如果超出限制，先清理一些旧条目
            self.evict_old_entries(entry.size_estimate);
        }

        // 检查是否仍然超出限制
        if entry.size_estimate <= MAX_CACHE_SIZE {
            // 移除旧条目（如果存在）并更新大小
            let old_entry_size = if let Some(old_entry) = self.cache.put(key, entry.clone()) {
                old_entry.size_estimate
            } else {
                0
            };

            // 更新当前缓存大小
            self.current_size = self.current_size.saturating_sub(old_entry_size);
            self.current_size += entry.size_estimate;
            true
        } else {
            false // 单个条目就超过了大小限制
        }
    }

    fn evict_old_entries(&mut self, new_entry_size: usize) {
        // 清理旧条目直到有足够的空间
        while self.current_size + new_entry_size > MAX_CACHE_SIZE {
            if let Some((_key, entry)) = self.cache.pop_lru() {
                self.current_size = self.current_size.saturating_sub(entry.size_estimate);
                if self.cache.len() == 0 {
                    break; // 如果缓存为空，跳出循环
                }
            } else {
                break; // 没有更多条目可以清除
            }
        }
    }

    fn get_current_size(&self) -> usize {
        self.current_size
    }

    fn get_entry_count(&self) -> usize {
        self.cache.len()
    }

    fn get_hit_count(&self) -> usize {
        self.hit_count
    }

    fn get_miss_count(&self) -> usize {
        self.miss_count
    }

    fn get_hit_rate(&self) -> f64 {
        let total_requests = self.hit_count + self.miss_count;
        if total_requests == 0 {
            0.0
        } else {
            self.hit_count as f64 / total_requests as f64
        }
    }

    fn reset_stats(&mut self) {
        self.hit_count = 0;
        self.miss_count = 0;
    }
}

// 获取缓存实例的函数
async fn get_cache_entry(key: &str) -> Option<CacheEntry> {
    let mut cache = GLOBAL_CACHE.write().await;
    if let Some(entry) = cache.get(key) {
        Some(entry.clone())
    } else {
        None
    }
}

async fn put_cache_entry(key: String, entry: CacheEntry) -> bool {
    GLOBAL_CACHE.write().await.put(key, entry)
}

async fn get_cache_stats() -> (usize, usize) {
    let cache = GLOBAL_CACHE.read().await;
    (cache.get_current_size(), cache.get_entry_count())
}

// 获取缓存命中率统计
async fn get_cache_hit_stats() -> (usize, usize, f64) {
    let cache = GLOBAL_CACHE.read().await;
    let hits = cache.get_hit_count();
    let misses = cache.get_miss_count();
    let hit_rate = cache.get_hit_rate();
    (hits, misses, hit_rate)
}

// 重置缓存统计
async fn reset_cache_stats() {
    GLOBAL_CACHE.write().await.reset_stats();
}

// ==================== 数据模型定义 ====================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub anchor_name: String,
    pub attention: i64,
    pub effective_days: i32,
    pub fans_count: i32,
    pub gift: f64,
    pub guard: f64,
    pub guard_1: i32,
    pub guard_2: i32,
    pub guard_3: i32,
    pub live_duration: String,
    pub live_time: String,
    pub month: String,
    pub room_id: u64,
    pub status: i32,
    pub super_chat: f64,
    pub title: String,
    pub total_revenue: f64,
    pub union: String,
    pub current_concurrency: Option<i32>,  // 即时同接人数，开播时显示具体数值，未开播时为null
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSession {
    pub start_time: String,
    pub end_time: String,
    pub duration_minutes: i32,
    pub start_guard_1: i32,
    pub start_guard_2: i32,
    pub start_guard_3: i32,
    pub end_guard_1: i32,
    pub end_guard_2: i32,
    pub end_guard_3: i32,
    pub start_fans_count: i32,
    pub end_fans_count: i32,
    pub danmaku_count: i32,
    pub gift: f64,
    pub guard: f64,
    pub super_chat: f64,
    pub total_revenue: f64,
    pub title: String,
    pub avg_concurrency: Option<f64>,      // 平均同接人数
    pub current_concurrency: Option<i32>,  // 即时同接人数
    pub max_concurrency: Option<i32>,      // 最高同接人数
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

    // 创建共享数据存储
    let shared_data: SharedData = Arc::new(RwLock::new(HashMap::new()));

    // 创建安全的CORS中间件层
    let cors_layer = CorsLayer::new()
        .allow_origin([
            "http://localhost:2992".parse::<axum::http::HeaderValue>().unwrap(),
            "http://127.0.0.1:2992".parse::<axum::http::HeaderValue>().unwrap(),
            "https://localhost:2992".parse::<axum::http::HeaderValue>().unwrap(),
            "https://127.0.0.1:2992".parse::<axum::http::HeaderValue>().unwrap(),
        ])
        .allow_methods([axum::http::Method::GET, axum::http::Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::USER_AGENT,
        ])
        .max_age(Duration::from_secs(86400)); // 24小时缓存预检请求

    // 构建Axum应用程序路由器
    let app = Router::new()
        .route("/gift", get(get_anchors))
        .route("/gift/by_month", get(get_anchors_by_month))
        .route("/gift/live_sessions", get(get_live_sessions))
        .route("/gift/sc", get(get_sc_history))
        .route("/cache/stats", get(get_cache_stats_endpoint))
        .nest_service("/assets", get_service(ServeDir::new("../rust_backend/dist/assets")))
        .route("/favicon.ico", get(favicon))
        .fallback(index_handler)
        // 添加中间件
        .layer(TraceLayer::new_for_http()) // 请求跟踪
        .layer(CompressionLayer::new())    // 响应压缩
        .layer(cors_layer)                 // CORS
        .with_state(shared_data);

    // 绑定TCP监听器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2992").await.unwrap();
    println!("Server running on http://0.0.0.0:2992");

    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}

async fn get_anchors(
    Query(query): Query<FilterQuery>,
    _data: State<SharedData>,
) -> Result<Json<ApiResponse>, StatusCode> {
    // 增加请求计数
    REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);
    
    let filter = query.filter.unwrap_or_else(|| "all".to_string());
    let anchors = fetch_anchor_data(&filter, None).await;

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
    
    let month = query.month.unwrap_or_else(|| chrono::Utc::now().format("%Y%m").to_string());
    let filter = query.filter.unwrap_or_else(|| "all".to_string());

    let anchors = fetch_anchor_data(&filter, Some(&month)).await;

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
    let month = query.month.unwrap_or_else(|| chrono::Utc::now().format("%Y%m").to_string());

    println!("Received request for room_id: {}, union: {}, month: {}", room_id, union, month);

    if room_id.is_empty() {
        return Ok(Json(LiveSessionResponse {
            sessions: vec![],
            room_id,
            queried_user: "未知主播".to_string(),
            union,
            title: format!("{}年{}月直播数据", &month[..4], &month[4..]),
            refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }));
    }

    let base_api_url = if union == "PSPlive" {
        "https://psp.qianqiuzy.cn/gift"
    } else {
        "https://vr.qianqiuzy.cn/gift"
    };

    let all_data = match fetch_anchor_data_by_url(base_api_url).await {
        Ok(data) => {
            let mut data_with_union = data;
            for item in &mut data_with_union {
                item.union = union.clone();
            }
            data_with_union
        },
        Err(_) => vec![],
    };

    let anchor_data = all_data.iter().find(|item| item.room_id.to_string() == room_id);
    let queried_user = if let Some(anchor) = anchor_data {
        anchor.anchor_name.clone()
    } else {
        "未知主播".to_string()
    };

    println!("Queried user: {}", queried_user);

    let sessions = fetch_live_session_data(&room_id, &union, &month).await;

    Ok(Json(LiveSessionResponse {
        sessions,
        room_id,
        queried_user,
        union,
        title: format!("{}年{}月直播数据", &month[..4], &month[4..]),
        refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }))
}

async fn get_sc_history(
    Query(query): Query<SCQuery>,
) -> Result<Json<SCResponse>, (StatusCode, String)> {
    // 增加请求计数
    REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);
    
    let room_id = query.room_id.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, "Missing room_id parameter".to_string())
    })?;

    // 验证room_id格式
    if !room_id.chars().all(|c| c.is_ascii_digit()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid room_id format".to_string()));
    }

    let base_url = if query.union.as_deref().unwrap_or("VirtuaReal") == "PSPlive" {
        "https://psp.qianqiuzy.cn"
    } else {
        "https://vr.qianqiuzy.cn"
    };

    let api_url = format!("{}/gift/sc?room_id={}", base_url, room_id);

    // 使用全局HTTP客户端
    let response = timeout(
        Duration::from_secs(15),
        HTTP_CLIENT.get(&api_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .header("Accept", "application/json")
            .send()
    ).await.map_err(|_| (StatusCode::REQUEST_TIMEOUT, "Request timeout".to_string()))?;

    let response = response.map_err(|e| {
        eprintln!("API请求失败: {}, URL: {}", e, api_url);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("API请求失败: {}", e))
    })?;

    if !response.status().is_success() {
        eprintln!("API响应错误: {} for URL: {}", response.status(), api_url);
        return Err((StatusCode::BAD_GATEWAY, "上游API响应错误".to_string()));
    }

    let json_text = timeout(
        Duration::from_secs(10),
        response.text()
    ).await.map_err(|_| (StatusCode::REQUEST_TIMEOUT, "Response timeout".to_string()))?;

    let json_text = json_text.map_err(|e| {
        eprintln!("读取响应体失败: {}, URL: {}", e, api_url);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("读取响应体失败: {}", e))
    })?;

    let raw_data: serde_json::Value = serde_json::from_str(&json_text)
        .map_err(|e| {
            eprintln!("JSON解析失败: {}, 响应内容: {:.200}", e, &json_text);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("JSON解析失败: {}", e))
        })?;

    let mut sc_list = Vec::new();

    if let Some(sc_array) = raw_data.as_array() {
        for item in sc_array {
            let send_time = item.get("send_time")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uname = item.get("uname")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uid = item.get("uid")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let price = item.get("price")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let message = item.get("message")
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
            let send_time = item.get("send_time")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uname = item.get("uname")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let uid = item.get("uid")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let price = item.get("price")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let message = item.get("message")
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
        month: chrono::Utc::now().format("%Y%m").to_string(),
        list: sc_list,
    }))
}

async fn fetch_anchor_data(filter: &str, month: Option<&str>) -> Vec<Anchor> {
    // 生成缓存键
    let cache_key = format!("{}_{}", filter, month.unwrap_or("current"));

    // 检查是否为过去月份的数据
    let is_past_month = if let Some(requested_month) = month {
        // 获取当前月份
        let current_date = chrono::Utc::now();
        let current_month = current_date.format("%Y%m").to_string();

        // 比较请求的月份是否早于当前月份
        requested_month < current_month.as_str()
    } else {
        false // 如果没有指定月份，则为当前数据，不缓存
    };

    // 如果是过去月份的数据，尝试从缓存获取
    if is_past_month {
        if let Some(cached_entry) = get_cache_entry(&cache_key).await {
            println!("从缓存获取数据: {}", cache_key);
            return cached_entry.data;
        }
    }

    let mut all_data = Vec::new();

    if filter == "all" || filter == "vr" {
        let vr_api_url = if let Some(m) = month {
            format!("https://vr.qianqiuzy.cn/gift/by_month?month={}", m)
        } else {
            "https://vr.qianqiuzy.cn/gift".to_string()
        };

        if let Ok(mut vr_data) = fetch_external_api(&HTTP_CLIENT, &vr_api_url).await {
            for item in &mut vr_data {
                item.union = "VirtuaReal".to_string();
            }
            all_data.extend(vr_data);
        }
    }

    if filter == "all" || filter == "psp" {
        let psp_api_url = if let Some(m) = month {
            format!("https://psp.qianqiuzy.cn/gift/by_month?month={}", m)
        } else {
            "https://psp.qianqiuzy.cn/gift".to_string()
        };

        if let Ok(mut psp_data) = fetch_external_api(&HTTP_CLIENT, &psp_api_url).await {
            for item in &mut psp_data {
                item.union = "PSPlive".to_string();
            }
            all_data.extend(psp_data);
        }
    }

    // 使用更高效的计算方法
    all_data.iter_mut().for_each(|item| {
        let gift = Decimal::from_str(&item.gift.to_string()).unwrap_or(Decimal::zero());
        let guard = Decimal::from_str(&item.guard.to_string()).unwrap_or(Decimal::zero());
        let super_chat = Decimal::from_str(&item.super_chat.to_string()).unwrap_or(Decimal::zero());
        item.total_revenue = (gift + guard + super_chat).to_f64().unwrap_or(0.0);
    });

    // 使用更高效的排序算法
    all_data.sort_by(|a, b| {
        b.total_revenue
            .partial_cmp(&a.total_revenue)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // 如果是过去月份的数据，将其缓存
    if is_past_month {
        // 估算数据大小（简单估算，实际项目中可能需要更精确的估算）
        let size_estimate = std::mem::size_of_val(&all_data);

        let cache_entry = CacheEntry {
            data: all_data.clone(),
            size_estimate,
            timestamp: std::time::SystemTime::now(),
        };

        let success = put_cache_entry(cache_key.clone(), cache_entry).await;
        if success {
            let (current_size, entry_count) = get_cache_stats().await;
            println!("数据已缓存: {}, 当前缓存大小: {} bytes, 条目数: {}, 缓存键: {}",
                     success, current_size, entry_count, cache_key);
        } else {
            println!("缓存失败，数据可能过大");
        }
    }

    all_data
}

// 缓存统计信息响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheStatsResponse {
    hits: usize,
    misses: usize,
    hit_rate: f64,
    current_size: usize,
    entry_count: usize,
    max_size: usize,
}

// 获取缓存统计信息的API端点
async fn get_cache_stats_endpoint() -> Result<Json<CacheStatsResponse>, StatusCode> {
    let (hits, misses, hit_rate) = get_cache_hit_stats().await;
    let (current_size, entry_count) = get_cache_stats().await;

    Ok(Json(CacheStatsResponse {
        hits,
        misses,
        hit_rate,
        current_size,
        entry_count,
        max_size: MAX_CACHE_SIZE,
    }))
}

async fn fetch_anchor_data_by_url(url: &str) -> Result<Vec<Anchor>, Box<dyn std::error::Error + Send + Sync>> {
    let data = fetch_external_api(&HTTP_CLIENT, url).await?;
    Ok(data)
}

async fn fetch_external_api(client: &Client, api_url: &str) -> Result<Vec<Anchor>, Box<dyn std::error::Error + Send + Sync>> {
    let response = timeout(
        Duration::from_secs(10),
        client.get(api_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
    ).await??;

    let json_text = timeout(
        Duration::from_secs(10),
        response.text()
    ).await??;

    let raw_data: serde_json::Value = serde_json::from_str(&json_text)?;

    let mut anchors = Vec::new();

    if let serde_json::Value::Array(items) = raw_data {
        for item in items {
            let anchor_name = item.get("anchor_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let attention = item.get("attention").and_then(|v| v.as_i64()).unwrap_or(0);
            let effective_days = item.get("effective_days").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let fans_count = item.get("fans_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let gift = item.get("gift").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let guard = item.get("guard").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let guard_1 = item.get("guard_1").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let guard_2 = item.get("guard_2").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let guard_3 = item.get("guard_3").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let live_duration = item.get("live_duration").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let live_time = item.get("live_time").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let month = item.get("month").and_then(|v| v.as_str()).unwrap_or(&chrono::Utc::now().format("%Y%m").to_string()).to_string();
            let room_id = item.get("room_id").and_then(|v| v.as_u64()).unwrap_or(0);
            let status = item.get("status").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let super_chat = item.get("super_chat").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();

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
                current_concurrency: item.get("current_concurrency").and_then(|v| v.as_i64()).map(|v| v as i32),  // 从外部API获取current_concurrency值
            });
        }
    }

    Ok(anchors)
}

async fn fetch_live_session_data(room_id: &str, union: &str, month: &str) -> Vec<LiveSession> {
    let base_url = if union == "VirtuaReal" || union.starts_with("vr") {
        "https://vr.qianqiuzy.cn"
    } else {
        "https://psp.qianqiuzy.cn"
    };

    let api_url = format!("{}/gift/live_sessions?room_id={}&month={}", base_url, room_id, month);

    println!("Fetching live sessions from: {}", api_url);

    match fetch_live_session_from_api(&HTTP_CLIENT, &api_url).await {
        Ok(sessions) => {
            println!("Successfully fetched {} sessions", sessions.len());
            sessions
        },
        Err(e) => {
            println!("Failed to fetch live sessions: {}", e);
            vec![]
        }
    }
}

async fn fetch_live_session_from_api(client: &Client, api_url: &str) -> Result<Vec<LiveSession>, Box<dyn std::error::Error + Send + Sync>> {
    println!("Making request to: {}", api_url);

    let response = timeout(
        Duration::from_secs(15),
        client.get(api_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .header("Accept", "application/json")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .send()
    ).await??;

    println!("Response status: {}", response.status());

    let json_text = timeout(
        Duration::from_secs(10),
        response.text()
    ).await??;

    println!("Response body length: {}", json_text.len());

    let raw_data: serde_json::Value = serde_json::from_str(&json_text)?;

    let sessions_array = if let Some(array) = raw_data.get("sessions").and_then(|v| v.as_array()) {
        array
    } else {
        raw_data.as_array().ok_or("Response is not an array or object with sessions field")?
    };

    let mut sessions = Vec::new();

    for item in sessions_array {
        let start_time = item.get("start_time").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let end_time = item.get("end_time").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let duration_minutes = 0;
        let start_guard_1 = item.get("start_guard_1").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let start_guard_2 = item.get("start_guard_2").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let start_guard_3 = item.get("start_guard_3").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let end_guard_1 = item.get("end_guard_1").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let end_guard_2 = item.get("end_guard_2").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let end_guard_3 = item.get("end_guard_3").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let start_fans_count = item.get("start_fans_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let end_fans_count = item.get("end_fans_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let danmaku_count = item.get("danmaku_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let gift = item.get("gift").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let guard = item.get("guard").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let super_chat = item.get("super_chat").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let total_revenue = gift + guard + super_chat;

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
            gift,
            guard,
            super_chat,
            total_revenue,
            title,
            avg_concurrency: item.get("avg_concurrency").and_then(|v| v.as_f64()),
            current_concurrency: item.get("current_concurrency").and_then(|v| v.as_i64()).map(|v| v as i32),
            max_concurrency: item.get("max_concurrency").and_then(|v| v.as_i64()).map(|v| v as i32),
        };

        println!("Created LiveSession: start_time={}, duration_minutes={}, gift={}, guard={}, super_chat={}",
                 live_session.start_time, live_session.duration_minutes, live_session.gift, live_session.guard, live_session.super_chat);

        sessions.push(live_session);
    }

    println!("Parsed {} sessions", sessions.len());
    Ok(sessions)
}

async fn index_handler() -> Html<String> {
    Html(std::fs::read_to_string("../rust_backend/dist/index.html").unwrap())
}

async fn favicon() -> impl axum::response::IntoResponse {
    axum::response::Response::builder()
        .header("content-type", "image/x-icon")
        .body(axum::body::Body::from(std::fs::read("../rust_backend/dist/favicon.ico").unwrap()))
        .unwrap()
}