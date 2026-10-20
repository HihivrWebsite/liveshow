# 头像 WebP 清单缓存

## 目标

后端只向浏览器发送 40×40、严格小于 100 KiB 的 WebP 头像。压缩图由 `cache_data/avatars_compressed/manifest.json` 单一清单索引；清单缺失、文件损坏或主播从未缓存时，服务器从 Bilibili CDN 下载原图、压缩并原子提交。前端继续使用 `defaultAvatars.js` 中的 JPEG 作为离线回退，在线拿到批量 WebP 后按同键覆盖。

## 请求路径

```text
GET /gift/avatar_proxy
    ├─ 清单与有效 WebP 命中 → 直接返回
    └─ 缺失 → 强制解析 face URL → 条件/普通下载 → WebP 压缩 → 原子提交 → 返回

GET /gift/avatars/batch
    └─ 从清单恢复全部有效 WebP → base64 WebP data URL
```

两个接口仍位于既有 10 秒 Gift HTTP 响应缓存内。因此清单刚更新时，完全相同的 HTTP 请求最迟可能在 10 秒后才看到新字节。

## 压缩与校验

- 目标尺寸固定 40×40，Lanczos3 精确缩放。
- 格式为无损 WebP；每次写入前检查：
  - 字节数严格 `< 100 * 1024`；
  - 文件头为 `RIFF....WEBP`；
  - `image` crate 能完整解码；
  - 解码尺寸为 40×40。
- 原图仍可放在旧 `cache_data/avatars/` 目录，但不参与对外响应，也不是压缩图的索引。

## 刷新

- 服务启动时同步迁移可成功的旧 `avatars_compressed/*.jpg`，然后立即启动一次在线刷新。
- 在线刷新完成后每 4 小时执行一次。
- 每轮先从 VR/PSP 榜单和工会 UID 重新取得 face URL；不复用旧 48 小时 face URL 内存缓存。清单中未出现在当前榜单的键也逐键强制查询。
- 优先发送 `If-None-Match` / `If-Modified-Since`，远端返回 304 时只更新清单检查时间和响应提供的验证器。
- 没有 304 时下载、压缩并比较 WebP 字节：
  - 字节不变：不替换文件、不增加版本，只更新 `checked_at` 和验证器；
  - 字节变化或首次缓存：写入新的唯一 WebP 文件，再原子更新清单。
- 下载、解码或压缩失败时保留旧清单和旧 WebP。

## 清单与原子性

`manifest.json` 保存键、WebP 文件名、face URL、ETag、Last-Modified、检查时间、内容更新时间和单调递增 revision。每次替换都使用唯一文件名：先原子落盘新 WebP，再原子替换清单；只有清单提交成功后才删除旧 WebP。清单写失败时删除新孤儿文件并继续使用旧条目。启动清理会移除临时 `.tmp` 和未被清单引用的 WebP，不会删除 `cache_data/avatars/` 原图。

## 迁移

启动时扫描 `avatars_compressed/*.jpg`。可解码的 JPEG 重新压缩为 WebP、加入清单；对应旧 JPEG 在清单成功提交后删除。损坏或不满足 WebP 规则的旧文件不会删除。

## 前端回退

`frontend/src/main.js` 先调用 `fetchAllAvatars()`，并在其 `.finally()` 中挂载 Vue 应用。`avatarCache.js` 同时只允许一个批量请求，成功后用 `data:image/webp;base64,...` 覆盖同键；请求失败或返回无效结构时保留 `defaultAvatars.js` 的 JPEG，随后仍正常挂载应用。
