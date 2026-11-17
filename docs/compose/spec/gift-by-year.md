---
feature: gift-by-year
status: delivered
updated: 2026-11-17
source: feat/gift-by-year
---

# 斗虫按年查询无感接入（gift-by-year）

## Report

**What was built** — 后端 `GET /gift/by_month` 支持 `year` 参数（与 `month` 互斥、四位年份且不晚于当年，先于上游 400），缓存键 `{source}_month_{room|all}_y{YYYY}`、TTL/落盘沿用当月/历史月分档，year 响应回填 `total_revenue` 并将缺失字段规范化为 null；前端 api 层新增 `getAnchorsByMonthChunks`，起止为同年 1–12 月时折叠为单次 `year=` 请求，替换 AnchorList 区间查询与导出两处逐月循环——UI 零改动，非整年区间与单月/默认入口行为不变。六份文档同步（endpoints、upstream-cache、cache-system、month-selector、anchor-list、changelog「未发布」条目）。

**Verification** — `cargo fmt --check`/`cargo check`/`cargo test` 全 0（37 passed，含新增 5）；`npm ci`/`npm run build` 全 0；本地冒烟五例（year=2026→200/231 条/`current_concurrency:null`、双参→400、`year=abc`→400、无参→当月 202611、year=2025→200/237 条）；完整服务浏览器 E2E：默认走 `/gift?filter=everything`，选 2025-01~2025-12 后仅 1 次 `GET /gift/by_month?year=2025&filter=everything`，标题与年度累计数据渲染/排序正常，控制台零报错。独立审查 APPROVE、零 critical。

## [S1] Problem

上游斗虫新增 `GET /gift/by_month?year=YYYY` 按年查询（年度累计快照，Redis 预热、只读缓存），可显著降低上游与本项目的请求压力。本项目现状：

- 后端 `GET /gift/by_month` 只认 `month=YYYYMM`，全库无 year 处理（`main.rs` `MonthQuery` → `upstream_cache.rs` `Key::url`）。
- 前端仅在「多月区间查询」与「导出」两处对 `/gift/by_month` 逐月循环（`AnchorList.vue` ~675、~2716），整年操作 = 12 × 源数 次请求。
- 用户要求：不新增任何按钮，无感集成进现有功能。

## [S2] Design

### S2.1 后端 `year` 参数（透传 + 校验）

- `MonthQuery` 增加可选 `year: Option<String>`；路由仍为 `GET /gift/by_month`，复用 `get_anchors_by_month` 与 `SourceSelection::resolve`（filter/union 语义不变，含 everything 默认）。
- 参数规则（镜像上游）：
  - `month` 与 `year` 同传 → 400；
  - 都不传 → 默认当前月（现状不变）；
  - `year` 必须为 4 位数字且 ≤ 当前年，否则 400。
- `year` 路径跳过 `normalize_month` / min_month 逐月过滤，直接以 year 拼上游 URL：`{base}/gift/by_month?year={YYYY}[&room_id=]`。
- 上游 base 与调用通道不变（`vr/psp/vup.qianqiuzy.cn`，`fetch_json_from_url`）。

### S2.2 缓存键与 TTL（upstream_cache）

- `Key` 增加 `year: Option<u16>`（与 `month` 互斥）。
- 磁盘/内存 id：`{source}_month_{room|all}_y{YYYY}`（如 `vr_month_all_y2026`），与 6 位月份 id `..._{YYYYMM}` 不碰撞；`by_endpoint` 指标键保持 `vr:/gift/by_month` 不变。
- TTL 沿用 Endpoint::Month 现行分档，按「year < 当前年」判定 historical：
  - 当年 year：非 historical，非空 300s / 空 60s，不落盘（同当月）；
  - 历史年 year：historical，非空 24h / 空 1h，落盘 `cache_data/upstream_v1/{id}.json`（同历史月）。
- 最外层 10 秒 HTTP 响应缓存（gift-10s-response-cache）自动覆盖，无需改动。

### S2.3 响应契约与字段规范化

