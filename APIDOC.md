# API 文档

## 缓存机制

本项目实现了智能缓存机制，以提高数据访问速度并减少对外部API的请求次数：

- **缓存策略**: 仅对过去月份的数据进行缓存，当前月份数据不缓存以确保实时性
- **缓存大小**: 限制为5GB上限，使用LRU（Least Recently Used）算法管理缓存条目
- **缓存内容**: 包括主播列表数据和直播会话数据
- **缓存键**: 由过滤条件和月份组成（如 "all_202509", "vr_202510"）
- **缓存失效**: 当达到大小限制时自动清理最少使用的条目
- **命中率统计**: 系统记录缓存命中和未命中的次数，并计算命中率

## 缓存命中率统计

系统提供缓存命中率统计功能：

- **命中次数**: 记录成功从缓存获取数据的次数
- **未命中次数**: 记录需要从外部API获取数据的次数
- **命中率**: 命中次数除以总请求数（命中次数 + 未命中次数）
- **统计访问**: 通过内部函数获取命中率统计信息

## 1. 获取主播列表 API

### 路径
`/gift`

### 方法
GET

### 输入参数
- `filter` (可选): 过滤条件
  - `all` (默认): 显示所有工会主播
  - `vr`: 仅显示 VirtuaReal 工会主播
  - `psp`: 仅显示 PSPlive 工会主播

### 输出格式
```json
{
  "anchors": [
    {
      "anchor_name": "主播名称",
      "attention": 关注数,
      "current_concurrency": 即时同接人数 (开播时显示具体数值，未开播时为null),
      "effective_days": 有效天数,
      "fans_count": 粉丝团人数,
      "gift": 礼物收入,
      "guard": 舰长收入,
      "guard_1": 舰长数量,
      "guard_2": 提督数量,
      "guard_3": 总督数量,
      "live_duration": "开播时长",
      "live_time": "开播时间",
      "month": "月份",
      "room_id": 直播间ID,
      "status": 状态,
      "super_chat": SC收入,
      "title": "标题",
      "total_revenue": 总营收,
      "union": "工会名称",
      
    }
  ],
  "refresh_time": "刷新时间",
  "filter": "过滤条件"
}
```

## 2. 按月份获取主播数据 API

### 路径
`/gift/by_month`

### 方法
GET

### 输入参数
- `month` (必需): 月份，格式为 YYYYMM (例如: 202509 表示 2025年9月)
- `filter` (可选): 过滤条件
  - `all` (默认): 显示所有工会主播
  - `vr`: 仅显示 VirtuaReal 工会主播
  - `psp`: 仅显示 PSPlive 工会主播

### 输出格式
```json
{
  "anchors": [
    {
      "anchor_name": "主播名称",
      "attention": 关注数,
      "current_concurrency": 即时同接人数 (开播时显示具体数值，未开播时为null),
      "effective_days": 有效天数,
      "fans_count": 粉丝团人数,
      "gift": 礼物收入,
      "guard": 舰长收入,
      "guard_1": 舰长数量,
      "guard_2": 提督数量,
      "guard_3": 总督数量,
      "live_duration": "开播时长",
      "live_time": "开播时间",
      "month": "月份",
      "room_id": 直播间ID,
      "status": 状态,
      "super_chat": SC收入,
      "title": "标题",
      "total_revenue": 总营收,
      "union": "工会名称"
    }
  ],
  "refresh_time": "刷新时间",
  "filter": "过滤条件",
  "month": "查询月份"
}
```

## 3. 获取直播会话详情 API

### 路径
`/gift/live_sessions`

### 方法
GET

### 输入参数
- `room_id` (必需): 直播间ID
- `union` (必需): 工会名称
  - `VirtuaReal`: VirtuaReal 工会
  - `PSPlive`: PSPlive 工会
- `month` (可选): 月份，格式为 YYYYMM (例如: 202509 表示 2025年9月)，默认为当前月份

### 输出格式
```json
{
  "sessions": [
    {
      "start_time": "开始时间",
      "end_time": "结束时间",
      "duration_minutes": 直播时长(分钟),
      "start_guard_1": 开播舰长数,
      "start_guard_2": 开播提督数,
      "start_guard_3": 开播总督数,
      "end_guard_1": 下播舰长数,
      "end_guard_2": 下播提督数,
      "end_guard_3": 下播总督数,
      "start_fans_count": 开播粉丝团数量,
      "end_fans_count": 下播粉丝团数量,
      "danmaku_count": 弹幕数,
      "gift": 礼物收入,
      "guard": 舰长收入,
      "super_chat": SC收入,
      "total_revenue": 总营收,
      "title": "直播标题",
      "avg_concurrency": 平均同接人数,
      "current_concurrency": 即时同接人数 (开播时显示具体数值，未开播时为null),
      "max_concurrency": 最高同接人数
    }
  ],
  "room_id": "直播间ID",
  "queried_user": "查询的主播名称",
  "union": "工会名称",
  "title": "页面标题",
  "refresh_time": "刷新时间"
}
```

## 4. 获取历史SC数据 API

### 路径
`/gift/sc`

### 方法
GET

### 输入参数
- `room_id` (必需): 直播间ID

### 输出格式
```json
{
  "room_id": 直播间ID,
  "month": "月份",
  "list": [
    {
      "send_time": "发送时间",
      "uname": "用户名",
      "uid": 用户ID,
      "price": 金额
    }
  ]
}
```

## 5. 前端路由

### 主页路由
- 路径: `/`
- 功能: 显示主播排行榜
- 查询参数: `filter` (可选)

### 按月查看路由
- 路径: `/by-month`
- 功能: 显示指定月份的主播数据
- 查询参数: `month` (必需), `filter` (可选)

### 直播详情路由
- 路径: `/live-sessions`
- 功能: 显示指定主播的直播会话详情
- 查询参数: `room_id` (必需), `union` (必需), `month` (可选)