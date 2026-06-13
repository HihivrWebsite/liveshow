# 缓存架构设计

## 缓存目标

- **历史数据**（上月及更早）：永不变更 → 永久缓存
- **当月数据**：每天1:30更新 → 需要时间感知缓存

## 缓存层次

### 1. 后端内存缓存（主要）

使用 LRU（Least Recently Used）算法管理，5GB 上限。

**缓存内容**：
- 主播列表数据（anchor data）
- 直播会话数据（live sessions）
- Attention 粉丝数数据

**缓存键格式**：
```
anchor_{filter}_{month}              # 主播列表
livesessions_{room_id}_{month}       # 直播会话
attention_{union}_{room_id}_{month}  # 粉丝数快照
```

### 2. 前端 localStorage 缓存（辅助）

```javascript
{
  key: `liveshow_cache_${room_id}_${month}`,
  value: {
    data: sessions,
    timestamp: Date.now(),
    expiry: 1 * 60 * 60 * 1000 // 1小时
  }
}
```

## 缓存策略

### 过期策略

| 数据类型 | 月份 | 过期时间 | 刷新时间 | 用途 |
|---------|------|--------|--------|------|
| 历史数据 | < 当月 | 永不过期 | 永不刷新 | 不会再变化 |
| 当月数据 | = 当月 | 1小时 | 1:30-2:00 | 每天更新一次 |

### 判断逻辑

```rust
// 判断是否为上月数据
fn is_past_month(month: &str) -> bool {
    let current_month = chrono::Utc::now().format("%Y%m").to_string();
    month < current_month.as_str()
}

// 判断当月缓存是否有效（1小时内）
fn is_cache_valid(cached_timestamp: SystemTime, current_time: SystemTime) -> bool {
    let duration_since_cache = current_time.duration_since(cached_timestamp)
        .unwrap_or_default();
    duration_since_cache.as_secs() < 3600
}

// 判断是否需要在此刻刷新（1:30-2:00之间）
fn should_refresh_now() -> bool {
    let now = chrono::Local::now();
    let hour = now.hour();
    let minute = now.minute();
    (hour == 1 && minute >= 30) || hour == 2
}
```

### 缓存流程

```
请求到达
    ↓
检查缓存键是否存在
    ├─ 命中历史数据 → 直接返回（< 50ms）
    ├─ 命中当月数据（< 1小时）→ 直接返回（< 50ms）
    ├─ 命中当月数据（> 1小时，不在刷新时段）→ 返回旧缓存
    ├─ 命中当月数据（> 1小时，在刷新时段）→ 调用API → 更新缓存
    └─ 未命中 → 调用外部API → 保存缓存 → 返回数据
```

## Attention 数据缓存（关键优化点）

### 问题

原始实现中，每次获取直播会话时为每个会话调用 2 次 attention API。20 场会话 = 40 次 API 调用，是主要性能瓶颈。

### 优化方案

- 批量获取单个主播单个月份的全部 attention 数据一次性缓存
- 同一主播、同一月份的所有会话共享一份 attention 缓存
- 不再为每个会话逐条调用 `/gift/attention`

### 性能对比

| 场景 | 优化前 | 优化后 | 提升 |
|------|--------|--------|------|
| 首次加载（20场会话） | 42次API调用，~30秒 | 3次API调用，~3-5秒 | 10倍 |
| 重复访问同月同主播 | 42次API调用，~30秒 | 0次API调用，< 100ms | 300+倍 |
| 同room_id多次查询 | 每次1-2秒 | 首次1-2秒，后续< 50ms | 20-40倍 |

## 懒加载方案

### 设计思路

1. 首页加载时只请求 `/gift` 或 `/gift/by_month`，不触发直播会话详情请求
2. 进入直播会话页面时先获取 `/gift/live_sessions`，仅加载基础会话列表（不计算 `new_fans_count`）
3. 用户展开某条会话详情或点击"刷新/补全数据"时，才计算对应会话的新增粉丝数

### 相关 API 端点

- `/gift/live_sessions` — 默认懒加载模式，不计算 `new_fans_count`
- `/gift/live_sessions_with_fans` — 完整计算模式，计算所有会话的 `new_fans_count`
- `/gift/session_fans_change` — 单个会话按需计算

## 缓存失效场景

1. **用户手动刷新/补全数据按钮** → 清除当月缓存
2. **系统启动** → 所有内存缓存清空
3. **月份切换** → 上月数据进入永久缓存

## 监控指标

通过 `/cache/stats` 端点可查看：

```json
{
  "anchor_cache_hits": 1234,
  "anchor_cache_misses": 56,
  "attention_cache_hits": 5678,
  "attention_cache_misses": 12,
  "hit_rate": "99.8%"
}
```

## 日志符号说明

| 符号 | 含义 | 说明 |
|------|------|------|
| 📊 | 请求开始 | 接收到客户端请求 |
| 🎯 | 缓存命中 | 直接返回缓存数据 |
| 📡 | 缓存未命中 | 首次请求该数据 |
| 🔄 | 缓存过期 | 需要刷新当月数据 |
| 🌐 | API调用 | 向外部服务发起请求 |
| 💾 | 缓存保存 | 数据已保存到内存缓存 |
| ⏱️ | 过期但可用 | 使用旧缓存数据 |
| ✅ | 成功 | 操作完成 |
| ❌ | 失败 | 操作失败 |

## 后续优化方向

1. **并发安全** — 缓存更新时添加互斥锁
2. **持久化存储** — 使用 Redis 或数据库持久化
3. **热点预加载** — 启动时预加载常访问数据
4. **自适应刷新** — 根据访问频率动态调整缓存策略
