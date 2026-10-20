# 共享上游数据缓存（v4.7.20）

## 缓存层与有效期

浏览器请求先经过既有 10 秒 Gift HTTP 响应缓存，再由 `upstream_cache.rs` 按数据源、上游接口、房间及月份共享数据。VR 与 PSP 完全隔离；`all/vr` 筛选、详情主播名查询及头像后台任务不再各自回源。同一源数据键的并发请求只执行一次上游请求。

| 数据 | 非空数据有效期 | 成功空结果有效期 | 持久化 |
| --- | --- | --- | --- |
| 当前 `/gift` 与显式当月 `/gift/by_month` | 5 分钟 | 60 秒 | 否 |
| 历史月份榜单 | 24 小时 | 1 小时 | 是 |
| 当月场次、Attention | 下一个本地时间 01:30 | 60 秒 | 是 |
| 历史场次、Attention | 24 小时 | 1 小时 | 是 |
| SC | 仅 HTTP 响应层 10 秒 | 同左 | 否 |

所有有效期均为访问时检查，不主动轮询刷新。当前月份按服务进程本地时区确定，跨月自动隔离。空结果与失败分开：HTTP 错误、无效 JSON、错误对象或错误数据形状不会进入源数据缓存；HTTP 响应层仍按原有规则缓存错误 10 秒。

源数据内存缓存上限 512 条、序列化大小估算 128 MiB，单响应最多 16 MiB；这不是整个进程 RSS 上限。历史缓存到期重取，允许归档补录和修正最终可见。磁盘采用 `cache_data/upstream_v1/`，包括数据源和抓取时间，原子替换写入。旧 `current_month/previous_months` 文件不再读取，不主动删除用户旧产物。

## 刷新与并发

`POST /cache/clear?room_id=1&month=202610&union=VirtuaReal`：

- 清除对应源的该房间场次、Attention 的内存和磁盘缓存，以及该月榜单和当前 `/gift` 源缓存。
- 不传 `union` 时兼容旧调用，清除 VR 和 PSP 两个源；传入 `PSPlive/psp` 选择 PSP，其余沿用 VR 默认源。
- 校验正整数房间号及有效 `YYYYMM`，无效参数返回 `ok:false` 且不清缓存；磁盘清理失败返回 500，不能报告成功。
- 清理等待已开始的源数据读写完成，然后删除，避免旧请求在清理成功后重新写回文件；外层响应缓存继续使用 generation 防止旧响应复活。
- 两个对比组件的“刷新/补全”先串行清理选中房间及月份，成功后清空全部组件范围缓存并重新查询。清理失败时保留已有数据并提示失败。

源数据缓存不增加全局出站限流，也不改变 SC 或其他 Gift 路由的 10 秒响应缓存。

## 监控与上线核实

`GET /cache/stats` 保留原顶层字段，新增 `upstream_cache`：

- `version/pid/started_at`：确认部署版本及进程重启。
- `totals.upstream_requests/upstream_errors/empty_responses`：实际执行的上游网络尝试、失败及成功空结果。
- `totals.hits/disk_hits/coalesced_hits/disk_errors`：内存、磁盘、合并命中及磁盘问题。
- `by_endpoint`：按 `vr:/gift`、`vr:/gift/live_sessions` 等源和接口拆分。
- `by_caller`：按 `anchor_list/session_name/avatars/live_sessions/attention/sc` 等触发入口拆分，次数归属发起请求的 leader。
- `entries/bytes/max_bytes/evictions/invalidations`：源数据内存容量与失效信息，不把磁盘文件数量混为内存数量。

顶层 `hits/misses/hit_rate` 现按全部 VR/PSP 源数据缓存统计，`misses` 包含未使用长缓存的 SC 请求；`gift_response_cache.misses` 仍仅是 handler 执行次数。只统计两次采样间相同 `version/pid/started_at` 的计数差，实际回源平均速率为 `Δupstream_requests / Δ秒数`。

上游请求附带 `User-Agent: liveshow-backend/4.7.20` 和 `X-Liveshow-Caller`，本地结构化日志记录版本、来源、接口、入口、房间及月份，不记录响应正文。上游访问日志如需区分同 IP 的其他程序，应额外记录这两个请求头；仅凭出口 IP 无法确认全部请求都来自 liveshow。此改动不修改 VR_douchong 或生产服务器日志配置。

上线后应核对版本、统计计数增量、空结果比例及同一历史房间月份的重复回源。离线 mock 测试不能证明生产服务器已部署，也不能把此前日志回放的节省比例视为上线保证。
