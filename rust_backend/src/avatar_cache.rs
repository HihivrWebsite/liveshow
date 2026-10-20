use chrono::{DateTime, Utc};
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::future::Future;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub const AVATAR_DIMENSION: u32 = 40;
pub const MAX_COMPRESSED_BYTES: usize = 100 * 1024;

const MANIFEST_FILE: &str = "manifest.json";
const MANIFEST_SCHEMA: u8 = 1;

#[derive(Debug)]
pub struct AvatarCacheError(String);

impl AvatarCacheError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for AvatarCacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for AvatarCacheError {}

impl From<std::io::Error> for AvatarCacheError {
    fn from(error: std::io::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<image::ImageError> for AvatarCacheError {
    fn from(error: image::ImageError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<serde_json::Error> for AvatarCacheError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<String> for AvatarCacheError {
    fn from(error: String) -> Self {
        Self::new(error)
    }
}

pub type AvatarCacheResult<T> = Result<T, AvatarCacheError>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AvatarEntry {
    pub file: String,
    #[serde(default)]
    pub face_url: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub last_modified: Option<String>,
    pub checked_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub revision: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Manifest {
    schema: u8,
    avatars: BTreeMap<String, AvatarEntry>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            schema: MANIFEST_SCHEMA,
            avatars: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConditionalHeaders {
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Clone, Debug)]
pub struct RemoteAvatarResponse {
    pub status: u16,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshOutcome {
    NotModified,
    Unchanged,
    Updated,
}

#[derive(Clone)]
pub struct AvatarStore {
    inner: Arc<AvatarStoreInner>,
}

struct AvatarStoreInner {
    directory: PathBuf,
    manifest: RwLock<Manifest>,
    commit_lock: Mutex<()>,
}

impl AvatarStore {
    pub fn open(directory: impl AsRef<Path>) -> AvatarCacheResult<Self> {
        let directory = directory.as_ref().to_path_buf();
        std::fs::create_dir_all(&directory)?;

        let manifest_path = directory.join(MANIFEST_FILE);
        let manifest = if manifest_path.exists() {
            let bytes = std::fs::read(&manifest_path)?;
            serde_json::from_slice::<Manifest>(&bytes)?
        } else {
            Manifest::default()
        };
        if manifest.schema != MANIFEST_SCHEMA {
            return Err(AvatarCacheError::new(format!(
                "unsupported avatar manifest schema {}",
                manifest.schema
            )));
        }

        let store = Self {
            inner: Arc::new(AvatarStoreInner {
                directory,
                manifest: RwLock::new(manifest),
                commit_lock: Mutex::new(()),
            }),
        };
        store.migrate_legacy_jpegs()?;
        store.remove_orphans()?;
        Ok(store)
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.inner.directory.join(MANIFEST_FILE)
    }

    pub async fn get(&self, key: &str) -> AvatarCacheResult<Option<Vec<u8>>> {
        let path = {
            let manifest = self.inner.manifest.read().await;
            match manifest.avatars.get(key) {
                Some(entry) => self.resolve_entry_path(entry)?,
                None => return Ok(None),
            }
        };

        match std::fs::read(path) {
            Ok(bytes) if validate_webp(&bytes).is_ok() => Ok(Some(bytes)),
            Ok(_) | Err(_) => Ok(None),
        }
    }

    pub async fn snapshot(&self) -> AvatarCacheResult<BTreeMap<String, Vec<u8>>> {
        let entries = {
            let manifest = self.inner.manifest.read().await;
            manifest
                .avatars
                .iter()
                .map(|(key, entry)| (key.clone(), entry.clone()))
                .collect::<Vec<_>>()
        };
        let mut result = BTreeMap::new();
        for (key, entry) in entries {
            let path = self.resolve_entry_path(&entry)?;
            if let Ok(bytes) = std::fs::read(path) {
                if validate_webp(&bytes).is_ok() {
                    result.insert(key, bytes);
                }
            }
        }
        Ok(result)
    }

    pub async fn entries(&self) -> BTreeMap<String, AvatarEntry> {
        self.inner.manifest.read().await.avatars.clone()
    }

    pub async fn conditional_headers(&self, key: &str) -> ConditionalHeaders {
        let manifest = self.inner.manifest.read().await;
        manifest
            .avatars
            .get(key)
            .map(|entry| ConditionalHeaders {
                etag: entry.etag.clone(),
                last_modified: entry.last_modified.clone(),
            })
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub(crate) async fn put_local(
        &self,
        key: &str,
        webp: &[u8],
        face_url: Option<String>,
    ) -> AvatarCacheResult<RefreshOutcome> {
        validate_webp(webp)?;
        self.install(key, webp.to_vec(), face_url, None, None, Utc::now())
            .await
    }

    pub async fn refresh_from_remote<F, Fut>(
        &self,
        key: &str,
        face_url: &str,
        fetch: F,
    ) -> AvatarCacheResult<RefreshOutcome>
    where
        F: FnOnce(ConditionalHeaders) -> Fut,
        Fut: Future<Output = AvatarCacheResult<RemoteAvatarResponse>>,
    {
        let current = self.get(key).await?;
        let conditional = if current.is_some() {
            self.conditional_headers(key).await
        } else {
            ConditionalHeaders::default()
        };
        let response = fetch(conditional).await?;

        if response.status == 304 {
            if current.is_none() {
                return Err(AvatarCacheError::new(
                    "remote returned 304 but compressed avatar is missing",
                ));
            }
            self.update_metadata(
                key,
                Some(face_url.to_string()),
                response.etag,
                response.last_modified,
            )
            .await?;
            return Ok(RefreshOutcome::NotModified);
        }
        if !(200..300).contains(&response.status) {
            return Err(AvatarCacheError::new(format!(
                "remote avatar returned HTTP {}",
                response.status
            )));
        }
        let body = response.body.ok_or_else(|| {
            AvatarCacheError::new("remote avatar response did not include a body")
        })?;
        let compressed = compress_avatar(&body)?;

        if current.as_deref() == Some(compressed.as_slice()) {
            self.update_metadata(
                key,
                Some(face_url.to_string()),
                response.etag,
                response.last_modified,
            )
            .await?;
            return Ok(RefreshOutcome::Unchanged);
        }

        self.install(
            key,
            compressed,
            Some(face_url.to_string()),
            response.etag,
            response.last_modified,
            Utc::now(),
        )
        .await
    }

    async fn update_metadata(
        &self,
        key: &str,
        face_url: Option<String>,
        etag: Option<String>,
        last_modified: Option<String>,
    ) -> AvatarCacheResult<()> {
        let _commit = self.inner.commit_lock.lock().await;
        let mut manifest = self.inner.manifest.write().await;
        let mut entry = manifest.avatars.get(key).cloned().ok_or_else(|| {
            AvatarCacheError::new(format!("avatar manifest entry missing for {key}"))
        })?;
        if let Some(face_url) = face_url {
            entry.face_url = Some(face_url);
        }
        entry.etag = etag;
        entry.last_modified = last_modified;
        entry.checked_at = Utc::now();
        let next_manifest = Manifest {
            schema: MANIFEST_SCHEMA,
            avatars: {
                let mut avatars = manifest.avatars.clone();
                avatars.insert(key.to_string(), entry);
                avatars
            },
        };
        self.write_manifest_atomic(&next_manifest)?;
        *manifest = next_manifest;
        Ok(())
    }

    async fn install(
        &self,
        key: &str,
        webp: Vec<u8>,
        face_url: Option<String>,
        etag: Option<String>,
        last_modified: Option<String>,
        now: DateTime<Utc>,
    ) -> AvatarCacheResult<RefreshOutcome> {
        validate_webp(&webp)?;
        let _commit = self.inner.commit_lock.lock().await;
        let mut manifest = self.inner.manifest.write().await;
        let previous = manifest.avatars.get(key);
        let outcome = RefreshOutcome::Updated;
        let file_name = format!(
            "{}-{}.webp",
            sanitize_key(key),
            uuid::Uuid::new_v4().simple()
        );
        let file_path = self.inner.directory.join(&file_name);
        write_atomic(&file_path, &webp)?;

        let previous_file = previous.map(|entry| entry.file.clone());
        let next_entry = AvatarEntry {
            file: file_name,
            face_url,
            etag,
            last_modified,
            checked_at: now,
            updated_at: match previous {
                Some(entry) => entry.updated_at,
                None => now,
            },
            revision: previous.map(|entry| entry.revision + 1).unwrap_or(0),
        };
        let mut avatars = manifest.avatars.clone();
        avatars.insert(key.to_string(), next_entry);
        let next_manifest = Manifest {
            schema: MANIFEST_SCHEMA,
            avatars,
        };

        if let Err(error) = self.write_manifest_atomic(&next_manifest) {
            let _ = std::fs::remove_file(&file_path);
            return Err(error);
        }
        *manifest = next_manifest;
        if let Some(previous_file) = previous_file {
            if previous_file != manifest.avatars[key].file {
                let _ = std::fs::remove_file(self.inner.directory.join(previous_file));
            }
        }
        Ok(outcome)
    }

    fn resolve_entry_path(&self, entry: &AvatarEntry) -> AvatarCacheResult<PathBuf> {
        if Path::new(&entry.file).is_absolute()
            || Path::new(&entry.file)
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Err(AvatarCacheError::new(format!(
                "unsafe avatar manifest path {}",
                entry.file
            )));
        }
        Ok(self.inner.directory.join(&entry.file))
    }

    fn write_manifest_atomic(&self, manifest: &Manifest) -> AvatarCacheResult<()> {
        let bytes = serde_json::to_vec_pretty(manifest)?;
        write_atomic(&self.manifest_path(), &bytes)
    }

    fn migrate_legacy_jpegs(&self) -> AvatarCacheResult<()> {
        let legacy_files = std::fs::read_dir(&self.inner.directory)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().and_then(|v| v.to_str()) == Some("jpg"))
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        if legacy_files.is_empty() {
            return Ok(());
        }

        let manifest = match std::fs::read(self.manifest_path()) {
            Ok(bytes) => serde_json::from_slice::<Manifest>(&bytes)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Manifest::default(),
            Err(error) => return Err(error.into()),
        };
        let mut avatars = manifest.avatars;
        let mut migrated_files = Vec::new();
        let mut created_webp_files = Vec::new();

        for legacy_path in legacy_files {
            let key = legacy_path
                .file_stem()
                .and_then(|value| value.to_str())
                .ok_or_else(|| AvatarCacheError::new("legacy avatar filename is not UTF-8"))?
                .to_string();
            if let Some(entry) = avatars.get(&key) {
                let existing_path = self.resolve_entry_path(entry)?;
                if std::fs::read(existing_path)
                    .map(|bytes| validate_webp(&bytes).is_ok())
                    .unwrap_or(false)
                {
                    migrated_files.push(legacy_path);
                    continue;
                }
            }
            let jpeg = std::fs::read(&legacy_path)?;
            let webp = match compress_avatar(&jpeg) {
                Ok(webp) => webp,
                Err(_) => continue,
            };
            let file_name = format!(
                "{}-{}.webp",
                sanitize_key(&key),
                uuid::Uuid::new_v4().simple()
            );
            let file_path = self.inner.directory.join(&file_name);
            write_atomic(&file_path, &webp)?;
            let now = Utc::now();
            avatars.insert(
                key,
                AvatarEntry {
                    file: file_name,
                    face_url: None,
                    etag: None,
                    last_modified: None,
                    checked_at: now,
                    updated_at: now,
                    revision: 0,
                },
            );
            migrated_files.push(legacy_path);
            created_webp_files.push(file_path);
        }

        if migrated_files.is_empty() {
            return Ok(());
        }
        let next_manifest = Manifest {
            schema: MANIFEST_SCHEMA,
            avatars,
        };
        if let Err(error) = self.write_manifest_atomic(&next_manifest) {
            for webp_path in created_webp_files {
                let _ = std::fs::remove_file(webp_path);
            }
            return Err(error);
        }
        // open() 尚未把 store 交给其他任务；try_write 可在同步启动阶段
        // 安全发布迁移结果，不使用会阻塞 async runtime 的 blocking_write。
        let mut published = self.inner.manifest.try_write().map_err(|_| {
            AvatarCacheError::new("avatar manifest was locked during startup migration")
        })?;
        *published = next_manifest;
        drop(published);
        for legacy_path in migrated_files {
            let _ = std::fs::remove_file(legacy_path);
        }
        Ok(())
    }

    fn remove_orphans(&self) -> AvatarCacheResult<()> {
        let referenced = {
            // 启动阶段独占，但保持与运行期读锁一致。
            // open 尚未进入异步任务，这里直接读清单文件避免跨期状态。
            match std::fs::read(self.manifest_path()) {
                Ok(bytes) => serde_json::from_slice::<Manifest>(&bytes)?
                    .avatars
                    .into_values()
                    .map(|entry| entry.file)
                    .collect::<std::collections::HashSet<_>>(),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => Default::default(),
                Err(error) => return Err(error.into()),
            }
        };
        for file in std::fs::read_dir(&self.inner.directory)? {
            let path = file?.path();
            let name = match path.file_name().and_then(|value| value.to_str()) {
                Some(value) => value.to_string(),
                None => continue,
            };
            if name.starts_with('.') && name.ends_with(".tmp") {
                let _ = std::fs::remove_file(path);
                continue;
            }
            if path.extension().and_then(|value| value.to_str()) == Some("webp")
                && !referenced.contains(&name)
            {
                let _ = std::fs::remove_file(path);
            }
        }
        Ok(())
    }
}

pub fn compress_avatar(image_bytes: &[u8]) -> AvatarCacheResult<Vec<u8>> {
    let image = image::load_from_memory(image_bytes)?;
    let resized = image.resize_exact(
        AVATAR_DIMENSION,
        AVATAR_DIMENSION,
        image::imageops::FilterType::Lanczos3,
    );
    let mut cursor = std::io::Cursor::new(Vec::new());
    resized.write_to(&mut cursor, ImageFormat::WebP)?;
    let webp = cursor.into_inner();
    validate_webp(&webp)?;
    Ok(webp)
}

pub fn validate_webp(bytes: &[u8]) -> AvatarCacheResult<()> {
    if bytes.len() >= MAX_COMPRESSED_BYTES {
        return Err(AvatarCacheError::new(format!(
            "compressed avatar is {} bytes; must be strictly below 102400",
            bytes.len()
        )));
    }
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return Err(AvatarCacheError::new("avatar is not a WebP file"));
    }
    let decoded = image::load_from_memory(bytes)?;
    if decoded.width() != AVATAR_DIMENSION || decoded.height() != AVATAR_DIMENSION {
        return Err(AvatarCacheError::new(format!(
            "compressed avatar is {}x{}; expected {}x{}",
            decoded.width(),
            decoded.height(),
            AVATAR_DIMENSION,
            AVATAR_DIMENSION
        )));
    }
    Ok(())
}

fn sanitize_key(key: &str) -> String {
    key.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> AvatarCacheResult<()> {
    let directory = path
        .parent()
        .ok_or_else(|| AvatarCacheError::new("avatar path has no parent directory"))?;
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| AvatarCacheError::new("avatar filename is not UTF-8"))?;
    let temporary_path = directory.join(format!(
        ".{}.{}.tmp",
        file_name,
        uuid::Uuid::new_v4().simple()
    ));

    let result = (|| -> AvatarCacheResult<()> {
        let mut file = std::fs::File::create(&temporary_path)?;
        file.write_all(bytes)?;
        file.flush()?;
        file.sync_all()?;
        std::fs::rename(&temporary_path, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary_path);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TempDirectory(PathBuf);

    impl TempDirectory {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "liveshow-avatar-test-{}",
                uuid::Uuid::new_v4().simple()
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn sample_image(seed: u8) -> Vec<u8> {
        let mut png = Vec::new();
        let image = image::RgbaImage::from_pixel(
            80,
            80,
            image::Rgba([
                seed.wrapping_mul(7),
                seed.wrapping_mul(11),
                seed.wrapping_mul(13),
                255,
            ]),
        );
        image
            .write_to(&mut std::io::Cursor::new(&mut png), ImageFormat::Png)
            .unwrap();
        png
    }

    #[test]
    fn compressed_avatar_is_strictly_under_limit_valid_webp_and_40_square() {
        let webp = compress_avatar(&sample_image(3)).unwrap();
        assert!(webp.len() < MAX_COMPRESSED_BYTES);
        assert_eq!(&webp[0..4], b"RIFF");
        assert_eq!(&webp[8..12], b"WEBP");
        validate_webp(&webp).unwrap();
    }

    #[test]
    fn corrupt_and_wrong_size_inputs_are_rejected() {
        assert!(compress_avatar(b"not an image").is_err());
        assert!(validate_webp(b"not an image").is_err());
        let mut oversized = vec![0; MAX_COMPRESSED_BYTES];
        oversized[0..4].copy_from_slice(b"RIFF");
        oversized[8..12].copy_from_slice(b"WEBP");
        assert!(validate_webp(&oversized).is_err());
    }

    #[tokio::test]
    async fn manifest_recovers_files_and_migration_removes_successful_jpegs() {
        let temp = TempDirectory::new();
        let jpeg_path = temp.0.join("123.jpg");
        std::fs::write(&jpeg_path, sample_image(5)).unwrap();
        let store = AvatarStore::open(&temp.0).unwrap();
        assert!(store.get("123").await.unwrap().is_some());
        assert!(!jpeg_path.exists());
        assert!(store.manifest_path().exists());
        drop(store);

        let reopened = AvatarStore::open(&temp.0).unwrap();
        let snapshot = reopened.snapshot().await.unwrap();
        assert!(snapshot.contains_key("123"));
        validate_webp(&snapshot["123"]).unwrap();
    }

    #[tokio::test]
    async fn atomic_replacement_is_serialized_and_concurrent_writes_leave_valid_manifest() {
        let temp = TempDirectory::new();
        let store = AvatarStore::open(&temp.0).unwrap();
        let mut tasks = Vec::new();
        for seed in 0..12u8 {
            let store = store.clone();
            tasks.push(tokio::spawn(async move {
                store
                    .put_local("same", &compress_avatar(&sample_image(seed)).unwrap(), None)
                    .await
                    .unwrap();
            }));
        }
        for task in tasks {
            task.await.unwrap();
        }
        let bytes = store.get("same").await.unwrap().unwrap();
        validate_webp(&bytes).unwrap();
        drop(store);
        let reopened = AvatarStore::open(&temp.0).unwrap();
        validate_webp(&reopened.get("same").await.unwrap().unwrap()).unwrap();
        assert!(std::fs::read_dir(&temp.0)
            .unwrap()
            .filter_map(Result::ok)
            .all(|entry| !entry.file_name().to_string_lossy().ends_with(".tmp")));
    }

    #[tokio::test]
    async fn manifest_write_failure_keeps_old_file_memory_and_reopenable_entry() {
        let temp = TempDirectory::new();
        let store = AvatarStore::open(&temp.0).unwrap();
        let old_webp = compress_avatar(&sample_image(7)).unwrap();
        store.put_local("keep", &old_webp, None).await.unwrap();
        let manifest_path = store.manifest_path();
        let original_manifest = std::fs::read(&manifest_path).unwrap();
        let old_file = store.entries().await["keep"].file.clone();

        std::fs::remove_file(&manifest_path).unwrap();
        std::fs::create_dir(&manifest_path).unwrap();
        let result = store
            .put_local("keep", &compress_avatar(&sample_image(8)).unwrap(), None)
            .await;
        assert!(result.is_err());
        assert_eq!(store.get("keep").await.unwrap().unwrap(), old_webp);
        assert_eq!(store.entries().await["keep"].file, old_file);

        std::fs::remove_dir(&manifest_path).unwrap();
        std::fs::write(&manifest_path, original_manifest).unwrap();
        drop(store);
        let reopened = AvatarStore::open(&temp.0).unwrap();
        assert_eq!(reopened.get("keep").await.unwrap().unwrap(), old_webp);
        assert_eq!(
            std::fs::read_dir(&temp.0)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.path().extension().and_then(|value| value.to_str()) == Some("webp")
                })
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn remote_refresh_distinguishes_unchanged_and_updated_images() {
        let temp = TempDirectory::new();
        let store = AvatarStore::open(&temp.0).unwrap();
        let first_webp = compress_avatar(&sample_image(1)).unwrap();
        let same_source = sample_image(1);
        let first_download = same_source.clone();
        let outcome = store
            .refresh_from_remote("1", "https://example.invalid/1", |conditional| async move {
                assert!(conditional.etag.is_none());
                Ok(RemoteAvatarResponse {
                    status: 200,
                    etag: Some("\"avatar-1\"".into()),
                    last_modified: None,
                    body: Some(first_download),
                })
            })
            .await
            .unwrap();
        assert_eq!(outcome, RefreshOutcome::Updated);
        let first_entry = store.entries().await["1"].clone();
        assert_eq!(store.get("1").await.unwrap().unwrap(), first_webp);

        let second_download = same_source.clone();
        let outcome = store
            .refresh_from_remote("1", "https://example.invalid/1", |conditional| async move {
                assert_eq!(conditional.etag.as_deref(), Some("\"avatar-1\""));
                Ok(RemoteAvatarResponse {
                    status: 200,
                    etag: Some("\"avatar-1\"".into()),
                    last_modified: None,
                    body: Some(second_download),
                })
            })
            .await
            .unwrap();
        assert_eq!(outcome, RefreshOutcome::Unchanged);
        let unchanged_entry = store.entries().await["1"].clone();
        assert_eq!(unchanged_entry.revision, first_entry.revision);
        assert_eq!(unchanged_entry.updated_at, first_entry.updated_at);
        assert!(unchanged_entry.checked_at > first_entry.checked_at);
        let updated_source = sample_image(42);
        let outcome = store
            .refresh_from_remote(
                "1",
                "https://example.invalid/updated",
                |conditional| async move {
                    assert_eq!(conditional.etag.as_deref(), Some("\"avatar-1\""));
                    Ok(RemoteAvatarResponse {
                        status: 200,
                        etag: Some("\"avatar-2\"".into()),
                        last_modified: None,
                        body: Some(updated_source),
                    })
                },
            )
            .await
            .unwrap();
        assert_eq!(outcome, RefreshOutcome::Updated);
        let updated_entry = store.entries().await["1"].clone();
        assert_eq!(updated_entry.revision, first_entry.revision + 1);
        assert_eq!(updated_entry.updated_at, first_entry.updated_at);
        assert_eq!(updated_entry.etag.as_deref(), Some("\"avatar-2\""));
        assert_eq!(
            updated_entry.face_url.as_deref(),
            Some("https://example.invalid/updated")
        );
    }

    #[tokio::test]
    async fn conditional_304_only_updates_check_metadata() {
        let temp = TempDirectory::new();
        let store = AvatarStore::open(&temp.0).unwrap();
        store
            .put_local("2", &compress_avatar(&sample_image(2)).unwrap(), None)
            .await
            .unwrap();
        store
            .inner
            .manifest
            .write()
            .await
            .avatars
            .get_mut("2")
            .unwrap()
            .etag = Some("\"stable\"".into());
        let before = store.entries().await["2"].clone();

        let outcome = store
            .refresh_from_remote("2", "https://example.invalid/2", |conditional| async move {
                assert_eq!(conditional.etag.as_deref(), Some("\"stable\""));
                Ok(RemoteAvatarResponse {
                    status: 304,
                    etag: None,
                    last_modified: None,
                    body: None,
                })
            })
            .await
            .unwrap();
        assert_eq!(outcome, RefreshOutcome::NotModified);
        let after = store.entries().await["2"].clone();
        assert_eq!(after.revision, before.revision);
        assert_eq!(after.updated_at, before.updated_at);
        assert_eq!(after.file, before.file);
        assert!(after.checked_at > before.checked_at);
    }
}
