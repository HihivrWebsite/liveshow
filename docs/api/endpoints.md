# API 接口文档

## 基础信息

- **Base URL**: `http://localhost:2992`
- **协议**: HTTP/RESTful
- **数据格式**: JSON

### 静态资源

- `GET /logo1.png`：本地品牌图片
- `GET /favicon.ico`：站点图标
- `GET /logo2.png`：已随旧 Logo 删除，不再提供

### Gift 接口通用 10 秒响应缓存

下列 10 个 `GET` 路由共用一层位于压缩中间件内侧的 10 秒内存响应缓存：

- `/gift`
- `/gift/by_month`
- `/gift/live_sessions`
- `/gift/live_sessions_with_fans`
- `/gift/sc`
- `/gift/attention`
- `/gift/avatar`
- `/gift/avatar_proxy`
- `/gift/avatars/batch`
- `/gift/fans_overlap`

该行为不新增 API 路径，也不改变业务请求参数和响应体：

- 10 秒内完全相同的规范请求复用完整状态码、端到端响应头和原始响应体。
- 同一缓存键的冷启动并发只执行一次 handler，其余请求等待后复用结果。
- 成功和错误响应都会缓存 10 秒；错误可能在上游恢复后最多延迟 10 秒可见。
- 响应包含 `X-Gift-Cache: HIT|MISS|COALESCED`：
  - `MISS`：执行 handler。
  - `HIT`：读取有效缓存。
  - `COALESCED`：等待同键单飞结果。
- 缓存键为 `GET|规范路径|规范查询`。唯一参数名排序、重复参数名保持原顺序、原值与未知参数均保留；不按业务字段归并，键超过 4096 字节则绕过。
- 单响应最大 16 MiB，整层上限 512 条和 64 MiB；`no-store`、非空 `Vary`、缺少可用长度（`Content-Length` 或精确 body size hint）、读取失败或超限的响应绕过缓存。

---

## 1. 获取主播列表

### 请求

