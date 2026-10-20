# v4.7.20 2026-10-20 卡片序号与头像恢复

## 日期

2026-10-20

## 原因

品牌与域名迁移提交删除了 `BaseCard.vue` 中的序号和头像渲染节点，导致主页主播卡及直播会话卡只显示标题。按历史实现恢复最小 DOM，同时保留迁移后的动态标题、品牌标题和 `yioio.top` 域名，并让应用等待头像批量请求结束后再挂载。

## 涉及文件

- `frontend/src/components/BaseCard.vue`
- `frontend/src/components/LiveSessions.vue`
- `frontend/src/main.js`
- `frontend/package.json`
- `frontend/package-lock.json`
- `rust_backend/Cargo.toml`
- `rust_backend/Cargo.lock`
- `docs/features/anchor-list.md`
- `docs/features/live-sessions.md`
- `docs/features/avatar-webp-cache.md`
- `docs/features/responsive-design.md`
- `docs/features/upstream-cache.md`
- `docs/development/testing.md`
- `docs/changes/changelog.md`

## 涉及模块

- 前端通用卡片组件
- 主播列表与直播会话列表
- 前端启动流程与头像 WebP/JPEG 回退
- 前后端版本标记
- 功能、测试与变更文档
