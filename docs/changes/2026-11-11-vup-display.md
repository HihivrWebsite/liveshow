# v4.7.22 2026-11-11 VUP 两级头像与梦魇Tsuki固定后缀

## 日期

2026-11-11

## 原因

VUP 栏需要与其他工会栏一致展示两级头像；同时，VUP 组内房间号 `851181` 的主播无论动态名称如何变化，名字后都必须固定显示 `（不是VUP）`。

## 涉及文件

- `frontend/src/utils/guildConfig.js`
- `frontend/src/components/AnchorList.vue`
- `frontend/src/components/AnchorBattle.vue`
- `frontend/src/components/ExpandedView.vue`
- `frontend/src/components/LiveSessions.vue`
- `frontend/src/components/NavigationTable.vue`
- `frontend/src/components/SimpleAnchorList.vue`
- `frontend/src/components/SuperChatDetail.vue`
- `frontend/package.json`
- `frontend/tests/vup-display.test.mjs`
- `rust_backend/src/main.rs`
- `rust_backend/cache_data/avatars_compressed/manifest.json`

## 涉及模块

- 前端统一显示名与 VUP 稳定身份覆盖：使用 `room_id=851181` 判断，不使用动态 `anchor_name`。
- 主播列表行内头像、卡片头像、导航表格、全部展开视图和详细直播页面统一复用头像与显示名逻辑。
- 后端仅执行 Rust 格式化，不改变生产逻辑；VUP 房间头像清单由运行时批量任务补齐。

## 兼容策略

- 仅 `union=VUP` 且 `room_id=851181` 的主播追加固定后缀。
- 已带后缀的名称不重复追加。
- 其他 VUP 和非 VUP 主播保持原名。
- 两级头像继续复用 `/gift/avatars/batch` 的 WebP 数据和离线 JPEG 回退。

## 测试结果

- `npm test`：10/10 通过，退出码 0。
- `npm run build`：通过，退出码 0；仅保留既有 chunk 大小警告。
- `cargo fmt --check`：通过，退出码 0。
- `cargo check --locked`：通过，退出码 0。
- `cargo test --locked`：通过，退出码 0；30 个测试全部成功。
- 证据：`test_screenshots/vup_two_level_avatar.png`

## 版本说明

本文档按 `v4.7.22` 记录；提交前同步 `frontend/package.json`、`frontend/package-lock.json`、`rust_backend/Cargo.toml`、`rust_backend/Cargo.lock`。
