# 粉丝团查重 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use compose:subagent (recommended) or compose:execute to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在斗虫榜中新增"粉丝团查重"功能，支持选择两个主播后分析粉丝重叠情况（双向百分比、等级分布、舰长等级统计）。

**Architecture:** 前端新增 `FanOverlap.vue` 全屏覆盖层组件（复用 AnchorBattle/RankComparison 的交互模式），后端新增 `/gift/fans_overlap` 端点代理上游 `fans.harei.cn` API 避免 CORS，事件桥接复用现有的 window CustomEvent 模式。

**Tech Stack:** Vue 3 (Composition API, `<script setup>`) + Chart.js + Rust/Axum + reqwest

## Global Constraints

- 所有 Git 操作需用户审核，不得自动执行
- 版本号三处同步：Cargo.toml / frontend/package.json / git tag
- Commit 标题格式：`版本号 YYYY-MM-DD_HH:MM PMA213X 简述`
- Commit 内容格式：`[type](scope) subject` + 文件列表
- Type 白名单：fix/feature/feature-wip/improvement/style/typo/refactor/performance/optimize/test/chore/revert/deps/community
- 代码命名英文，文档/注释中文
- 每次修改代码必须同步更新 docs/changes/、docs/features/
- 按钮颜色：`#9B59B6`（紫色），与其他三个功能区分
- 上游 API 地址：`https://fans.harei.cn`
- rooms.json 中 VirtuaReal/PSPlive 主播的 room_id / uid / medal 需与上游同步

---

## File Structure

| 操作 | 文件 | 职责 |
|------|------|------|
| 新建 | `frontend/src/components/FanOverlap.vue` | 粉丝团查重全屏覆盖层组件 |
| 修改 | `frontend/src/api/index.js` | 新增 `getFanOverlap` API 函数 |
| 修改 | `frontend/src/App.vue` | 抽屉新增第四个按钮 |
| 修改 | `frontend/src/components/NavigationTable.vue` | 导航表格新增第四个按钮 |
| 修改 | `frontend/src/components/AnchorList.vue` | 新增 window event 监听 + FanOverlap 组件挂载 |
| 修改 | `rust_backend/src/main.rs` | 新增 `/gift/fans_overlap` 代理端点 |
| 新建 | `rust_backend/fans_rooms.json` | 上游主播房间配置（从 bilifanscache 同步） |
| 新建 | `docs/features/fan-overlap.md` | 功能文档 |
| 修改 | `docs/changes/changelog.md` | 变更日志 |

---

### Task 1: 后端代理端点 — `/gift/fans_overlap`

**Covers:** 后端 API 代理层

**Files:**
- Modify: `rust_backend/src/main.rs`
- Create: `rust_backend/fans_rooms.json`

**Interfaces:**
- Consumes: 上游 `https://fans.harei.cn/fans?room_id=<id>` 返回 `{code, medal: {uid: level}, guard_level: {uid: level}}`
- Produces: `GET /gift/fans_overlap?room_id_a=<id>&room_id_b=<id>` 返回 `{success, data: {a_total, b_total, intersection, a_in_b, b_in_a, a_levels, b_levels, a_guard, b_guard, shared_guard}}`

- [ ] **Step 1: 创建 fans_rooms.json 配置文件**

从上游 bilifanscache 的 rooms.json 中提取 VirtuaReal/PSPlive 相关主播，保存为本地配置：

```json
{
  "1820703922": {"uid": 1048135385, "name": "花礼Harei", "medal": "小礼猫"},
  "21481907": {"uid": 16728928, "name": "音遊o泡", "medal": "音遊猫"}
}
```

实际文件应包含上游 rooms.json 中所有 ~130 个主播条目。可通过脚本从上游 API 或 GitHub 仓库自动拉取。

- [ ] **Step 2: 在 main.rs 中新增 fans_overlap handler**

在路由注册区域（约 L1128 之后）添加：

```rust
.route("/gift/fans_overlap", get(get_fans_overlap))
```

在 handler 函数区域添加（建议在 `get_attention_data` 附近）：

