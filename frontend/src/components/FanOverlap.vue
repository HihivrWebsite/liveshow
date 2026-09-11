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

          <!-- 交集总数 -->
          <div class="intersection-bar">
            <span class="intersection-label">交集</span>
            <span class="intersection-value">{{ pair.intersection }}</span>
            <span class="intersection-sub">共同粉丝团</span>
          </div>

          <!-- 双向百分比对比条 -->
          <div class="overlap-bars">
            <div class="bar-row">
              <span class="bar-label">{{ getAnchorName(pair.a_room_id) }} → {{ getAnchorName(pair.b_room_id) }}</span>
              <div class="bar-track">
                <div class="bar-fill" :style="{ width: pair.a_in_b_percent + '%' }"></div>
              </div>
              <span class="bar-percent">{{ pair.a_in_b_percent }}%</span>
            </div>
            <div class="bar-row">
              <span class="bar-label">{{ getAnchorName(pair.b_room_id) }} → {{ getAnchorName(pair.a_room_id) }}</span>
              <div class="bar-track">
                <div class="bar-fill" :style="{ width: pair.b_in_a_percent + '%' }"></div>
              </div>
              <span class="bar-percent">{{ pair.b_in_a_percent }}%</span>
            </div>
          </div>

          <!-- 等级分布（CSS 条形图） -->
          <div class="level-section">
            <h4>等级分布</h4>
            <div v-for="(lv, i) in pair.a_levels" :key="i" class="level-row">
              <span class="level-label">{{ lv.range }}</span>
              <div class="level-bars">
                <div class="level-bar level-a" :style="{ width: getBarWidth(lv.count, pair.a_total) + '%' }"></div>
                <span class="level-count">{{ lv.count }}</span>
              </div>
              <div class="level-bars">
                <div class="level-bar level-b" :style="{ width: getBarWidth(pair.b_levels[i].count, pair.b_total) + '%' }"></div>
                <span class="level-count">{{ pair.b_levels[i].count }}</span>
              </div>
            </div>
          </div>

          <!-- 舰长统计 -->
          <div class="guard-section">
            <div class="guard-row" v-for="(g, i) in pair.a_guard" :key="i">
              <span class="guard-tier">{{ g.tier }}</span>
              <span class="guard-a">{{ g.count }}</span>
              <span class="guard-vs">vs</span>
              <span class="guard-b">{{ pair.b_guard[i].count }}</span>
            </div>
            <div class="guard-row shared">
              <span class="guard-tier">共同上舰</span>
              <span class="guard-shared">{{ pair.shared_guard }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 说明 -->
      <div class="explanation">
        <p>————说明————</p>
        <p>A→B：A中拥有B牌子占A的比重</p>
        <p>等级分布：交集粉丝按牌子等级分桶</p>
        <p>舰长统计：交集粉丝中的舰长/提督/总督数量</p>
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

/* ========== 卡片网格 ========== */
.pair-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(420px, 1fr));
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

/* ========== 交集横条 ========== */
.intersection-bar {
  text-align: center;
  padding: 12px;
  margin-bottom: 16px;
  background: #FFE5B4;
  border-radius: 12px;
}

.intersection-value {
  font-size: 28px;
  font-weight: bold;
  color: #E74C3C;
}

.intersection-label {
  color: #666;
  font-size: 12px;
  margin-right: 8px;
}

.intersection-sub {
  color: #888;
  font-size: 11px;
  margin-left: 8px;
}

/* ========== 双向百分比对比条 ========== */
.overlap-bars {
  margin-bottom: 16px;
}

.bar-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.bar-label {
  color: #666;
  font-size: 11px;
  min-width: 120px;
  text-align: right;
  flex-shrink: 0;
}

.bar-track {
  flex: 1;
  height: 16px;
  background: rgba(0, 0, 0, 0.08);
  border-radius: 8px;
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #FF6600, #FFC633);
  border-radius: 8px;
  transition: width 0.5s ease;
}

.bar-percent {
  color: #333;
  font-size: 12px;
  font-weight: bold;
  min-width: 50px;
  flex-shrink: 0;
}

/* ========== 等级分布（CSS 条形图） ========== */
.level-section {
  margin-bottom: 12px;
}

.level-section h4 {
  color: #666;
  font-size: 12px;
  margin: 0 0 6px 0;
}

.level-row {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-bottom: 3px;
}

.level-label {
  color: #888;
  font-size: 10px;
  min-width: 32px;
  text-align: right;
  flex-shrink: 0;
}

.level-bars {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 4px;
}

.level-bar {
  height: 10px;
  border-radius: 5px;
  min-width: 2px;
  transition: width 0.3s ease;
}

.level-a {
  background: #FF6600;
}

.level-b {
  background: #FFC633;
}

.level-count {
  color: #888;
  font-size: 10px;
  min-width: 24px;
  flex-shrink: 0;
}

/* ========== 舰长统计 ========== */
.guard-section {
  border-top: 1px solid #FFE5B4;
  padding-top: 10px;
}

.guard-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
  font-size: 12px;
}

.guard-tier {
  color: #666;
  min-width: 50px;
}

.guard-a {
  color: #FF6600;
  font-weight: bold;
}

.guard-vs {
  color: #999;
}

.guard-b {
  color: #FFC633;
  font-weight: bold;
}

.guard-shared {
  color: #E74C3C;
  font-weight: bold;
}

.guard-row.shared {
  border-top: 1px dashed #FFE5B4;
  margin-top: 4px;
  padding-top: 8px;
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
@media (max-width: 480px) {
  .pair-grid {
    grid-template-columns: 1fr;
  }

  .bar-label {
    min-width: 80px;
    font-size: 10px;
  }
}
</style>
