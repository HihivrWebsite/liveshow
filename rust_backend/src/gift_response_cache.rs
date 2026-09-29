//! `/gift*` HTTP 响应的最外层 10 秒缓存与同键请求合并。
//!
//! 该缓存位于外层压缩中间件内侧，保存 handler 返回的未压缩原始响应。
//! 它只负责抑制完全相同请求的并发风暴，不替代 main.rs 中已有的业务缓存。

use axum::body::{to_bytes, Body, Bytes, HttpBody as _};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http::header::{CACHE_CONTROL, CONNECTION, CONTENT_LENGTH, TRANSFER_ENCODING, VARY};
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

const GIFT_CACHE_TTL: Duration = Duration::from_secs(10);
const GIFT_CACHE_MAX_ENTRIES: usize = 512;
const GIFT_CACHE_MAX_BYTES: usize = 64 * 1024 * 1024;
const GIFT_CACHE_MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;
const GIFT_CACHE_MAX_KEY_BYTES: usize = 4096;

const X_GIFT_CACHE: &str = "x-gift-cache";

/// `/cache/stats` 中的短缓存诊断数据。
///
/// `misses` 是实际执行 handler 的次数，不等同于远程 API 调用次数。
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GiftResponseCacheStats {
    pub ttl_seconds: u64,
    pub hits: u64,
    pub coalesced_hits: u64,
    pub misses: u64,
    pub expired: u64,
    pub evictions: u64,
    pub bypasses: u64,
    pub buffer_errors: u64,
    pub entries: u64,
    pub bytes: u64,
    pub inflight_keys: u64,
    pub hit_rate: f64,
}

/// 已缓存的一份完整、未压缩 HTTP 响应。
#[derive(Debug, Clone)]
pub struct CachedGiftResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
    #[cfg_attr(not(test), allow(dead_code))]
    stored_at: Instant,
    expires_at: Instant,
    generation: u64,
}

#[derive(Debug, Default, Clone, Copy)]
struct Counters {
    hits: u64,
    coalesced_hits: u64,
    misses: u64,
    expired: u64,
    evictions: u64,
    bypasses: u64,
    buffer_errors: u64,
}

struct CacheState {
    entries: LruCache<String, CachedGiftResponse>,
    current_bytes: usize,
    inflight: HashMap<String, Arc<tokio::sync::Mutex<()>>>,
    generation: u64,
    cleanup_generation: u64,
    counters: Counters,
}

impl CacheState {
    fn new(max_entries: usize) -> Self {
        Self {
            entries: LruCache::new(
                NonZeroUsize::new(max_entries.max(1)).expect("cache capacity is non-zero"),
            ),
            current_bytes: 0,
            inflight: HashMap::new(),
            generation: 0,
            cleanup_generation: 0,
            counters: Counters::default(),
        }
    }

    fn purge_expired(&mut self, now: Instant) {
        self.cleanup_generation = self.cleanup_generation.wrapping_add(1);
        let expired_keys: Vec<String> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.expires_at <= now)
            .map(|(key, _)| key.clone())
            .collect();
        if !expired_keys.is_empty() {
            tracing::debug!(
                cleanup_generation = self.cleanup_generation,
                expired = expired_keys.len(),
                "清理过期的 gift 响应"
            );
        }

