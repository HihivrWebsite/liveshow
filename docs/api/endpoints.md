# API 接口文档

## 基础信息

- **Base URL**: `http://localhost:2992`
- **协议**: HTTP/RESTful
- **数据格式**: JSON

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
      "max_concurrency": 2000
    }
  ],
  "room_id": "1713546334",
  "queried_user": "主播名称",
  "union": "VirtuaReal",
  "title": "页面标题",
  "refresh_time": "2026-01-15 12:00:00"
}
```

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

## 5. 获取历史 SC 数据

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

## 6. 缓存统计

### 请求

```
GET /cache/stats
```

### 响应

```json
{
  "anchor_cache_hits": 1234,
  "anchor_cache_misses": 56,
  "attention_cache_hits": 5678,
  "attention_cache_misses": 12,
  "hit_rate": "99.8%"
}
```

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
