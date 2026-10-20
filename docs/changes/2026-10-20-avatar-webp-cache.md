# 头像 WebP 清单缓存与四小时刷新

## 日期

2026-10-20

## 原因

原头像代理返回原始 JPEG，40×40 缓存由多个独立文件和 48 小时 face URL/下载周期管理，批量接口依赖内存状态，启动任务未完成时可能返回空列表，也无法区分远端头像是否真正更新。

## 变更

- 新增 `AvatarStore`，以 `manifest.json` 索引全部对外 WebP。
- `/gift/avatar_proxy` 改为清单优先，缺失时下载并压缩；MIME 改为 `image/webp`。
- `/gift/avatars/batch` 返回 WebP data URL，可从清单立即恢复。
- 旧 `avatars_compressed/*.jpg` 启动时迁移；成功提交清单后删除，`cache_data/avatars/` 原图保留。
- 启动后立即在线刷新，此后每 4 小时刷新；每轮强制获取新 face URL。
- 更新检测优先 ETag/Last-Modified/304，否则下载压缩后比较 WebP 字节。
- 文件与清单原子提交，写失败保留旧缓存，并发提交串行。
- 前端恢复单例批量在线刷新，保留 JPEG 默认头像离线回退。
- 为迁移、损坏输入、格式/大小、清单恢复、并发原子替换、304/不变/更新和 MIME 新增回归测试。

## 涉及文件

- `rust_backend/src/avatar_cache.rs`
- `rust_backend/src/main.rs`
- `frontend/src/utils/avatarCache.js`
- `frontend/src/main.js`
- `frontend/tests/cache-refresh.test.mjs`
- `rust_backend/cache_data/avatars_compressed/*.jpg`（91 个成功迁移后删除）
- `rust_backend/cache_data/avatars_compressed/*.webp`（91 个新缓存）
- `rust_backend/cache_data/avatars_compressed/manifest.json`（新增）
- `docs/features/avatar-webp-cache.md`
- `docs/architecture/avatar-cache.md`
- `docs/architecture/cache-system.md`
- `docs/architecture/overview.md`
- `docs/api/endpoints.md`
- `docs/development/backend-guide.md`
- `docs/development/testing.md`
- `AGENTS.md`

## 测试

- `CARGO_TARGET_DIR=/home/pma213x/.cache/liveshow-rust-target cargo test --locked`
- `npm run test:cache`
- `npm run build`
