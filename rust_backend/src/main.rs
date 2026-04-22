use axum::http::StatusCode;
use axum::{
    extract::{Query, State},
    response::Html,
    routing::{get, get_service},
    Json, Router,
};
use chrono::{NaiveDateTime, Timelike};
use lru::LruCache;
use reqwest::Client;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::path::Path;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tokio::time::timeout;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer,
};
use tracing_subscriber;

// ==================== 缓存相关定义 ====================

// 缓存条目结构，包含数据和大小估算
#[derive(Clone)]
struct CacheEntry {
    data: Vec<Anchor>,
    size_estimate: usize,             // 大小估算（字节）
    timestamp: std::time::SystemTime, // 缓存时间戳
}

// Attention数据缓存条目 - 存储某个月份的所有日期的关注数快照
#[derive(Clone, Debug)]
struct AttentionCacheEntry {
    data: serde_json::Value, // 原始JSON数据 {"YYYYMMDD": count, ...}
    timestamp: std::time::SystemTime,
    is_past_month: bool, // 是否为上月及更早数据
}

// LiveSessions数据缓存条目
#[derive(Clone, Debug)]
struct LiveSessionsCacheEntry {
    data: Vec<LiveSession>,
    timestamp: std::time::SystemTime,
    is_past_month: bool,
}