```rust
/// 粉丝团查重：代理上游 bilifanscache API 并计算交集
async fn get_fans_overlap(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let room_id_a = params.get("room_id_a")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let room_id_b = params.get("room_id_b")
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let client = reqwest::Client::new();
    let base_url = "https://fans.harei.cn";

    // 并行请求两个房间的粉丝数据
    let (resp_a, resp_b) = tokio::join!(
        client.get(format!("{}/fans?room_id={}", base_url, room_id_a))
            .header("User-Agent", "Mozilla/5.0")
            .send(),
        client.get(format!("{}/fans?room_id={}", base_url, room_id_b))
            .header("User-Agent", "Mozilla/5.0")
            .send(),
    );

    let json_a: serde_json::Value = resp_a.map_err(|_| StatusCode::BAD_GATEWAY)?.json().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let json_b: serde_json::Value = resp_b.map_err(|_| StatusCode::BAD_GATEWAY)?.json().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    if json_a["code"].as_i64() != Some(0) || json_b["code"].as_i64() != Some(0) {
        return Err(StatusCode::BAD_GATEWAY);
    }

    let medal_a = json_a["medal"].as_object().unwrap();
    let medal_b = json_b["medal"].as_object().unwrap();
    let guard_a = json_a["guard_level"].as_object();
    let guard_b = json_b["guard_level"].as_object();

    let a_total = medal_a.len();
    let b_total = medal_b.len();

    // 计算交集 UID
    let a_uids: std::collections::HashSet<String> = medal_a.keys().cloned().collect();
    let b_uids: std::collections::HashSet<String> = medal_b.keys().cloned().collect();
    let intersection: Vec<&String> = a_uids.intersection(&b_uids).collect();
    let inter_count = intersection.len();

    // 等级分桶统计
    let buckets = [(1, 10), (11, 20), (21, 30), (31, 40), (41, 50), (51, 60)];
    let mut a_levels = Vec::new();
    let mut b_levels = Vec::new();

    for &(lo, hi) in &buckets {
        let count_a = intersection.iter()
            .filter(|uid| {
                medal_a.get(uid.as_str())
                    .and_then(|v| v.as_i64())
                    .map_or(false, |l| l >= lo && l <= hi)
            })
            .count();
        let count_b = intersection.iter()
            .filter(|uid| {
                medal_b.get(uid.as_str())
                    .and_then(|v| v.as_i64())
                    .map_or(false, |l| l >= lo && l <= hi)
            })
            .count();
        a_levels.push(serde_json::json!({"range": format!("{}~{}", lo, hi), "count": count_a}));
        b_levels.push(serde_json::json!({"range": format!("{}~{}", lo, hi), "count": count_b}));
    }

    // 舰长等级统计
    let guard_names = [(3, "舰长"), (2, "提督"), (1, "总督")];
    let mut a_guard = Vec::new();
    let mut b_guard = Vec::new();

    for &(level, name) in &guard_names {
        let count_a = if let Some(g) = guard_a {
            intersection.iter()
                .filter(|uid| g.get(uid.as_str()).and_then(|v| v.as_i64()) == Some(level))
                .count()
        } else { 0 };
        let count_b = if let Some(g) = guard_b {
            intersection.iter()
                .filter(|uid| g.get(uid.as_str()).and_then(|v| v.as_i64()) == Some(level))
                .count()
        } else { 0 };
        a_guard.push(serde_json::json!({"tier": name, "count": count_a}));
        b_guard.push(serde_json::json!({"tier": name, "count": count_b}));
    }

    // 共同上舰
    let shared_guard = intersection.iter()
        .filter(|uid| {
            let ga = guard_a.and_then(|g| g.get(uid.as_str())).and_then(|v| v.as_i64()).unwrap_or(0);
            let gb = guard_b.and_then(|g| g.get(uid.as_str())).and_then(|v| v.as_i64()).unwrap_or(0);
            ga > 0 && gb > 0
        })
        .count();

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "a_total": a_total,
            "b_total": b_total,
            "intersection": inter_count,
            "a_in_b_percent": if a_total > 0 { inter_count as f64 / a_total as f64 * 100.0 } else { 0.0 },
            "b_in_a_percent": if b_total > 0 { inter_count as f64 / b_total as f64 * 100.0 } else { 0.0 },
            "a_name": json_a["medal"].as_object().map(|_| ""),  // 需从 rooms.json 获取
            "b_name": json_b["medal"].as_object().map(|_| ""),
            "a_levels": a_levels,
            "b_levels": b_levels,
            "a_guard": a_guard,
            "b_guard": b_guard,
            "shared_guard": shared_guard
        }
    })))
}
```

- [ ] **Step 3: 编译验证**

Run: `cd rust_backend && cargo build 2>&1 | tail -5`
Expected: `Finished dev [unoptimized + debuginfo]`

