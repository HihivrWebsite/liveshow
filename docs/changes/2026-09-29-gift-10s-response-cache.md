# v4.7.16 gift 接口 10 秒响应缓存实现记录

## 日期

2026-09-29

## 变更原因

远程上游可能被首页、月份切换、直播会话、头像等请求同时触发。为避免完全相同的冷启动请求在短时间内重复执行远程查询，需要在所有 `GET /gift*` 路由外增加短 TTL 响应缓存，并合并同键并发。

## 涉及文件

- `rust_backend/src/main.rs` — 注册缓存模块并挂载 10 个 `/gift*` 路由
- `rust_backend/src/gift_response_cache.rs` — 键规范、响应缓冲、LRU、单飞、generation 与统计
- `rust_backend/Cargo.toml` — 后端版本升级
- `rust_backend/Cargo.lock` — 后端锁文件版本同步
- `frontend/package.json` — 前端版本升级
- `frontend/package-lock.json` — 前端锁文件根版本同步
- `docs/features/gift-10s-response-cache.md` — 新增功能文档
- `docs/architecture/cache-system.md` — 更新缓存层次与失效规则
- `docs/api/endpoints.md` — 补充响应缓存、诊断头与统计字段
- `docs/development/testing.md` — 补充缓存测试步骤
- `docs/development/backend-guide.md` — 更新路由和缓存开发说明
- `docs/changes/changelog.md` — 新增 `v4.7.16`

## 涉及模块

- Gift HTTP 响应缓存
- `/cache/stats` 与 `POST /cache/clear`
- 前后端版本管理

## 实现内容

- 为 10 个 `GET /gift*` 路由增加独立的 10 秒内存响应缓存
- 以 `/gift` 路径过滤的缓存中间件最先挂载为内层，随后由 Trace、Compression 和 CORS 包裹
- 使用按键 single-flight 合并同键冷启动并发，leader 取消时自动释放锁
- 缓存完整状态码、端到端响应头与原始字节，错误响应同样缓存 10 秒
- 采用 512 条、64 MiB、单响应 16 MiB 的有界 LRU
- 使用 `Content-Length` 或精确 body size hint 判断可缓存长度，兼容 Axum 在中间件之后补写长度的响应
- 按规范路径与规范查询生成缓存键，不按业务参数擅自归并
- 响应增加 `X-Gift-Cache: HIT|MISS|COALESCED`
- `/cache/stats` 增加 `gift_response_cache` 诊断对象
- 有效 `POST /cache/clear` 清空短缓存并递增 generation，阻止清理前结果回写
- 版本从 `4.7.15` 升级至 `4.7.16`

## 版本管理

- 本变更与尚未提交的 `v4.7.15` 前端关注按钮功能分开审核、分开提交。
- 未自动执行任何 Git 操作。
