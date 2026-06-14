<template>
  <div class="rank-comparison">
    <HeaderSection />

    <GlassDialog
      :visible="showDateModal"
      title="选择日期范围"
      width="460px"
      @close="closeDateModal"
      @update:visible="(v) => !v && closeDateModal()"
    >
      <div class="modal-form">
        <div class="form-group">
          <label class="form-label">起始月份:</label>
          <GlassInput type="month" v-model="startDate" placeholder="请选择起始月份" />
        </div>
        <div class="form-group">
          <label class="form-label">结束月份:</label>
          <GlassInput type="month" v-model="endDate" placeholder="请选择结束月份" />
        </div>
      </div>
      <template #footer>
        <GlassButton variant="secondary" @click="confirmDateRange">
          <Check :size="16" /> 确定
        </GlassButton>
        <GlassButton variant="default" @click="closeDateModal">
          <X :size="16" /> 取消
        </GlassButton>
      </template>
    </GlassDialog>

    <GlassDialog
      :visible="showMetricModal"
      title="选择对比指标"
      width="460px"
      @close="closeMetricModal"
      @update:visible="(v) => !v && closeMetricModal()"
    >
      <select v-model="selectedMetric" class="metric-select">
        <option v-for="(label, key) in metricNameMap" :key="key" :value="key">{{ label }}</option>
      </select>
      <template #footer>
        <GlassButton variant="secondary" @click="confirmMetric">
          <Check :size="16" /> 确定
        </GlassButton>
        <GlassButton variant="default" @click="closeMetricModal">
          <X :size="16" /> 取消
        </GlassButton>
      </template>
    </GlassDialog>

    <GlassCard v-if="chartVisible" class="chart-page" padding="30px">
      <div class="chart-header">
        <h2 class="chart-title">{{ currentMetricName }}排名变化</h2>
        <div class="battle-info">
          <span><Calendar :size="14" /> {{ startDate }} 至 {{ endDate }}</span>
          <span><Users :size="14" /> {{ selectedAnchors.length }} 人</span>
        </div>
      </div>

      <GlassCard variant="subtle" class="chart-legend" padding="20px">
        <span v-for="anchor in selectedAnchors" :key="anchor.room_id">
          <input type="checkbox" :checked="visibleAnchors.includes(anchor.room_id)"
            @change="toggleAnchorVisibility(anchor.room_id)">
          <img :src="avatarDataUrls[anchor.room_id] || ''" 
            class="legend-avatar">
          <span class="legend-dot" :style="{ backgroundColor: getAnchorColor(anchor.room_id) }"></span>
          {{ anchor.anchor_name }}
        </span>
      </GlassCard>

      <GlassCard variant="strong" class="chart-scroll-wrapper" padding="30px">
        <div class="chart-container">
          <div ref="chartCanvas" style="width:6000px;height:1400px;"></div>
        </div>
      </GlassCard>

      <div class="action-buttons">
        <GlassButton variant="default" @click="goBack">
          <ArrowLeft :size="16" /> 返回主页
        </GlassButton>
        <GlassButton variant="primary" @click="resetSelection">
          <RotateCcw :size="16" /> 重新选择
        </GlassButton>
        <GlassButton variant="info" :disabled="isRefreshing" @click="refreshData">
          <RefreshCw :size="16" /> 刷新/补全数据
        </GlassButton>
        <GlassButton variant="success" @click="exportChart">
          <Download :size="16" /> 导出图表
        </GlassButton>
      </div>

      <div v-if="isRefreshing" class="refreshing-overlay">
        <div class="spinner"></div>
        <p>正在刷新数据...</p>
        <p v-if="currentRefreshingAnchor">正在获取：{{ currentRefreshingAnchor }}</p>
      </div>
    </GlassCard>

    <FooterSection />
  </div>
</template>

<script>
import * as echarts from 'echarts'
import '@/utils/echartsTheme.js'
import { anchorAPI } from '@/api'
import { getMonthRange } from '@/utils/monthUtils'
import { getAvatar, scaleAvatar } from '@/utils/avatarCache'
import HeaderSection from '@/components/HeaderSection.vue'
import FooterSection from '@/components/FooterSection.vue'
import GlassDialog from '@/components/ui/GlassDialog.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import GlassInput from '@/components/ui/GlassInput.vue'
import GlassCard from '@/components/ui/GlassCard.vue'
import { Check, X, Calendar, Users, ArrowLeft, RotateCcw, RefreshCw, Download } from 'lucide-vue-next'

const colorPalette = [
  'var(--color-accent)', '#00BCD4', 'var(--color-primary)', '#00BCD4', '#9C27B0',
  '#FF9F40', '#FF6B6B', '#00BCD4', '#45B7D1', '#96CEB4',
  '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE'
]