- [ ] **Step 4: 测试代理端点**

Run: `cargo run &` 然后 `curl "http://localhost:2992/gift/fans_overlap?room_id_a=1820703922&room_id_b=21481907" | jq .`
Expected: 返回包含 `success: true` 和 `data` 的 JSON

- [ ] **Step 5: Commit**

---

### Task 2: 前端 API 层 — `getFanOverlap`

**Covers:** 前端 API 调用

**Files:**
- Modify: `frontend/src/api/index.js`

**Interfaces:**
- Consumes: `GET /gift/fans_overlap?room_id_a=X&room_id_b=Y`
- Produces: `getFanOverlap(roomIdA, roomIdB): Promise<{success, data}>`

- [ ] **Step 1: 在 api/index.js 中添加函数**

在文件末尾的 export 之前添加：

```js
/**
 * 粉丝团查重 - 分析两个主播粉丝重叠情况
 * @param {number} roomIdA - 主播A的房间号
 * @param {number} roomIdB - 主播B的房间号
 */
export const getFanOverlap = (roomIdA, roomIdB) => {
  return request.get('/gift/fans_overlap', {
    params: { room_id_a: roomIdA, room_id_b: roomIdB }
  })
}
```

确保在文件末尾的导出对象中加入 `getFanOverlap`。

- [ ] **Step 2: Commit**

---

### Task 3: FanOverlap.vue 组件 — 选择主播 + 结果展示

**Covers:** 前端核心交互组件

**Files:**
- Create: `frontend/src/components/FanOverlap.vue`

**Interfaces:**
- Consumes: `initialAnchors` prop (Array) — 从 AnchorList 传入的已选主播
- Consumes: `getFanOverlap(roomIdA, roomIdB)` API
- Produces: `@close` event

- [ ] **Step 1: 创建 FanOverlap.vue 骨架**

组件结构复用 AnchorBattle 的三步流程模式：

```vue
<template>
  <!-- 全屏覆盖层 -->
  <div class="fan-overlap-overlay" v-if="!result">
    <div class="fan-overlap-modal">
      <div class="modal-header">
        <h3>🔍 粉丝团查重</h3>
        <button class="close-btn" @click="$emit('close')">✕</button>
      </div>

      <!-- 步骤1：选择两个主播 -->
      <div class="step-content" v-if="step === 'select'">
        <p class="step-hint">选择两个主播进行粉丝团查重分析</p>
        <div class="anchor-selectors">
          <div class="selector-group">
            <label>主播A：</label>
            <select v-model="selectedA" class="anchor-select">
              <option value="">请选择...</option>
              <option v-for="anchor in anchors" :key="anchor.room_id" :value="anchor">
                {{ anchor.name }} ({{ anchor.union }})
              </option>
            </select>
          </div>
          <div class="vs-divider">VS</div>
          <div class="selector-group">
            <label>主播B：</label>
            <select v-model="selectedB" class="anchor-select">
              <option value="">请选择...</option>
              <option v-for="anchor in anchors" :key="anchor.room_id" :value="anchor">
                {{ anchor.name }} ({{ anchor.union }})
              </option>
            </select>
          </div>
        </div>
        <div class="action-bar">
          <button class="confirm-btn" @click="startAnalysis" :disabled="!selectedA || !selectedB || loading">
            {{ loading ? '分析中...' : '开始查重' }}
          </button>
        </div>
        <p v-if="error" class="error-text">{{ error }}</p>
      </div>
    </div>
  </div>

  <!-- 结果展示（全屏覆盖） -->
  <div class="fan-overlap-result" v-if="result">
    <div class="result-header">
      <h3>🔍 {{ result.a_name }} VS {{ result.b_name }} 粉丝团查重</h3>
      <button class="back-btn" @click="resetSelection">← 返回</button>
      <button class="close-btn" @click="$emit('close')">✕</button>
    </div>

    <div class="result-body">
      <!-- 总览 -->
      <div class="overview-section">
        <div class="stat-card">
          <div class="stat-label">{{ result.a_name }} 粉丝团总数</div>
          <div class="stat-value">{{ result.a_total }}</div>
        </div>
        <div class="stat-card highlight">
          <div class="stat-label">交集粉丝</div>
          <div class="stat-value">{{ result.intersection }}</div>
        </div>
        <div class="stat-card">
          <div class="stat-label">{{ result.b_name }} 粉丝团总数</div>
          <div class="stat-value">{{ result.b_total }}</div>
        </div>
      </div>

      <!-- 双向百分比 -->
      <div class="direction-section">
        <div class="direction-card">
          <span class="direction-label">{{ result.a_name }}&{{ result.b_name }}</span>
          <span class="direction-value">{{ result.intersection }} {{ result.a_in_b_percent.toFixed(2) }}%</span>
        </div>
        <div class="direction-card">
          <span class="direction-label">{{ result.b_name }}&{{ result.a_name }}</span>
          <span class="direction-value">{{ result.intersection }} {{ result.b_in_a_percent.toFixed(2) }}%</span>
        </div>
      </div>

      <!-- 等级分布柱状图 -->
      <div class="chart-section">
        <h4>等级分布</h4>
        <canvas ref="levelChart" width="800" height="300"></canvas>
      </div>

      <!-- 舰长等级统计 -->
      <div class="guard-section">
        <h4>舰长等级统计</h4>
        <div class="guard-grid">
          <div class="guard-row" v-for="(item, i) in result.a_guard" :key="i">
            <span class="guard-tier">{{ item.tier }}</span>
            <span class="guard-a">{{ result.a_name }}: {{ item.count }} ({{ (item.count / result.a_total * 100).toFixed(2) }}%)</span>
            <span class="guard-b">{{ result.b_name }}: {{ result.b_guard[i].count }} ({{ (result.b_guard[i].count / result.b_total * 100).toFixed(2) }}%)</span>
          </div>
          <div class="guard-row shared">
            <span class="guard-tier">共同上舰</span>
            <span class="guard-value">{{ result.shared_guard }}</span>
          </div>
        </div>
      </div>

      <!-- 说明 -->
      <div class="explanation-section">
        <p>————说明————</p>
        <p>{{ result.a_name }}&{{ result.b_name }}：{{ result.a_name }}中拥有{{ result.b_name }}牌子占{{ result.a_name }}的比重</p>
        <p>{{ result.a_name }}&{{ result.b_name }} 1~10级：{{ result.a_name }}中拥有{{ result.b_name }}牌子且{{ result.a_name }}为1~10级</p>
        <p>{{ result.a_name }}&{{ result.b_name }} 舰长：{{ result.a_name }}中拥有{{ result.b_name }}牌子且为{{ result.a_name }}舰长</p>
        <p>共同上舰：两者均有上舰（不论等级）</p>
      </div>
    </div>
  </div>
</template>
```

