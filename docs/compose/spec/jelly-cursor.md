---
feature: jelly-cursor
status: delivered
updated: 2026-11-12
source: main
---

# Jelly 果冻光标替换

## Report

**What was built** — 用 iowebsite `JellyCursor.vue` 的果冻物理原样替换 liveshow 旧 Cursor 四件套：新增 `JellyCursor.vue`、`App.vue` 挂载、删除 Provider/Dot/Ring/Ripple 与 `useCursor.ts`；颜色映射到 liveshow 既有色板；文档同步。

**Verification** — `npm run build` PASS；`cargo build` PASS；后端 2992 / Vite 3000 已启动；旧引用 grep 0 残留；用户人工确认无问题。

## [S1] Problem

liveshow 现有自定义光标为 `CursorProvider/Dot/Ring/Ripple` + `useCursor.ts`（弹簧参数、`#FFC633` 圆环、hover 放大、点击 Ripple、磁性按钮）。用户要求从 iowebsite 仓库的 `JellyCursor.vue` **尽可能原样**复制果冻光标，替换现有实现，并启动前后端验证。

与现状的冲突（已由用户拍板）：

- 色板：iowebsite 用玫瑰色 `#b1646e` / `rgba(177,100,110,*)`，liveshow AGENTS 禁止新色值 → **映射到现有 6 色色板**。
- 交互：iowebsite Jelly 无 hover 放大 / 点击 Ripple / 磁性 → **纯 Jelly，删除这三项**。
- 工作区：AGENTS「禁止自动 git 操作」→ **主检出原地改，不建 worktree、不 commit**。

## [S2] Design

### S2.1 源实现（iowebsite，行为基准）

路径：`/run/media/pma213x/0C6A8CC86A8CAFCE/Ubuntu2/CodeU/iowebsite/src/components/JellyCursor.vue` + `src/styles/main.css` 中 `.jelly-cursor*` / `.custom-cursor-enabled` 规则。

行为契约（原样保留）：

1. **启用条件**：`(hover: hover) and (pointer: fine)` 且非 `prefers-reduced-motion: reduce`；用 `matchMedia` 监听 change。启用时给 `<html>` 加 `custom-cursor-enabled`，整页 `cursor: none !important`。
2. **DOM**：单组件层 `.jelly-cursor`（fixed、z-index 9999、pointer-events none、inset 0），内含 `.jelly-cursor__ring` 与 `.jelly-cursor__dot`。
3. **可见性**：`pointermove`（非 touch）后 `is-visible`；`pointerout` 且 `relatedTarget === null` 或 window blur 时隐藏。
4. **物理（rAF，步长按 16.667ms 归一，step clamp 到 [0.25, 2]）**：
   - 位置弹簧：`v += (target - pos) * 0.18 * step`；`v *= 0.76^step`；`pos += v * step`。
   - 形变：`speed = hypot(vx,vy)`；`angle = atan2`（speed > 0.08 才更新）；`targetStretch = min(1 + speed * 0.018, 1.88)`；形变弹簧 `sv += (target - stretch) * 0.16 * step`；`sv *= 0.81^step`；`stretch` clamp [0.7, 1.94]；`squash = 1 / sqrt(stretch)`。
   - dot：`translate3d(px, py) translate(-50%,-50%)`（无弹簧，直接跟指针）。
   - ring：`translate3d(ringX, ringY) translate(-50%,-50%) rotate(angle) scale(stretch, squash)`。
5. **CSS 形状**：ring 48×48、2px 边框、圆角 50%、半透明底 + inset 高光 + 轻阴影；dot 8×8；opacity 过渡 180ms；`will-change: transform`。
6. **媒体查询回退**：`(hover: none), (pointer: coarse)` 与 `prefers-reduced-motion: reduce` 下隐藏 `.jelly-cursor` 并恢复 `cursor: auto`。

### S2.2 liveshow 接入与色板映射

- 组件落点：`frontend/src/components/ui/cursor/JellyCursor.vue`（新建，脚本/结构对照源文件，可 TS 化）。
- `App.vue`：`<CursorProvider />` → `<JellyCursor />`。
- **删除**（纯 Jelly 决策）：
  - `CursorProvider.vue`、`CursorDot.vue`、`CursorRing.vue`、`CursorRipple.vue`
  - `composables/useCursor.ts`（若无其他引用）
  - hover 放大、点击 Ripple、`[data-magnetic]` 磁性绑定
- **色板映射**（仅使用现有 6 色及其透明度，禁止新 hex）：
  - dot 背景：`#FFC633`
  - ring 边框：`rgba(246, 177, 0, 0.62)`（对应源 `rgba(177,100,110,0.62)`）
  - ring 背景：`rgba(255, 243, 208, 0.16)`（`#FFF3D0` 半透明，对应源白底 0.16）
  - ring 阴影 inset：`rgba(255, 243, 208, 0.65)`；外阴影：`rgba(246, 177, 0, 0.17)`
  - dot 光晕：`rgba(246, 177, 0, 0.12)`；dot 投影：`rgba(93, 75, 36, 0.32)`（`#5D4B24` 半透明，对应源深色投影）
- 隐藏系统指针：优先沿用源的 `.custom-cursor-enabled` 类名（挂在 `<html>`），删除旧 `.cursor-provider ~ *` 规则；媒体查询回退 CSS 一并迁入组件或全局样式。
- 层级：保持 ring/dot 同层即可（源无 hover 时的 z 分层需求）；`pointer-events: none` 必须保留。

### S2.3 文档同步

- 重写 `docs/features/cursor-system.md`：启用条件、Jelly 物理参数、色板映射、无 hover/ripple/磁性。
- 新增 `docs/changes/<date>-jelly-cursor-replacement.md`（按仓库 changes 惯例）。
- 不在本次改 `docs/frontend-refactor/design-spec.md`（其无光标专章）。

### S2.4 验证边界

- `cd frontend && npm run build` 成功（产物到 `rust_backend/dist`）。
- `cargo build` + 启动后端（端口 2992，托管 dist）；可选 vite dev 3000 联调。
- 浏览器冒烟：桌面指针下系统光标隐藏、dot 跟手、ring 弹簧滞后 + 快速移动拉伸；触屏/reduced-motion 回退系统光标；点击按钮仍可操作（无 Ripple 属预期）。
- 不引入新色值；不执行 git commit/push。

## [S3] Out of Scope

- 不保留/不迁移 hover 放大、点击 Ripple、磁性按钮。
- 不复制 iowebsite 玫瑰色或其它新色。
- 不做 git worktree / commit / push / PR。
- 不改后端 API、缓存、头像等非光标逻辑。
- 不升级依赖大版本；不重构无关前端代码。

## Tasks

- [x] T1: 新增 `JellyCursor.vue` 并挂到 `App.vue` — acceptance: 果冻物理与启用/回退条件与 iowebsite 源一致，颜色仅用 liveshow 色板 (covers: S2.1, S2.2)
- [x] T2: 删除旧 Cursor 四件套与 `useCursor.ts` — acceptance: 仓库内无残留引用，`npm run build` 通过 (depends: T1, covers: S2.2)
- [x] T3: 同步 `docs/features/cursor-system.md` 与 changes 记录 — acceptance: 文档描述与最终实现一致 (depends: T2, covers: S2.3)
- [x] T4: 构建并启动前后端完成冒烟验证 — acceptance: 2992 可访问，光标行为符合 S2.4 (depends: T2, covers: S2.4)

## Journey log