        for key in expired_keys {
            if let Some(entry) = self.entries.pop(&key) {
                self.current_bytes = self.current_bytes.saturating_sub(entry.body.len());
                self.counters.expired = self.counters.expired.saturating_add(1);
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct GiftResponseCache {
    state: Arc<Mutex<CacheState>>,
    ttl: Duration,
    max_entries: usize,
    max_bytes: usize,
    max_response_bytes: usize,
    max_key_bytes: usize,
}

impl Default for GiftResponseCache {
    fn default() -> Self {
        Self::with_limits(
            GIFT_CACHE_TTL,
            GIFT_CACHE_MAX_ENTRIES,
            GIFT_CACHE_MAX_BYTES,
            GIFT_CACHE_MAX_RESPONSE_BYTES,
            GIFT_CACHE_MAX_KEY_BYTES,
        )
    }
}

impl std::fmt::Debug for GiftResponseCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GiftResponseCache")
            .field("ttl", &self.ttl)
            .field("max_entries", &self.max_entries)
            .field("max_bytes", &self.max_bytes)
            .field("max_response_bytes", &self.max_response_bytes)
            .field("max_key_bytes", &self.max_key_bytes)
            .finish_non_exhaustive()
    }
}

impl GiftResponseCache {
    fn with_limits(
        ttl: Duration,
        max_entries: usize,
        max_bytes: usize,
        max_response_bytes: usize,
        max_key_bytes: usize,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(CacheState::new(max_entries))),
            ttl,
            max_entries,
            max_bytes,
            max_response_bytes,
            max_key_bytes,
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, CacheState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    async fn execute<F>(&self, key: Option<String>, response_future: F) -> Response
    where
        F: Future<Output = Response> + Send,
    {
        let valid_key = key.filter(|key| key.len() <= self.max_key_bytes);
        if valid_key.is_none() {
            {
                let mut state = self.lock_state();
                state.counters.bypasses = state.counters.bypasses.saturating_add(1);
            }
            return with_cache_header(response_future.await, "MISS");
        }
        let key = valid_key.expect("validated cache key");

        if let Some(entry) = self.lookup(&key, LookupKind::Direct) {
            return build_cached_response(&entry, "HIT");
        }

        let mut registration = self.register_inflight(key.clone());
        let lock_guard = registration.lock.clone().lock_owned().await;

        if let Some(entry) = self.lookup(&key, LookupKind::Coalesced) {
            drop(lock_guard);
            registration.finish();
            return build_cached_response(&entry, "COALESCED");
        }

        let generation;
        {
            let mut state = self.lock_state();
            generation = state.generation;
            state.counters.misses = state.counters.misses.saturating_add(1);
        }

        let response = response_future.await;
        let response = self.buffer_or_bypass(response, &key, generation).await;

        drop(lock_guard);
        registration.finish();
        response
    }

    fn lookup(&self, key: &str, kind: LookupKind) -> Option<CachedGiftResponse> {
        let mut state = self.lock_state();
        let now = Instant::now();
        state.purge_expired(now);
        let entry = state.entries.get(key).cloned();
        if entry.is_some() {
            match kind {
                LookupKind::Direct => {
                    state.counters.hits = state.counters.hits.saturating_add(1);
                }
                LookupKind::Coalesced => {
                    state.counters.coalesced_hits = state.counters.coalesced_hits.saturating_add(1);
                }
            }
        }
        entry
    }

    fn register_inflight(&self, key: String) -> InflightRegistration {
        let lock = {
            let mut state = self.lock_state();
            state
                .inflight
                .entry(key.clone())
                .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
                .clone()
        };

        InflightRegistration {
            state: Arc::clone(&self.state),
            key,
            lock,
            active: false,
        }
    }

    async fn buffer_or_bypass(&self, response: Response, key: &str, generation: u64) -> Response {
        if response_has_bypass_headers(response.headers()) {
            self.count_bypass();
            return with_cache_header(response, "MISS");
        }

        let content_length = response_content_length(response.headers()).or_else(|| {
            response
                .body()
                .size_hint()
                .exact()
                .map(|size| size as usize)
        });
        let Some(content_length) = content_length else {
            self.count_bypass();
            return with_cache_header(response, "MISS");
        };
        if content_length > self.max_response_bytes {
            self.count_bypass();
            return with_cache_header(response, "MISS");
        }

        let (parts, body) = response.into_parts();
        let read_limit = self
            .max_response_bytes
            .saturating_add(1)
            .min(content_length.saturating_add(1));
        let buffered = to_bytes(body, read_limit.max(1)).await;
        let body = match buffered {
            Ok(body) => body,
            Err(error) => {
                tracing::debug!(
                    %error,
                    key,
                    "gift 响应读取失败，绕过响应缓存"
                );
                self.count_buffer_error();
                let mut response = Response::from_parts(
                    strip_frame_headers(parts),
                    Body::from("gift response buffering failed"),
                );
                set_cache_header(&mut response, "MISS");
                return response;
            }
        };

        if body.len() != content_length || body.len() > self.max_response_bytes {
            self.count_bypass();
            let mut response = Response::from_parts(strip_frame_headers(parts), Body::from(body));
            set_cache_header(&mut response, "MISS");
            return response;
        }

        let stored_headers = end_to_end_headers(&parts.headers);
        let now = Instant::now();
        let cached = CachedGiftResponse {
            status: parts.status,
            headers: stored_headers,
            body: body.clone(),
            stored_at: now,
            expires_at: now + self.ttl,
            generation,
        };
        self.store(key, cached);

        let mut response = Response::from_parts(strip_frame_headers(parts), Body::from(body));
        set_cache_header(&mut response, "MISS");
        response
    }