- 响应仍为 `ByMonthResponse`（`anchors`、`refresh_time`、`filter/source/union` 等）；新增 `year` 字段回显请求年份（按月请求为 null），`month` 字段置为同一年份字符串（前端从不读取该字段，仅保持结构存在）。
- 上游 year 响应与按月的差异（已实探，2026-11-17，三源 vr/psp/vup 的 `?year=2025`/`?year=2026` 均 200；字段清单以实探为准）：
  - 结构仍为按房间裸数组；行字段全集：`room_id`、`anchor_name`、`attention`、`status`、`gift`、`guard`、`super_chat`、`payer_count`、`steel_coin_count`、`blind_box_count`、`blind_box_profit`、`live_duration`、`effective_days`、`live_time`、`title`、`year`、`guard_1/2/3`、`fans_count`、`danmaku{total,captain,admiral,governor,normal}`、`whale_dependency`；无 `month`、无 `top*`、无 `total_revenue`（后端计算回填）；
  - `payer_count` / `steel_coin_count` / `whale_dependency` 实探恒为 null → `whale_dependency` 为 null 使 `top1/top5/top10/top1_percent` 解析为 null → 前端未消费，原样透传 null；
  - 实探发现 `current_concurrency` 在 year **与** month 响应中均不返回（`?month=202610` 对照亦缺失）→ 后端既有解析路径统一规范化为 `current_concurrency: null`（Option 序列化恒带该键），使前端 `!== null` 判空走「未开播」分支，不显示 `formatNumber(undefined)`；
  - `year` 替代 `month`（字符串如 `"2026"`）；`danmaku` 对象存在但实探 2025 历史年各值全 null、2026 当年部分非空；不返回 total/时薪；
  - `fans_count` / `guard_1/2/3`：当年（2026）非空，历史年（2025）实探全 null → 后端 `parse_to_i64` 归 0（与按月历史条目既有的 null→0 行为一致，前端 `|| 0` 容错）。
- 条目级必须保留：`room_id`、`anchor_name`、`gift`、`guard`、`super_chat`、`attention`、`fans_count`、`effective_days`、`total_revenue`、`top*`、`danmaku*`（缺失且前端消费的字段一律按 null 规范化，见上）。
- `total_revenue` 必须存在（区间合并排序依赖，`AnchorList.vue` ~2766）。
- 上游错误体实探：`?month=202601&year=2026` → 400 `{"error":"year 与 month 不能同时传入"}`；`?year=abc` → 400 `{"error":"year 参数无效，须为四位年份且不能晚于当前年"}`（三源一致，本地校验镜像此规则先于上游拒绝）。

### S2.4 前端整年折叠（无 UI 改动）

- `frontend/src/api/index.js` 新增共享 helper（如 `getAnchorsByMonthChunks({ startMonth, endMonth, filter, union })`）：
  - 折叠规则（纯整数数学，遵循 monthUtils 约定）：`startMonth` 为某年 1 月且 `endMonth` 为**同年** 12 月（`start%100==1 && end%100==12 && (start/100|0)==(end/100|0)`）→ 单次 `GET /gift/by_month?year={YYYY}&filter=…[&union=…]`，返回 1 个 chunk；
  - 否则 → 保持现状：`getMonthRange` 逐月循环 `getAnchorsByMonth`，返回 N 个 chunk。
  - 返回值统一为 `chunks: Anchor[][]`（或等价数组），**合并/排序/展示逻辑零改动**。
- 替换两处调用点（仅此两处，均在 `AnchorList.vue`）：
  1. 多月区间查询 ~2716；
  2. 导出 ~675。
- 单月选择、默认入口 `/gift`、`SimpleAnchorList` / `ExpandedView` 单月分支均不改。
- 整年结果的无感语义（单 chunk 走既有合并路径自然成立）：
  - `attention` / `fans_count` 取末 → 即 year 快照值；
  - `top*` / `danmaku*` 首月拷贝 → 即 year 值；
  - 已知退化（接受）：整年结果「即时同接」恒显示「未开播」（年度快照无同接概念）。

### S2.5 文档同步

- `docs/api/endpoints.md`：`GET /gift/by_month` 增加 `year` 参数、互斥规则、响应差异。
- `docs/features/upstream-cache.md`：缓存表增加 year 行（当年/历史年分档）。
- `docs/architecture/cache-system.md`：键构成补 year。
- `docs/features/month-selector.md`（或 anchor-list 相关文档）：记录「整年区间内部折叠」行为。
- `docs/changes/changelog.md`：新增条目。

## [S3] Out of Scope

