# 自定义鼠标系统（Jelly 果冻光标）

自 `2026-11-12` 起，liveshow 的自定义光标为单组件果冻（Jelly）实现：
`frontend/src/components/ui/cursor/JellyCursor.vue`。行为与物理参数移植自
iowebsite 的 `JellyCursor.vue`，颜色映射到 liveshow 现有色板。

## 启用条件

仅在同时满足以下条件时启用自定义鼠标：

- `(hover: hover) and (pointer: fine)` 为真；
- 不存在 `prefers-reduced-motion: reduce`。

两项均由 `window.matchMedia` 检测并监听 `change`，运行时可动态开关。
启用时给 `<html>` 挂 `custom-cursor-enabled` 类，整页
`cursor: none !important` 隐藏系统指针。

不启用的场景：触屏/粗指针设备、`prefers-reduced-motion: reduce`、组件卸载后。

## DOM 与可见性

- 单层 `.jelly-cursor`：`position: fixed`、`inset: 0`、`z-index: 9999`、
  `pointer-events: none`、`aria-hidden="true"`，内含
  `.jelly-cursor__ring`（48×48 圆环）与 `.jelly-cursor__dot`（8×8 圆点）。
- `pointermove`（`pointerType !== 'touch'`）后显示（`is-visible`，opacity 过渡 180ms）；
  `pointerout` 且 `relatedTarget === null`，或 window `blur` 时隐藏。
- 首次 `pointermove` 时圆环直接吸附到指针位置，避免入场漂移。

## 物理（rAF 弹簧）

`requestAnimationFrame` 连续积分，帧间隔按 16.667ms 归一为 `step`，
`step` 钳制到 `[0.25, 2]`；隐藏标签页自动停止循环（rAF 冻结）。

- **位置弹簧**（圆环）：
  `v += (target - pos) * 0.18 * step`；`v *= 0.76^step`；`pos += v * step`。
- **形变**：
  - `speed = hypot(vx, vy)`；
  - `angle = atan2(vy, vx)`，仅在 `speed > 0.08` 时更新（低速保留最后方向，防抖动）；
  - `targetStretch = min(1 + speed * 0.018, 1.88)`；
  - 形变弹簧 `sv += (targetStretch - stretch) * 0.16 * step`；`sv *= 0.81^step`；
  - `stretch` 钳制 `[0.7, 1.94]`；`squash = 1 / sqrt(stretch)`。
- **渲染**：
  - dot：`translate3d(px, py) translate(-50%, -50%)`，直接跟随指针，无弹簧；
  - ring：`translate3d(rx, ry) translate(-50%, -50%) rotate(angle) scale(stretch, squash)`。
  - 只改 `transform`（`will-change: transform`），无 CSS transition 形变、
    Canvas、WebGL 或 GSAP 参与。

## 色板映射

仅使用 liveshow 现有色值及其透明度，未引入新色：

| 用途 | 值 | 对应源（iowebsite 玫瑰色系） |
| --- | --- | --- |
| dot 背景 | `#FFC633` | `var(--rose)` |
| ring 边框 | `rgba(246, 177, 0, 0.62)` | `rgba(177, 100, 110, 0.62)` |
| ring 背景 | `rgba(255, 243, 208, 0.16)` | `rgba(255, 255, 255, 0.16)` |
| ring inset 高光 | `rgba(255, 243, 208, 0.65)` | `rgba(255, 255, 255, 0.65)` |
| ring 外阴影 | `rgba(246, 177, 0, 0.17)` | `rgba(177, 100, 110, 0.17)` |
| dot 光晕 | `rgba(246, 177, 0, 0.12)` | `rgba(177, 100, 110, 0.12)` |
| dot 投影 | `rgba(93, 75, 36, 0.32)` | `rgba(80, 43, 50, 0.32)` |

`#F6B100` / `#FFF3D0` / `#5D4B24` 的透明度变体即上表 rgba；
`#FFC633` 为项目既有光标色（旧实现同色）。

## 回退

- `@media (hover: none), (pointer: coarse)`：隐藏 `.jelly-cursor`，恢复
  `cursor: auto !important`。
- `@media (prefers-reduced-motion: reduce)`：同上隐藏并恢复系统光标
  （JS 层也会因 `matchMedia` 监听而 `deactivate`；不复制 iowebsite 的全局
  动画关闭规则，仅保留光标相关部分）。
- 触屏 `pointermove`（`pointerType === 'touch'`）不更新位置、不显示光标层。

## 已移除的旧交互

替换掉旧 `CursorProvider/CursorDot/CursorRing/CursorRipple` + `useCursor.ts` 实现，
以下能力**不再保留**（与 iowebsite Jelly 行为对齐）：

- hover `button` / `a` / `[data-cursor-hover]` 外环放大；
- 点击 Ripple 涟漪、点击态外环收缩；
- `[data-magnetic]` 磁性按钮吸附。

按钮、链接仍可正常点击操作，仅无自定义点击反馈。
