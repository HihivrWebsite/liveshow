# 自定义鼠标系统

## 启用条件

仅在同时满足以下条件时启用自定义鼠标：

- 页面宽度不小于 768px；
- `(hover: hover) and (pointer: fine)` 为真；
- 不存在 `prefers-reduced-motion: reduce`。

移动端、触屏设备和减少动态效果模式不添加 `custom-cursor-active`，继续使用系统鼠标。桌面启用时该类挂在 `<html>` 上隐藏系统指针；鼠标层固定在页面最上层且 `pointer-events: none`。

## 跟随与形变

- 中心 8px 点由 `mousemove` 直接更新 `transform`，不经过弹簧延迟。
- 32px 外环通过连续 `requestAnimationFrame` 积分位置弹簧，参数为 stiffness `95`、damping `16`，因此快速移动时会滞后并产生果冻感。
- 形变弹簧参数为 stiffness `110`、damping `10`；速度参考值为 `850px/s`，最大横向拉伸 `1.55`、垂直收缩 `0.68`。
- 形变方向使用 `atan2(vy, vx)`，速度接近零时保留最后方向，避免抖动；形变量随速度回到 `1`，静止后恢复正圆。
- 圆环只在 `requestAnimationFrame` 中修改 `transform`，不使用 CSS transition、Canvas、WebGL 或 GSAP 形变；隐藏标签页停止动画循环。

## 交互回归

- Hover `button`、`a`、`[data-cursor-hover]` 时外环继续放大。
- 点击继续触发现有 Ripple，外环点击态继续缩小。
- 圆环背景透明、无模糊/滤镜，颜色保持现有 `#FFC633`。
- 浏览器缩放和 DPR 变化使用 CSS 像素坐标，不进行设备像素换算，避免错位。