- 不新增任何按钮、页面、路由或 UI 控件。
- 混合区间拆分（区间内完整年用 year + 零头逐月）不做。
- 后端 `start/end` 区间参数、后端自动区间优化不做。
- 非整年区间仍逐月循环，不优化。
- 默认入口 `/gift`（当前月）语义不动；`/gift` 无 filter 默认不动。
- `payer_count` / `steel_coin_count` / `whale_dependency` 展示（前端本就未用）。
- 非整年区间中「结束月晚于当前月」的未来月份请求行为维持现状。

## Tasks

- [x] T1: 后端 `/gift/by_month` 支持 `year`（互斥/格式校验、上游透传、Key 缓存键与 TTL 分档、`current_concurrency: null` 规范化、响应 `year` 回显）— acceptance: 本地 curl `?year=2026` 返回年度数组且含 `current_concurrency:null`；`?month=202601&year=2026` 与非法 year 均 400；不传参数仍回当前月；新增单测覆盖 Key id/url/校验且 `cargo test` 通过 (covers: S2.1, S2.2, S2.3)
- [x] T2: 上游 year 响应字段实探并固化规范化清单 — acceptance: 对至少一个上游源实 curl `year=`，记录完整字段列表与缺失字段，S2.3 清单按实探修正 (depends: T1)
- [x] T3: 前端 api 层整年折叠 helper 并替换 AnchorList 两处逐月循环 — acceptance: 构建通过；整年区间（如 202501–202512）仅发 1 次 `/gift/by_month?year=…`；非整年区间（如 202506–202509）仍逐月；合并后排序/展示与现状一致 (covers: S2.4, depends: T1)
- [x] T4: 文档同步（endpoints / upstream-cache / cache-system / month-selector 或 anchor-list / changelog）— acceptance: 各文档与最终行为一致，含 year 参数、缓存分档、折叠规则 (covers: S2.5, depends: T1)

## Journey log

- 2026-11-17 后端实现代理：上游实探完成（每 URL 单次 curl，`--max-time 15`）：三源 `?year=2025`/`?year=2026` 均 200（裸数组，vr 74/70 条、psp 25/23 条、vup 138/138 条）；`?month=202601&year=2026` 均 400「year 与 month 不能同时传入」；`?year=abc` 均 400「year 参数无效，须为四位年份且不能晚于当前年」；另对照探 `vr?month=202610` 200 用于字段比对。字段结论已固化进 S2.3：year 行无 `current_concurrency`（month 行同样没有）、`whale_dependency`/`payer_count`/`steel_coin_count` 恒 null、`year` 字符串替代 `month`、`total_revenue` 缺失由后端计算、历史年 `danmaku` 全 null、历史年 `fans_count`/`guard_*` null→0。
- 2026-11-17 后端实现（T1）：`MonthQuery.year` + 双参/格式/未来年 400（先于上游拒绝）；`Key{month|year}` 互斥，id `{source}_month_{room|all}_y{YYYY}`，url `?year={YYYY}`，historical=year<当前年，TTL/落盘沿用当月/历史月分档，`by_endpoint` 指标键不变；`ByMonthResponse.year` 回显、`month` 置年份字符串；新增单测（Key id/url/metric、normalize_year、TTL 分档、双参与非法/未来年 400 且零上游请求、year 行缺失字段序列化为 null）。验证：`cargo fmt --check`/`cargo check`/`cargo test` 均 0（37 passed）。
- 2026-11-17 前端实现（T3）+ E2E：`getAnchorsByMonthChunks` 纯整数折叠（含 `parseYearMonth` 归一，兼容 `YYYYMM`/`YYYY-MM` 入参），AnchorList 导出与区间查询两处替换；四组判定 node 实测（整年 FOLD、非整年/跨年 LOOP）。完整服务浏览器验证：2025 整年区间仅 1 次 `year=2025` 请求，年度累计渲染正常、控制台零报错。
- 2026-11-17 审查（APPROVE，零 critical）。提交卫生：`rust_backend/cache_data/` 为入库目录，跑服务会产生大量 avatar webp 与 `*_month_all_y2025.json` 运行时 diff，提交时须显式剔除（或单独提交）；`Cargo.lock` 4.7.23→4.7.24 系 HEAD 既有版本漂移同步，保留。non-critical 备忘：折叠边界（跨年区间）暂无自动化测试，靠代码走查。