    fn store(&self, key: &str, entry: CachedGiftResponse) {
        let mut state = self.lock_state();
        if state.generation != entry.generation {
            tracing::debug!(
                key,
                started_generation = entry.generation,
                current_generation = state.generation,
                "gift 响应在缓存清理后完成，拒绝写入旧结果"
            );
            return;
        }

        state.purge_expired(Instant::now());
        let new_size = entry.body.len();
        if new_size > self.max_bytes || new_size > self.max_response_bytes {
            state.counters.bypasses = state.counters.bypasses.saturating_add(1);
            return;
        }

        while state.entries.len() >= self.max_entries
            || state.current_bytes.saturating_add(new_size) > self.max_bytes
        {
            let Some((_, evicted)) = state.entries.pop_lru() else {
                break;
            };
            state.current_bytes = state.current_bytes.saturating_sub(evicted.body.len());
            state.counters.evictions = state.counters.evictions.saturating_add(1);
            if state.entries.len() < self.max_entries
                && state.current_bytes.saturating_add(new_size) <= self.max_bytes
            {
                break;
            }
        }

        if state.entries.len() >= self.max_entries
            || state.current_bytes.saturating_add(new_size) > self.max_bytes
        {
            state.counters.bypasses = state.counters.bypasses.saturating_add(1);
            return;
        }

        if let Some(previous) = state.entries.put(key.to_owned(), entry) {
            state.current_bytes = state.current_bytes.saturating_sub(previous.body.len());
        }
        state.current_bytes = state.current_bytes.saturating_add(new_size);
    }

    fn clear(&self) {
        let mut state = self.lock_state();
        state.generation = state.generation.wrapping_add(1);
        state.cleanup_generation = state.cleanup_generation.wrapping_add(1);
        state.entries.clear();
        state.current_bytes = 0;
    }

    fn stats(&self) -> GiftResponseCacheStats {
        let mut state = self.lock_state();
        state.purge_expired(Instant::now());
        let counters = state.counters;
        let entries = state.entries.len() as u64;
        let bytes = state.current_bytes as u64;
        let inflight_keys = state.inflight.len() as u64;
        let successful_lookups = counters.hits.saturating_add(counters.coalesced_hits);
        let observations = successful_lookups.saturating_add(counters.misses);
        let hit_rate = if observations == 0 {
            0.0
        } else {
            successful_lookups as f64 / observations as f64
        };

        GiftResponseCacheStats {
            ttl_seconds: self.ttl.as_secs(),
            hits: counters.hits,
            coalesced_hits: counters.coalesced_hits,
            misses: counters.misses,
            expired: counters.expired,
            evictions: counters.evictions,
            bypasses: counters.bypasses,
            buffer_errors: counters.buffer_errors,
            entries,
            bytes,
            inflight_keys,
            hit_rate,
        }
    }

    fn count_bypass(&self) {
        let mut state = self.lock_state();
        state.counters.bypasses = state.counters.bypasses.saturating_add(1);
    }

