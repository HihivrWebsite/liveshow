# v4.7.27 2026-11-24 斗虫新字段接入（表情包弹幕 + danmaku_dependency）

## 原因

上游 `/gift`、`/gift/by_month`、`/gift/live_sessions` 新增表情包弹幕数、当月/当场去重发言人数、弹幕前排占比 `danmaku_dependency`。弹幕占比与既有金主 `whale_dependency` 的 `top*` 语义不同，必须独立命名接入，历史月大量 `null`/`status=unavailable|partial` 需按 status 判空。

## 实施范围

- `rust_backend/src/main.rs`：
  - `Anchor` 新增 `danmaku_emoticon`、`danmaku_top1/top5/top10/top1_percent`、`danmaku_dependency_status`、`danmaku_user_count`（均 `Option` + `#[serde(default)]`，旧缓存 JSON 缺字段反序列化不失败）；解析自上游 `danmaku.emoticon`、`danmaku_dependency.*`、`danmaku_user_count`。
  - `LiveSession` 新增 `emoticon_danmaku_count`、弹幕占比四元组 + `danmaku_dependency_status`、`danmaku_user_count`；解析自 `emoticon_danmaku_count`、`danmaku_dependency.*`、`danmaku_user_count`。
  - 金主 `top1/top5/top10/top1_percent`（whale）字段名与语义未改动。
  - 年快照行测试 `year_snapshot_rows_normalize_missing_frontend_fields_to_null` 补充新字段断言。
  - `main_optimized.rs` 非构建目标（未 `mod` 进 `main.rs`，Cargo 无 `[[bin]]` 覆盖），保持不动（与 everything-filter 变更一致）。
- `frontend/src/utils/dataProcessor.js`：新增 `isDanmakuDependencyReady`、`formatDanmakuDependencyPercent`（status 判空 + 1 位小数百分比，不可用返回 `—`）。
- `frontend/src/components/AnchorList.vue`：卡片 fields 与导出截图 fields 新增表情包弹幕、弹幕发言人数、弹幕Top1/5/10/1%占比（带「弹幕」标签）；多月导出合并时对弹幕占比/表情包/发言人数保留最新有数据月份（与金主比率合并策略一致）。
- `frontend/src/components/LiveSessions.vue`：场次卡片 fields 同步上述字段。
- `frontend/src/components/ExpandedView.vue`：展开视图主播/场次 fields 同步上述字段；金主占比标签保持不变。
- 文档：`docs/api/endpoints.md`（/gift、/gift/live_sessions 字段表）、`docs/features/danmaku-breakdown.md`（表情包 + danmaku_dependency + 与 whale 区分 + status 判空）、本变更记录、`docs/changes/changelog.md`。

## 模块

后端数据模型与上游解析、前端列表/场次/展开视图展示、API 与功能文档。

## 约束与回归

- 禁止复用/覆盖金主 `top*`；弹幕占比一律 `danmaku_top*` + `danmaku_dependency_status`。
- `status` 非可用（`unavailable`/缺失）显示 `—`；不只看 `!= null`。
- 百分比 1 位小数；无新色值；月份/数据源切换逻辑未改；无 git 操作。
- 未改 `package.json`/`Cargo.toml` 版本号（提交时按惯例递增 patch 至 4.7.26）。