// 全局Attention缓存管理器
lazy_static::lazy_static! {
    static ref ATTENTION_CACHE: Arc<RwLock<HashMap<String, AttentionCacheEntry>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

// 全局LiveSessions缓存管理器
lazy_static::lazy_static! {
    static ref LIVE_SESSIONS_CACHE: Arc<RwLock<HashMap<String, LiveSessionsCacheEntry>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

// ==================== 缓存JSON持久化函数 ====================

/// 计算数据大小（字节数）
fn estimate_json_size(json: &serde_json::Value) -> usize {
    serde_json::to_string(json).unwrap_or_default().len()
}

/// 将缓存保存为JSON文件
async fn save_cache_to_file(
    key: &str,
    data: &serde_json::Value,
    is_past_month: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dir = if is_past_month {
        "cache_data/previous_months"
    } else {
        "cache_data/current_month"
    };

    // 创建目录
    tokio::fs::create_dir_all(dir).await?;

    let file_path = format!("{}/{}.json", dir, key);
    let json_str = serde_json::to_string_pretty(data)?;

    let mut file = tokio::fs::File::create(&file_path).await?;
    file.write_all(json_str.as_bytes()).await?;
    file.flush().await?;

    println!("💾 [缓存持久化] 已保存JSON文件 - {}", file_path);
    Ok(())
}

/// 从JSON文件加载缓存
async fn load_cache_from_file(key: &str, is_past_month: bool) -> Option<serde_json::Value> {
    let dir = if is_past_month {
        "cache_data/previous_months"
    } else {
        "cache_data/current_month"
    };

    let file_path = format!("{}/{}.json", dir, key);

    match tokio::fs::read_to_string(&file_path).await {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(data) => {
                println!("📂 [缓存加载] 从磁盘加载 - {}", file_path);
                Some(data)
            }
            Err(e) => {
                eprintln!("❌ [缓存加载] JSON解析失败 - {}: {}", file_path, e);
                None
            }
        },
        Err(_) => None,
    }
}

// ==================== Attention缓存管理器 ====================

/// 判断是否为上月数据
fn is_past_month(month: &str) -> bool {
    let current_month = chrono::Utc::now().format("%Y%m").to_string();
    month < current_month.as_str()
}

/// 判断当月缓存是否有效（1小时内）
fn is_cache_valid(
    cached_timestamp: std::time::SystemTime,
    current_time: std::time::SystemTime,
) -> bool {
    let duration_since_cache = current_time
        .duration_since(cached_timestamp)
        .unwrap_or_default();
    duration_since_cache.as_secs() < 3600 // 1小时
}

/// 判断是否需要在此刻刷新（每天1:30到2:00之间）
fn should_refresh_now(is_past_month: bool) -> bool {
    if is_past_month {
        return false; // 历史数据不刷新
    }

    let now = chrono::Local::now();
    let hour = now.hour();
    let minute = now.minute();

    // 每天1:30到次日2:00之间，允许刷新
    (hour == 1 && minute >= 30) || hour == 2
}

/// 批量获取单月份所有日期的attention数据
async fn fetch_attention_month_data(
    client: &Client,
    base_url: &str,
    room_id: &str,
    month: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let api_url = format!(
        "{}/gift/attention?room_id={}&month={}",
        base_url, room_id, month
    );
    println!("批量获取attention数据: {}", api_url);

    let response = timeout(
        Duration::from_secs(30), // 增加超时时间
        client
            .get(&api_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Accept", "application/json")
            .send(),
    )
    .await??;

    let json_text = timeout(Duration::from_secs(10), response.text()).await??;
    let json_value: serde_json::Value = serde_json::from_str(&json_text)?;
    Ok(json_value)
}

/// 获取attention缓存数据，如果缓存无效则刷新
async fn get_attention_cache_data(
    client: &Client,
    room_id: &str,
    month: &str,
    union: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let cache_key = format!("attention_{}_{}_{}", union, room_id, month);
    let current_time = std::time::SystemTime::now();
    let is_past = is_past_month(month);

    // 检查内存缓存
    {
        let cache = ATTENTION_CACHE.read().await;
        if let Some(entry) = cache.get(&cache_key) {
            // 历史数据永久有效
            if is_past {
                println!(
                    "🎯 [缓存命中-内存] 历史数据 - key={} (上月数据永久有效)",
                    cache_key
                );
                return Ok(entry.data.clone());
            }

            // 当月数据检查是否有效
            if is_cache_valid(entry.timestamp, current_time) {
                let size = estimate_json_size(&entry.data);
                println!(
                    "🎯 [缓存命中-内存] 当月数据 - key={} (1小时内有效) - 大小: {} bytes",
                    cache_key, size
                );
                return Ok(entry.data.clone());
            }

            // 缓存过期，检查是否可以刷新
            if !should_refresh_now(false) {
                println!(
                    "⏱️ [缓存过期-使用旧] 不在刷新时间窗口 - key={} (使用旧缓存)",
                    cache_key
                );
                return Ok(entry.data.clone());
            }

            println!(
                "🔄 [缓存需刷新] 当月数据过期 - key={} (超过1小时且已过1:30)",
                cache_key
            );
        }
    }

    // 尝试从磁盘加载缓存
    if let Some(loaded_data) = load_cache_from_file(&cache_key, is_past).await {
        let size = estimate_json_size(&loaded_data);
        println!(
            "📂 [缓存命中-磁盘] 从文件加载 - key={} - 大小: {} bytes",
            cache_key, size
        );

        // 将磁盘缓存重新加入内存
        {
            let mut cache = ATTENTION_CACHE.write().await;
            cache.insert(
                cache_key.clone(),
                AttentionCacheEntry {
                    data: loaded_data.clone(),
                    timestamp: current_time,
                    is_past_month: is_past,
                },
            );
        }

        return Ok(loaded_data);
    }

    println!("📡 [缓存未命中] 需要调用外部API - key={}", cache_key);

    // 需要获取新数据
    println!(
        "🌐 [API调用开始] 向外部服务获取数据 - room_id={}, month={}, union={}",
        room_id, month, union
    );
    let base_url = if union == "VirtuaReal" || union.starts_with("vr") {
        "https://vr.qianqiuzy.cn"
    } else {
        "https://psp.qianqiuzy.cn"
    };

    let data = fetch_attention_month_data(client, &base_url, room_id, month).await?;
    let size = estimate_json_size(&data);

    println!(
        "✅ [API调用成功] 获取数据完成 - room_id={}, month={} - 大小: {} bytes",
        room_id, month, size
    );

    // 保存到磁盘
    if let Err(e) = save_cache_to_file(&cache_key, &data, is_past).await {
        eprintln!("⚠️  [持久化失败] 无法保存JSON文件 - {}: {}", cache_key, e);
    }

    // 更新内存缓存
    {
        let mut cache = ATTENTION_CACHE.write().await;
        cache.insert(
            cache_key.clone(),
            AttentionCacheEntry {
                data: data.clone(),
                timestamp: current_time,
                is_past_month: is_past,
            },
        );
        println!(
            "💾 [缓存保存-内存] 成功保存到内存 - key={} (过期策略: {})",
            cache_key,
            if is_past { "永久" } else { "1小时" }
        );
    }

    Ok(data)
}

/// 从缓存的attention数据中获取指定日期的粉丝数
fn get_attention_from_cache(cache_data: &serde_json::Value, date: &str) -> Option<i64> {
    if let Some(obj) = cache_data.as_object() {
        if let Some(value) = obj.get(date) {
            if let Some(num) = value.as_i64() {
                return Some(num);
            }
            // 尝试作为字符串解析
            if let Some(str_val) = value.as_str() {
                if let Ok(num) = str_val.parse::<i64>() {
                    return Some(num);
                }
            }
        }
    }
    None
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
    hit_count: usize,  // 缓存命中次数
    miss_count: usize, // 缓存未命中次数
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

// ==================== Attention缓存管理 ====================

/// 判断当月缓存是否仍然有效（距上次更新<1小时）
fn is_attention_cache_valid(cached_timestamp: std::time::SystemTime) -> bool {
    match std::time::SystemTime::now().duration_since(cached_timestamp) {
        Ok(duration) => duration.as_secs() < 3600, // 1小时 = 3600秒
        Err(_) => false,
    }
}

/// 获取Attention缓存数据
async fn get_attention_cache(
    room_id: &str,
    month: &str,
    _union: &str,
) -> Option<serde_json::Value> {
    let cache_key = format!("attention_{}_{}", room_id, month);
    let cache = ATTENTION_CACHE.read().await;

    if let Some(entry) = cache.get(&cache_key) {
        // 检查缓存有效性
        if entry.is_past_month || is_attention_cache_valid(entry.timestamp) {
            println!("✅ Attention缓存命中: {}", cache_key);
            return Some(entry.data.clone());
        }
        println!("⏰ Attention缓存过期，需要重新获取");
    }
    None
}

/// 存储Attention缓存数据
async fn put_attention_cache(room_id: &str, month: &str, data: serde_json::Value) {
    let cache_key = format!("attention_{}_{}", room_id, month);
    let is_past = is_past_month(month);

    let mut cache = ATTENTION_CACHE.write().await;
    cache.insert(
        cache_key.clone(),
        AttentionCacheEntry {
            data,
            timestamp: std::time::SystemTime::now(),
            is_past_month: is_past,
        },
    );

    println!(
        "💾 Attention缓存已保存: {} (过去月份: {})",
        cache_key, is_past
    );
}

/// 清除指定月份的Attention缓存（用户手动刷新时调用）
async fn clear_attention_cache_for_month(month: &str) {
    let mut cache = ATTENTION_CACHE.write().await;
    cache.retain(|key, _| !key.ends_with(month));
    println!("🗑️ 已清除月份 {} 的Attention缓存", month);
}

// ==================== LiveSessions缓存管理 ====================

/// 获取LiveSessions缓存数据
async fn get_live_sessions_cache(
    room_id: &str,
    _union: &str,
    month: &str,
) -> Option<Vec<LiveSession>> {
    let cache_key = format!("livesessions_{}_{}", room_id, month);
    let cache = LIVE_SESSIONS_CACHE.read().await;

    if let Some(entry) = cache.get(&cache_key) {
        // 检查缓存有效性
        if entry.is_past_month || is_attention_cache_valid(entry.timestamp) {
            println!("✅ LiveSessions缓存命中: {}", cache_key);
            return Some(entry.data.clone());
        }
        println!("⏰ LiveSessions缓存过期，需要重新获取");
    }
    None
}

/// 存储LiveSessions缓存数据
async fn put_live_sessions_cache(room_id: &str, _union: &str, month: &str, data: Vec<LiveSession>) {
    let cache_key = format!("livesessions_{}_{}", room_id, month);
    let is_past = is_past_month(month);

    // 保存到磁盘
    let json_data = serde_json::to_value(&data).unwrap_or(serde_json::Value::Null);
    if let Err(e) = save_cache_to_file(&cache_key, &json_data, is_past).await {
        eprintln!(
            "⚠️  [LiveSessions持久化失败] 无法保存JSON文件 - {}: {}",
            cache_key, e
        );
    }

    let mut cache = LIVE_SESSIONS_CACHE.write().await;
    cache.insert(
        cache_key.clone(),
        LiveSessionsCacheEntry {
            data,
            timestamp: std::time::SystemTime::now(),
            is_past_month: is_past,
        },
    );

    println!(
        "💾 LiveSessions缓存已保存: {} (过去月份: {})",
        cache_key, is_past
    );
}

/// 清除指定主播月份的LiveSessions缓存（用户手动刷新时调用）
async fn clear_live_sessions_cache_for_month(room_id: &str, month: &str) {
    let mut cache = LIVE_SESSIONS_CACHE.write().await;
    cache.retain(|key, _| !key.contains(room_id) || !key.ends_with(month));
    println!(
        "🗑️ 已清除主播 {} 月份 {} 的LiveSessions缓存",
        room_id, month
    );
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
    pub gift: f64,
    pub guard: f64,
    pub super_chat: f64,
    pub total_revenue: f64,
    pub title: String,
    pub avg_concurrency: Option<f64>,
    pub current_concurrency: Option<i64>,
    pub max_concurrency: Option<i64>,
    pub new_fans_count: i64,
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
struct SessionFansChangeQuery {
    room_id: Option<String>,
    union: Option<String>,
    start_time: String,
    end_time: Option<String>,
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
        .route(
            "/gift/live_sessions_with_fans",
            get(get_live_sessions_with_fans),
        )
        .route(
            "/gift/session_fans_change",
            get(calculate_session_fans_change_endpoint),
        )
        .route("/gift/sc", get(get_sc_history))
        .route("/gift/attention", get(get_attention_data))
        .route("/cache/stats", get(get_cache_stats_endpoint))
        .nest_service(
            "/assets",
            get_service(ServeDir::new("../rust_backend/dist/assets")),
        )
        .route("/favicon.ico", get(favicon))
        .fallback(index_handler)
        // 添加中间件
        .layer(TraceLayer::new_for_http()) // 请求跟踪
        .layer(CompressionLayer::new()) // 响应压缩
        .layer(cors_layer) // CORS
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

    let month = query
        .month
        .unwrap_or_else(|| chrono::Utc::now().format("%Y%m").to_string());
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
    let month = query
        .month
        .unwrap_or_else(|| chrono::Utc::now().format("%Y%m").to_string());

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

    let sessions = fetch_live_session_data(&HTTP_CLIENT, &room_id, &union, &month).await;

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

    let base_url = if query.union.as_deref().unwrap_or("VirtuaReal") == "PSPlive" {
        "https://psp.qianqiuzy.cn"
    } else {
        "https://vr.qianqiuzy.cn"
    };

    let api_url = format!("{}/gift/sc?room_id={}", base_url, room_id);

    // 使用全局HTTP客户端
    let response = timeout(
        Duration::from_secs(15),
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
    .map_err(|_| (StatusCode::REQUEST_TIMEOUT, "Request timeout".to_string()))?;

    let response = response.map_err(|e| {
        eprintln!("API请求失败: {}, URL: {}", e, api_url);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("API请求失败: {}", e),
        )
    })?;

    if !response.status().is_success() {
        eprintln!("API响应错误: {} for URL: {}", response.status(), api_url);
        return Err((StatusCode::BAD_GATEWAY, "上游API响应错误".to_string()));
    }

    let json_text = timeout(Duration::from_secs(10), response.text())
        .await
        .map_err(|_| (StatusCode::REQUEST_TIMEOUT, "Response timeout".to_string()))?;

    let json_text = json_text.map_err(|e| {
        eprintln!("读取响应体失败: {}, URL: {}", e, api_url);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("读取响应体失败: {}", e),
        )
    })?;

    let raw_data: serde_json::Value = serde_json::from_str(&json_text).map_err(|e| {
        eprintln!("JSON解析失败: {}, 响应内容: {:.200}", e, &json_text);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("JSON解析失败: {}", e),
        )
    })?;

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
            println!(
                "数据已缓存: {}, 当前缓存大小: {} bytes, 条目数: {}, 缓存键: {}",
                success, current_size, entry_count, cache_key
            );
        } else {
            println!("缓存失败，数据可能过大");
        }
    }

    all_data
}

