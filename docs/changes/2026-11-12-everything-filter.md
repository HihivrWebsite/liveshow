# v4.7.24 2026-11-12 新增全部榜单筛选 everything

## 原因

用户要求榜单顶栏最左边新增「全部」按钮并作为默认选中项，聚合 VR+PSP+VUP 全部成员。既有 `filter=all` 语义固定为 VR+PSP（维阿PSP斗虫榜），改语义会破坏旧链接与文档，故新增独立筛选键 `everything`，`all` 保持不变。

## 实施范围

- `frontend/src/utils/guildConfig.js`：`GUILD_FILTERS.everything`（label `全部`，title 与 `all` 同为 `咿喔io粉丝站维阿PSP斗虫榜`，unions `VirtuaReal/PSPlive/VUP`）置顶于 `GUILD_FILTER_OPTIONS`（顺序 全部 / 维阿PSP / 维阿 / PSPlive / VUP）；`getGuildFilterConfig` / `requireGuildFilterConfig` / `getGuildBoardTitle` 经通用键查找直接接受 `everything`；对比图注释改为说明 `all` 仅 VR+PSP、`everything` 为三源全集。
- `frontend/src/components/AnchorList.vue`、`frontend/src/components/SimpleAnchorList.vue`：`currentFilter` 初始值、`route.query.filter` 回退、路由 watcher 回退全部由 `'all'` 改为 `'everything'`。
- `rust_backend/src/upstream_cache.rs`：`SourceSelection` 新增 `Everything` 变体；`from_filter("everything")` 解析成功；`Everything.sources()` = 全部 `enabled` 源（VR+PSP+VUP），`All.sources()` 仍仅 VR+PSP；`source_name()` / `union_name()` 对 Everything 返回 `"everything"`；新增纯单元测试（`source_selection_everything_covers_all_enabled_sources`、`resolve_everything_rejects_union_and_keeps_default_all`）。
- `frontend/src/components/HeaderSection.vue`：功能指南工会筛选改为五个按钮描述。
- 文档：`docs/features/data-sources.md`、`docs/api/endpoints.md`、`docs/features/anchor-list.md`、`docs/features/upstream-cache.md` 同步 `everything` 枚举与选择规则，新增本变更记录。

## 模块

前端榜单筛选、后端 SourceSelection 聚合、文档。

## 约束与回归

- `filter=all` 仍仅 VR+PSP，带 `filter=all` 的旧书签/链接行为不变；后端无 `filter` 查询参数仍默认 `SourceSelection::All`（VR+PSP）。
- 「VR PSP对比图」按钮与入口守卫保持 `currentFilter === 'all'` / `!== 'all'`，`everything` 页不显示；`GUILD_COMPARISON_CONFIG` 不扩 VUP。
- `getAnchorDisplayName` VUP 后缀规则不改（`everything` 与 `all` 一样不追加后缀）。
- `filter=everything` + 任意 `union` 因 union 解析为 `One` ≠ `Everything` 返回 400；未知 filter 仍 400。
- 未改 `main_optimized.rs`、缓存键结构、详情 `union` 接口、头像逻辑；未新增色值。
- `npm run build` 与 `cargo test`/`cargo build` 通过（见下）；未执行 git 操作，未改 `package.json`/`Cargo.toml` 版本号（提交时按惯例递增 patch）。
