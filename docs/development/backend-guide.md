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
│   ├── gift_response_cache.rs # Gift 10秒 HTTP 响应缓存
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
| GET /gift/live_sessions_with_fans | get_live_sessions_with_fans | 含粉丝计算的直播会话 |
| GET /gift/attention | get_attention_data | 粉丝数快照 |
| GET /gift/sc | get_sc_history | SC历史 |
| GET /gift/avatar | get_avatar | 主播头像 URL |
| GET /gift/avatar_proxy | get_avatar_proxy | 头像代理图片 |
| GET /gift/avatars/batch | get_avatars_batch | 批量头像 |
| GET /gift/fans_overlap | get_fans_overlap | 粉丝重合度 |
| GET /assets/*** | ServeDir | 静态资源 |
| GET /cache/stats | 缓存统计 | 缓存命中率 |
| POST /cache/clear | 清除缓存 | 参数有效时同时清 Gift 短缓存 |

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
- `GiftResponseCache` — 10 个 `/gift*` 路由最外层 HTTP 响应缓存

### Gift 10 秒响应缓存

`gift_response_cache.rs` 只负责 HTTP 响应层，不替代 handler 内部锚点、Attention、LiveSessions 和头像缓存：

- 固定 TTL 10 秒，使用单调时钟；完整缓存状态码、端到端头和原始字节。
- 缓存键为 `GET|规范路径|规范查询`：唯一参数名排序、重复参数名保持原顺序、保留原值和未知参数；不按业务字段归并，超过 4096 字节绕过。
- 每键 single-flight：直查缓存、获取锁、二次直查、执行 handler、写缓存；leader 取消时自动释放锁。
- 有界 LRU 上限为 512 条、64 MiB，单响应 16 MiB；不满足读取条件的响应绕过。
- 缓存成功和错误响应，上游错误最多延迟 10 秒恢复可见。
- 响应追加 `X-Gift-Cache: HIT|MISS|COALESCED`。
- 缓存层位于全局 Compression 内侧，命中结果仍可按请求进行 gzip 或 identity 压缩。
- 有效 `POST /cache/clear` 清空该层并递增 generation，阻止慢请求写回清理前结果；无效调用维持 `ok: false`，不清该层。
- `/cache/stats` 在 `gift_response_cache` 对象中输出 TTL、命中、合并命中、miss、过期、淘汰、绕过、缓冲错误、条目、字节、在途键和命中率。

`gift_response_cache.misses` 仅统计执行 handler 的次数，不等于远程 API 调用数。

### 响应缓存测试要求

修改缓存或路由后至少执行：

```bash
cd rust_backend
cargo fmt --check
cargo check
cargo test
```

单元测试必须覆盖：键隔离、10 秒过期、错误缓存、二进制响应、同键 50 路并发单飞、leader 取消释放锁、LRU 容量淘汰、clear generation 失效、`gzip`/`identity` 压缩兼容、绕过条件和统计计数。运行时还要对全部 10 个 `/gift*` 路由验证首命中、重复命中、过期、并发、有效/无效 clear 和原 `/cache/stats` 字段。

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