    fn count_buffer_error(&self) {
        let mut state = self.lock_state();
        state.counters.buffer_errors = state.counters.buffer_errors.saturating_add(1);
        state.counters.bypasses = state.counters.bypasses.saturating_add(1);
    }
}

#[derive(Clone, Copy)]
enum LookupKind {
    Direct,
    Coalesced,
}

struct InflightRegistration {
    state: Arc<Mutex<CacheState>>,
    key: String,
    lock: Arc<tokio::sync::Mutex<()>>,
    active: bool,
}

impl InflightRegistration {
    fn finish(&mut self) {
        self.active = false;
        if Arc::strong_count(&self.lock) > 2 {
            return;
        }
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if state
            .inflight
            .get(&self.key)
            .is_some_and(|current| Arc::ptr_eq(current, &self.lock))
        {
            state.inflight.remove(&self.key);
        }
    }
}

impl Drop for InflightRegistration {
    fn drop(&mut self) {
        self.finish();
    }
}

lazy_static::lazy_static! {
    static ref GIFT_RESPONSE_CACHE: GiftResponseCache = GiftResponseCache::default();
}

/// `/gift*` 专用响应缓存中间件。
pub async fn gift_response_cache_middleware(request: Request, next: Next) -> Response {
    let Some(key) = canonical_gift_key(&request) else {
        return next.run(request).await;
    };
    GIFT_RESPONSE_CACHE
        .execute(Some(key), next.run(request))
        .await
}

/// 返回当前短缓存的诊断统计。
pub fn gift_response_cache_stats() -> GiftResponseCacheStats {
    GIFT_RESPONSE_CACHE.stats()
}

/// 清空短缓存并推进 generation，使清理前启动的慢响应无法回写。
pub fn clear_gift_response_cache() {
    GIFT_RESPONSE_CACHE.clear();
}

fn canonical_gift_key(request: &Request) -> Option<String> {
    if request.method() != Method::GET {
        return None;
    }

    let path = request.uri().path();
    let path = if path.is_empty() { "/" } else { path };
    if !path.starts_with("/gift") {
        return None;
    }
    let query = request.uri().query().unwrap_or("");
    let normalized_query = canonical_query(query);
    Some(format!("GET|{path}|{normalized_query}"))
}

fn canonical_query(query: &str) -> String {
    if query.is_empty() {
        return String::new();
    }

    let mut parameters: Vec<QueryParam> = query
        .split('&')
        .enumerate()
        .map(|(order, pair)| {
            let name_end = pair.find('=').unwrap_or(pair.len());
            QueryParam {
                decoded_name: decode_query_component(&pair.as_bytes()[..name_end]),
                raw: pair.to_owned(),
                order,
            }
        })
        .collect();

    let has_duplicate_names = (0..parameters.len()).any(|left| {
        (left + 1..parameters.len())
            .any(|right| parameters[left].decoded_name == parameters[right].decoded_name)
    });
    if !has_duplicate_names {
        parameters.sort_by(|left, right| {
            left.decoded_name
                .cmp(&right.decoded_name)
                .then_with(|| left.order.cmp(&right.order))
        });
    }
    parameters
        .into_iter()
        .map(|parameter| parameter.raw)
        .collect::<Vec<_>>()
        .join("&")
}

struct QueryParam {
    decoded_name: Vec<u8>,
    raw: String,
    order: usize,
}

fn decode_query_component(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        match input[index] {
            b'+' => {
                output.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < input.len() => {
                let high = hex_value(input[index + 1]);
                let low = hex_value(input[index + 2]);
                match (high, low) {
                    (Some(high), Some(low)) => {
                        output.push((high << 4) | low);
                        index += 3;
                    }
                    _ => {
                        output.push(input[index]);
                        index += 1;
                    }
                }
            }
            byte => {
                output.push(byte);
                index += 1;
            }
        }
    }
    output
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn response_content_length(headers: &HeaderMap) -> Option<usize> {
    let mut values = headers.get_all(CONTENT_LENGTH).iter();
    let value = values.next()?;
    if values.next().is_some() {
        return None;
    }
    value.to_str().ok()?.trim().parse().ok()
}

fn response_has_bypass_headers(headers: &HeaderMap) -> bool {
    if headers
        .get_all(CACHE_CONTROL)
        .iter()
        .flat_map(|value| value.to_str().ok().unwrap_or_default().split(','))
        .any(|directive| directive.trim().eq_ignore_ascii_case("no-store"))
    {
        return true;
    }

    headers.get(VARY).is_some_and(|value| {
        !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_whitespace())
    })
}

fn end_to_end_headers(headers: &HeaderMap) -> HeaderMap {
    let mut output = headers.clone();
    remove_frame_headers(&mut output);
    output
}

fn strip_frame_headers(parts: http::response::Parts) -> http::response::Parts {
    let mut parts = parts;
    remove_frame_headers(&mut parts.headers);
    parts
}

fn remove_frame_headers(headers: &mut HeaderMap) {
    headers.remove(CONTENT_LENGTH);
    headers.remove(TRANSFER_ENCODING);
    headers.remove(CONNECTION);
    headers.remove("keep-alive");
    headers.remove("proxy-connection");
    headers.remove("trailer");
    headers.remove("upgrade");
    headers.remove(X_GIFT_CACHE);
}

fn build_cached_response(entry: &CachedGiftResponse, cache_status: &'static str) -> Response {
    let mut response = Response::new(Body::from(entry.body.clone()));
    *response.status_mut() = entry.status;
    *response.headers_mut() = end_to_end_headers(&entry.headers);
    set_cache_header(&mut response, cache_status);
    response
}

fn with_cache_header(mut response: Response, cache_status: &'static str) -> Response {
    set_cache_header(&mut response, cache_status);
    response
}

fn set_cache_header(response: &mut Response, cache_status: &'static str) {
    response.headers_mut().remove(X_GIFT_CACHE);
    response
        .headers_mut()
        .insert(X_GIFT_CACHE, HeaderValue::from_static(cache_status));
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::{ACCEPT_ENCODING, CONTENT_TYPE};
    use std::convert::Infallible;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::task::{Context, Poll};
    use tokio::sync::{oneshot, Barrier};
    use tower::{Service, ServiceBuilder};
    use tower_http::compression::CompressionLayer;

    const JPEG: &[u8] = &[
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0xff, 0xd9,
    ];

    fn response_with_body(status: StatusCode, content_type: &str, body: Bytes) -> Response {
        let mut response = Response::new(Body::from(body.clone()));
        *response.status_mut() = status;
        response.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_str(content_type).expect("valid content type"),
        );
        response
            .headers_mut()
            .insert(CONTENT_LENGTH, HeaderValue::from(body.len()));
        response
    }

