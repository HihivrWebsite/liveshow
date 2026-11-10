# 弹幕分级统计 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use compose:subagent (recommended) or compose:execute to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 透传上游新增的弹幕分级数据（普通/舰长/提督/总督弹幕数），在主页卡片、直播会话详情、展开详情、恶意斗虫和排名对比中展示。

**Architecture:** 后端在 Anchor 结构体新增 `danmaku` 二级字段（Option），在 LiveSession 结构体新增 4 个分级弹幕字段（Option<i64>），JSON 解析时从上游 `danmaku` 对象/`captain_danmaku_count` 等字段提取。前端在各展示点读取并渲染。

**Tech Stack:** Rust (Axum/serde) + Vue 3 (Composition API)

## Global Constraints

- 版本号递增 patch（v4.7.9 → v4.7.10 → ...）
- Commit 标题格式：`版本号 YYYY-MM-DD_HH:MM PMA213X [type](scope) 描述`
- Type 白名单：fix/feature/improvement/style/typo/refactor/performance/optimize/test/chore/revert/deps/community
- 代码命名英文，注释中文
- 每次修改代码必须更新 docs/changes/、docs/features/

---

## Task 1: 后端透传 Anchor 的 danmaku 字段

**Files:**
- Modify: `rust_backend/src/main.rs`

**Interfaces:**
- Consumes: 上游 `/gift` 返回的 `danmaku` 对象 `{ total, captain, admiral, governor, normal }`
- Produces: `Anchor` 结构体新增 `danmaku_total`, `danmaku_captain`, `danmaku_admiral`, `danmaku_governor`, `danmaku_normal`（全部 `Option<i64>`）

- [ ] **Step 1: Anchor 结构体新增字段**

在 `rust_backend/src/main.rs` 的 Anchor 结构体中（约行 942 后）添加：

```rust
pub danmaku_total: Option<i64>,
pub danmaku_captain: Option<i64>,
pub danmaku_admiral: Option<i64>,
pub danmaku_governor: Option<i64>,
pub danmaku_normal: Option<i64>,
```

- [ ] **Step 2: JSON 解析逻辑**

在 `fetch_external_api` 函数中 Anchor 构造处，添加：

```rust
let danmaku = item.get("danmaku");
let danmaku_total = danmaku.and_then(|d| d.get("total")).and_then(|v| v.as_i64());
let danmaku_captain = danmaku.and_then(|d| d.get("captain")).and_then(|v| v.as_i64());
let danmaku_admiral = danmaku.and_then(|d| d.get("admiral")).and_then(|v| v.as_i64());
let danmaku_governor = danmaku.and_then(|d| d.get("governor")).and_then(|v| v.as_i64());
let danmaku_normal = danmaku.and_then(|d| d.get("normal")).and_then(|v| v.as_i64());
```

然后在 Anchor 实例构造中传入这些字段。

- [ ] **Step 3: 编译验证**

Run: `source ~/.cargo/env && cd rust_backend && cargo build --release 2>&1 | tail -5`
Expected: `Finished release [optimized]`

- [ ] **Step 4: Commit**

```bash
git add rust_backend/src/main.rs rust_backend/Cargo.toml frontend/package.json
git commit -m "v4.7.10 2026-09-14 HH:MM PMA213X [feature](danmaku) 后端透传Anchor弹幕分级字段"
```

---

## Task 2: 后端透传 LiveSession 的弹幕分级字段

**Files:**
- Modify: `rust_backend/src/main.rs`

**Interfaces:**
- Consumes: 上游 `/gift/live_sessions` 返回的 `captain_danmaku_count`, `admiral_danmaku_count`, `governor_danmaku_count`, `normal_danmaku_count`
- Produces: `LiveSession` 结构体新增 4 个字段（`Option<i64>`）

- [ ] **Step 1: LiveSession 结构体新增字段**

在 `rust_backend/src/main.rs` 的 LiveSession 结构体中（约行 969 后）添加：

```rust
pub captain_danmaku_count: Option<i64>,
pub admiral_danmaku_count: Option<i64>,
pub governor_danmaku_count: Option<i64>,
pub normal_danmaku_count: Option<i64>,
```

- [ ] **Step 2: JSON 解析逻辑**

在 LiveSession JSON 解析处（约行 1901 附近 `danmaku_count` 解析之后）添加：