// 缓存统计信息响应结构 - 增强版，包含更详细的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

// 计算大小格式字符串（字节转MB）
fn format_size_mb(bytes: usize) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    format!("{:.2} MB", mb)
}

// 获取缓存统计信息的API端点 - 增强版
async fn get_cache_stats_endpoint() -> Result<Json<CacheStatsResponse>, StatusCode> {
    let (hits, misses, hit_rate) = get_cache_hit_stats().await;
    let (current_size, entry_count) = get_cache_stats().await;

    // 获取Attention缓存条目数
    let attention_entries = {
        let cache = ATTENTION_CACHE.read().await;
        cache.len()
    };

    // 获取LiveSessions缓存条目数
    let live_sessions_entries = {
        let cache = LIVE_SESSIONS_CACHE.read().await;
        cache.len()
    };

    let hit_rate_percentage = format!("{:.2}%", hit_rate * 100.0);
    let current_size_mb = format_size_mb(current_size);
    let max_size_mb = format_size_mb(MAX_CACHE_SIZE);

    Ok(Json(CacheStatsResponse {
        hits,
        misses,
        hit_rate,
        hit_rate_percentage,
        current_size,
        current_size_mb,
        entry_count,
        max_size: MAX_CACHE_SIZE,
        max_size_mb,
        attention_entries,
        live_sessions_entries,
    }))
}

