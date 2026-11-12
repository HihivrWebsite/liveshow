---
feature: everything-filter
status: delivered
updated: 2026-11-12
source: main
---

# 全部榜单筛选（everything）

## Report

**What was built** — 榜单最左新增「全部」（`filter=everything`，VR+PSP+VUP）并作为进入默认；`filter=all` 仍为维阿PSP（仅 VR+PSP）；其余四按钮不变；后端 `SourceSelection::Everything` 三源聚合；文档与功能指南同步。

**Verification** — `npm run build` PASS；`npm test` 12/12 PASS；`cargo test` 32/32 PASS（`CARGO_TARGET_DIR=/tmp/...`）；冒烟：`everything` 182 条含三工会、`all` 81 条无 VUP、`everything+union` 400、前端 :3000 200；审查 APPROVE 无 critical。

## [S1] Problem

榜单顶栏现有四个筛选按钮：维阿PSP斗虫榜（`filter=all`，仅 VR+PSP）、维阿斗虫榜、PSPlive斗虫榜、VUP斗虫榜。用户要求：

- 在**最左边**新增「**全部**」按钮；
- 作为**进入站点的默认选中项**；
- 「全部」包含**所有成员**（VR + PSP + VUP）；
- 其余四个按钮的标签、数据范围与行为**不变**。

现状约束：后端/文档中 `filter=all` 固定只聚合 `Vr+Psp`，VUP 必须 `filter=vup` 才进入列表；不能把 `all` 语义改成含 VUP（会破坏旧链接与文档）。

## [S2] Design

### S2.1 筛选键与按钮顺序

已拍板：

| 决策 | 取值 |
| --- | --- |
| 新筛选 query 值 | **`everything`** |
| 「全部」是否含 VUP | **是**（VR+PSP+Vup 三源） |
| 旧 `filter=all` | **不变**，仍 = 维阿PSP（仅 VR+PSP） |
| 「VR PSP对比图」显示条件 | **仅** `currentFilter === 'all'`（维阿PSP），`everything` 页不显示 |
| `everything` 的 title | **沿用** `咿喔io粉丝站维阿PSP斗虫榜`（与 `all` 同 title） |
| 「全部」label | `全部` |

`GUILD_FILTER_OPTIONS` 顺序（从左到右）：

1. `everything` — label `全部`
2. `all` — label `维阿PSP斗虫榜`（原有）
3. `vr` — `维阿斗虫榜`
4. `psp` — `PSPlive斗虫榜`
5. `vup` — `VUP斗虫榜`

`GUILD_FILTERS.everything` 契约：

```js
{
  filter: 'everything',
  label: '全部',
  title: '咿喔io粉丝站维阿PSP斗虫榜', // 与 all 相同
  unions: ['VirtuaReal', 'PSPlive', 'VUP'],
  uid: null
}
```

`getGuildFilterConfig` / `requireGuildFilterConfig` / `getGuildBoardTitle` 接受 `everything`；未知键仍 400/`GuildConfigError`。

### S2.2 默认筛选

- 前端进入与路由无 `filter` 时，默认 **`everything`**（替换当前 `|| 'all'`）。
- `AnchorList.vue`、`SimpleAnchorList.vue` 的 `currentFilter` 初始值与 `route.query.filter || …` 回退改为 `everything`。
- **后端无 `filter` 查询参数时仍默认 `SourceSelection::All`（VR+PSP）**，不改 API 兼容语义；列表页始终显式带 `filter`。
- 带 `filter=all` 的旧书签/链接继续打开维阿PSP，行为不变。

### S2.3 后端 `everything` 聚合

`rust_backend/src/upstream_cache.rs` 的 `SourceSelection`：