```
GET /gift
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| filter | string | 否 | 过滤条件：`all`(默认)、`vr`、`psp` |

### 响应

```json
{
  "anchors": [
    {
      "anchor_name": "主播名称",
      "attention": 87138,
      "current_concurrency": 1234,
      "effective_days": 25,
      "fans_count": 500,
      "gift": 12345.67,
      "guard": 8901.23,
      "guard_1": 5,
      "guard_2": 3,
      "guard_3": 1,
      "live_duration": "120:30:00",
      "live_time": "2026-01-15 20:00:00",
      "month": "202601",
      "room_id": 1713546334,
      "status": 1,
      "super_chat": 4567.89,
      "title": "直播标题",
      "total_revenue": 25814.79,
      "union": "VirtuaReal"
    }
  ],
  "refresh_time": "2026-01-15 12:00:00",
  "filter": "all"
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| anchor_name | string | 主播名称 |
| attention | int | 关注数 |
| current_concurrency | int/null | 即时同接人数（开播时显示，未开播为null） |
| effective_days | int | 有效天数 |
| fans_count | int | 粉丝团人数 |
| gift | float | 礼物收入 |
| guard | float | 舰长收入 |
| guard_1 | int | 舰长数量 |
| guard_2 | int | 提督数量 |
| guard_3 | int | 总督数量 |
| live_duration | string | 直播时长 |
| live_time | string | 直播时间 |
| month | string | 月份 |
| room_id | int | 直播间ID |
| status | int | 状态（0-未开播，1-直播中） |
| super_chat | float | SC收入 |
| title | string | 直播标题 |
| total_revenue | float | 总营收 |
| union | string | 工会名称 |

---

## 2. 按月份获取主播数据

### 请求

```
GET /gift/by_month
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| month | string | 是 | 月份，格式 YYYYMM（如 202601） |
| filter | string | 否 | 过滤条件：`all`(默认)、`vr`、`psp` |

### 响应

```json
{
  "anchors": [...],
  "refresh_time": "2026-01-15 12:00:00",
  "filter": "all",
  "month": "202601"
}
```

---

## 3. 获取直播会话详情

### 请求

```
GET /gift/live_sessions
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| room_id | string | 是 | 直播间ID |
| union | string | 是 | 工会名称：`VirtuaReal` 或 `PSPlive` |
| month | string | 否 | 月份，格式 YYYYMM，默认当前月份 |

### 响应

```json
{
  "sessions": [
    {
      "start_time": "2026-01-15 20:00:00",
      "end_time": "2026-01-15 22:00:00",
      "duration_minutes": 120,
      "start_guard_1": 5,
      "start_guard_2": 3,
      "start_guard_3": 1,
      "end_guard_1": 6,
      "end_guard_2": 3,
      "end_guard_3": 1,
      "start_fans_count": 500,
      "end_fans_count": 510,
      "danmaku_count": 1234,
      "gift": 1234.56,
      "guard": 567.89,
      "super_chat": 234.56,
      "total_revenue": 2037.01,
      "title": "直播标题",
      "avg_concurrency": 1500,
      "current_concurrency": 1600,
      "max_concurrency": 2000,
      "new_fans_count": 150,
      "start_attention": 87138,
      "end_attention": 87288
    }
  ],
  "room_id": "1713546334",
  "queried_user": "主播名称",
  "union": "VirtuaReal",
  "title": "页面标题",
  "refresh_time": "2026-01-15 12:00:00"
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| start_time | string | 开播时间 |
| end_time | string/null | 下播时间（直播进行中为 null） |
| duration_minutes | int | 直播时长（分钟） |
| start_guard_1 | int | 开播时舰长数量 |
| end_guard_1 | int/null | 下播时舰长数量 |
| start_guard_2 | int | 开播时提督数量 |
| end_guard_2 | int/null | 下播时提督数量 |
| start_guard_3 | int | 开播时总督数量 |
| end_guard_3 | int/null | 下播时总督数量 |
| start_fans_count | int | 开播时粉丝团人数 |
| end_fans_count | int/null | 下播时粉丝团人数 |
| danmaku_count | int | 弹幕数 |
| gift | float | 礼物收入 |
| guard | float | 舰长收入 |
| super_chat | float | SC收入 |
| total_revenue | float | 总营收 |
| title | string | 直播标题 |
| avg_concurrency | float/null | 平均同接 |
| current_concurrency | int/null | 即时同接 |
| max_concurrency | int/null | 最高同接 |
| new_fans_count | int | 新增粉丝数（-1 表示未计算，0 表示无变化） |
| start_attention | int/null | 开播时粉丝数（来自外部 API） |
| end_attention | int/null | 下播时粉丝数（来自外部 API） |

---

## 3.1 获取直播会话（含粉丝计算）

### 请求

```
GET /gift/live_sessions_with_fans
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| room_id | string | 是 | 直播间ID |
| union | string | 是 | 工会名称：`VirtuaReal` 或 `PSPlive` |
| month | string | 否 | 月份，格式 YYYYMM，默认当前月份 |

### 响应

与 `/gift/live_sessions` 相同，`new_fans_count` 字段已直接从外部 API 的 `start_attention` 和 `end_attention` 计算得出。

---

## 4. 获取粉丝数快照

### 请求

```
GET /gift/attention
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| room_id | int | 是 | 直播间ID，必须为正整数 |
| month | string | 否 | 月份，支持 YYYYMM 或 YYYY-MM 格式，默认当前月份 |
| union | string | 否 | 工会名称，用于缓存键生成 |

### 响应

```json
{
  "room_id": 1713546334,
  "month": "202603",
  "attention": [
    {"20260327": "87138"},
    {"20260328": "87181"},
    {"20260329": "87202"},
    {"20260330": "87233"}
  ]
}
```

### 错误响应

- HTTP 400: 参数错误
- HTTP 500: 数据库查询失败

---

## 5. 获取主播头像

### 请求

```
GET /gift/avatar
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| room_id | string | 是 | 直播间ID |

### 响应

```json
{
  "room_id": 1713546334,
  "face": "https://i1.hdslb.com/bfs/face/xxx.jpg"
}
```

### 说明

- 头像 URL 缓存 48 小时自动刷新
- 数据来源：Bilibili API `get_anchor_in_room`
- 缓存未命中时调用外部 API，命中时直接返回缓存

---

## 5.1 主播头像代理

### 请求

```
GET /gift/avatar_proxy
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| room_id | string | 否* | 直播间ID |
| uid | string | 否* | Bilibili UID，与 room_id 二选一 |

### 响应

- Content-Type: image/webp
- Cache-Control: max-age=86400（浏览器缓存24小时）
- 响应体：40×40、严格小于 100 KiB 的 WebP 头像二进制数据

### 说明

- 优先从 `cache_data/avatars_compressed/manifest.json` 及有效 WebP 文件直接返回
- 清单缺失、文件损坏或首次请求时，后端代理 Bilibili CDN 原图并压缩为 WebP
- 后端请求时设置 `Referer: https://www.bilibili.com/`
- 每次进程启动及以后每 4 小时强制检查远端更新；更新优先使用 ETag、Last-Modified 和 304
- face URL 内存查询仍为 48 小时，但定时刷新不复用该旧 URL
- 参数无效返回 400；远端不存在返回 404；下载或压缩失败返回 502

---

## 5.2 批量头像

### 请求

```
GET /gift/avatars/batch
```

### 响应

```json
{
  "avatars": {
    "1713546334": "data:image/webp;base64,UklGR...",
    "uid_413748120": "data:image/webp;base64,UklGR..."
  },
  "count": 2
}
```

### 说明

- 数据从 WebP 清单恢复，不等待启动批量刷新任务完成。
- 所有值为 WebP base64 data URL；前端 `defaultAvatars.js` 保留旧 JPEG 作为离线回退。
- 本接口仍在 10 秒 Gift HTTP 响应缓存内，清单刚变化时相同请求可能最长延迟 10 秒可见。

---

## 6. 获取历史 SC 数据

### 请求

```
GET /gift/sc
```

### 参数

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| room_id | int | 是 | 直播间ID |
| union | string | 否 | 工会名称 |

### 响应

```json
{
  "room_id": 1713546334,
  "month": "202601",
  "list": [
    {
      "send_time": "2026-01-15 20:30:00",
      "uname": "用户名",
      "uid": 123456,
      "price": 100.00,
      "message": "消息内容"
    }
  ]
}
```

---

## 7. 缓存统计

### 请求

```
GET /cache/stats
```

### 响应

```json
{
  "hits": 1234,
  "misses": 56,
  "hit_rate": 0.9566,
  "hit_rate_percentage": "95.66%",
  "current_size": 1048576,
  "current_size_mb": "1.00 MB",
  "max_size": 134217728,
  "max_size_mb": "128.00 MB",
  "entry_count": 48,
  "attention_entries": 10,
  "live_sessions_entries": 20,
  "upstream_cache": {
    "version": "4.7.19",
    "pid": 1234,
    "started_at": "2026-10-20T10:00:00+08:00",
    "totals": {
      "hits": 1200,
      "disk_hits": 20,
      "coalesced_hits": 14,
      "upstream_requests": 56,
      "upstream_errors": 1,
      "empty_responses": 3,
      "disk_errors": 0
    },
    "by_endpoint": {},
    "by_caller": {}
  },
  "gift_response_cache": {
    "ttl_seconds": 10,
    "hits": 100,
    "coalesced_hits": 12,
    "misses": 50,
    "expired": 4,
    "evictions": 0,
    "bypasses": 1,
    "buffer_errors": 0,
    "entries": 48,
    "bytes": 1048576,
    "inflight_keys": 0,
    "hit_rate": "0.667"
  }
}
```

`gift_response_cache.misses` 表示执行 handler 的次数，不等同于远程 API 调用数；handler 内部既有缓存可能进一步减少远程调用。

实际回源统计使用 `upstream_cache.totals.upstream_requests`，并可按 `by_endpoint` 和 `by_caller` 拆分。源缓存及原顶层字段的统计口径见[共享上游缓存说明](../features/upstream-cache.md)。示例中的 `upstream_cache` 省略了容量和失效字段。

### 缓存清除对响应缓存的影响

有效调用 `POST /cache/clear?room_id=...&month=...` 会清空 Gift 10 秒响应缓存并递增 generation。清除期间已经启动的慢请求不能把清除前结果写回。无参数或参数无效时，接口维持既有的 `ok: false` 行为，不清除该短缓存。

同时清理该房间及月份的场次、Attention 内存与磁盘缓存、对应月榜单及当前主播列表源缓存。可选 `union=VirtuaReal|PSPlive` 限定来源，不传则清理两个来源。房间号必须为正整数，月份必须为有效 `YYYYMM`；磁盘清理失败返回 500，客户端必须确认 `ok:true` 后再刷新。

---

## 前端路由

| 路径 | 功能 | 查询参数 |
|------|------|---------|
| `/` | 主播排行榜 | `filter` (可选) |
| `/by-month` | 按月查看主播数据 | `month` (必需), `filter` (可选) |
| `/live-sessions` | 直播会话详情 | `room_id` (必需), `union` (必需), `month` (可选) |
| `/superchat-detail` | SC详情 | `room_id`, `union` |
| `/expanded-view` | 全展开卡片视图 | - |
| `/expanded-view/:source` | 特定来源展开视图 | - |
| 恶意斗虫 | 多主播数据对比分析 | 通过主页按钮访问 |

---

## 数据获取优化机制

### 超时配置

- 超时时间：30 秒（30000ms）
- 应对慢响应 API，减少网络波动导致的失败

### 自动重试

- 重试次数：最多 3 次
- 重试间隔：2 秒
- 触发条件：API 请求失败

### 数据完整性检测

- 检测标准：单个主播数据少于 10 条视为不完整
- 警告提示：页面显示"部分主播数据不完整"警告

### 刷新/补全数据

- 智能检测数据缺失或数据量不足的主播
- 只重新获取缺失的主播数据
- 刷新时显示当前正在获取哪个主播的数据
