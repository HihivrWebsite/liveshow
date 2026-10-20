# v4.7.19 2026-10-20 品牌与域名迁移

## 日期

2026-10-20

## 原因

将站点品牌由“向阳Hihi”迁移为“咿喔io”，域名由 `hihivr.top` 迁移至 `yioio.top`，并更新页首头像、本地导出 Logo、页面标题、卡片域名、功能指南迁移提示和关注按钮视觉。

## 涉及文件

- `frontend/README.md`
- `frontend/package.json`
- `frontend/package-lock.json`
- `frontend/index.html`
- `frontend/src/components/HeaderSection.vue`
- `frontend/src/components/BaseCard.vue`
- `frontend/src/components/SuperChatDetail.vue`
- `frontend/src/components/NavigationTable.vue`
- `frontend/src/components/SimpleAnchorList.vue`
- `frontend/src/components/LiveSessions.vue`
- `frontend/src/components/ExpandedView.vue`
- `frontend/src/components/AnchorBattle.vue`
- `frontend/src/components/AnchorList.vue`
- `frontend/src/components/FooterSection.vue`
- `frontend/public/logo1.png`
- `frontend/public/logo2.png`
- `frontend/public/favicon.ico`
- `rust_backend/src/main.rs`
- `rust_backend/Cargo.toml`
- `rust_backend/Cargo.lock`
- `docs/api/endpoints.md`
- `docs/architecture/cache-system.md`
- `docs/features/header-actions.md`
- `docs/features/anchor-list.md`
- `docs/features/upstream-cache.md`
- `docs/changes/changelog.md`

## 涉及模块

- 前端页首与功能指南
- 主站跳转与关注入口
- 主播卡片与列表标题
- 导出截图与画布文字
- HTML 元数据、站点图标与静态资源
- 后端静态资源路由与版本号
- 品牌/域名扫描与变更文档

## 验证结果

- `frontend`: `npm install`，退出码 `0`
- `frontend`: `npm run build`，退出码 `0`，成功生成 `rust_backend/dist`
- `rust_backend`: 修复损坏的 NTFS 构建产物后，`cargo build`，退出码 `0`
- Cargo 构建目录切换为 `/home/pma213x/.cache/liveshow-rust-target`，成功生成 `debug/liveshow-backend`
- 前后端版本统一为 `4.7.19`
- 未启动完整服务、未执行浏览器验证、未生成新浏览器截图
- 活跃前端与 Rust 源码中旧品牌、旧域名、旧头像直链和 `logo2.png` 扫描仅保留功能指南中按要求出现的旧域名迁移提示
- 历史快照、缓存、默认主播头像、`rust_backend/dist` 构建产物和 `.playwright-cli` 未手工改写

## 图片信息

- 图床页面：`https://ibb.co/yBQ0KDkp`
- 图片直链：`https://i.ibb.co/vvj1fbdh/io2t.png`
- 原图：PNG，`4134×4134`，`5898758` 字节
- `frontend/public/logo1.png`：PNG，`512×512`，RGBA，`254797` 字节
- `frontend/public/favicon.ico`：ICO，`32×32`，`2238` 字节
