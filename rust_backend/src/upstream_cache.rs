//! Shared, source-scoped API cache. HTTP response caching remains a separate layer.
use chrono::{Local, TimeZone};
use lru::LruCache;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex as AsyncMutex, RwLock};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
const MAX_ENTRIES: usize = 512;
const MAX_BYTES: usize = 128 * 1024 * 1024;
const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;
const HISTORY_TTL: Duration = Duration::from_secs(24 * 3600);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Vr,
    Psp,
}

impl Source {
    pub fn from_union(union: &str) -> Self {
        if union.eq_ignore_ascii_case("PSPlive") || union.eq_ignore_ascii_case("psp") {
            Self::Psp
        } else {
            Self::Vr
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Vr => "vr",
            Self::Psp => "psp",
        }
    }
    fn base(self) -> &'static str {
        match self {
            Self::Vr => "https://vr.qianqiuzy.cn",
            Self::Psp => "https://psp.qianqiuzy.cn",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endpoint {
    Gift,
    Month,
    Sessions,
    Attention,
    Sc,
}

impl Endpoint {
    fn path(self) -> &'static str {
        match self {
            Self::Gift => "/gift",
            Self::Month => "/gift/by_month",
            Self::Sessions => "/gift/live_sessions",
            Self::Attention => "/gift/attention",
            Self::Sc => "/gift/sc",
        }
    }
    fn slug(self) -> &'static str {
        match self {
            Self::Gift => "gift",
            Self::Month => "month",
            Self::Sessions => "sessions",
            Self::Attention => "attention",
            Self::Sc => "sc",
        }
    }
}

pub fn normalize_month(month: &str) -> Option<String> {
    if month.len() != 6 || !month.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let year: i32 = month[..4].parse().ok()?;
    let mon: u32 = month[4..].parse().ok()?;
    chrono::NaiveDate::from_ymd_opt(year, mon, 1)?;
    if year < 1900 {
        return None;
    }
    Some(month.to_owned())
}

#[derive(Clone, Debug)]
pub struct Key {
    pub source: Source,
    pub endpoint: Endpoint,
    room: Option<String>,
    month: String,
}

