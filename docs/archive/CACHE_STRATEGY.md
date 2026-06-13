# 缓存优化策略文档

## 🎯 缓存目标

由于数据特性：
- **历史数据**（上月及更早）：永不变更 → 永久缓存
- **当月数据**：每天1:30更新 → 需要时间感知缓存

## 📊 缓存架构设计

### 1. Attention数据缓存（关键优化点）

**当前问题**：

- 每次获取直播会话时，为每个会话调用2次attention API
- 20场会话 = 40次API调用 → 是最主要的性能瓶颈

**优化方案**：

- 批量获取单个主播单个月份的全部attention数据一次性缓存
- 不再为每个会话逐条调用 `/gift/attention`
- 对于同一个主播、同一个月份，所有会话共享一份attention缓存

**批量获取 attention 数据的实现思路**：

```rust
async fn fetch_attention_month_data(
    client: &Client,
    base_url: &str,
    room_id: &str,
    month: &str,
) -> Result<serde_json::Value, Error> {
    let api_url = format!("{}/gift/attention?room_id={}&month={}", base_url, room_id, month);
    let response = client.get(&api_url).send().await?;
    let json_text = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&json_text)?;
    Ok(json_value)
}
```

**缓存策略**：

- 缓存键：`attention_vr_{room_id}_{month}` 和 `attention_psp_{room_id}_{month}`
- 上月数据：永久缓存
- 当月数据：
  - 首次获取后缓存
  - 如果距上次缓存 > 1小时且当前时间已过1:30，则刷新缓存
  - 其他时间直接使用缓存

### 2. 懒加载方案（前端需要时再获取）

**目标**：

- 初始加载首页时不计算 `new_fans_count`
- 仅在用户展开直播会话详情或查看特定主播会话时才触发attention计算

**实现思路**：

1. 首页加载时只请求 `/gift` 或 `/gift/by_month`，不触发直播会话详情请求。
2. 进入直播会话页面时先获取 `/gift/live_sessions`，仅加载基础会话列表。
3. 用户展开某条会话详情，或者点击“刷新/补全数据”时，才计算对应会话的新增粉丝数。
4. attention计算依赖于批量缓存数据，不再逐条请求。

**前端懒加载建议**：

- 在 `LiveSessions` 或 `ExpandedView` 页面中，先渲染会话列表。
- 每个会话行增加“计算新增粉丝”按钮或“展开详情”行为。
- 点击后请求后端一个新的接口或参数来计算该会话的 `new_fans_count`。

### 3. LiveSessions数据缓存

**缓存键**：`livesessions_{room_id}_{month}`

- 上月数据：永久缓存
- 当月数据：同attention数据策略

**收益**：

- 减少对外部API的调用
- 特别是用户在同一月份多次查询同个主播时

### 4. 前端缓存（辅助）

使用 localStorage + 时间戳验证：

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

---

## 🔄 缓存更新流程

### 判断是否为上月数据

```rust
fn is_past_month(month: &str) -> bool {
    let current_month = chrono::Utc::now().format("%Y%m").to_string();
    month < current_month.as_str()
}
```

### 判断当月缓存是否有效

```rust
fn is_cache_valid(cached_timestamp: SystemTime, current_time: SystemTime) -> bool {
    let duration_since_cache = current_time.duration_since(cached_timestamp)
        .unwrap_or_default();
    
    // 距上次更新 < 1小时，则缓存有效
    duration_since_cache.as_secs() < 3600
}
```

### 判断是否需要在此刻刷新

```rust
fn should_refresh_now(is_past_month: bool) -> bool {
    if is_past_month {
        return false; // 历史数据不刷新
    }
    
    let now = chrono::Local::now();
    let hour = now.hour();
    let minute = now.minute();
    
    // 每天1:30到次日1:40之间，允许刷新
    // (hour == 1 && minute >= 30) || (hour == 2 && minute < 10)
    (hour == 1 && minute >= 30) || hour == 2
}
```

---

## 📈 性能提升预期

### 当前（优化前）

- 首次加载页面 + 查看1个主播20场会话：
  - 1 × `/gift` + 1 × `/live_sessions` + 40 × `/gift/attention` = **42次API调用**
  - 耗时：~30秒（如果每次0.5-1秒）

### 优化后

- **第一次**查看主播（缓存未命中）：
  - 1 × `/gift` + 1 × `/live_sessions` + 1 × `/gift/attention?month=YYYYMMDD` (批量) = **3次API调用**
  - 耗时：~3-5秒

- **第二次**查看同月份同主播（缓存命中）：
  - 直接返回缓存 = **0次API调用**
  - 耗时：<100ms

- **性能提升**：13.3倍 → **快14倍！**

---

## 🔧 实现步骤

### 步骤1：后端缓存管理器 ✅ 已完成

- 添加 `AttentionCacheEntry` 和 `LiveSessionsCacheEntry` 结构体
- 实现批量获取单月份所有日期的attention数据 (`fetch_attention_month_data`)
- 实现缓存管理函数 (`get_attention_cache_data`, `get_attention_from_cache`)
- 集成到全局缓存系统

### 步骤2：优化attention获取 ✅ 已完成

- 修改 `calculate_session_fans_change` 使用批量缓存数据
- 不再为每个会话逐条调用 `/gift/attention`
- 实现懒加载模式：`fetch_live_session_data_with_fans_calc`

### 步骤3：前端懒加载方案 ✅ 已完成

#### 新增API端点

1. **`/gift/live_sessions`** (默认懒加载模式)
   - 不计算 `new_fans_count`
   - 仅返回基础会话数据
   - 用于初始页面加载

2. **`/gift/live_sessions_with_fans`** (完整计算模式)
   - 计算所有会话的 `new_fans_count`
   - 用于需要完整数据的场景

3. **`/gift/session_fans_change`** (单个会话计算)
   - 参数：`room_id`, `union`, `start_time`, `end_time`
   - 返回：`fans_change` 值
   - 用于用户展开会话详情时按需计算

#### 前端实现建议

```javascript
// 初始加载 - 懒加载模式
const sessions = await fetch('/gift/live_sessions?room_id=123&month=202412');

// 用户点击展开会话详情时
const fansChange = await fetch('/gift/session_fans_change?' + new URLSearchParams({
  room_id: session.room_id,
  union: session.union,
  start_time: session.start_time,
  end_time: session.end_time
}));
```

### 步骤4：前端缓存

- 添加localStorage缓存逻辑
- 响应式数据更新

---

## ⚠️ 缓存失效场景

1. **用户手动刷新/补全数据按钮** → 清除当月缓存
2. **系统启动** → 所有内存缓存清空（但可持久化到数据库）
3. **月份切换** → 上月数据进入永久缓存

---

## 📝 监控指标

添加到缓存统计中：

- `attention_cache_hits`：attention缓存命中数
- `attention_cache_misses`：attention缓存未命中数
- `livesessions_cache_hits`：会话缓存命中数
- `livesessions_cache_misses`：会话缓存未命中数

