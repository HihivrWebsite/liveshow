<template>
  <div class="fan-overlap">
    <!-- 加载中 -->
    <div v-if="loading" class="loading-overlay">
      <div class="loading-hint">⏳ 正在查询粉丝重合度数据...</div>
    </div>

    <!-- 结果展示 -->
    <div v-if="pairs.length > 0" class="result-page">
      <div class="result-header">
        <h2>🔍 粉丝重合度查询（{{ anchors.length }}人，{{ pairs.length }}组配对）</h2>
        <div class="header-actions">
          <button @click="$emit('close')" class="go-back-btn">✕ 关闭</button>
        </div>
      </div>

      <!-- 卡片网格 -->
      <div class="pair-grid">
        <div v-for="(pair, idx) in pairs" :key="idx" class="pair-card">
          <!-- 卡片头部：头像 + 名字 + VS -->
          <div class="card-header">
            <div class="card-anchor">
              <img :src="getAvatar(pair.a_room_id)" :alt="getAnchorName(pair.a_room_id)" class="anchor-avatar">
              <span class="anchor-name">{{ getAnchorName(pair.a_room_id) }}</span>
              <span class="anchor-total">{{ pair.a_total }}人</span>
            </div>
            <div class="vs-badge">VS</div>
            <div class="card-anchor">
              <img :src="getAvatar(pair.b_room_id)" :alt="getAnchorName(pair.b_room_id)" class="anchor-avatar">
              <span class="anchor-name">{{ getAnchorName(pair.b_room_id) }}</span>
              <span class="anchor-total">{{ pair.b_total }}人</span>
            </div>
          </div>

          <!-- 关键数据大数字展示 -->
          <div class="hero-stats">
            <div class="hero-stat hero-intersection">
              <div class="hero-number">{{ pair.intersection }}</div>
              <div class="hero-label">交集粉丝</div>
            </div>
            <div class="hero-stat">
              <div class="hero-number hero-orange">{{ pair.a_in_b_percent }}%</div>
              <div class="hero-label">{{ getAnchorName(pair.a_room_id) }} → {{ getAnchorName(pair.b_room_id) }}</div>
            </div>
            <div class="hero-stat">
              <div class="hero-number hero-yellow">{{ pair.b_in_a_percent }}%</div>
              <div class="hero-label">{{ getAnchorName(pair.b_room_id) }} → {{ getAnchorName(pair.a_room_id) }}</div>
            </div>
          </div>

          <!-- 韦恩图/交集可视化 -->
          <div class="venn-container">
            <div class="venn-diagram">
              <div class="venn-circle venn-a">
                <span class="venn-total">{{ pair.a_total }}</span>
                <span class="venn-name">{{ getAnchorName(pair.a_room_id) }}</span>
              </div>
              <div class="venn-circle venn-b">
                <span class="venn-total">{{ pair.b_total }}</span>
                <span class="venn-name">{{ getAnchorName(pair.b_room_id) }}</span>
              </div>
              <div class="venn-intersection">
                <span class="venn-intersect-num">{{ pair.intersection }}</span>
                <span class="venn-intersect-label">交集</span>
              </div>
            </div>
          </div>

          <!-- 双向百分比水平对比条 -->
          <div class="overlap-section">
            <h4 class="section-title">双向占比</h4>
            <div class="bar-row">
              <span class="bar-label">{{ getAnchorName(pair.a_room_id) }}→{{ getAnchorName(pair.b_room_id) }}</span>
              <div class="bar-track">
                <div class="bar-fill bar-fill-a" :style="{ width: Math.max(pair.a_in_b_percent, 3) + '%' }">
                  <span class="bar-inner-text" v-if="pair.a_in_b_percent > 15">{{ pair.a_in_b_percent }}%</span>
                </div>
              </div>
              <span class="bar-percent bar-percent-a">{{ pair.a_in_b_percent }}%</span>
              <span class="bar-count">({{ pair.intersection }}/{{ pair.a_total }})</span>
            </div>
            <div class="bar-row">
              <span class="bar-label">{{ getAnchorName(pair.b_room_id) }}→{{ getAnchorName(pair.a_room_id) }}</span>
              <div class="bar-track">
                <div class="bar-fill bar-fill-b" :style="{ width: Math.max(pair.b_in_a_percent, 3) + '%' }">
                  <span class="bar-inner-text" v-if="pair.b_in_a_percent > 15">{{ pair.b_in_a_percent }}%</span>
                </div>
              </div>
              <span class="bar-percent bar-percent-b">{{ pair.b_in_a_percent }}%</span>
              <span class="bar-count">({{ pair.intersection }}/{{ pair.b_total }})</span>
            </div>
          </div>

          <!-- 等级分布 — 双色并排分组条形图 -->
          <div class="level-section">
            <h4 class="section-title">等级分布</h4>
            <div class="level-legend">
              <span class="legend-item"><span class="legend-dot dot-a"></span>{{ getAnchorName(pair.a_room_id) }}</span>
              <span class="legend-item"><span class="legend-dot dot-b"></span>{{ getAnchorName(pair.b_room_id) }}</span>
            </div>
            <div v-for="(lv, i) in pair.a_levels" :key="i" class="level-group">
              <span class="level-label">{{ lv.range }}</span>
              <div class="level-dual-bars">
                <!-- A 的条形 -->
                <div class="level-bar-wrapper">
                  <div class="level-bar-track">
                    <div class="level-bar level-a" :style="{ width: getBarWidth(lv.count, pair.a_total) + '%' }"></div>
                  </div>
                  <span class="level-bar-value">{{ lv.count }} <small>{{ formatPercent(lv.count, pair.a_total) }}</small></span>
                </div>
                <!-- B 的条形（并排） -->
                <div class="level-bar-wrapper">
                  <div class="level-bar-track">
                    <div class="level-bar level-b" :style="{ width: getBarWidth(pair.b_levels[i].count, pair.b_total) + '%' }"></div>
                  </div>
                  <span class="level-bar-value">{{ pair.b_levels[i].count }} <small>{{ formatPercent(pair.b_levels[i].count, pair.b_total) }}</small></span>
                </div>
              </div>
            </div>
          </div>

          <!-- 舰长统计 — 对比表格 -->
          <div class="guard-section">
            <h4 class="section-title">舰长统计</h4>
            <table class="guard-table">
              <thead>
                <tr>
                  <th>等级</th>
                  <th class="col-a">{{ getAnchorName(pair.a_room_id) }}</th>
                  <th class="col-vs"></th>
                  <th class="col-b">{{ getAnchorName(pair.b_room_id) }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(g, i) in pair.a_guard" :key="i">
                  <td class="guard-tier">{{ g.tier }}</td>
                  <td class="col-a">{{ g.count }} <small>{{ formatPercent(g.count, pair.a_total) }}</small></td>
                  <td class="col-vs">vs</td>
                  <td class="col-b">{{ pair.b_guard[i].count }} <small>{{ formatPercent(pair.b_guard[i].count, pair.b_total) }}</small></td>
                </tr>
                <tr class="guard-shared-row">
                  <td class="guard-tier">共同上舰</td>
                  <td colspan="3" class="guard-shared-val">{{ pair.shared_guard }} <small>{{ formatPercent(pair.shared_guard, pair.a_total) }}</small></td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- 说明 -->
      <div class="explanation">
        <p>————说明————</p>
        <p>{{ getAnchorName(pairs[0].a_room_id) }}&{{ getAnchorName(pairs[0].b_room_id) }}：{{ getAnchorName(pairs[0].a_room_id) }}中拥有{{ getAnchorName(pairs[0].b_room_id) }}牌子占{{ getAnchorName(pairs[0].a_room_id) }}的比重</p>
        <p>{{ getAnchorName(pairs[0].a_room_id) }}&{{ getAnchorName(pairs[0].b_room_id) }} 1~10级：{{ getAnchorName(pairs[0].a_room_id) }}中拥有{{ getAnchorName(pairs[0].b_room_id) }}牌子且{{ getAnchorName(pairs[0].a_room_id) }}为1~10级</p>
        <p>{{ getAnchorName(pairs[0].a_room_id) }}&{{ getAnchorName(pairs[0].b_room_id) }} 舰长：{{ getAnchorName(pairs[0].a_room_id) }}中拥有{{ getAnchorName(pairs[0].b_room_id) }}牌子且为{{ getAnchorName(pairs[0].a_room_id) }}舰长</p>
        <p>共同上舰：两者均有上舰（不论等级）</p>
      </div>
    </div>

    <!-- 错误 -->
    <div v-if="error" class="error-hint">❌ {{ error }}<button @click="$emit('close')" class="go-back-btn">关闭</button></div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { anchorAPI } from '@/api'
import { getAvatarSync } from '@/utils/avatarCache'

// Props: 从导航表格传入的已勾选主播列表
const props = defineProps({
  initialAnchors: { type: Array, default: () => [] }
})

const emit = defineEmits(['close'])

// 响应式状态
const loading = ref(true)
const error = ref('')
const pairs = ref([])       // 所有配对结果
const anchors = ref([])     // 主播列表
const roomNameMap = ref({}) // 房间号->名称映射
const roomAvatarMap = ref({}) // 房间号->头像URL映射

// 根据房间号获取主播名称（统一用字符串key）
function getAnchorName(roomId) {
  return roomNameMap.value[String(roomId)] || roomId
}

// 根据房间号获取头像URL（使用 avatarCache）
function getAvatar(roomId) {
  return getAvatarSync(roomId) || `/gift/avatar?room_id=${roomId}`
}

// 计算条形图宽度百分比（最小 2% 保证可见）
function getBarWidth(count, total) {
  return total > 0 ? Math.max((count / total) * 100, 2) : 0
}

// 格式化百分比（占总数）
function formatPercent(count, total) {
  return total > 0 ? (count / total * 100).toFixed(2) + '%' : '0%'
}

// 生命周期
onMounted(async () => {
  // 校验：至少需要 2 个主播
  if (props.initialAnchors.length < 2) {
    error.value = '请先在导航表格中勾选至少 2 个主播'
    loading.value = false
    return
  }

  // 构建房间号->名称/头像映射
  const nameMap = {}
  const avatarMap = {}
  const roomIds = []
  for (const anchor of props.initialAnchors) {
    nameMap[String(anchor.room_id)] = anchor.anchor_name || anchor.name || String(anchor.room_id)
    roomIds.push(anchor.room_id)
  }
  roomNameMap.value = nameMap
  roomAvatarMap.value = avatarMap
  anchors.value = props.initialAnchors

  try {
    const resp = await anchorAPI.getFanOverlapMulti(roomIds)
    if (resp && resp.success && resp.pairs) {
      pairs.value = resp.pairs
    } else {
      error.value = '查询失败，请稍后再试'
    }
  } catch (e) {
    console.error('粉丝重合度查询失败:', e)
    error.value = '粉丝重合度查询失败，请稍后再试'
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
/* 全屏覆盖层 — 与 AnchorBattle/RankComparison 风格一致 */
.fan-overlap {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  z-index: 1000;
  overflow-y: auto;
  padding: 30px;
}

/* ========== 结果页面 ========== */
.result-page {
  max-width: 1200px;
  margin: 0 auto;
}

/* 顶部操作栏 */
.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.result-header h2 {
  color: white;
  font-size: 1.5rem;
  margin: 0;
}

.go-back-btn {
  background: linear-gradient(45deg, #9B59B6, #8E44AD);
  color: white;
  border: none;
  border-radius: 25px;
  padding: 10px 24px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.3s ease;
}

.go-back-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(155, 89, 182, 0.4);
}

/* ========== 通用区块标题 ========== */
.section-title {
  color: #666;
  font-size: 13px;
  font-weight: bold;
  margin: 0 0 10px 0;
  padding-bottom: 6px;
  border-bottom: 1px dashed #FFE5B4;
}

/* ========== 卡片网格 ========== */
.pair-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(460px, 1fr));
  gap: 20px;
}

.pair-card {
  background: #FFF8E1;
  border: 1px solid #FFE5B4;
  border-radius: 16px;
  padding: 20px;
  transition: transform 0.2s, box-shadow 0.2s;
}

.pair-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 30px rgba(255, 165, 0, 0.2);
}

/* ========== 卡片头部：头像 + VS ========== */
.card-header {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin-bottom: 16px;
}

.card-anchor {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.anchor-avatar {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  border: 2px solid #FF6600;
  object-fit: cover;
}

.anchor-name {
  color: #333;
  font-weight: bold;
  font-size: 14px;
}

.anchor-total {
  color: #888;
  font-size: 12px;
}

.vs-badge {
  background: #FF6600;
  color: white;
  font-weight: bold;
  border-radius: 50%;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  flex-shrink: 0;
}

/* ========== 关键数据大数字展示 ========== */
.hero-stats {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-bottom: 18px;
  padding: 14px 10px;
  background: linear-gradient(135deg, #FFF5E6, #FFF0D4);
  border-radius: 12px;
  border: 1px solid #FFE5B4;
}

.hero-stat {
  text-align: center;
  flex: 1;
}

.hero-number {
  font-size: 26px;
  font-weight: 900;
  color: #E74C3C;
  line-height: 1.2;
}

.hero-intersection .hero-number {
  font-size: 36px;
  color: #E74C3C;
  text-shadow: 0 2px 8px rgba(231, 76, 60, 0.2);
}

.hero-orange {
  color: #FF6600 !important;
}

.hero-yellow {
  color: #E6A800 !important;
}

.hero-label {
  font-size: 11px;
  color: #888;
  margin-top: 4px;
  line-height: 1.3;
}

/* ========== 韦恩图/交集可视化 ========== */
.venn-container {
  display: flex;
  justify-content: center;
  margin-bottom: 18px;
  padding: 10px 0;
}

.venn-diagram {
  position: relative;
  width: 280px;
  height: 160px;
}

.venn-circle {
  position: absolute;
  width: 160px;
  height: 160px;
  border-radius: 50%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  top: 0;
}

.venn-a {
  left: 0;
  background: rgba(255, 102, 0, 0.2);
  border: 3px solid #FF6600;
}

.venn-b {
  right: 0;
  background: rgba(255, 198, 51, 0.2);
  border: 3px solid #FFC633;
}

.venn-total {
  font-size: 20px;
  font-weight: 900;
  color: #333;
  line-height: 1;
}

.venn-name {
  font-size: 10px;
  color: #666;
  margin-top: 2px;
}

.venn-intersection {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  z-index: 2;
  background: rgba(255, 248, 225, 0.85);
  border-radius: 50%;
  width: 64px;
  height: 64px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 10px rgba(231, 76, 60, 0.3);
}

.venn-intersect-num {
  font-size: 20px;
  font-weight: 900;
  color: #E74C3C;
  line-height: 1;
}

.venn-intersect-label {
  font-size: 10px;
  color: #E74C3C;
  font-weight: bold;
}

/* ========== 双向百分比水平对比条 ========== */
.overlap-section {
  margin-bottom: 18px;
}

.bar-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.bar-label {
  color: #555;
  font-size: 11px;
  min-width: 80px;
  text-align: right;
  flex-shrink: 0;
  font-weight: bold;
}

.bar-track {
  flex: 1;
  height: 22px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 11px;
  overflow: hidden;
  box-shadow: inset 0 1px 3px rgba(0,0,0,0.1);
}

.bar-fill {
  height: 100%;
  border-radius: 11px;
  transition: width 0.6s ease;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding-right: 6px;
  min-width: 4px;
}

.bar-fill-a {
  background: linear-gradient(90deg, #FF6600, #FF8C33);
}

.bar-fill-b {
  background: linear-gradient(90deg, #FFC633, #FFD966);
}

.bar-inner-text {
  color: white;
  font-size: 11px;
  font-weight: bold;
  text-shadow: 0 1px 2px rgba(0,0,0,0.3);
}

.bar-percent {
  font-size: 15px;
  font-weight: 900;
  min-width: 50px;
  text-align: right;
  flex-shrink: 0;
}

.bar-percent-a {
  color: #FF6600;
}

.bar-percent-b {
  color: #E6A800;
}

.bar-count {
  color: #999;
  font-size: 11px;
  min-width: 60px;
  flex-shrink: 0;
}

/* ========== 等级分布（双色并排条形图） ========== */
.level-section {
  margin-bottom: 18px;
}

.level-legend {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-bottom: 8px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: #666;
}

.legend-dot {
  width: 10px;
  height: 10px;
  border-radius: 3px;
}

.dot-a {
  background: #FF6600;
}

.dot-b {
  background: #FFC633;
}

.level-group {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}

.level-label {
  color: #888;
  font-size: 10px;
  min-width: 32px;
  text-align: right;
  flex-shrink: 0;
  font-weight: bold;
}

.level-dual-bars {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.level-bar-wrapper {
  display: flex;
  align-items: center;
  gap: 6px;
}

.level-bar-track {
  flex: 1;
  height: 10px;
  background: rgba(0, 0, 0, 0.06);
  border-radius: 5px;
  overflow: hidden;
}

.level-bar {
  height: 100%;
  border-radius: 5px;
  min-width: 2px;
  transition: width 0.4s ease;
}

.level-a {
  background: #FF6600;
}

.level-b {
  background: #FFC633;
}

.level-bar-value {
  color: #555;
  font-size: 11px;
  min-width: 80px;
  flex-shrink: 0;
  font-weight: bold;
}

.level-bar-value small {
  color: #999;
  font-weight: normal;
}

/* ========== 舰长统计 — 对比表格 ========== */
.guard-section {
  border-top: 1px solid #FFE5B4;
  padding-top: 12px;
}

.guard-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.guard-table thead th {
  padding: 6px 8px;
  text-align: center;
  color: #888;
  font-size: 11px;
  font-weight: bold;
  border-bottom: 2px solid #FFE5B4;
}

.guard-table thead th.col-a {
  color: #FF6600;
}

.guard-table thead th.col-b {
  color: #E6A800;
}

.guard-table tbody td {
  padding: 6px 8px;
  text-align: center;
  border-bottom: 1px solid #FFF0D4;
}

.guard-table tbody tr:last-child td {
  border-bottom: none;
}

.guard-tier {
  color: #666;
  font-weight: bold;
  text-align: left !important;
}

.guard-table .col-a {
  color: #FF6600;
  font-weight: bold;
}

.guard-table .col-a small {
  color: #FF9933;
  font-weight: normal;
}

.guard-table .col-vs {
  color: #ccc;
  font-size: 11px;
}

.guard-table .col-b {
  color: #E6A800;
  font-weight: bold;
}

.guard-table .col-b small {
  color: #CC9900;
  font-weight: normal;
}

.guard-shared-row {
  background: #FFF5C2;
}

.guard-shared-row td {
  border-top: 2px dashed #FFE5B4 !important;
}

.guard-shared-val {
  color: #E74C3C !important;
  font-weight: bold;
  text-align: center !important;
  font-size: 15px;
}

.guard-shared-val small {
  color: #E67E73;
  font-weight: normal;
  font-size: 12px;
}

/* ========== Loading ========== */
.loading-overlay {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
}

.loading-hint {
  color: white;
  font-size: 18px;
  padding: 20px 40px;
  background: rgba(26, 26, 46, 0.9);
  border-radius: 12px;
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

/* ========== Error ========== */
.error-hint {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: #1a1a2e;
  padding: 30px;
  border-radius: 16px;
  color: #E74C3C;
  text-align: center;
  z-index: 1001;
}

.error-hint .go-back-btn {
  display: inline-block;
  margin-top: 16px;
}

/* ========== 说明 ========== */
.explanation {
  margin-top: 24px;
  padding: 16px;
  background: #FFF5C2;
  border: 1px solid #FFE5B4;
  border-radius: 12px;
  color: #666;
  font-size: 12px;
  line-height: 1.8;
}

.explanation p {
  margin: 0;
  text-align: center;
}

/* ========== 响应式 ========== */
@media (max-width: 520px) {
  .pair-grid {
    grid-template-columns: 1fr;
  }

  .bar-label {
    min-width: 60px;
    font-size: 10px;
  }

  .hero-stats {
    flex-direction: column;
    gap: 10px;
  }

  .venn-diagram {
    width: 220px;
    height: 130px;
  }

  .venn-circle {
    width: 130px;
    height: 130px;
  }

  .bar-count {
    display: none;
  }
}
</style>