async fn fetch_anchor_data_by_url(
    url: &str,
) -> Result<Vec<Anchor>, Box<dyn std::error::Error + Send + Sync>> {
    let data = fetch_external_api(&HTTP_CLIENT, url).await?;
    Ok(data)
}

async fn fetch_external_api(
    client: &Client,
    api_url: &str,
) -> Result<Vec<Anchor>, Box<dyn std::error::Error + Send + Sync>> {
    let response = timeout(
        Duration::from_secs(10),
        client
            .get(api_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send(),
    )
    .await??;

    let json_text = timeout(Duration::from_secs(10), response.text()).await??;

    let raw_data: serde_json::Value = serde_json::from_str(&json_text)?;

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
                .unwrap_or(&chrono::Utc::now().format("%Y%m").to_string())
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
            });
        }
    }

    Ok(anchors)
}

async fn fetch_live_session_data(
    client: &Client,
    room_id: &str,
    union: &str,
    month: &str,
) -> Vec<LiveSession> {
    fetch_live_session_data_with_fans_calc(client, room_id, union, month, false).await
}

async fn fetch_live_session_data_with_fans_calc(
    client: &Client,
    room_id: &str,
    union: &str,
    month: &str,
    calculate_fans_change: bool,
) -> Vec<LiveSession> {
    let cache_key = format!("livesessions_{}_{}", room_id, month);
    let is_past = is_past_month(month);

    // 首先检查内存缓存
    if let Some(cached_sessions) = get_live_sessions_cache(room_id, union, month).await {
        println!(
            "🎯 [缓存命中-内存] LiveSessions数据 - room_id={}, month={}, union={} (缓存条目数: {})",
            room_id,
            month,
            union,
            cached_sessions.len()
        );

        let mut sessions = cached_sessions;

        // 只有在需要时才计算新增粉丝数（懒加载）
        if calculate_fans_change {
            println!("🔄 [粉丝计算] 开始计算所有会话的新增粉丝数...");
            for session in &mut sessions {
                session.new_fans_count = calculate_session_fans_change(
                    client,
                    &session.start_time,
                    &session.end_time,
                    room_id,
                    union,
                )
                .await;
            }
            println!("✅ [粉丝计算] 完成所有会话的粉丝数计算");
        } else {
            println!("⏭️ [跳过计算] 懒加载模式，跳过粉丝数计算");
        }

        return sessions;
    }

    // 尝试从磁盘加载缓存
    if let Some(loaded_data) = load_cache_from_file(&cache_key, is_past).await {
        if let Ok(sessions) = serde_json::from_value::<Vec<LiveSession>>(loaded_data.clone()) {
            println!(
                "📂 [缓存命中-磁盘] 从文件加载LiveSessions - room_id={}, month={}, union={} (条目数: {})",
                room_id, month, union, sessions.len()
            );

            // 将磁盘缓存重新加入内存
            put_live_sessions_cache(room_id, union, month, sessions.clone()).await;

            let mut sessions = sessions;

            // 只有在需要时才计算新增粉丝数（懒加载）
            if calculate_fans_change {
                println!("🔄 [粉丝计算] 开始计算所有会话的新增粉丝数...");
                for session in &mut sessions {
                    session.new_fans_count = calculate_session_fans_change(
                        client,
                        &session.start_time,
                        &session.end_time,
                        room_id,
                        union,
                    )
                    .await;
                }
                println!("✅ [粉丝计算] 完成所有会话的粉丝数计算");
            } else {
                println!("⏭️ [跳过计算] 懒加载模式，跳过粉丝数计算");
            }

            return sessions;
        }
    }

    // 缓存未命中，从API获取
    let base_url = if union == "VirtuaReal" || union.starts_with("vr") {
        "https://vr.qianqiuzy.cn"
    } else {
        "https://psp.qianqiuzy.cn"
    };

    let api_url = format!(
        "{}/gift/live_sessions?room_id={}&month={}",
        base_url, room_id, month
    );

    println!(
        "📡 [缓存未命中] 需要调用外部API获取LiveSessions - room_id={}, month={}, union={}",
        room_id, month, union
    );
    println!("🌐 [API调用] Fetching live sessions from: {}", api_url);

    let mut sessions = match fetch_live_session_from_api(client, &api_url).await {
        Ok(sessions) => {
            println!("✅ [API调用成功] 获取到 {} 个直播会话", sessions.len());
            sessions
        }
        Err(e) => {
            println!("❌ [API调用失败] 获取直播会话失败: {}", e);
            vec![]
        }
    };

    // 保存到缓存
    if !sessions.is_empty() {
        put_live_sessions_cache(room_id, union, month, sessions.clone()).await;
    }

    // 只有在需要时才计算新增粉丝数（懒加载）
    if calculate_fans_change {
        println!("🔄 [粉丝计算] 开始计算所有会话的新增粉丝数...");
        for session in &mut sessions {
            session.new_fans_count = calculate_session_fans_change(
                client,
                &session.start_time,
                &session.end_time,
                room_id,
                union,
            )
            .await;
        }
        println!("✅ [粉丝计算] 完成所有会话的粉丝数计算");
    } else {
        println!("⏭️ [跳过计算] 懒加载模式，跳过粉丝数计算");
    }

    sessions
}