impl Key {
    pub fn new(
        source: Source,
        endpoint: Endpoint,
        room: Option<&str>,
        month: &str,
    ) -> Result<Self, Error> {
        let month = normalize_month(month).ok_or("month must be YYYYMM")?;
        let room = match room {
            Some(room) => {
                if !room.bytes().all(|b| b.is_ascii_digit()) {
                    return Err("invalid room_id".into());
                }
                let id = room.parse::<u64>()?;
                if id == 0 {
                    return Err("room_id must be positive".into());
                }
                Some(id.to_string())
            }
            None => None,
        };
        if matches!(
            endpoint,
            Endpoint::Sessions | Endpoint::Attention | Endpoint::Sc
        ) && room.is_none()
        {
            return Err("room_id required".into());
        }
        Ok(Self {
            source,
            endpoint,
            room,
            month,
        })
    }
    fn id(&self) -> String {
        format!(
            "{}_{}_{}_{}",
            self.source.name(),
            self.endpoint.slug(),
            self.room.as_deref().unwrap_or("all"),
            self.month
        )
    }
    fn metric(&self) -> String {
        format!("{}:{}", self.source.name(), self.endpoint.path())
    }
    fn url(&self) -> String {
        let mut url = format!("{}{}", self.source.base(), self.endpoint.path());
        if self.endpoint == Endpoint::Gift {
            return url;
        }
        if self.endpoint == Endpoint::Sc {
            url.push_str(&format!("?room_id={}", self.room.as_deref().unwrap()));
        } else {
            url.push_str(&format!("?month={}", self.month));
            if let Some(room) = &self.room {
                url.push_str(&format!("&room_id={room}"));
            }
        }
        url
    }
    fn historical(&self) -> bool {
        self.month < Local::now().format("%Y%m").to_string()
    }
    fn persistent(&self) -> bool {
        matches!(self.endpoint, Endpoint::Sessions | Endpoint::Attention)
            || (self.endpoint == Endpoint::Month && self.historical())
    }
    fn rows<'a>(&self, data: &'a Value) -> Option<&'a Value> {
        match self.endpoint {
            Endpoint::Gift | Endpoint::Month => Some(data),
            Endpoint::Sessions => data
                .get("sessions")
                .or_else(|| data.as_array().map(|_| data)),
            Endpoint::Sc => data.get("list").or_else(|| data.as_array().map(|_| data)),
            Endpoint::Attention => data.get("attention").or(Some(data)),
        }
    }
    fn validate(&self, data: &Value) -> Result<(), Error> {
        if data.get("error").is_some() {
            return Err("upstream returned an error payload".into());
        }
        let rows = self
            .rows(data)
            .ok_or("upstream response missing data array")?;
        if rows
            .as_array()
            .is_some_and(|rows| rows.iter().all(Value::is_object))
            || (self.endpoint == Endpoint::Attention && rows.is_object())
        {
            Ok(())
        } else {
            Err("invalid upstream response shape".into())
        }
    }
    fn empty(&self, data: &Value) -> bool {
        self.rows(data).is_some_and(|v| {
            v.as_array().is_some_and(Vec::is_empty)
                || v.as_object().is_some_and(serde_json::Map::is_empty)
        })
    }
    fn fresh(&self, entry: &Entry, now: SystemTime) -> bool {
        let Ok(age) = now.duration_since(entry.fetched_at) else {
            return false;
        };
        if self.empty(&entry.data) {
            return age < Duration::from_secs(if self.historical() { 3600 } else { 60 });
        }
        if self.historical() {
            return age < HISTORY_TTL;
        }
        if matches!(self.endpoint, Endpoint::Gift | Endpoint::Month) {
            return age < Duration::from_secs(300);
        }
        // Compare the refresh boundary of the fetched date, not today's date:
        // an old entry must not become valid again after midnight.
        let fetched: chrono::DateTime<Local> = entry.fetched_at.into();
        let Some(boundary) = fetched.date_naive().and_hms_opt(1, 30, 0) else {
            return false;
        };
        let boundary = if fetched.naive_local() < boundary {
            boundary
        } else {
            boundary + chrono::Duration::days(1)
        };
        Local
            .from_local_datetime(&boundary)
            .earliest()
            .is_some_and(|end| {
                let now: chrono::DateTime<Local> = now.into();
                now < end
            })
    }
}

#[derive(Clone)]
struct Entry {
    data: Value,
    fetched_at: SystemTime,
    bytes: usize,
}

#[derive(Serialize, Deserialize)]
struct DiskEntry {
    key: String,
    fetched_at: u64,
    data: Value,
}

#[derive(Clone, Default, Serialize)]
pub struct Metrics {
    pub hits: u64,
    pub disk_hits: u64,
    pub coalesced_hits: u64,
    pub upstream_requests: u64,
    pub upstream_errors: u64,
    pub empty_responses: u64,
    pub disk_errors: u64,
}

#[derive(Serialize)]
pub struct Stats {
    pub version: &'static str,
    pub pid: u32,
    pub started_at: String,
    pub entries: usize,
    pub bytes: usize,
    pub max_bytes: usize,
    pub attention_entries: usize,
    pub live_sessions_entries: usize,
    pub evictions: u64,
    pub invalidations: u64,
    pub totals: Metrics,
    pub by_endpoint: BTreeMap<String, Metrics>,
    pub by_caller: BTreeMap<String, Metrics>,
}

enum Event {
    Hit,
    DiskHit,
    Coalesced,
    Request,
    Error,
    Empty,
    DiskError,
}
impl Metrics {
    fn record(&mut self, event: &Event) {
        match event {
            Event::Hit => self.hits += 1,
            Event::DiskHit => self.disk_hits += 1,
            Event::Coalesced => self.coalesced_hits += 1,
            Event::Request => self.upstream_requests += 1,
            Event::Error => self.upstream_errors += 1,
            Event::Empty => self.empty_responses += 1,
            Event::DiskError => self.disk_errors += 1,
        }
    }
}