    fn request(method: &str, uri: &str) -> Request {
        Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .expect("valid request")
    }

    async fn body_bytes(response: Response) -> Bytes {
        to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body")
    }

    #[test]
    fn canonical_key_sorts_unique_names_and_preserves_duplicate_order() {
        assert_eq!(decode_query_component(b"%61"), b"a");
        let sorted = canonical_gift_key(&request("GET", "/gift?b=two&unknown=x&a=1&month=2026-09"))
            .expect("GET key");
        assert_eq!(sorted, "GET|/gift|a=1&b=two&month=2026-09&unknown=x");

        let encoded_sorted =
            canonical_gift_key(&request("GET", "/gift?%61=1&b=2")).expect("GET key");
        assert_eq!(encoded_sorted, "GET|/gift|%61=1&b=2");

        let duplicate =
            canonical_gift_key(&request("GET", "/gift?month=first&union=all&month=second"))
                .expect("GET key");
        assert_eq!(duplicate, "GET|/gift|month=first&union=all&month=second");

        let first_room = canonical_gift_key(&request("GET", "/gift?room_id=1")).unwrap();
        let second_room = canonical_gift_key(&request("GET", "/gift?room_id=2")).unwrap();
        assert_ne!(first_room, second_room);
        assert_eq!(canonical_gift_key(&request("POST", "/gift")), None);
    }

    #[test]
    fn ttl_is_exactly_ten_seconds_and_expiration_is_counted_once() {
        let cache = GiftResponseCache::default();
        assert_eq!(cache.stats().ttl_seconds, 10);

        let now = Instant::now();
        let entry = CachedGiftResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Bytes::from_static(b"body"),
            stored_at: now,
            expires_at: now + GIFT_CACHE_TTL,
            generation: 0,
        };
        assert_eq!(
            entry.expires_at.duration_since(entry.stored_at),
            GIFT_CACHE_TTL
        );