- [ ] **Step 2: 实现 script 逻辑**

```vue
<script setup>
import { ref, onMounted, nextTick, watch } from 'vue'
import { getFanOverlap } from '@/api'
import { Chart, registerables } from 'chart.js'
Chart.register(...registerables)

const props = defineProps({
  initialAnchors: { type: Array, default: () => [] }
})
const emit = defineEmits(['close'])

const anchors = ref([])
const selectedA = ref(null)
const selectedB = ref(null)
const loading = ref(false)
const error = ref('')
const step = ref('select')
const result = ref(null)
const levelChart = ref(null)
let chartInstance = null

onMounted(() => {
  anchors.value = [...props.initialAnchors]
})

const startAnalysis = async () => {
  if (!selectedA.value || !selectedB.value) return
  loading.value = true
  error.value = ''
  try {
    const resp = await getFanOverlap(selectedA.value.room_id, selectedB.value.room_id)
    if (resp.success) {
      resp.data.a_name = selectedA.value.name
      resp.data.b_name = selectedB.value.name
      result.value = resp.data
      step.value = 'result'
      await nextTick()
      renderLevelChart()
    } else {
      error.value = '查询失败，请稍后再试'
    }
  } catch (e) {
    error.value = '粉丝团缓存查询失败，请稍后再试'
  } finally {
    loading.value = false
  }
}

const renderLevelChart = () => {
  if (!levelChart.value || !result.value) return
  if (chartInstance) chartInstance.destroy()

  const labels = result.value.a_levels.map(l => l.range)
  chartInstance = new Chart(levelChart.value, {
    type: 'bar',
    data: {
      labels,
      datasets: [
        {
          label: `${result.value.a_name} 牌子等级`,
          data: result.value.a_levels.map(l => l.count),
          backgroundColor: 'rgba(255, 107, 107, 0.7)',
          borderColor: '#FF6B6B',
          borderWidth: 1
        },
        {
          label: `${result.value.b_name} 牌子等级`,
          data: result.value.b_levels.map(l => l.count),
          backgroundColor: 'rgba(75, 192, 192, 0.7)',
          borderColor: '#4BC0C0',
          borderWidth: 1
        }
      ]
    },
    options: {
      responsive: true,
      plugins: {
        title: { display: true, text: '交集粉丝等级分布' }
      },
      scales: {
        y: { beginAtZero: true, title: { display: true, text: '人数' } },
        x: { title: { display: true, text: '牌子等级' } }
      }
    }
  })
}

const resetSelection = () => {
  result.value = null
  step.value = 'select'
  selectedA.value = null
  selectedB.value = null
  if (chartInstance) { chartInstance.destroy(); chartInstance = null }
}
</script>
```