struct State {
    entries: LruCache<String, Entry>,
    bytes: usize,
    evictions: u64,
    invalidations: u64,
    totals: Metrics,
    endpoints: BTreeMap<String, Metrics>,
    callers: BTreeMap<String, Metrics>,
    locks: HashMap<String, Weak<AsyncMutex<()>>>,
}

pub struct UpstreamCache {
    root: PathBuf,
    state: Mutex<State>,
    // Clear waits for in-flight reads/writes. Nothing started before a successful
    // clear can repopulate memory or disk after it returns.
    refresh_gate: RwLock<()>,
    started_at: String,
}

impl UpstreamCache {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            refresh_gate: RwLock::new(()),
            started_at: Local::now().to_rfc3339(),
            state: Mutex::new(State {
                entries: LruCache::new(NonZeroUsize::new(MAX_ENTRIES).unwrap()),
                bytes: 0,
                evictions: 0,
                invalidations: 0,
                totals: Metrics::default(),
                endpoints: BTreeMap::new(),
                callers: BTreeMap::new(),
                locks: HashMap::new(),
            }),
        }
    }
    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn event(&self, key: &Key, caller: &str, event: Event) {
        let mut s = self.state();
        s.totals.record(&event);
        s.endpoints.entry(key.metric()).or_default().record(&event);
        s.callers
            .entry(caller.to_owned())
            .or_default()
            .record(&event);
    }
    fn lookup(&self, key: &Key) -> Option<Value> {
        let mut s = self.state();
        let entry = s.entries.get(&key.id())?;
        if key.fresh(entry, SystemTime::now()) {
            return Some(entry.data.clone());
        }
        if let Some(entry) = s.entries.pop(&key.id()) {
            s.bytes = s.bytes.saturating_sub(entry.bytes);
        }
        None
    }
    fn store(&self, key: &Key, entry: Entry) {
        if entry.bytes > MAX_RESPONSE_BYTES {
            return;
        }
        let mut s = self.state();
        if let Some(old) = s.entries.pop(&key.id()) {
            s.bytes -= old.bytes;
        }
        while s.bytes + entry.bytes > MAX_BYTES {
            if let Some((_, old)) = s.entries.pop_lru() {
                s.bytes -= old.bytes;
                s.evictions += 1;
            } else {
                break;
            }
        }
        s.bytes += entry.bytes;
        if let Some((_, old)) = s.entries.push(key.id(), entry) {
            s.bytes -= old.bytes;
            s.evictions += 1;
        }
    }
    fn path(&self, key: &Key) -> PathBuf {
        self.root.join(format!("{}.json", key.id()))
    }
    async fn disk_load(&self, key: &Key, caller: &str) -> Option<Entry> {
        if !key.persistent() {
            return None;
        }
        let bytes = match tokio::fs::read(self.path(key)).await {
            Ok(bytes) if bytes.len() <= MAX_RESPONSE_BYTES + 4096 => bytes,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return None,
            _ => {
                self.event(key, caller, Event::DiskError);
                return None;
            }
        };
        let entry = serde_json::from_slice::<DiskEntry>(&bytes)
            .ok()
            .and_then(|disk| {
                if disk.key != key.id() || key.validate(&disk.data).is_err() {
                    return None;
                }
                Some(Entry {
                    bytes: serde_json::to_vec(&disk.data).ok()?.len(),
                    data: disk.data,
                    fetched_at: UNIX_EPOCH.checked_add(Duration::from_secs(disk.fetched_at))?,
                })
            });
        let Some(entry) = entry else {
            self.event(key, caller, Event::DiskError);
            return None;
        };
        key.fresh(&entry, SystemTime::now()).then_some(entry)
    }
    async fn disk_store(&self, key: &Key, caller: &str, entry: &Entry) {
        if !key.persistent() || entry.bytes > MAX_RESPONSE_BYTES {
            return;
        }
        let path = self.path(key);
        let tmp = path.with_extension(format!("{}.tmp", uuid::Uuid::new_v4()));
        let result: Result<(), Error> = async {
            tokio::fs::create_dir_all(&self.root).await?;
            let bytes = serde_json::to_vec(&DiskEntry {
                key: key.id(),
                fetched_at: entry.fetched_at.duration_since(UNIX_EPOCH)?.as_secs(),
                data: entry.data.clone(),
            })?;
            tokio::fs::write(&tmp, bytes).await?;
            tokio::fs::rename(&tmp, &path).await?;
            Ok(())
        }
        .await;
        if result.is_err() {
            self.event(key, caller, Event::DiskError);
            let _ = tokio::fs::remove_file(tmp).await;
            tracing::warn!(
                source = key.source.name(),
                endpoint = key.endpoint.path(),
                "upstream cache disk write failed"
            );
        }
    }
    pub async fn get(&self, client: &Client, key: Key, caller: &str) -> Result<Value, Error> {
        self.get_with(key.clone(), caller, || fetch_json(client, &key, caller))
            .await
    }
    async fn get_with<F, Fut>(&self, key: Key, caller: &str, fetch: F) -> Result<Value, Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Value, Error>>,
    {
        let lock = {
            let mut s = self.state();
            s.locks.retain(|_, lock| lock.strong_count() > 0);
            let slot = s.locks.entry(key.id()).or_default();
            match slot.upgrade() {
                Some(lock) => lock,
                None => {
                    let lock = Arc::new(AsyncMutex::new(()));
                    *slot = Arc::downgrade(&lock);
                    lock
                }
            }
        };
        let waited = lock.try_lock().is_err();
        let _key_guard = lock.lock().await;
        let _refresh_guard = self.refresh_gate.read().await;
        if let Some(data) = self.lookup(&key) {
            self.event(
                &key,
                caller,
                if waited { Event::Coalesced } else { Event::Hit },
            );
            return Ok(data);
        }
        if let Some(entry) = self.disk_load(&key, caller).await {
            self.event(&key, caller, Event::DiskHit);
            let data = entry.data.clone();
            self.store(&key, entry);
            return Ok(data);
        }
        let data = self.request_with(&key, caller, fetch).await?;
        let entry = Entry {
            bytes: serde_json::to_vec(&data)?.len(),
            data: data.clone(),
            fetched_at: SystemTime::now(),
        };
        self.disk_store(&key, caller, &entry).await;
        self.store(&key, entry);
        Ok(data)
    }
    async fn request_with<F, Fut>(&self, key: &Key, caller: &str, fetch: F) -> Result<Value, Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Value, Error>>,
    {
        self.event(key, caller, Event::Request);
        tracing::info!(
            version = env!("CARGO_PKG_VERSION"),
            source = key.source.name(),
            endpoint = key.endpoint.path(),
            caller,
            room_id = key.room.as_deref().unwrap_or("all"),
            month = key.month,
            "upstream request"
        );
        let result = match fetch().await {
            Ok(data) => key.validate(&data).map(|_| data),
            Err(e) => Err(e),
        };
        match &result {
            Ok(data) if key.empty(data) => self.event(key, caller, Event::Empty),
            Err(_) => {
                self.event(key, caller, Event::Error);
                tracing::warn!(
                    source = key.source.name(),
                    endpoint = key.endpoint.path(),
                    caller,
                    "upstream request failed"
                );
            }
            _ => {}
        }
        result
    }
    pub async fn uncached(&self, client: &Client, key: Key, caller: &str) -> Result<Value, Error> {
        self.request_with(&key, caller, || fetch_json(client, &key, caller))
            .await
    }
    pub async fn clear(&self, sources: &[Source], room: &str, month: &str) -> Result<(), Error> {
        // Validate everything before mutating anything or constructing file paths.
        let mut keys = Vec::new();
        for &source in sources {
            for endpoint in [
                Endpoint::Gift,
                Endpoint::Month,
                Endpoint::Sessions,
                Endpoint::Attention,
            ] {
                let selected_month = if endpoint == Endpoint::Gift {
                    Local::now().format("%Y%m").to_string()
                } else {
                    month.to_owned()
                };
                keys.push(Key::new(
                    source,
                    endpoint,
                    matches!(endpoint, Endpoint::Sessions | Endpoint::Attention).then_some(room),
                    &selected_month,
                )?);
            }
        }
        let _guard = self.refresh_gate.write().await;
        for key in &keys {
            match tokio::fs::remove_file(self.path(key)).await {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    self.event(key, "manual_refresh", Event::DiskError);
                    return Err(e.into());
                }
            }
        }
        let mut s = self.state();
        for key in keys {
            if let Some(entry) = s.entries.pop(&key.id()) {
                s.bytes -= entry.bytes;
            }
        }
        s.invalidations += 1;
        Ok(())
    }
    pub fn stats(&self) -> Stats {
        let s = self.state();
        Stats {
            version: env!("CARGO_PKG_VERSION"),
            pid: std::process::id(),
            started_at: self.started_at.clone(),
            entries: s.entries.len(),
            bytes: s.bytes,
            max_bytes: MAX_BYTES,
            attention_entries: s
                .entries
                .iter()
                .filter(|(k, _)| k.contains("_attention_"))
                .count(),
            live_sessions_entries: s
                .entries
                .iter()
                .filter(|(k, _)| k.contains("_sessions_"))
                .count(),
            evictions: s.evictions,
            invalidations: s.invalidations,
            totals: s.totals.clone(),
            by_endpoint: s.endpoints.clone(),
            by_caller: s.callers.clone(),
        }
    }
}

