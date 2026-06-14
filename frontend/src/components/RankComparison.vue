<template>
  <div class="rank-comparison">
    <HeaderSection />

    <div v-if="showDateModal" class="modal-overlay" @click="closeDateModal">
      <div class="modal-content" @click.stop>
        <h3>📊 选择日期范围</h3>
        <div class="form-group">
          <label>起始月份:</label>
          <input type="month" v-model="startDate" class="month-input" min="2025-08">
        </div>
        <div class="form-group">
          <label>结束月份:</label>
          <input type="month" v-model="endDate" class="month-input" min="2025-08">
        </div>
        <div class="button-group">
          <button @click="confirmDateRange" class="confirm-btn">确定</button>
          <button @click="closeDateModal" class="cancel-btn">取消</button>
        </div>
      </div>
    </div>

    <div v-if="showMetricModal" class="modal-overlay" @click="closeMetricModal">
      <div class="modal-content" @click.stop>
        <h3>📊 选择对比指标</h3>
        <select v-model="selectedMetric" class="metric-select">
          <option v-for="(label, key) in metricNameMap" :key="key" :value="key">{{ label }}</option>
        </select>
        <div class="button-group">
          <button @click="confirmMetric" class="confirm-btn">确定</button>
          <button @click="closeMetricModal" class="cancel-btn">取消</button>
        </div>
      </div>
    </div>

    <div v-if="chartVisible" class="chart-page">
      <div class="chart-header">
        <h2>{{ currentMetricName }}排名变化</h2>
        <div class="battle-info">
          <span>日期：{{ startDate }} 至 {{ endDate }}</span>
          <span>主播：{{ selectedAnchors.length }} 人</span>
        </div>
      </div>

      <div class="chart-legend">
        <span v-for="anchor in selectedAnchors" :key="anchor.room_id">
          <input type="checkbox" :checked="visibleAnchors.includes(anchor.room_id)"
            @change="toggleAnchorVisibility(anchor.room_id)">
          <img :src="avatarDataUrls[anchor.room_id] || ''" 
            class="legend-avatar">
          <span class="legend-dot" :style="{ backgroundColor: getAnchorColor(anchor.room_id) }"></span>
          {{ anchor.anchor_name }}
        </span>
      </div>

      <div class="chart-scroll-wrapper">
        <div class="chart-container">
          <canvas ref="chartCanvas"></canvas>
        </div>
      </div>

      <div class="action-buttons">
        <button @click="goBack" class="btn btn-back">返回主页</button>
        <button @click="resetSelection" class="btn btn-reset">重新选择</button>
        <button @click="refreshData" class="btn btn-refresh" :disabled="isRefreshing">刷新/补全数据</button>
        <button @click="exportChart" class="btn btn-export">导出图表</button>
      </div>

      <div v-if="isRefreshing" class="refreshing-overlay">
        <div class="spinner"></div>
        <p>正在刷新数据...</p>
        <p v-if="currentRefreshingAnchor">正在获取：{{ currentRefreshingAnchor }}</p>
      </div>
    </div>

    <FooterSection />
  </div>
</template>

<script>
import { Chart, registerables } from 'chart.js'
import { anchorAPI } from '@/api'
import { getMonthRange } from '@/utils/monthUtils'
import { getAvatar, scaleAvatar } from '@/utils/avatarCache'
import HeaderSection from '@/components/HeaderSection.vue'
import FooterSection from '@/components/FooterSection.vue'

Chart.register(...registerables)

const colorPalette = [
  '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF',
  '#FF9F40', '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4',
  '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE'
]