- [ ] **Step 3: 添加样式**

样式复用 AnchorBattle 的全屏覆盖层风格，按钮颜色用 `#9B59B6`（紫色）：

```vue
<style scoped>
.fan-overlap-overlay, .fan-overlap-result {
  position: fixed; top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0, 0, 0, 0.85); z-index: 1000;
  display: flex; align-items: center; justify-content: center;
  color: white; overflow-y: auto;
}
.fan-overlap-modal {
  background: #1a1a2e; border-radius: 16px; padding: 24px;
  max-width: 600px; width: 90%;
}
.modal-header, .result-header {
  display: flex; justify-content: space-between; align-items: center;
  margin-bottom: 20px;
}
.close-btn, .back-btn {
  background: none; border: none; color: white; font-size: 18px; cursor: pointer;
}
.confirm-btn {
  background: #9B59B6; color: white; border: none; padding: 12px 32px;
  border-radius: 25px; font-size: 16px; font-weight: bold; cursor: pointer;
}
.confirm-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.anchor-select {
  width: 100%; padding: 10px; border-radius: 8px; border: 1px solid #444;
  background: #2a2a3e; color: white; font-size: 14px;
}
.vs-divider {
  text-align: center; font-size: 24px; font-weight: bold; color: #9B59B6;
  margin: 12px 0;
}
.result-body { max-width: 900px; margin: 0 auto; padding: 20px; }
.overview-section { display: flex; gap: 20px; justify-content: center; margin: 20px 0; }
.stat-card {
  background: #2a2a3e; border-radius: 12px; padding: 16px 24px; text-align: center;
}
.stat-card.highlight { border: 2px solid #9B59B6; }
.stat-label { font-size: 13px; color: #aaa; margin-bottom: 4px; }
.stat-value { font-size: 28px; font-weight: bold; }
.direction-section { display: flex; gap: 20px; justify-content: center; margin: 20px 0; }
.direction-card {
  background: #2a2a3e; border-radius: 12px; padding: 12px 20px;
  display: flex; gap: 12px; align-items: center;
}
.direction-label { font-weight: bold; color: #9B59B6; }
.direction-value { font-size: 18px; }
.chart-section { margin: 24px 0; }
.guard-section { margin: 24px 0; }
.guard-grid { display: flex; flex-direction: column; gap: 8px; }
.guard-row {
  display: flex; gap: 20px; align-items: center;
  background: #2a2a3e; border-radius: 8px; padding: 10px 16px;
}
.guard-row.shared { border: 1px solid #9B59B6; }
.guard-tier { font-weight: bold; min-width: 60px; }
.explanation-section {
  margin-top: 24px; padding: 16px; background: #2a2a3e;
  border-radius: 12px; font-size: 13px; color: #aaa; line-height: 1.8;
}
.error-text { color: #ff6b6b; margin-top: 8px; }
.step-hint { color: #aaa; margin-bottom: 16px; }
</style>
```

- [ ] **Step 4: Commit**

---

### Task 4: 入口点集成 — App.vue + NavigationTable + AnchorList

**Covers:** UI 入口和事件桥接

**Files:**
- Modify: `frontend/src/App.vue`
- Modify: `frontend/src/components/NavigationTable.vue`
- Modify: `frontend/src/components/AnchorList.vue`

**Interfaces:**
- Consumes: `FanOverlap.vue` 组件
- Produces: `window.dispatchEvent('popup-open-fan-overlap')` 事件

- [ ] **Step 1: App.vue — 抽屉新增按钮**

在 `.drawer-content` 中排名对比按钮之后添加：

```html
<div class="drawer-item">
  <button class="drawer-btn fan-overlap-btn" @click="emitFanOverlap">
    🔍 粉丝团查重 ({{ selectedCount }})
  </button>
  <p class="drawer-desc">分析两个主播的粉丝重叠情况</p>
</div>
```