        cache
            .lock_state()
            .entries
            .put("GET|/gift|".to_owned(), entry.clone());
        cache.lock_state().current_bytes = entry.body.len();

        assert!(cache.lookup("GET|/gift|", LookupKind::Direct).is_some());
        cache
            .lock_state()
            .entries
            .get_mut("GET|/gift|")
            .unwrap()
            .expires_at = Instant::now();
        assert!(cache.lookup("GET|/gift|", LookupKind::Direct).is_none());
        assert_eq!(cache.stats().expired, 1);
        assert_eq!(cache.stats().expired, 1);
        assert_eq!(cache.stats().entries, 0);
        assert_eq!(cache.stats().bytes, 0);
    }

    #[tokio::test]
    async fn caches_error_and_binary_responses_with_status_and_content_type() {
        let cache = GiftResponseCache::default();

        let error_first = cache
            .execute(Some("GET|/gift/error|".to_owned()), async {
                response_with_body(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "text/plain; charset=utf-8",
                    Bytes::from_static(b"upstream failed"),
                )
            })
            .await;
        assert_eq!(error_first.headers()[X_GIFT_CACHE], "MISS");
        let error_second = cache
            .execute(Some("GET|/gift/error|".to_owned()), async {
                unreachable!("cached error response must be reused")
            })
            .await;
        assert_eq!(error_second.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error_second.headers()[X_GIFT_CACHE], "HIT");
        assert_eq!(
            error_second.headers()[CONTENT_TYPE],
            "text/plain; charset=utf-8"
        );
        assert_eq!(
            body_bytes(error_second).await,
            Bytes::from_static(b"upstream failed")
        );

        let binary_first = cache
            .execute(Some("GET|/gift/avatar|".to_owned()), async {
                response_with_body(StatusCode::OK, "image/jpeg", Bytes::from_static(JPEG))
            })
            .await;
        assert_eq!(binary_first.headers()[X_GIFT_CACHE], "MISS");
        let binary_second = cache
            .execute(Some("GET|/gift/avatar|".to_owned()), async {
                unreachable!("cached binary response must be reused")
            })
            .await;
        assert_eq!(binary_second.headers()[CONTENT_TYPE], "image/jpeg");
        assert_eq!(body_bytes(binary_second).await, Bytes::from_static(JPEG));

        let stats = cache.stats();
        assert_eq!(stats.misses, 2);
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.hit_rate, 0.5);
    }

    #[tokio::test]
    async fn bypasses_no_store_vary_unknown_length_and_oversize_responses() {
        let cache = GiftResponseCache::default();

        let mut no_store = response_with_body(
            StatusCode::OK,
            "application/json",
            Bytes::from_static(b"no-store"),
        );
        no_store
            .headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
        let first = cache
            .execute(Some("GET|/gift/no-store|".to_owned()), async { no_store })
            .await;
        assert_eq!(first.headers()[X_GIFT_CACHE], "MISS");
        let second = cache
            .execute(Some("GET|/gift/no-store|".to_owned()), async {
                let mut response = response_with_body(
                    StatusCode::OK,
                    "application/json",
                    Bytes::from_static(b"no-store"),
                );
                response
                    .headers_mut()
                    .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
                response
            })
            .await;
        assert_eq!(second.headers()[X_GIFT_CACHE], "MISS");

        let mut vary = response_with_body(
            StatusCode::OK,
            "application/json",
            Bytes::from_static(b"vary"),
        );
        vary.headers_mut()
            .insert(VARY, HeaderValue::from_static("Accept-Encoding"));
        cache
            .execute(Some("GET|/gift/vary|".to_owned()), async { vary })
            .await;

        let unknown_length_stream = futures_util::stream::unfold(false, |done| async move {
            if done {
                None
            } else {
                Some((
                    Ok::<Bytes, Infallible>(Bytes::from_static(b"unknown length")),
                    true,
                ))
            }
        });
        cache
            .execute(Some("GET|/gift/no-length|".to_owned()), async {
                Response::new(Body::from_stream(unknown_length_stream))
            })
            .await;

        let long_body = vec![b'x'; GIFT_CACHE_MAX_RESPONSE_BYTES + 1];
        cache
            .execute(Some("GET|/gift/oversize|".to_owned()), async {
                response_with_body(
                    StatusCode::OK,
                    "application/octet-stream",
                    Bytes::from(long_body),
                )
            })
            .await;

        let stats = cache.stats();
        let cached_keys = cache
            .lock_state()
            .entries
            .iter()
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();
        assert_eq!(stats.entries, 0, "cached_keys={cached_keys:?}");
        assert_eq!(stats.misses, 5);
        assert_eq!(stats.bypasses, 5);
    }

    #[tokio::test]
    async fn fifty_same_key_requests_execute_handler_once() {
        let cache = GiftResponseCache::default();
        let barrier = Arc::new(Barrier::new(50));
        let executions = Arc::new(AtomicUsize::new(0));
        let mut tasks = Vec::with_capacity(50);

        for _ in 0..50 {
            let cache = cache.clone();
            let barrier = Arc::clone(&barrier);
            let executions = Arc::clone(&executions);
            tasks.push(tokio::spawn(async move {
                barrier.wait().await;
                cache
                    .execute(
                        Some("GET|/gift/concurrent|room_id=1".to_owned()),
                        async move {
                            executions.fetch_add(1, Ordering::SeqCst);
                            tokio::time::sleep(Duration::from_millis(300)).await;
                            response_with_body(
                                StatusCode::OK,
                                "application/json",
                                Bytes::from_static(b"same response"),
                            )
                        },
                    )
                    .await
            }));
        }

        for task in tasks {
            let response = task.await.expect("request task");
            assert_eq!(
                body_bytes(response).await,
                Bytes::from_static(b"same response")
            );
        }
        assert_eq!(executions.load(Ordering::SeqCst), 1);
        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert!(stats.hits + stats.coalesced_hits >= 49);
        assert_eq!(stats.inflight_keys, 0);
    }

    #[tokio::test]
    async fn canceling_leader_releases_key_lock_for_next_request() {
        let cache = GiftResponseCache::default();
        let (started_tx, started_rx) = oneshot::channel::<()>();
        let (_never_tx, never_rx) = oneshot::channel::<()>();
        let task = tokio::spawn({
            let cache = cache.clone();
            async move {
                cache
                    .execute(Some("GET|/gift/cancel|".to_owned()), async move {
                        let _ = started_tx.send(());
                        let _ = never_rx.await;
                        response_with_body(
                            StatusCode::OK,
                            "application/json",
                            Bytes::from_static(b"canceled"),
                        )
                    })
                    .await
            }
        });
        started_rx.await.expect("leader started");
        assert_eq!(cache.stats().inflight_keys, 1);
        task.abort();
        let _ = task.await;

        let response = tokio::time::timeout(
            Duration::from_secs(1),
            cache.execute(Some("GET|/gift/cancel|".to_owned()), async {
                response_with_body(
                    StatusCode::OK,
                    "application/json",
                    Bytes::from_static(b"after cancel"),
                )
            }),
        )
        .await
        .expect("next request must not wait forever");
        assert_eq!(
            body_bytes(response).await,
            Bytes::from_static(b"after cancel")
        );
        assert_eq!(cache.stats().inflight_keys, 0);
    }

    #[tokio::test]
    async fn bounded_lru_evicts_oldest_entry() {
        let cache =
            GiftResponseCache::with_limits(GIFT_CACHE_TTL, 2, 1024, 256, GIFT_CACHE_MAX_KEY_BYTES);
        let mut executions = 0;

        for key in ["GET|/gift|a=1", "GET|/gift|a=2", "GET|/gift|a=3"] {
            cache
                .execute(Some(key.to_owned()), async {
                    executions += 1;
                    response_with_body(
                        StatusCode::OK,
                        "application/json",
                        Bytes::from_static(b"1234"),
                    )
                })
                .await;
        }
        assert_eq!(cache.stats().entries, 2);
        assert_eq!(cache.stats().evictions, 1);

        cache
            .execute(Some("GET|/gift|a=1".to_owned()), async {
                executions += 1;
                response_with_body(
                    StatusCode::OK,
                    "application/json",
                    Bytes::from_static(b"new"),
                )
            })
            .await;
        assert_eq!(executions, 4);
        assert_eq!(cache.stats().evictions, 2);
    }

    #[tokio::test]
    async fn clear_invalidates_entries_and_rejects_old_inflight_write() {
        let cache = GiftResponseCache::default();
        let (started_tx, started_rx) = oneshot::channel::<()>();
        let (release_tx, release_rx) = oneshot::channel::<()>();
        let slow_task = tokio::spawn({
            let cache = cache.clone();
            async move {
                cache
                    .execute(Some("GET|/gift/slow|".to_owned()), async move {
                        let _ = started_tx.send(());
                        let _ = release_rx.await;
                        response_with_body(
                            StatusCode::OK,
                            "application/json",
                            Bytes::from_static(b"old"),
                        )
                    })
                    .await
            }
        });
        started_rx.await.expect("slow request started");
        cache.clear();
        release_tx.send(()).expect("release slow request");
        let old_response = slow_task.await.expect("slow task");
        assert_eq!(body_bytes(old_response).await, Bytes::from_static(b"old"));
        assert_eq!(cache.stats().entries, 0);

        let new_response = cache
            .execute(Some("GET|/gift/slow|".to_owned()), async {
                response_with_body(
                    StatusCode::OK,
                    "application/json",
                    Bytes::from_static(b"new"),
                )
            })
            .await;
        assert_eq!(body_bytes(new_response).await, Bytes::from_static(b"new"));
        let cached_response = cache
            .execute(Some("GET|/gift/slow|".to_owned()), async {
                unreachable!("new generation response must be cached")
            })
            .await;
        assert_eq!(cached_response.headers()[X_GIFT_CACHE], "HIT");
        assert_eq!(
            body_bytes(cached_response).await,
            Bytes::from_static(b"new")
        );
    }

    #[tokio::test]
    async fn cache_sits_inside_compression_layer_for_gzip_and_identity() {
        let cache = GiftResponseCache::default();
        let payload = Bytes::from(vec![b'c'; 4096]);
        let mut compressed_service = ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .service(CompressionTestService { cache, payload });

        let gzip_request = Request::builder()
            .uri("/gift/compressed")
            .header(ACCEPT_ENCODING, "gzip")
            .body(Body::empty())
            .unwrap();
        let gzip_response = call_service(&mut compressed_service, gzip_request).await;
        assert_eq!(gzip_response.headers()[X_GIFT_CACHE], "MISS");
        assert_eq!(
            gzip_response
                .headers()
                .get(http::header::CONTENT_ENCODING)
                .and_then(|value| value.to_str().ok()),
            Some("gzip"),
            "headers={:?}",
            gzip_response.headers()
        );
        let identity_request = Request::builder()
            .uri("/gift/compressed")
            .header(ACCEPT_ENCODING, "identity")
            .body(Body::empty())
            .unwrap();
        let identity_response = call_service(&mut compressed_service, identity_request).await;
        assert_eq!(identity_response.headers()[X_GIFT_CACHE], "HIT");
        assert!(!identity_response
            .headers()
            .contains_key(http::header::CONTENT_ENCODING));
    }

    #[derive(Clone)]
    struct CompressionTestService {
        cache: GiftResponseCache,
        payload: Bytes,
    }

    impl Service<Request> for CompressionTestService {
        type Response = Response;
        type Error = Infallible;
        type Future = Pin<Box<dyn Future<Output = Result<Response, Infallible>> + Send + 'static>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _request: Request) -> Self::Future {
            let cache = self.cache.clone();
            let payload = self.payload.clone();
            Box::pin(async move {
                Ok(cache
                    .execute(Some("GET|/gift/compressed|".to_owned()), async move {
                        response_with_body(StatusCode::OK, "application/json", payload)
                    })
                    .await)
            })
        }
    }

    async fn call_service<S>(service: &mut S, request: Request) -> S::Response
    where
        S: Service<Request>,
        S::Error: std::fmt::Debug,
    {
        std::future::poll_fn(|context| service.poll_ready(context))
            .await
            .expect("service readiness");
        service.call(request).await.expect("service response")
    }
}
