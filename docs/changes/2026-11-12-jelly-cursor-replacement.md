# v4.7.23 2026-11-12 Jelly 果冻光标替换

## 原因

用户要求从 iowebsite 仓库的 `JellyCursor.vue` 尽可能原样移植果冻光标，替换 liveshow 现有 `CursorProvider/Dot/Ring/Ripple` + `useCursor.ts` 实现。iowebsite 玫瑰色系与 liveshow 色板不符，按既有色板映射；iowebsite Jelly 无 hover 放大 / 点击 Ripple / 磁性按钮，按纯 Jelly 决策删除这三项交互。

## 实施范围

- `frontend/src/components/ui/cursor/JellyCursor.vue`（新增）：单组件果冻光标，`<script setup lang="ts">`，行为与物理参数逐行对照 iowebsite 源实现（matchMedia 启用条件、`custom-cursor-enabled` 挂 `<html>`、pointermove/out/blur 可见性、rAF 位置弹簧 0.18/0.76、形变弹簧 0.16/0.81、stretch 钳制 [0.7, 1.94]、targetStretch 上限 1.88、速度系数 0.018、角度阈值 0.08、step 钳制 [0.25, 2]）；组件内非 scoped `<style>` 承载 `.jelly-cursor*` 布局与媒体查询回退。
- `frontend/src/App.vue`：`<CursorProvider />` → `<JellyCursor />`，同步更新 import 与 Options API `components` 注册。
- 删除：`CursorProvider.vue`、`CursorDot.vue`、`CursorRing.vue`、`CursorRipple.vue`、`composables/useCursor.ts`（仓库内无其他引用）。
- 文档：重写 `docs/features/cursor-system.md`，新增本变更记录。

## 模块

前端 Cursor System、前端运行时启用策略。

## 约束与回归

- 颜色仅用现有色值及透明度：dot `#FFC633`；ring 边框 `rgba(246, 177, 0, 0.62)`、背景 `rgba(255, 243, 208, 0.16)`、inset `rgba(255, 243, 208, 0.65)`、外阴影 `rgba(246, 177, 0, 0.17)`；dot 光晕 `rgba(246, 177, 0, 0.12)`、投影 `rgba(93, 75, 36, 0.32)`。未新增 hex 色值。
- 移除 hover 放大、点击 Ripple、`[data-magnetic]` 磁性绑定，不迁移不保留。
- 移动端、触屏、`prefers-reduced-motion: reduce` 与 `(hover: none), (pointer: coarse)` 下隐藏光标层并恢复 `cursor: auto`；不复制 iowebsite 的全局 reduced-motion 动画关闭规则。
- 保持 `pointer-events: none`、`z-index: 9999`、`aria-hidden="true"`；按钮/链接点击不受影响。
- `npm run build` 通过；未执行 git 操作，未改 `package.json` 版本号（提交时按惯例递增 patch）。