async fn fetch_live_session_from_api(
    client: &Client,
    api_url: &str,
) -> Result<Vec<LiveSession>, Box<dyn std::error::Error + Send + Sync>> {
    // 不显示详细的API调用日志，只在缓存未命中时显示
    let response = timeout(
        Duration::from_secs(15),
        client.get(api_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .header("Accept", "application/json")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .send()
    ).await??;

    let json_text = timeout(Duration::from_secs(10), response.text()).await??;

    let raw_data: serde_json::Value = serde_json::from_str(&json_text)?;

    let sessions_array = if let Some(array) = raw_data.get("sessions").and_then(|v| v.as_array()) {
        array
    } else {
        raw_data
            .as_array()
            .ok_or("Response is not an array or object with sessions field")?
    };

    let mut sessions = Vec::new();

    for (index, item) in sessions_array.iter().enumerate() {
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
            avg_concurrency,
            current_concurrency,
            max_concurrency,
            new_fans_count: -1, // 暂时设为 -1，后续通过 attention API 计算
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
        .unwrap_or_else(|| chrono::Utc::now().format("%Y%m").to_string());

    let union = query.union.unwrap_or_else(|| "VirtuaReal".to_string());

    println!(
        "📊 [/gift/attention] room_id={}, month={}, union={}",
        room_id, month, union
    );

    // 使用缓存机制获取数据
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

/// 从外部 API 获取粉丝数快照数据
async fn fetch_attention_from_api(
    client: &Client,
    api_url: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let response = timeout(
        Duration::from_secs(10),
        client
            .get(api_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Accept", "application/json")
            .send(),
    )
    .await??;

    let json_text = timeout(Duration::from_secs(10), response.text()).await??;

    let data: serde_json::Value = serde_json::from_str(&json_text)?;
    Ok(data)
}

/// 从日期时间字符串中提取日期部分 (YYYYMMDD 格式)
fn parse_date_from_datetime(datetime: &str) -> String {
    if datetime.is_empty() {
        return String::new();
    }
    // 格式："2026-01-01 19:55:04" -> "20260101"
    let date_part = datetime.split(' ').next().unwrap_or("");
    date_part.replace("-", "")
}

/// 获取前一天的日期 (YYYYMMDD 格式)
fn get_previous_date(date_str: &str) -> Option<String> {
    if date_str.len() != 8 {
        return None;
    }

    let year = date_str[..4].parse::<i32>().ok()?;
    let month = date_str[4..6].parse::<u32>().ok()?;
    let day = date_str[6..8].parse::<u32>().ok()?;

    let current_date = chrono::NaiveDate::from_ymd_opt(year, month, day)?;
    let prev_date = current_date.pred_opt()?;

    Some(prev_date.format("%Y%m%d").to_string())
}

/// 获取指定日期的粉丝数
async fn get_attention_for_date(
    client: &Client,
    base_url: &str,
    room_id: &str,
    date: &str, // YYYYMMDD
) -> Option<i64> {
    let month = &date[..6]; // YYYYMM
    let api_url = format!(
        "{}/gift/attention?room_id={}&month={}",
        base_url, room_id, month
    );

    match fetch_attention_from_api(client, &api_url).await {
        Ok(response) => {
            // 从 attention 数组中查找对应日期
            if let Some(attention_array) = response.get("attention").and_then(|v| v.as_array()) {
                for snapshot in attention_array {
                    if let Some(count) = snapshot.get(date).and_then(|v| v.as_str()) {
                        return count.parse::<i64>().ok();
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}

/// 计算单次直播的粉丝数变化
async fn calculate_session_fans_change(
    client: &Client,
    start_time: &str,
    end_time: &str,
    room_id: &str,
    union: &str,
) -> i64 {
    // 1. 解析开始和结束时间
    let start_date = parse_date_from_datetime(start_time);

    if start_date.is_empty() {
        return -1;
    }

    // 2. 计算结束日期（如果未下播则使用当前时间）
    let end_date = if end_time.is_empty() {
        chrono::Utc::now().format("%Y%m%d").to_string()
    } else {
        parse_date_from_datetime(end_time)
    };

    // 3. 计算前一天
    let start_date_prev = match get_previous_date(&start_date) {
        Some(d) => d,
        None => return -1,
    };

    let end_date_prev = match get_previous_date(&end_date) {
        Some(d) => d,
        None => return -1,
    };

    // 4. 获取月份（开始和结束应该在同一月份）
    let month = &start_date[..6]; // YYYYMM

    println!(
        "📊 [粉丝变化计算] 开始 - room_id={}, 直播期间: {} 到 {}",
        room_id, start_date, end_date
    );

    // 5. 从缓存获取attention数据
    let attention_data = match get_attention_cache_data(client, room_id, month, union).await {
        Ok(data) => {
            println!(
                "✅ [粉丝变化计算] 成功获取attention数据 - room_id={}, month={}",
                room_id, month
            );
            data
        }
        Err(e) => {
            eprintln!(
                "❌ [粉丝变化计算] 获取attention数据失败 - room_id={}, error={}",
                room_id, e
            );
            return -1;
        }
    };

    // 6. 从缓存数据中提取粉丝数
    let start_attention = match get_attention_from_cache(&attention_data, &start_date_prev) {
        Some(count) => {
            println!("   📈 开始日期粉丝数 - {} = {}", start_date_prev, count);
            count
        }
        None => {
            eprintln!(
                "❌ [粉丝变化计算] 缓存中未找到日期 {} 的粉丝数",
                start_date_prev
            );
            return -1;
        }
    };

    let end_attention = match get_attention_from_cache(&attention_data, &end_date_prev) {
        Some(count) => {
            println!("   📈 结束日期粉丝数 - {} = {}", end_date_prev, count);
            count
        }
        None => {
            eprintln!(
                "❌ [粉丝变化计算] 缓存中未找到日期 {} 的粉丝数",
                end_date_prev
            );
            return -1;
        }
    };

    // 7. 计算变化
    let fans_change = end_attention - start_attention;
    println!(
        "✅ [粉丝变化计算] 完成 - room_id={}, 粉丝变化: {} - {} = {}",
        room_id, end_attention, start_attention, fans_change
    );
    fans_change
}

/// 计算单个会话的粉丝变化（懒加载用）
async fn calculate_session_fans_change_endpoint(
    Query(query): Query<SessionFansChangeQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let room_id = query.room_id.unwrap_or_default();
    let union = query.union.unwrap_or_else(|| "VirtuaReal".to_string());
    let end_time = query.end_time.as_deref().unwrap_or("");

    if room_id.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let fans_change =
        calculate_session_fans_change(&HTTP_CLIENT, &query.start_time, end_time, &room_id, &union)
            .await;

    Ok(Json(serde_json::json!({
        "fans_change": fans_change,
        "room_id": room_id,
        "union": union,
        "start_time": query.start_time,
        "end_time": query.end_time
    })))
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
        .unwrap_or_else(|| chrono::Utc::now().format("%Y%m").to_string());

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
        fetch_live_session_data_with_fans_calc(&HTTP_CLIENT, &room_id, &union, &month, true).await;

    Ok(Json(LiveSessionResponse {
        sessions,
        room_id,
        queried_user,
        union,
        title: format!("{}年{}月直播数据", &month[..4], &month[4..]),
        refresh_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }))
}

async fn index_handler() -> Html<String> {
    Html(std::fs::read_to_string("../rust_backend/dist/index.html").unwrap())
}

async fn favicon() -> impl axum::response::IntoResponse {
    axum::response::Response::builder()
        .header("content-type", "image/x-icon")
        .body(axum::body::Body::from(
            std::fs::read("../rust_backend/dist/favicon.ico").unwrap(),
        ))
        .unwrap()
}
