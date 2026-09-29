# v4.7.15 关注咿喔ioo按钮实现记录

## 日期

2026-09-29

## 变更原因

需要在页眉和功能指南中增加统一的咿喔ioo Bilibili空间关注入口，并以醒目的红色闪光按钮保证入口具有足够视觉吸引力。

## 涉及文件

- `frontend/src/components/HeaderSection.vue` — 两处入口、闪光动画和响应式样式
- `frontend/package.json` — 前端版本升级
- `frontend/package-lock.json` — 前端锁文件根版本同步
- `rust_backend/Cargo.toml` — 后端版本升级
- `rust_backend/Cargo.lock` — 后端锁文件版本同步
- `docs/features/header-actions.md` — 新增功能文档
- `docs/changes/changelog.md` — 新增版本记录

## 涉及模块

- HeaderSection 页眉
- 功能指南弹窗
- 版本管理

## 实现内容

- 在页眉主站按钮与功能指南按钮之间新增关注入口
- 在功能指南欢迎语与“一、核心功能”之间新增相同入口
- 两个入口统一使用 `https://space.bilibili.com/85996232`，在新标签页打开
- 新增 `#B22222` 主色按钮、呼吸光晕、扫光和星光动画
- 新增 Hover、键盘焦点、减少动态效果及多断点响应式规则
- 版本从 `4.7.14` 升级至 `4.7.15`