export default {
  name: 'RankComparison',
  components: { HeaderSection, FooterSection },
  props: {
    initialAnchors: { type: Array, default: () => [] }
  },
  data() {
    return {
      selectedAnchors: [],
      startDate: '',
      endDate: '',
      selectedMetric: 'gift',
      currentMetric: '',
      showDateModal: true,
      showMetricModal: false,
      chartVisible: false,
      sessionsData: [],
      visibleAnchors: [],
      battleChart: null,
      dataCache: new Map(),
      isRefreshing: false,
      currentRefreshingAnchor: '',
      anchorColors: {},
      colorIndex: 0,
      avatarDataUrls: {},
      metricNameMap: {
        duration: '直播时长',
        gift: '礼物收入',
        guard: '舰长收入',
        superChat: 'SC收入',
        totalRevenue: '总营收',
        newGuard3: '新增总督',
        newGuard2: '新增提督',
        newGuard1: '新增舰长',
        newFans: '新增粉丝团',
        danmaku: '弹幕数',
        avgConcurrency: '平均同接',
        maxConcurrency: '最高同接',
        newFansCount: '新增粉丝数'
      }
    }
  },
  computed: {
    currentMetricName() {
      return this.metricNameMap[this.currentMetric] || ''
    }
  },
  mounted() {
    if (this.initialAnchors.length > 0) {
      this.selectedAnchors = [...this.initialAnchors]
    }
    const now = new Date()
    this.startDate = now.toISOString().slice(0, 7)
    this.endDate = this.startDate
  },
  beforeDestroy() {
    if (this.battleChart) {
      this.battleChart.destroy()
    }
  },
  methods: {
    getAnchorColor(roomId) {
      if (!this.anchorColors[roomId]) {
        this.anchorColors[roomId] = colorPalette[this.colorIndex % colorPalette.length]
        this.colorIndex++
      }
      return this.anchorColors[roomId]
    },

    closeDateModal() {
      this.showDateModal = false
      this.$emit('close')
    },
    closeMetricModal() {
      this.showMetricModal = false
    },

    confirmDateRange() {
      if (!this.startDate || !this.endDate) {
        alert('请选择起始和结束月份')
        return
      }
      if (this.startDate > this.endDate) {
        alert('起始月份不能晚于结束月份')
        return
      }
      this.showDateModal = false
      this.showMetricModal = true
    },

    confirmMetric() {
      this.currentMetric = this.selectedMetric
      this.showMetricModal = false
      this.chartVisible = true
      this.$nextTick(() => {
        this.fetchBattleData()
      })
    },

    async fetchBattleData() {
      const cacheKey = this.getCacheKey()
      if (this.isCacheValid(cacheKey)) {
        const cached = this.dataCache.get(cacheKey)
        this.sessionsData = cached.sessions
        this.onDataReady()
        return
      }

      this.sessionsData = new Array(this.selectedAnchors.length).fill(null).map(() => [])

      for (let i = 0; i < this.selectedAnchors.length; i++) {
        await this.fetchSingleAnchorData(this.selectedAnchors[i], i)
        if (i < this.selectedAnchors.length - 1) {
          await new Promise(r => setTimeout(r, 600))
        }
      }

      this.dataCache.set(cacheKey, {
        sessions: this.sessionsData,
        timestamp: Date.now(),
        expiry: 2 * 60 * 60 * 1000
      })

      this.onDataReady()
    },

    async fetchSingleAnchorData(anchor, index) {
      const months = getMonthRange(this.startDate, this.endDate)
      for (const month of months) {
        let success = false
        for (let retry = 0; retry < 3; retry++) {
          try {
            const response = await anchorAPI.getLiveSessions(anchor.room_id, anchor.union, month)
            const sessions = response.sessions || []
            this.sessionsData[index].push(...sessions)
            success = true
            break
          } catch (err) {
            console.error(`获取 ${anchor.anchor_name} ${month} 数据失败 (重试 ${retry + 1}):`, err)
            if (retry < 2) await new Promise(r => setTimeout(r, 2000))
          }
        }
      }
    },

    onDataReady() {
      this.visibleAnchors = this.selectedAnchors.map(a => a.room_id)
      this.$nextTick(() => {
        setTimeout(() => {
          this.renderChart()
        }, 100)
      })
    },

    calculateRankings() {
      const startDateObj = new Date(this.startDate + '-01')
      const endDateObj = new Date(this.endDate + '-01')
      endDateObj.setMonth(endDateObj.getMonth() + 1)
      endDateObj.setDate(0)

      const allDates = []
      const current = new Date(startDateObj)
      while (current <= endDateObj) {
        const y = current.getFullYear()
        const m = String(current.getMonth() + 1).padStart(2, '0')
        const d = String(current.getDate()).padStart(2, '0')
        allDates.push(`${y}-${m}-${d}`)
        current.setDate(current.getDate() + 1)
      }

      const dailyValues = {}
      this.selectedAnchors.forEach((anchor, i) => {
        const roomId = anchor.room_id
        dailyValues[roomId] = {}
        const sessions = this.sessionsData[i] || []
        sessions.forEach(session => {
          const date = session.start_time ? session.start_time.split(' ')[0] : null
          if (!date) return
          const value = this.getMetricValue(session)
          dailyValues[roomId][date] = (dailyValues[roomId][date] || 0) + value
        })
      })

      const cumulativeValues = {}
      this.selectedAnchors.forEach(anchor => {
        const roomId = anchor.room_id
        cumulativeValues[roomId] = {}
        let cumulative = 0
        allDates.forEach(date => {
          cumulative += dailyValues[roomId][date] || 0
          cumulativeValues[roomId][date] = cumulative
        })
      })

      const rankings = {}
      allDates.forEach(date => {
        const dayValues = this.selectedAnchors.map(anchor => ({
          roomId: anchor.room_id,
          value: cumulativeValues[anchor.room_id][date] || 0
        }))
        dayValues.sort((a, b) => b.value - a.value)
        rankings[date] = {}
        dayValues.forEach((item, index) => {
          rankings[date][item.roomId] = index + 1
        })
      })

      return { allDates, rankings, cumulativeValues }
    },

    getMetricValue(session) {
      const m = this.currentMetric
      switch (m) {
        case 'duration': return parseFloat(session.duration_minutes) || 0
        case 'gift': return parseFloat(session.gift) || 0
        case 'guard': return parseFloat(session.guard) || 0
        case 'superChat': return parseFloat(session.super_chat) || 0
        case 'totalRevenue': return parseFloat(session.total_revenue) || 0
        case 'newGuard3': return (parseInt(session.end_guard_3) || 0) - (parseInt(session.start_guard_3) || 0)
        case 'newGuard2': return (parseInt(session.end_guard_2) || 0) - (parseInt(session.start_guard_2) || 0)
        case 'newGuard1': return (parseInt(session.end_guard_1) || 0) - (parseInt(session.start_guard_1) || 0)
        case 'newFans': return (parseInt(session.end_fans_count) || 0) - (parseInt(session.start_fans_count) || 0)
        case 'danmaku': return parseInt(session.danmaku_count) || 0
        case 'avgConcurrency': return parseFloat(session.avg_concurrency) || 0
        case 'maxConcurrency': return parseInt(session.max_concurrency) || 0
        case 'newFansCount': return parseInt(session.new_fans_count) || 0
        default: return 0
      }
    },

    async renderChart() {
      const { allDates, rankings } = this.calculateRankings()

      await this.$nextTick()
      const canvas = this.$refs.chartCanvas
      if (!canvas) return

      canvas.width = 6000
      canvas.height = 1400

      if (this.battleChart) {
        this.battleChart.destroy()
      }

      const scaledAvatars = {}
      const avatarDataUrls = {}
      for (const anchor of this.selectedAnchors) {
        const img = await getAvatar(anchor.room_id)
        if (img) {
          scaledAvatars[anchor.room_id] = scaleAvatar(img, 20)
          const c = document.createElement('canvas')
          c.width = img.naturalWidth; c.height = img.naturalHeight
          c.getContext('2d').drawImage(img, 0, 0)
          avatarDataUrls[anchor.room_id] = c.toDataURL('image/jpeg', 0.8)
        }
      }
      this.avatarDataUrls = avatarDataUrls

      const datasets = this.selectedAnchors
        .filter(a => this.visibleAnchors.includes(a.room_id))
        .map((anchor) => {
          const roomId = anchor.room_id
          const data = allDates.map(date => rankings[date][roomId] || null)
          const avatarImg = scaledAvatars[roomId]
          return {
            label: anchor.anchor_name,
            data,
            borderColor: this.getAnchorColor(roomId),
            backgroundColor: this.getAnchorColor(roomId) + '20',
            pointStyle: avatarImg || 'circle',
            pointRadius: avatarImg ? 10 : 5,
            pointHoverRadius: avatarImg ? 14 : 8,
            pointBorderColor: '#fff',
            pointBorderWidth: 2,
            borderWidth: 3,
            fill: false,
            tension: 0.3,
            spanGaps: true
          }
        })

      const labels = allDates.map(d => d.substring(5))

      this.battleChart = new Chart(canvas.getContext('2d'), {
        type: 'line',
        data: { labels, datasets },
        options: {
          responsive: false,
          animation: { duration: 300, easing: 'easeOutQuart' },
          plugins: {
            legend: { display: false },
            title: {
              display: true,
              text: `${this.currentMetricName}排名变化 - ${this.startDate} 至 ${this.endDate}`,
              font: { size: 36, weight: 'bold' },
              color: '#FF6600',
              padding: { top: 20, bottom: 30 }
            },
            tooltip: {
              mode: 'index',
              intersect: false,
              callbacks: {
                label: (context) => {
                  const rank = context.parsed.y
                  return `${context.dataset.label}: 第${rank}名`
                }
              }
            }
          },
          scales: {
            y: {
              reverse: true,
              min: 1,
              max: this.selectedAnchors.length,
              ticks: {
                stepSize: 1,
                font: { size: 18 },
                callback: (value) => `第${value}名`
              },
              title: {
                display: true,
                text: '排名',
                font: { size: 24, weight: 'bold' }
              }
            },
            x: {
              ticks: {
                font: { size: 14 },
                maxRotation: 45
              },
              title: {
                display: true,
                text: '日期',
                font: { size: 24, weight: 'bold' }
              }
            }
          },
          interaction: {
            mode: 'nearest',
            axis: 'x'
          }
        }
      })
    },

    toggleAnchorVisibility(roomId) {
      const index = this.visibleAnchors.indexOf(roomId)
      if (index > -1) {
        if (this.visibleAnchors.length > 1) {
          this.visibleAnchors.splice(index, 1)
        }
      } else {
        this.visibleAnchors.push(roomId)
      }
      this.renderChart()
    },

    getCacheKey() {
      return this.selectedAnchors.map(a => a.room_id).sort().join('_') + '_' + this.startDate + '_' + this.endDate + '_rank'
    },
    isCacheValid(cacheKey) {
      const cached = this.dataCache.get(cacheKey)
      if (!cached) return false
      return Date.now() - cached.timestamp < cached.expiry
    },

    async refreshData() {
      this.isRefreshing = true
      try {
        const cacheKey = this.getCacheKey()
        this.dataCache.delete(cacheKey)
        await this.fetchBattleData()
      } finally {
        this.isRefreshing = false
      }
    },

    exportChart() {
      if (!this.battleChart) return
      const link = document.createElement('a')
      link.download = `排名对比_${this.currentMetricName}_${this.startDate}-${this.endDate}.png`
      link.href = this.battleChart.toBase64Image()
      link.click()
    },

    goBack() { this.$emit('close') },
    resetSelection() {
      this.chartVisible = false
      this.showDateModal = true
      if (this.battleChart) {
        this.battleChart.destroy()
        this.battleChart = null
      }
    }
  }
}
</script>