在 methods 中添加：

```js
emitFanOverlap() {
  window.dispatchEvent(new CustomEvent('popup-open-fan-overlap'))
}
```

在 style 中添加按钮颜色：

```css
.fan-overlap-btn { background: #9B59B6; color: white; }
```

- [ ] **Step 2: NavigationTable.vue — 新增按钮**

在 `.battle-controls` 中排名对比按钮之后添加：

```html
<button
  @click.stop="openFanOverlap"
  class="fan-overlap-btn">
  🔍 粉丝团查重 ({{ selectedAnchors.length }})
</button>
```

在 methods 中添加：

```js
openFanOverlap() {
  if (this.selectedAnchors.length < 2) {
    this.battleHintVisible = true
    setTimeout(() => { this.battleHintVisible = false }, 2000)
    return
  }
  this.$emit('open-fan-overlap', this.selectedAnchors)
}
```

在 style 中添加：

```css
.fan-overlap-btn {
  background: linear-gradient(45deg, #9B59B6, #8E44AD);
  color: white; border: none; border-radius: 25px;
  padding: 12px 24px; font-size: 14px; font-weight: bold; cursor: pointer;
  box-shadow: 0 4px 15px rgba(155, 89, 182, 0.4);
}
```

- [ ] **Step 3: AnchorList.vue — 事件桥接和组件挂载**

在 `import` 区域添加：

```js
import FanOverlap from './FanOverlap.vue'
```

在 `setup()` 中添加状态：

```js
const showFanOverlap = ref(false)
const fanOverlapAnchors = ref([])
```

在模板中 AnchorBattle 和 RankComparison 之后添加：

```html
<FanOverlap
  v-if="showFanOverlap"
  :initial-anchors="fanOverlapAnchors"
  @close="closeFanOverlapModal"
/>
```

在 NavigationTable 组件标签上添加事件监听：

```html
@open-fan-overlap="openFanOverlapModal"
```

添加方法：

```js
const openFanOverlapModal = (selected) => {
  fanOverlapAnchors.value = selected
  showFanOverlap.value = true
}
const closeFanOverlapModal = () => {
  showFanOverlap.value = false
  fanOverlapAnchors.value = []
}
```

在 `onMounted` 中添加 window event 监听：

```js
const handlePopupFanOverlap = () => {
  if (selectedAnchorsForExport.value.length < 2) {
    alert('请先在导航表格中勾选至少2个主播')
    return
  }
  openFanOverlapModal(selectedAnchorsForExport.value)
}
window.addEventListener('popup-open-fan-overlap', handlePopupFanOverlap)
```

在 `onUnmounted` 中清理：

```js
window.removeEventListener('popup-open-fan-overlap', handlePopupFanOverlap)
```

- [ ] **Step 4: Commit**

---

### Task 5: 文档更新

**Covers:** 文档同步

**Files:**
- Create: `docs/features/fan-overlap.md`
- Modify: `docs/changes/changelog.md`

- [ ] **Step 1: 创建 docs/features/fan-overlap.md**

```markdown
# 粉丝团查重

## 功能概述

分析两个主播粉丝团的重叠情况，展示双向重叠百分比、等级分布和舰长等级统计。

## 数据来源

上游 API：`https://fans.harei.cn`（bilifanscache 服务）

- `GET /fans?room_id=<id>` — 获取某主播全部粉丝 UID + 牌子等级 + 舰长等级
- 缓存刷新周期：~6.5 分钟（130 个主播，每个 3 秒间隔）

## 计算逻辑

- **双向百分比**：A&B = |A∩B|/|A|×100%，B&A = |A∩B|/|B|×100%
- **等级分布**：交集粉丝按牌子等级分桶（1-10/11-20/21-30/31-40/41-50/51-60）
- **舰长等级**：舰长(3)/提督(2)/总督(1) 各级别统计
- **共同上舰**：双方均有上舰的粉丝数

## 前端入口

1. 右侧抽屉 → 🔍 粉丝团查重 按钮
2. 导航表格上方 → 🔍 粉丝团查重 按钮

## 后端端点

`GET /gift/fans_overlap?room_id_a=<id>&room_id_b=<id>`

代理上游 bilifanscache API，返回交集计算结果。
```

- [ ] **Step 2: 更新 docs/changes/changelog.md**

在最新版本条目之后添加新条目（版本号待定）。

- [ ] **Step 3: Commit**
