# 头像缓存架构

头像分为两个互不替代的层：

1. `AVATAR_CACHE` 只保存短期查询所需 face URL，默认 48 小时有效。
2. `AvatarStore` 通过 `cache_data/avatars_compressed/manifest.json` 作为所有对外压缩图的唯一持久索引。

## 后端模块

- `rust_backend/src/avatar_cache.rs`
  - `AvatarStore::open()`：读取清单、迁移旧 JPEG、清理临时文件和孤儿 WebP。
  - `compress_avatar()` / `validate_webp()`：生成并严格验证 40×40、小于 100 KiB 的 WebP。
  - `refresh_from_remote()`：处理条件请求、304、字节不变和内容更新。
  - 内部 `commit_lock` 串行清单提交，`RwLock` 保护内存清单；新文件使用唯一 UUID 文件名，旧文件在清单提交成功后删除。
- `rust_backend/src/main.rs`
  - `fetch_remote_avatar_response()`：设置 Bilibili Referer 和条件请求头，读取 ETag/Last-Modified。
  - `batch_download_avatars()`：启动时执行一次，以后每 4 小时强制刷新 face URL 和头像内容。
  - `get_avatar_proxy()`：清单命中直接返回 WebP；缺失时下载、压缩、提交。
  - `get_avatars_batch()`：从清单读取全部有效 WebP，不依赖批量刷新任务的内存状态。

## 启动顺序

```text
初始化 tracing
  → 打开并同步迁移 avatar manifest
  → 构建 Axum 路由
  → 监听端口
  → 后台任务立即在线刷新
  → 每 4 小时再次刷新
```

迁移发生在请求处理前，因此首个 `/gift/avatars/batch` 可以立即恢复旧头像，不等待上游刷新完成。

## 失败边界

- WebP 文件先以唯一名称原子写入，清单随后原子替换；清单失败不会改变现有条目。
- 清单成功后删除旧文件，崩溃遗留的未引用 WebP 由下次启动清理。
- 原始 `cache_data/avatars/` 不删除，但不再作为代理返回来源。
- `cache_data/avatars_compressed/` 只应包含清单引用的 `.webp`、`manifest.json`，以及迁移失败时仍保留的损坏 `.jpg`。