<style scoped>
.rank-comparison { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); z-index: 1000; overflow-y: auto }
.modal-overlay { display: flex; align-items: center; justify-content: center; min-height: 100vh }
.modal-content { background: #FFF8E1; border: 2px solid #FFC633; border-radius: 20px; padding: 30px; min-width: 400px; max-width: 500px; box-shadow: 0 10px 40px rgba(0,0,0,0.3) }
.modal-content h3 { color: #FFC633; text-align: center; margin-bottom: 20px }
.form-group { margin-bottom: 15px }
.form-group label { display: block; margin-bottom: 5px; color: #333; font-weight: bold }
.month-input { width: 100%; padding: 10px; border: 2px solid #FFC633; border-radius: 10px; font-size: 1rem }
.metric-select { width: 100%; padding: 12px; border: 2px solid #FFC633; border-radius: 10px; font-size: 1rem; margin-bottom: 15px; background: white }
.button-group { display: flex; gap: 10px; justify-content: center }
.confirm-btn, .cancel-btn { padding: 10px 20px; border: none; border-radius: 15px; cursor: pointer; font-weight: bold; transition: all 0.3s ease }
.confirm-btn { background: linear-gradient(45deg, #f9729a, #f75982); color: white }
.cancel-btn { background: linear-gradient(45deg, #6c757d, #5a6268); color: white }
.confirm-btn:hover, .cancel-btn:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.3) }
.chart-page { background: #FFF8E1; min-height: 100vh; padding: 30px }
.chart-header { text-align: center; margin-bottom: 30px }
.chart-header h2 { color: #FFC633; font-size: 2rem; margin-bottom: 15px }
.battle-info { color: #f9729a; font-size: 1rem }
.battle-info span { margin: 0 20px }
.action-buttons { display: flex; gap: 20px; justify-content: center; margin: 30px 0; flex-wrap: wrap }
.btn { padding: 14px 28px; border: none; border-radius: 25px; cursor: pointer; font-size: 1rem; font-weight: bold; transition: all 0.3s ease }
.btn-back { background: linear-gradient(45deg, #6c757d, #5a6268); color: white }
.btn-reset { background: linear-gradient(45deg, #FFC633, #FFA500); color: #333 }
.btn-refresh { background: linear-gradient(45deg, #17a2b8, #138496); color: white }
.btn-refresh:disabled { background: #ccc; cursor: not-allowed; transform: none; box-shadow: none }
.btn-export { background: linear-gradient(45deg, #28a745, #218838); color: white }
.btn:hover:not(:disabled) { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.3) }
.refreshing-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 2000; color: white; font-size: 1.2rem }
.spinner { width: 60px; height: 60px; border: 6px solid rgba(255,255,255,0.3); border-top-color: #FFC633; border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 20px }
@keyframes spin { to { transform: rotate(360deg) } }
.refreshing-overlay p { margin: 10px 0; color: #FFC633; font-weight: bold }
.chart-scroll-wrapper { width: 100%; overflow-x: auto; overflow-y: hidden; background: white; border-radius: 20px; padding: 30px; margin: 30px auto; border: 2px solid #FFC633; box-shadow: 0 8px 24px rgba(255,198,51,0.3); position: relative }
.chart-scroll-wrapper::-webkit-scrollbar { height: 14px }
.chart-scroll-wrapper::-webkit-scrollbar-track { background: #f1f1f1; border-radius: 10px }
.chart-scroll-wrapper::-webkit-scrollbar-thumb { background: linear-gradient(45deg, #FFC633, #FFA500); border-radius: 10px }
.chart-scroll-wrapper::-webkit-scrollbar-thumb:hover { background: linear-gradient(45deg, #FFA500, #FF8C00) }
.chart-container { height: 1400px !important; width: 6000px !important; position: relative; overflow: hidden }
.chart-container canvas { width: 6000px !important; height: 1400px !important; max-width: none !important; max-height: none !important }
.chart-legend { text-align: center; padding: 20px; background: #FEEFEF; border-radius: 20px; margin: 20px auto; max-width: 1400px; font-size: 1rem }
.chart-legend span { margin: 8px 15px; display: inline-flex; align-items: center; cursor: pointer; font-size: 0.95rem }
.chart-legend input[type="checkbox"] { margin-right: 5px }
.legend-dot { width: 16px; height: 16px; border-radius: 50%; margin: 0 8px; display: inline-block }
.legend-avatar { width: 24px; height: 24px; border-radius: 50%; object-fit: cover; vertical-align: middle; margin-right: 4px; border: 1px solid #FFC633 }
</style>