```rust
let captain_danmaku_count = item.get("captain_danmaku_count").and_then(|v| v.as_i64());
let admiral_danmaku_count = item.get("admiral_danmaku_count").and_then(|v| v.as_i64());
let governor_danmaku_count = item.get("governor_danmaku_count").and_then(|v| v.as_i64());
let normal_danmaku_count = item.get("normal_danmaku_count").and_then(|v| v.as_i64());
```

然后在 LiveSession 实例构造中传入。

- [ ] **Step 3: 编译验证**

Run: `cargo build --release 2>&1 | tail -5`

- [ ] **Step 4: Commit**

---

## Task 3: 前端主页卡片展示弹幕分级

**Files:**
- Modify: `frontend/src/components/AnchorList.vue`

**Interfaces:**
- Consumes: `anchor.danmaku_total`, `anchor.danmaku_captain`, `anchor.danmaku_admiral`, `anchor.danmaku_governor`, `anchor.danmaku_normal`
- Produces: 卡片 fields 数组新增弹幕分级字段

- [ ] **Step 1: AnchorList.vue 主页卡片 fields 数组**

在 `getExportFields` 函数和主页 `fields` 数组中，`'即时同接'` 字段之后、金主依赖度之前，添加：

```javascript
...(anchor.danmaku_total != null ? [
  { label: '总弹幕', value: formatNumber(anchor.danmaku_total), type: 'number' },
  { label: '普通弹幕', value: formatNumber(anchor.danmaku_normal || 0), type: 'number' },
  { label: '舰长弹幕', value: formatNumber(anchor.danmaku_captain || 0), type: 'number' },
  { label: '提督弹幕', value: formatNumber(anchor.danmaku_admiral || 0), type: 'number' },
  { label: '总督弹幕', value: formatNumber(anchor.danmaku_governor || 0), type: 'number' }
] : [])
```

- [ ] **Step 2: 构建验证**

Run: `cd frontend && npm run build 2>&1 | tail -5`

- [ ] **Step 3: Commit**

---

## Task 4: 前端直播会话详情展示弹幕分级

**Files:**
- Modify: `frontend/src/components/LiveSessions.vue`
- Modify: `frontend/src/components/ExpandedView.vue`

**Interfaces:**
- Consumes: `session.captain_danmaku_count`, `session.admiral_danmaku_count`, `session.governor_danmaku_count`, `session.normal_danmaku_count`
- Produces: 卡片字段 + 趋势图数据集

- [ ] **Step 1: LiveSessions.vue — 卡片字段**

在 BaseCard fields 数组中 `'弹幕数'` 字段之后添加：

```javascript
...(session.captain_danmaku_count != null ? [
  { label: '舰长弹幕', value: formatNumber(session.captain_danmaku_count || 0), type: 'number' },
  { label: '提督弹幕', value: formatNumber(session.governor_danmaku_count || 0), type: 'number' },
  { label: '总督弹幕', value: formatNumber(session.admiral_danmaku_count || 0), type: 'number' },
  { label: '普通弹幕', value: formatNumber(session.normal_danmaku_count || 0), type: 'number' }
] : [])
```

- [ ] **Step 2: LiveSessions.vue — 趋势图新增数据集**

在 Chart.js 数据集数组中，现有 `danmaku` 数据集之后，添加 4 条分级弹幕趋势线：

```javascript
// 舰长弹幕（深红色）
...(captainDanmakuData.some(v => v > 0) ? [{
  label: '舰长弹幕',
  data: captainDanmakuData,
  borderColor: '#e74c3c',
  backgroundColor: 'rgba(231, 76, 60, 0.1)',
  yAxisID: 'y1',
  fill: false,
  pointRadius: 3,
  tension: 0.4,
  pointStyle: 'triangle'
}] : []),
// 提督弹幕（紫色）
...(admiralDanmakuData.some(v => v > 0) ? [{
  label: '提督弹幕',
  data: admiralDanmakuData,
  borderColor: '#9b59b6',
  backgroundColor: 'rgba(155, 89, 182, 0.1)',
  yAxisID: 'y1',
  fill: false,
  pointRadius: 3,
  tension: 0.4,
  pointStyle: 'rect'
}] : []),
// 总督弹幕（金色）
...(governorDanmakuData.some(v => v > 0) ? [{
  label: '总督弹幕',
  data: governorDanmakuData,
  borderColor: '#f39c12',
  backgroundColor: 'rgba(243, 156, 18, 0.1)',
  yAxisID: 'y1',
  fill: false,
  pointRadius: 3,
  tension: 0.4,
  pointStyle: 'rectRot'
}] : []),
// 普通弹幕（灰色）
...(normalDanmakuData.some(v => v > 0) ? [{
  label: '普通弹幕',
  data: normalDanmakuData,
  borderColor: '#95a5a6',
  backgroundColor: 'rgba(149, 165, 166, 0.1)',
  yAxisID: 'y1',
  fill: false,
  pointRadius: 3,
  tension: 0.4,
  pointStyle: 'circle'
}] : [])
```

