# gift 接口 10 秒响应缓存

## 功能概述

为所有 `GET /gift*` 路由增加最外层 HTTP 响应缓存，以固定 10 秒 TTL 合并完全相同请求的冷启动并发，减少同一远程查询被同时大量触发。该缓存不会改变请求路径或业务响应结构，只复用已经生成的完整 HTTP 响应。

本缓存不负责全局限流出站请求，也不替代 handler 内已有的锚点、Attention、LiveSessions、头像 URL 与头像文件缓存。统计中的 `misses` 表示缓存未命中后执行 handler 的次数，不等同于远程 API 调用数。

## 覆盖范围

以下 10 个路由挂载独立响应缓存：

1. `GET /gift`
2. `GET /gift/by_month`
3. `GET /gift/live_sessions`
4. `GET /gift/live_sessions_with_fans`
5. `GET /gift/sc`
6. `GET /gift/attention`
7. `GET /gift/avatar`
8. `GET /gift/avatar_proxy`
9. `GET /gift/avatars/batch`
10. `GET /gift/fans_overlap`

缓存层位于全局 Compression 内侧：首次响应先进入缓存，之后命中响应再由外层压缩，因此 `gzip` 与 `identity` 请求都可正常解码。

## 缓存规则

- TTL 固定为 10 秒，使用单调时钟判断过期。
- 完整保存状态码、端到端响应头和原始响应体，覆盖 JSON、错误文本、JPEG 及批量头像二进制响应。
- HTTP 成功和错误响应均缓存，包括 400、404、500、502、超时等 handler 已经生成的响应，以抑制错误风暴；代价是上游恢复后最多延迟 10 秒可见。
- 独立有界 LRU：最多 512 条、总计 64 MiB、单响应最大 16 MiB。
- 仅缓存具备可用长度（`Content-Length` 或精确 body size hint）且单响应不超过 16 MiB 的响应。
- `no-store`、非空 `Vary`、缺少可用长度、读取失败或超限的响应绕过缓存。
- 重建缓存响应时移除 `content-length`、`transfer-encoding`、`connection`、`keep-alive` 等连接或帧相关头，保留 `Content-Type`、`Cache-Control` 等业务响应头，并追加 `X-Gift-Cache`。

## 缓存键

键格式为 `GET|规范路径|规范查询`：

- 参数名唯一时按参数名排序，参数值保持请求原值。
- 存在重复参数名时保持原始查询顺序，不重排同名参数。
- 保留未知参数，不按默认值、业务语义或参数别名合并。
- 不按房间号、月份、工会等业务规则擅自归并不同请求。
- 编码后的键超过 4096 字节时绕过缓存。

因此 `room_id`、`month`、`filter`、`union` 或未知参数不同的请求一定隔离；参数名顺序不同但每个参数名唯一的等价查询会归入同一键。

## 同键并发合并（single-flight）

每个缓存键维护异步单飞锁，处理顺序为：

```text
直接查缓存
    → 获取键锁
    → 二次查缓存
    → 执行 handler
    → 写入缓存
    → 释放键锁
```

- 首个未命中请求为 leader，只执行一次 handler。
- 后续同键请求等待 leader 完成并复用结果，响应标记为 `COALESCED`。
- 等待期间发生二次命中时直接复用，不重复执行 handler。
- leader future 被取消时单飞锁自动释放，等待者可以继续接管，不悬挂等待。
- 缓存命中（包括过期前再次命中）标记为 `HIT`，执行 handler 的未命中请求标记为 `MISS`。

## 诊断响应头

| 值 | 含义 |
|---|---|
| `X-Gift-Cache: MISS` | 未命中，执行了 handler |
| `X-Gift-Cache: HIT` | 读取到有效的 10 秒缓存 |
| `X-Gift-Cache: COALESCED` | 等待同键单飞结果并复用 |

该头用于排障；客户端不应依赖它参与业务判断。

## 缓存统计

`GET /cache/stats` 的 `gift_response_cache` 对象包含：

| 字段 | 含义 |
|---|---|
| `ttl_seconds` | 固定 TTL 秒数 |
| `hits` | 有效缓存命中数 |
| `coalesced_hits` | 等待后复用 leader 结果的命中数 |
| `misses` | 实际执行 handler 的次数 |
| `expired` | 发现键存在但已过期的次数 |
| `evictions` | LRU 容量淘汰次数 |
| `bypasses` | 因响应不满足缓存条件而绕过的次数 |
| `buffer_errors` | 读取或缓冲响应失败次数 |
| `entries` | 当前条目数 |
| `bytes` | 当前缓存原始字节总数 |
| `inflight_keys` | 当前在途单飞键数 |
| `hit_rate` | `hits` 与缓存查询次数计算出的命中率 |

## 手动清除与 generation

有效调用：

```http
POST /cache/clear?room_id=1713546334&month=202604
```

会清空这层 10 秒响应缓存并递增 generation。generation 用于防止清除期间启动的慢请求把清除前结果重新写回：旧 generation 的 leader 完成后只允许写入当前 generation，不会使已清除的键复活。

无参数或参数无效的 `POST /cache/clear` 继续保持既有 `ok: false` 行为，不清除短缓存。

## 与既有缓存的关系

该缓存是 HTTP 响应层的第一道缓存；handler 内部仍可使用原有日级头像、月度、5 分钟、每日 01:30 刷新和 48 小时等缓存。两层命中相互独立，短缓存不会修改底层数据，也不会替代手动刷新已有缓存的语义。

## 观察方式

```bash
curl -i "http://localhost:2992/gift?filter=all"
curl -i "http://localhost:2992/gift?filter=all"
curl -s "http://localhost:2992/cache/stats"
```

连续请求应在 10 秒内看到 `MISS` 后转为 `HIT`；等待超过 10 秒后再次变为 `MISS`。
