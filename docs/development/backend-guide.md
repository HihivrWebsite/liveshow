# 后端开发指南

## 技术栈

- Rust
- Axum（Web 框架）
- Tokio（异步运行时）
- Serde（序列化/反序列化）

## 目录结构

```
rust_backend/
├── src/
│   ├── main.rs              # 主程序（路由、业务逻辑、缓存）
│   ├── main_optimized.rs    # 优化版主程序
│   └── middleware.rs        # 中间件
├── cache_data/              # 缓存数据目录
├── Cargo.toml               # 依赖配置
├── Cargo.toml.optimized     # 优化版依赖配置
├── Cargo.lock               # 依赖锁定文件
├── dist/                    # 分发目录
└── target/                  # 编译输出目录
```

## 数据模型

### Anchor（主播）

```rust
struct Anchor {
    anchor_name: String,      // 主播名称
    attention: i64,           // 关注数
    current_concurrency: Option<i64>, // 即时同接
    effective_days: i32,      // 有效天数
    fans_count: i32,          // 粉丝团数量
    gift: f64,                // 礼物收入
    guard: f64,               // 舰长收入
    guard_1: i32,             // 舰长数量
    guard_2: i32,             // 提督数量
    guard_3: i32,             // 总督数量
    live_duration: String,    // 直播时长
    live_time: String,        // 直播时间
    month: String,            // 月份
    room_id: u64,             // 房间ID
    status: i32,              // 状态（0-未开播，1-直播中）
    super_chat: f64,          // SC收入
    title: String,            // 标题
    total_revenue: f64,       // 总营收
    union: String,            // 工会
}
```

### LiveSession（直播会话）

```rust
struct LiveSession {
    start_time: String,       // 开始时间
    end_time: String,         // 结束时间
    duration_minutes: i32,    // 持续分钟数
    start_guard_1/2/3: i32,   // 开始时舰长数量
    end_guard_1/2/3: i32,     // 结束时舰长数量
    start_fans_count: i32,    // 开始时粉丝团数量
    end_fans_count: i32,      // 结束时粉丝团数量
    danmaku_count: i32,       // 弹幕数
    gift: f64,                // 礼物收入
    guard: f64,               // 舰长收入
    super_chat: f64,          // SC收入
    total_revenue: f64,       // 总营收
    title: String,            // 标题
    avg_concurrency: Option<i64>,  // 平均同接
    current_concurrency: Option<i64>, // 即时同接
    max_concurrency: Option<i64>,  // 最高同接
}
```

### SuperChat（SC消息）

```rust
struct SuperChat {
    send_time: String,        // 发送时间
    uname: String,            // 用户名
    uid: u64,                 // 用户ID
    price: f64,               // 价格
    message: String,          // 消息内容
}
```

## 路由处理

| 路由 | 处理函数 | 说明 |
|------|---------|------|
| GET /gift | get_anchors | 获取主播列表 |
| GET /gift/by_month | get_anchors_by_month | 按月份获取 |
| GET /gift/live_sessions | get_live_sessions | 直播会话详情 |
| GET /gift/attention | get_attention_data | 粉丝数快照 |
| GET /gift/sc | get_sc_history | SC历史 |
| GET /assets/*** | ServeDir | 静态资源 |
| GET /cache/stats | 缓存统计 | 缓存命中率 |

## 业务逻辑

### 数据获取

- `fetch_anchor_data(filter, month)` — 获取主播数据
- `fetch_anchor_data_by_url(url)` — 根据URL获取数据
- `fetch_external_api(client, api_url)` — 从外部API获取
- `fetch_live_session_data(room_id, union, month)` — 直播会话数据
- `fetch_sc_history` — SC历史数据

### 缓存管理

- `get_attention_cache_data()` — 获取 attention 缓存数据
- `is_past_month(month)` — 判断是否为历史月份
- `is_cache_valid(timestamp, current_time)` — 判断缓存是否有效

## 编译与运行

开发模式：
```bash
cargo run
```

生产模式：
```bash
cargo build --release
```

优化版配置：
```bash
cargo build --release --manifest-path Cargo.toml.optimized
```

## 代码风格

- 遵循 Rust 官方风格指南
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行代码检查