在数据收集循环中同步添加对应的数组收集。

- [ ] **Step 3: ExpandedView.vue — 展开详情字段**

在 `'弹幕数'` 字段之后添加弹幕分级字段（与 LiveSessions 卡片相同的条件渲染逻辑）。

- [ ] **Step 4: 构建验证**

Run: `npm run build 2>&1 | tail -5`

- [ ] **Step 5: Commit**

---

## Task 5: 前端恶意斗虫和排名对比新增弹幕分级指标

**Files:**
- Modify: `frontend/src/components/AnchorBattle.vue`
- Modify: `frontend/src/components/RankComparison.vue`

**Interfaces:**
- Consumes: `session.captain_danmaku_count`, `session.admiral_danmaku_count`, `session.governor_danmaku_count`, `session.normal_danmaku_count`
- Produces: 指标下拉新增 4 个选项 + 指标值提取函数

- [ ] **Step 1: AnchorBattle.vue — 指标下拉新增**

在两个下拉框中，`💭 弹幕数` 选项之后添加：

```html
<option value="captainDanmaku">🔱 舰长弹幕</option>
<option value="admiralDanmaku">🎖️ 提督弹幕</option>
<option value="governorDanmaku">👑 总督弹幕</option>
<option value="normalDanmaku">💬 普通弹幕</option>
```

在指标名称映射中添加：
```javascript
captainDanmaku: '舰长弹幕',
admiralDanmaku: '提督弹幕',
governorDanmaku: '总督弹幕',
normalDanmaku: '普通弹幕'
```

在指标值提取函数中添加：
```javascript
captainDanmaku: () => session.captain_danmaku_count || 0,
admiralDanmaku: () => session.admiral_danmaku_count || 0,
governorDanmaku: () => session.governor_danmaku_count || 0,
normalDanmaku: () => session.normal_danmaku_count || 0
```

- [ ] **Step 2: RankComparison.vue — 同样新增指标**

参照 AnchorBattle，在指标名称映射和指标值提取 case 中添加 4 个新指标。

- [ ] **Step 3: 构建验证**

Run: `npm run build 2>&1 | tail -5`

- [ ] **Step 4: Commit**

---

## Task 6: 文档更新

**Files:**
- Modify: `docs/features/`
- Modify: `docs/changes/changelog.md`

- [ ] **Step 1: 创建 docs/features/danmaku-breakdown.md**

```markdown
# 弹幕分级统计

## 功能概述

主播直播数据新增弹幕分级统计，按用户身份分为普通弹幕、舰长弹幕、提督弹幕、总督弹幕。

## 数据来源

上游 API (vr.qianqiuzy.cn/psp.qianqiuzy.cn) 的 `danmaku` 二级对象。

## 展示位置

- 主播卡片：总弹幕 + 分级弹幕（当数据存在时）
- 直播会话详情：弹幕数 + 分级弹幕 + 趋势图
- 展开详情：弹幕分级字段
- 恶意斗虫：新增 4 个指标选项
- 排名对比：新增 4 个指标选项
```

- [ ] **Step 2: 更新 changelog.md**

- [ ] **Step 3: Commit**

---

## Task 7: 编译验证 + 推送

- [ ] **Step 1: 后端编译**

Run: `source ~/.cargo/env && cd rust_backend && cargo build --release 2>&1 | tail -5`

- [ ] **Step 2: 前端构建**

Run: `cd frontend && npm run build 2>&1 | tail -5`

- [ ] **Step 3: 推送**

```bash
git push origin main
```