async fn fetch_json(client: &Client, key: &Key, caller: &str) -> Result<Value, Error> {
    fetch_json_from_url(client, &key.url(), caller).await
}

async fn fetch_json_from_url(client: &Client, url: &str, caller: &str) -> Result<Value, Error> {
    let mut response = client
        .get(url)
        .header(
            "User-Agent",
            concat!("liveshow-backend/", env!("CARGO_PKG_VERSION")),
        )
        .header("X-Liveshow-Caller", caller)
        .header("Accept", "application/json")
        .send()
        .await?
        .error_for_status()?;
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if bytes.len() + chunk.len() > MAX_RESPONSE_BYTES {
            return Err("upstream response exceeds 16 MiB".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(serde_json::from_slice(&bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::sync::{oneshot, Notify};

    struct TestDir(PathBuf);
    impl TestDir {
        fn new() -> Self {
            Self(std::env::temp_dir().join(format!("liveshow-cache-test-{}", uuid::Uuid::new_v4())))
        }
        fn cache(&self) -> UpstreamCache {
            UpstreamCache::new(self.0.clone())
        }
    }
    impl Drop for TestDir {
        fn drop(&mut self) {
            if self.0.exists() {
                std::fs::remove_dir_all(&self.0).unwrap();
            }
        }
    }
    fn current(endpoint: Endpoint, room: Option<&str>) -> Key {
        Key::new(
            Source::Vr,
            endpoint,
            room,
            &Local::now().format("%Y%m").to_string(),
        )
        .unwrap()
    }
    fn historical(room: &str) -> Key {
        Key::new(Source::Vr, Endpoint::Sessions, Some(room), "200001").unwrap()
    }
    fn entry(data: Value, age: u64) -> Entry {
        Entry {
            bytes: serde_json::to_vec(&data).unwrap().len(),
            data,
            fetched_at: SystemTime::now() - Duration::from_secs(age),
        }
    }

    #[tokio::test]
    async fn callers_share_source_cache_and_explicit_current_month_is_cached() {
        let dir = TestDir::new();
        let cache = dir.cache();
        let key = current(Endpoint::Gift, None);
        for caller in ["anchor_list", "session_name", "avatars"] {
            assert_eq!(
                cache
                    .get_with(key.clone(), caller, || async {
                        Ok(json!([{"room_id": 1}]))
                    })
                    .await
                    .unwrap(),
                json!([{"room_id": 1}])
            );
        }
        let month = current(Endpoint::Month, None);
        cache
            .get_with(month.clone(), "all_filter", || async {
                Ok(json!([{"room_id": 1}]))
            })
            .await
            .unwrap();
        cache
            .get_with(month, "vr_filter", || async {
                panic!("filters must share a source snapshot")
            })
            .await
            .unwrap();
        let mut psp = key;
        psp.source = Source::Psp;
        cache
            .get_with(psp, "anchor_list", || async { Ok(json!([{"room_id": 2}])) })
            .await
            .unwrap();
        let stats = cache.stats();
        assert_eq!(stats.totals.upstream_requests, 3);
        assert_eq!(stats.totals.hits, 3);
        assert_eq!(stats.by_caller["session_name"].upstream_requests, 0);
        assert_eq!(stats.by_endpoint["vr:/gift"].upstream_requests, 1);
    }

    #[tokio::test]
    async fn concurrent_callers_and_aliases_fetch_once() {
        let dir = TestDir::new();
        let cache = Arc::new(dir.cache());
        let counter = Arc::new(AtomicUsize::new(0));
        let mut tasks = tokio::task::JoinSet::new();
        for i in 0..20 {
            let cache = cache.clone();
            let counter = counter.clone();
            tasks.spawn(async move {
                let key = current(
                    Endpoint::Sessions,
                    Some(if i % 2 == 0 { "01" } else { "1" }),
                );
                cache
                    .get_with(key, "live_sessions", || async {
                        counter.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(Duration::from_millis(30)).await;
                        Ok(json!({"sessions": []}))
                    })
                    .await
                    .unwrap()
            });
        }
        while let Some(result) = tasks.join_next().await {
            assert_eq!(result.unwrap(), json!({"sessions": []}));
        }
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(cache.stats().totals.empty_responses, 1);
        assert_eq!(
            cache.stats().totals.coalesced_hits + cache.stats().totals.hits,
            19
        );
    }

    #[tokio::test]
    async fn empty_results_survive_restart_but_errors_do_not_poison_cache() {
        let dir = TestDir::new();
        let key = historical("42");
        dir.cache()
            .get_with(key.clone(), "test", || async {
                Ok(json!({"sessions": []}))
            })
            .await
            .unwrap();
        let cache = dir.cache();
        cache
            .get_with(key.clone(), "test", || async {
                panic!("valid empty result must load from disk")
            })
            .await
            .unwrap();
        assert_eq!(cache.stats().totals.disk_hits, 1);
        let other = historical("43");
        for data in [
            json!({"error": "unavailable"}),
            json!({"sessions": "bad"}),
            json!({"sessions": [null]}),
        ] {
            assert!(cache
                .get_with(other.clone(), "test", || async { Ok(data) })
                .await
                .is_err());
        }
        assert!(cache
            .get_with(other.clone(), "test", || async {
                Err("network failure".into())
            })
            .await
            .is_err());
        assert!(!cache.path(&other).exists());
        cache
            .get_with(other, "test", || async {
                Ok(json!({"sessions": [{"title": "recovered"}]}))
            })
            .await
            .unwrap();
        assert_eq!(cache.stats().totals.upstream_errors, 4);
    }

    #[test]
    fn ttl_boundaries_and_old_daily_entries_never_revive() {
        let now = SystemTime::now();
        let root = current(Endpoint::Gift, None);
        assert!(root.fresh(&entry(json!([{}]), 299), now));
        assert!(!root.fresh(&entry(json!([{}]), 301), now));
        let key = current(Endpoint::Sessions, Some("1"));
        assert!(key.fresh(&entry(json!({"sessions": []}), 59), now));
        assert!(!key.fresh(&entry(json!({"sessions": []}), 61), now));
        let past = historical("1");
        assert!(past.fresh(&entry(json!({"sessions": []}), 3599), now));
        assert!(!past.fresh(&entry(json!({"sessions": []}), 3601), now));
        assert!(past.fresh(&entry(json!({"sessions": [{}]}), 86399), now));
        assert!(!past.fresh(&entry(json!({"sessions": [{}]}), 86401), now));
        let fetched = Local::now().date_naive().and_hms_opt(2, 0, 0).unwrap();
        let mut daily = entry(json!({"sessions": [{}]}), 0);
        daily.fetched_at = Local
            .from_local_datetime(&fetched)
            .earliest()
            .unwrap()
            .into();
        for (days, hour, minute, expected) in
            [(1, 1, 29, true), (1, 1, 30, false), (2, 0, 0, false)]
        {
            let date = fetched.date() + chrono::Duration::days(days);
            let time = Local
                .from_local_datetime(&date.and_hms_opt(hour, minute, 0).unwrap())
                .earliest()
                .unwrap();
            assert_eq!(key.fresh(&daily, time.into()), expected);
        }
    }

    #[tokio::test]
    async fn expired_or_corrupt_disk_entries_refetch() {
        let dir = TestDir::new();
        let cache = dir.cache();
        let key = historical("1");
        cache
            .disk_store(&key, "test", &entry(json!({"sessions": [{}]}), 86401))
            .await;
        let value = cache
            .get_with(key.clone(), "test", || async {
                Ok(json!({"sessions": [{"title": "new"}]}))
            })
            .await
            .unwrap();
        assert_eq!(value["sessions"][0]["title"], "new");
        tokio::fs::write(cache.path(&key), b"not json")
            .await
            .unwrap();
        let restarted = dir.cache();
        restarted
            .get_with(key, "test", || async { Ok(json!({"sessions": []})) })
            .await
            .unwrap();
        assert_eq!(restarted.stats().totals.disk_errors, 1);
        assert_eq!(restarted.stats().totals.upstream_requests, 1);
    }

    #[tokio::test]
    async fn clear_removes_disk_and_memory_but_preserves_other_sources_and_rooms() {
        let dir = TestDir::new();
        let cache = dir.cache();
        let vr = historical("1");
        let mut psp = vr.clone();
        psp.source = Source::Psp;
        let other = historical("2");
        for key in [&vr, &psp, &other] {
            cache
                .get_with(key.clone(), "test", || async {
                    Ok(json!({"sessions": [{}]}))
                })
                .await
                .unwrap();
        }
        cache.clear(&[Source::Vr], "1", "200001").await.unwrap();
        assert!(!cache.path(&vr).exists());
        assert!(cache.path(&psp).exists());
        assert!(cache.path(&other).exists());
        cache
            .get_with(vr, "test", || async {
                Ok(json!({"sessions": [{"title": "fresh"}]}))
            })
            .await
            .unwrap();
        for key in [psp, other] {
            cache
                .get_with(key, "test", || async { panic!("unrelated entry cleared") })
                .await
                .unwrap();
        }
        assert_eq!(cache.stats().totals.upstream_requests, 4);
        assert_eq!(cache.stats().invalidations, 1);
        assert!(cache
            .clear(&[Source::Vr], "../escape", "200001")
            .await
            .is_err());
        assert!(cache.clear(&[Source::Vr], "1", "../../").await.is_err());
        assert_eq!(cache.stats().invalidations, 1);
    }

    #[tokio::test]
    async fn failed_disk_clear_is_reported_and_does_not_claim_invalidation() {
        let dir = TestDir::new();
        let cache = dir.cache();
        let key = historical("1");
        tokio::fs::create_dir_all(cache.path(&key)).await.unwrap();
        cache.store(&key, entry(json!({"sessions": [{}]}), 0));
        assert!(cache.clear(&[Source::Vr], "1", "200001").await.is_err());
        assert!(cache.lookup(&key).is_some());
        assert_eq!(cache.stats().invalidations, 0);
        assert_eq!(cache.stats().totals.disk_errors, 1);
    }

    #[tokio::test]
    async fn clearing_during_fetch_cannot_resurrect_old_disk_data() {
        let dir = TestDir::new();
        let cache = Arc::new(dir.cache());
        let key = historical("1");
        let (started_tx, started_rx) = oneshot::channel();
        let release = Arc::new(Notify::new());
        let worker = tokio::spawn({
            let cache = cache.clone();
            let key = key.clone();
            let release = release.clone();
            async move {
                cache
                    .get_with(key, "test", || async {
                        started_tx.send(()).unwrap();
                        release.notified().await;
                        Ok(json!({"sessions": [{}]}))
                    })
                    .await
                    .unwrap()
            }
        });
        started_rx.await.unwrap();
        let clear = tokio::spawn({
            let cache = cache.clone();
            async move {
                cache.clear(&[Source::Vr], "1", "200001").await.unwrap();
            }
        });
        tokio::task::yield_now().await;
        assert!(!clear.is_finished());
        release.notify_one();
        worker.await.unwrap();
        clear.await.unwrap();
        assert!(!cache.path(&key).exists());
        assert!(cache.lookup(&key).is_none());
        dir.cache()
            .get_with(key, "test", || async {
                Ok(json!({"sessions": [{"title": "after clear"}]}))
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn cancelled_leader_releases_single_flight() {
        let dir = TestDir::new();
        let cache = Arc::new(dir.cache());
        let key = historical("1");
        let (tx, rx) = oneshot::channel();
        let worker = tokio::spawn({
            let cache = cache.clone();
            let key = key.clone();
            async move {
                cache
                    .get_with(key, "test", || async {
                        tx.send(()).unwrap();
                        std::future::pending().await
                    })
                    .await
            }
        });
        rx.await.unwrap();
        worker.abort();
        assert!(worker.await.unwrap_err().is_cancelled());
        tokio::time::timeout(
            Duration::from_secs(1),
            cache.get_with(key, "test", || async { Ok(json!({"sessions": []})) }),
        )
        .await
        .unwrap()
        .unwrap();
    }

    #[test]
    fn lru_capacity_and_source_keys_are_bounded_and_isolated() {
        let dir = TestDir::new();
        let cache = dir.cache();
        for room in 1..=MAX_ENTRIES + 1 {
            cache.store(
                &historical(&room.to_string()),
                entry(json!({"sessions": []}), 0),
            );
        }
        assert_eq!(cache.stats().entries, MAX_ENTRIES);
        assert_eq!(cache.stats().evictions, 1);
        assert!(cache.lookup(&historical("1")).is_none());
        assert!(cache.stats().bytes <= cache.stats().max_bytes);
        assert!(normalize_month("202613").is_none());
        assert!(normalize_month("20261").is_none());
        assert!(normalize_month("２０２６１０").is_none());
        assert_eq!(Source::from_union("psp"), Source::Psp);
    }

    #[tokio::test]
    async fn local_http_checks_status_and_identifies_the_caller() {
        use axum::{routing::get, Router};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let app = Router::new()
            .route(
                "/ok",
                get(|headers: axum::http::HeaderMap| async move {
                    assert_eq!(
                        headers["user-agent"],
                        concat!("liveshow-backend/", env!("CARGO_PKG_VERSION"))
                    );
                    assert_eq!(headers["x-liveshow-caller"], "integration_test");
                    axum::Json(json!({"sessions": []}))
                }),
            )
            .route(
                "/error",
                get(|| async {
                    (
                        axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        "{\"sessions\":[]}",
                    )
                }),
            );
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let client = Client::builder().no_proxy().build().unwrap();
        assert_eq!(
            fetch_json_from_url(&client, &format!("http://{address}/ok"), "integration_test")
                .await
                .unwrap(),
            json!({"sessions": []})
        );
        assert!(fetch_json_from_url(
            &client,
            &format!("http://{address}/error"),
            "integration_test"
        )
        .await
        .is_err());
        task.abort();
    }
}
