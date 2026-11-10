# v4.7.21 2026-10-21 自定义鼠标弹簧果冻形变

## 原因

用户要求将自定义鼠标改为中心点实时跟随、外环弹簧延迟跟随，并按移动方向产生横向拉伸与垂直收缩的果冻效果；颜色、点击反馈、Hover 放大、移动端禁用和系统指针回退保持不变。

## 实施范围

- `frontend/src/composables/useCursor.ts`：统一指针状态、可用性判断、`prefers-reduced-motion` 和指针订阅；仅在启用桌面设备时添加 `custom-cursor-active`。
- `frontend/src/components/ui/cursor/CursorDot.vue`：中心点通过指针事件直接更新 `transform`，避免 Vue 每帧重渲染。
- `frontend/src/components/ui/cursor/CursorRing.vue`：使用 `requestAnimationFrame` 连续积分位置弹簧和形变弹簧；速度驱动沿瞬时方向的横向拉伸、垂直收缩，停止后自然回圆。
- `frontend/src/components/ui/cursor/CursorProvider.vue`：固定最上层鼠标层，保留 `pointer-events: none`、Hover/磁性交互和 Ripple 入口；启用时隐藏系统鼠标。

## 模块

前端 Cursor System、前端运行时启用策略。

## 约束与回归

- 光标颜色继续使用现有 `#FFC633`，不新增色值。
- 圆环背景透明，无 `filter`、`backdrop-filter`、模糊或渐变。
- 移动端、触屏设备和 `prefers-reduced-motion: reduce` 不启用自定义鼠标，继续使用系统指针。
- 保留现有点击 Ripple、Hover 放大、点击收缩、层级和 `pointer-events: none`。