- 新增变体 `Everything`（或等价命名）。
- `from_filter("everything") => Some(Everything)`。
- `All::sources()` **仍** 仅 `[Vr, Psp]`。
- `Everything::sources()` = 全部 `enabled` 源 `[Vr, Psp, Vup]`（按现有 `enabled` 过滤）。
- `source_name()` / 响应 `source` 字段：`Everything => "everything"`；`union` 字段同理用稳定标识 `"everything"`（与现有 `All => "all"` 风格一致）。
- `resolve()`：`filter=everything` 合法；与 `union` 同时给出且 union 非 everything 语义时，沿用现有「不一致 → 400」——`union` 只支持单一工会，故 **`filter=everything` + 任意 `union` → 400**（因 union 解析为 `One`，与 `Everything` 不相等）。
- 未知 filter 仍 400。

`main.rs` 的 `get_anchors` / `get_anchors_by_month` 经 `SourceSelection::resolve` 自动获得三源列表；**不改** 无 filter 默认。

### S2.4 名称与对比图

- `getAnchorDisplayName`：不改逻辑。`filter === 'everything'` 时 VUP 后缀规则与 `'all'` 一样**不**触发（后缀仅 `vup` 或 null/undefined）；梦魇 Tsuki 仅在 VUP 筛选或无 filter 时追加后缀——与现测例一致。
- 「VR PSP对比图」按钮：`v-if` 保持 `currentFilter === 'all'`，不改为 everything。
- `GUILD_COMPARISON_CONFIG` 仍仅 VR/PSP，不扩 VUP。

### S2.5 文档同步

- `docs/features/data-sources.md`：选择规则补 `filter=everything` 三源聚合；`all` 仍 VR+PSP。
- `docs/api/endpoints.md`：filter 枚举加入 `everything`；说明默认（无 filter）仍 all。
- `docs/features/anchor-list.md`：工会筛选改为 全部 / 维阿PSP / VR / PSP / VUP。
- `docs/features/upstream-cache.md`：若写死 `all/vr/psp/vup` 枚举则补 `everything`。
- `HeaderSection.vue` 功能指南：筛选按钮描述更新为五个（含全部）。
- 新增 `docs/changes/2026-11-12-everything-filter.md`。

### S2.6 测试边界

- 后端单元测试（`upstream_cache` tests 或邻近）：
  - `from_filter("everything")` 解析成功；
  - `All.sources()` 不含 `Vup`；
  - `Everything.sources()` 含三源；
  - `filter=everything` + `union=VirtuaReal` → resolve 失败（None/400）。
- 前端 `guildConfig`：可扩展 `vup-display.test.mjs` 或新测例断言 `GUILD_FILTER_OPTIONS[0].filter === 'everything'`、`requireGuildFilterConfig('everything')` 的 label/title。
- 构建：`npm run build`、`cargo build`（及 `cargo test` 若可行）。

## [S3] Out of Scope

- 不改 `filter=all` 的 VR+PSP 语义。
- 不把 VR PSP 对比图扩到 everything 或 VUP。
- 不改详情 `union` 接口、缓存键结构、头像逻辑。
- 不自动 git 提交/push（除非用户另行要求）。
- 不改 `main_optimized.rs`（非本二进制入口；若确认死代码也不在本任务清理）。

## Tasks

- [x] T1: 前端 `guildConfig` 增加 `everything` 并置顶默认 — acceptance: 按钮最左为「全部」，无 filter 时默认 everything，其余四键不变 (covers: S2.1, S2.2)
- [x] T2: `AnchorList` / `SimpleAnchorList` 默认与路由回退改为 everything — acceptance: 进入列表页选中「全部」并请求 `filter=everything` (depends: T1, covers: S2.2)
- [x] T3: 后端 `SourceSelection::everything` 三源聚合与单测 — acceptance: `filter=everything` 返回 VR+PSP+VUP；`all` 仍仅 VR+PSP；单测通过 (covers: S2.3)
- [x] T4: 文档与功能指南同步 — acceptance: data-sources / endpoints / anchor-list / changes / HeaderSection 与实现一致 (depends: T1, T3, covers: S2.5)
- [x] T5: 构建与前后端冒烟 — acceptance: `npm run build` + `cargo build`/`cargo test` 通过；页面默认全部且四旧按钮正常 (depends: T2, T3, covers: S2.6)

## Journey log