export default {
  name: 'RankComparison',
  components: { HeaderSection, FooterSection, GlassDialog, GlassButton, GlassInput, GlassCard, Check, X, Calendar, Users, ArrowLeft, RotateCcw, RefreshCw, Download },
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
      this.battleChart.dispose()
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
      const dom = this.$refs.chartCanvas
      if (!dom) return

      if (this.battleChart) {
        this.battleChart.dispose()
      }

      const avatarDataUrls = {}
      const avatarSymbols = {}
      const symbolSize = 36
      for (const anchor of this.selectedAnchors) {
        const img = await getAvatar(anchor.room_id)
        if (img) {
          const c = document.createElement('canvas')
          c.width = img.naturalWidth; c.height = img.naturalHeight
          c.getContext('2d').drawImage(img, 0, 0)
          avatarDataUrls[anchor.room_id] = c.toDataURL('image/jpeg', 0.8)
          const circular = scaleAvatar(img, symbolSize)
          if (circular) {
            avatarSymbols[anchor.room_id] = circular.toDataURL('image/png')
          }
        }
      }
      this.avatarDataUrls = avatarDataUrls

      const visible = this.selectedAnchors.filter(a => this.visibleAnchors.includes(a.room_id))
      const series = visible.map((anchor) => {
        const roomId = anchor.room_id
        const data = allDates.map(date => rankings[date][roomId] || null)
        const color = this.getAnchorColor(roomId)
        const hasAvatar = !!avatarSymbols[roomId]
        return {
          name: anchor.anchor_name,
          type: 'line',
          data,
          smooth: 0.3,
          symbol: hasAvatar ? `image://${avatarSymbols[roomId]}` : 'circle',
          symbolSize: hasAvatar ? symbolSize : 10,
          lineStyle: { width: 3, color },
          itemStyle: {
            color: hasAvatar ? 'transparent' : color,
            borderColor: hasAvatar ? 'transparent' : '#fff',
            borderWidth: hasAvatar ? 0 : 2
          },
          emphasis: {
            itemStyle: { symbolSize: hasAvatar ? symbolSize * 1.3 : 14 }
          }
        }
      })

      this.battleChart = echarts.init(dom, 'liveshow')
      this.battleChart.setOption({
        title: {
          text: `${this.currentMetricName}排名变化 - ${this.startDate} 至 ${this.endDate}`,
          left: 'center',
          top: 20,
          textStyle: { fontSize: 36, fontWeight: 'bold', color: 'var(--color-primary)' }
        },
        tooltip: {
          trigger: 'axis',
          axisPointer: { type: 'cross' },
          formatter: (params) => {
            let lines = [params[0].axisValueLabel]
            params.forEach(p => {
              if (p.value != null) lines.push(`${p.seriesName}: 第${p.value}名`)
            })
            return lines.join('<br>')
          }
        },
        legend: { show: false },
        grid: { left: 100, right: 100, top: 120, bottom: 80 },
        xAxis: {
          type: 'category',
          data: allDates,
          axisLabel: { fontSize: 14, rotate: 45 },
          name: '日期',
          nameTextStyle: { fontSize: 24, fontWeight: 'bold' }
        },
        yAxis: {
          type: 'value',
          inverse: true,
          min: 1,
          max: this.selectedAnchors.length,
          interval: 1,
          axisLabel: {
            fontSize: 18,
            formatter: (v) => `第${v}名`
          },
          name: '排名',
          nameTextStyle: { fontSize: 24, fontWeight: 'bold' }
        },
        series,
        animation: true,
        animationDuration: 300,
        animationEasing: 'cubicOut'
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
      link.href = this.battleChart.getDataURL({ type: 'png', pixelRatio: 2, backgroundColor: '#fff' })
      link.click()
    },

    goBack() { this.$emit('close') },
    resetSelection() {
      this.chartVisible = false
      this.showDateModal = true
      if (this.battleChart) {
        this.battleChart.dispose()
        this.battleChart = null
      }
    }
  }
}
</script>

<style scoped>
.rank-comparison {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  z-index: 1000;
  overflow-y: auto;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.form-label {
  color: var(--color-text-main);
  font-weight: bold;
  font-size: 0.95rem;
}

.metric-select {
  width: 100%;
  padding: 12px 20px;
  border: 2px solid rgba(142, 123, 80, 0.25);
  border-radius: 24px;
  font-size: 1rem;
  background: rgba(255, 248, 225, 0.6);
  backdrop-filter: blur(var(--glass-blur));
  color: var(--color-text-main);
  outline: none;
  transition: border-color 0.25s ease, box-shadow 0.25s ease;
  box-shadow: var(--shadow-default);
}

.metric-select:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(246, 177, 0, 0.2), var(--shadow-hover);
}

.chart-page {
  min-height: 100vh;
}

.chart-header {
  text-align: center;
  margin-bottom: 30px;
}

.chart-title {
  color: var(--color-primary);
  font-size: 2rem;
  margin-bottom: 15px;
}

.battle-info {
  color: var(--color-accent);
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 20px;
}

.battle-info span {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.action-buttons {
  display: flex;
  gap: 20px;
  justify-content: center;
  margin: 30px 0;
  flex-wrap: wrap;
}

.refreshing-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  color: white;
  font-size: 1.2rem;
}

.spinner {
  width: 60px;
  height: 60px;
  border: 6px solid rgba(255, 255, 255, 0.3);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 20px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.refreshing-overlay p {
  margin: 10px 0;
  color: var(--color-primary);
  font-weight: bold;
}

.chart-scroll-wrapper {
  width: 100%;
  overflow-x: auto;
  overflow-y: hidden;
  margin: 30px auto;
  position: relative;
}

.chart-scroll-wrapper::-webkit-scrollbar {
  height: 14px;
}

.chart-scroll-wrapper::-webkit-scrollbar-track {
  background: rgba(142, 123, 80, 0.1);
  border-radius: 10px;
}

.chart-scroll-wrapper::-webkit-scrollbar-thumb {
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  border-radius: 10px;
}

.chart-scroll-wrapper::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
}

.chart-container {
  height: 1400px !important;
  width: 6000px !important;
  position: relative;
  overflow: hidden;
}

.chart-legend {
  max-width: 1400px;
  font-size: 1rem;
  margin: 20px auto;
}

.chart-legend span {
  margin: 8px 15px;
  display: inline-flex;
  align-items: center;
  cursor: pointer;
  font-size: 0.95rem;
}

.chart-legend input[type="checkbox"] {
  margin-right: 5px;
}

.legend-dot {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  margin: 0 8px;
  display: inline-block;
}

.legend-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  object-fit: cover;
  vertical-align: middle;
  margin-right: 4px;
  border: 1px solid var(--color-primary);
}
</style>
