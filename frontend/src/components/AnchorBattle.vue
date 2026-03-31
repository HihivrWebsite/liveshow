<template>
  <div class="anchor-battle">
    <!-- 页眉 -->
    <HeaderSection />
    
    <!-- 日期选择模态框 -->
    <div v-if="showDateModal" class="modal-overlay" @click="closeDateModal">
      <div class="modal-content" @click.stop>
        <h3>📅 选择日期范围</h3>
        <div class="form-group">
          <label>起始月份:</label>
          <input type="month" v-model="startDate" class="month-input">
        </div>
        <div class="form-group">
          <label>结束月份:</label>
          <input type="month" v-model="endDate" class="month-input">
        </div>
        <p class="hint">💡 可选择任意月份范围</p>
        <div class="button-group">
          <button @click="confirmDateRange" class="confirm-btn">确定</button>
          <button @click="closeDateModal" class="cancel-btn">取消</button>
        </div>
      </div>
    </div>

    <!-- 指标选择模态框 -->
    <div v-if="showMetricModal" class="modal-overlay" @click="closeMetricModal">
      <div class="modal-content" @click.stop>
        <h3>📊 选择对比项目</h3>
        <select v-model="selectedMetric" class="metric-select">
          <option value="duration">📈 直播时长</option>
          <option value="gift">💰 礼物收入</option>
          <option value="guard">🛡️ 舰长收入</option>
          <option value="superChat">💬 SC 收入</option>
          <option value="totalRevenue">💵 总营收</option>
          <option value="newGuard3">👑 新增总督</option>
          <option value="newGuard2">🎖️ 新增提督</option>
          <option value="newGuard1">⚓ 新增舰长</option>
          <option value="newFans">👥 新增粉丝团</option>
          <option value="danmaku">💭 弹幕数</option>
          <option value="avgConcurrency">📊 平均同接</option>
          <option value="maxConcurrency">📈 最高同接</option>
          <option value="newFansCount">🆕 新增粉丝数</option>
        </select>
        <div class="button-group">
          <button @click="confirmMetric" class="confirm-btn">确定</button>
          <button @click="closeMetricModal" class="cancel-btn">取消</button>
        </div>
      </div>
    </div>

    <!-- 图表页面 -->
    <div v-if="chartVisible" class="chart-page">
      <div class="chart-header">
        <h2>🎯 恶意斗虫 - 对比分析</h2>
        <div class="battle-info">
          <span>主播：{{ anchorNames }}</span>
          <span>日期：{{ startDate }} 至 {{ endDate }}</span>
        </div>
        <div v-if="loading" class="loading-hint">
          ⏳ 正在加载数据，请稍候...（数据量较大可能需要几秒钟）
        </div>
        <div v-if="error" class="error-hint">
          ❌ {{ error }}
        </div>
        <div v-if="hasIncompleteData" class="warning-hint">
          ⚠️ 部分主播数据不完整，可点击"刷新/补全数据"按钮重试
        </div>
      </div>

      <!-- 指标切换 -->
      <div class="metric-selector">
        <label>📊 对比项目：</label>
        <select v-model="currentMetric" @change="onMetricChange" class="metric-select-inline">
          <option value="duration">📈 直播时长</option>
          <option value="gift">💰 礼物收入</option>
          <option value="guard">🛡️ 舰长收入</option>
          <option value="superChat">💬 SC 收入</option>
          <option value="totalRevenue">💵 总营收</option>
          <option value="newGuard3">👑 新增总督</option>
          <option value="newGuard2">🎖️ 新增提督</option>
          <option value="newGuard1">⚓ 新增舰长</option>
          <option value="newFans">👥 新增粉丝团</option>
          <option value="danmaku">💭 弹幕数</option>
          <option value="avgConcurrency">📊 平均同接</option>
          <option value="maxConcurrency">📈 最高同接</option>
          <option value="newFansCount">🆕 新增粉丝数</option>
        </select>
        <span class="metric-hint">💡 下拉快速切换对比指标</span>
      </div>

      <!-- 人物选择（图例）移到上面 -->
      <div class="chart-legend">
        <span v-for="anchor in selectedAnchors" :key="anchor.room_id">
          <input 
            type="checkbox" 
            :checked="visibleAnchors.includes(anchor.room_id)" 
            @change="toggleAnchorVisibility(anchor.room_id)">
          <span class="legend-dot" :style="{ backgroundColor: getAnchorColor(anchor.room_id) }"></span>
          {{ anchor.anchor_name }}
        </span>
      </div>

      <!-- 操作按钮 -->
      <div class="action-buttons">
        <button @click="goBack" class="btn btn-back">返回主页</button>
        <button @click="resetSelection" class="btn btn-reset">重新选择</button>
        <button @click="refreshData" class="btn btn-refresh" :disabled="isRefreshing">🔄 刷新/补全数据</button>
        <button @click="exportChart" class="btn btn-export">导出图表</button>
        <button @click="toggleDebug" class="btn btn-debug">{{ showDebug ? '隐藏 Debug' : '显示 Debug' }}</button>
      </div>

      <!-- 刷新中覆盖层 -->
      <div v-if="isRefreshing" class="refreshing-overlay">
        <div class="spinner"></div>
        <p>正在刷新数据，请稍候...</p>
        <p v-if="currentRefreshingAnchor">
          正在获取 {{ currentRefreshingAnchor.anchor_name }} 的数据...
        </p>
      </div>

      <!-- 图表区域 - 横向滚动 -->
      <div class="chart-scroll-wrapper">
        <div class="chart-container">
          <canvas ref="battleChart"></canvas>
        </div>
      </div>

      <!-- Debug 面板 -->
      <div v-if="showDebug" class="debug-panel">
        <h3>🔍 Debug 数据</h3>

        <div class="debug-section">
          <h4>📊 数据概览</h4>
          <div class="debug-info">
            <p>总会话数：{{ sessions.length }}</p>
            <p>主播数：{{ selectedAnchors.length }}</p>
            <p>可见主播数：{{ visibleAnchors.length }}</p>
            <p>当前指标：{{ currentMetric }} ({{ metricNameMap[currentMetric] }})</p>
          </div>
        </div>

        <div class="debug-section">
          <h4>📈 图表数据</h4>
          <div class="debug-info">
            <p>标签数：{{ debugChartData.labels ? debugChartData.labels.length : 0 }}</p>
            <p>数据集数：{{ debugChartData.datasets ? debugChartData.datasets.length : 0 }}</p>
          </div>
        </div>

        <div class="debug-section">
          <h4>📋 原始会话数据 (前 10 条)</h4>
          <pre class="debug-json">{{ debugRawData }}</pre>
        </div>

        <div class="debug-section">
          <h4>📊 图表数据集详情</h4>
          <pre class="debug-json">{{ debugDatasetsInfo }}</pre>
        </div>
      </div>
    </div>
    
    <!-- 页脚 -->
    <FooterSection />
  </div>
</template>

<script>
import { Chart, registerables } from 'chart.js'
import { anchorAPI } from '@/api'
import HeaderSection from '@/components/HeaderSection.vue'
import FooterSection from '@/components/FooterSection.vue'

Chart.register(...registerables)

// 主播颜色映射
const anchorColors = {}
const colorPalette = [
  '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', 
  '#FF9F40', '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4',
  '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE'
]

function getAnchorColor(roomId) {
  if (!anchorColors[roomId]) {
    anchorColors[roomId] = colorPalette[Object.keys(anchorColors).length % colorPalette.length]
  }
  return anchorColors[roomId]
}

export default {
  name: 'AnchorBattle',
  components: {
    HeaderSection,
    FooterSection
  },
  props: {
    initialAnchors: { type: Array, default: () => [] }
  },
  data() {
    return {
      selectedAnchors: [],
      startDate: '',
      endDate: '',
      selectedMetric: 'totalRevenue',
      currentMetric: 'totalRevenue',
      showDateModal: true,
      showMetricModal: false,
      chartVisible: false,
      sessions: [],
      sessionsData: [],  // 存储每个主播的会话数据
      visibleAnchors: [],
      dataCache: new Map(),
      loading: false,
      error: null,
      battleChart: null,
      showDebug: false,
      debugChartData: { labels: [], datasets: [] },
      debugRawData: '[]',
      debugDatasetsInfo: '[]',
      isRefreshing: false,  // 标记是否正在刷新
      currentRefreshingAnchor: null  // 当前正在刷新数据的主播
    }
  },
  computed: {
    anchorNames() {
      return this.selectedAnchors.map(a => a.anchor_name).join('、')
    },
    metricNameMap() {
      return {
        duration: '直播时长',
        gift: '礼物收入',
        guard: '舰长收入',
        superChat: 'SC 收入',
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
    },
    hasIncompleteData() {
      // 检查是否有主播数据缺失或数据量过少
      return this.sessionsData.some((sessions, index) => {
        return !sessions || sessions.length < 10
      })
    }
  },
  watch: {
    // 监听 chartVisible，当图表页面显示时才获取数据和渲染图表
    chartVisible: {
      handler(newVal) {
        if (newVal) {
          this.$nextTick(() => {
            this.fetchBattleData()
          })
        }
      },
      immediate: false
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
    getAnchorColor,
    
    closeDateModal() { 
      this.showDateModal = false
      this.$emit('close') 
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
    
    closeMetricModal() { 
      this.showMetricModal = false
      this.showDateModal = true 
    },
    
    confirmMetric() { 
      this.showMetricModal = false
      this.chartVisible = true
      // 不立即调用 fetchBattleData，让 watch 来处理
    },
    
    // 数据获取主流程
    async fetchBattleData() {
      this.loading = true
      this.error = null

      try {
        const cacheKey = this.getCacheKey()

        // 1. 检查缓存
        if (this.isCacheValid(cacheKey)) {
          console.log('✅ 使用缓存数据')
          this.sessions = this.dataCache.get(cacheKey).sessions
          this.sessionsData = this.selectedAnchors.map(() => [])
          this.onDataReady()
          return
        }

        console.log('🔄 开始获取数据...')

        // 2. 初始化 sessionsData
        this.sessionsData = new Array(this.selectedAnchors.length).fill(null).map(() => [])

        // 3. 串行获取各主播数据（增加延迟避免 API 限流）
        for (let i = 0; i < this.selectedAnchors.length; i++) {
          const anchor = this.selectedAnchors[i]
          console.log(`📡 正在获取 ${anchor.anchor_name} 的数据... (${i + 1}/${this.selectedAnchors.length})`)
          await this.fetchSingleAnchorData(anchor, i)
          // 增加延迟到 600ms，避免 API 限流
          if (i < this.selectedAnchors.length - 1) {
            await new Promise(resolve => setTimeout(resolve, 600))
          }
        }

        // 4. 合并所有主播的数据
        const allSessions = this.sessionsData.flat()

        // 5. 排序
        allSessions.sort((a, b) =>
          new Date(a.start_time) - new Date(b.start_time)
        )

        // 6. 更新数据
        this.sessions = allSessions

        console.log(`✅ 获取 ${allSessions.length} 场直播数据`)

        // 7. 更新缓存
        this.dataCache.set(cacheKey, {
          sessions: allSessions,
          timestamp: Date.now(),
          expiry: 2 * 60 * 60 * 1000  // 2 小时
        })

        // 8. 数据准备好
        this.onDataReady()

      } catch (err) {
        console.error('❌ 获取数据失败:', err)
        this.error = err.message || '获取数据失败，请稍后重试'
      } finally {
        this.loading = false
      }
    },
    
    // 获取单个主播的数据
    async fetchAnchorData(anchor) {
      const months = this.getMonthsBetween(this.startDate, this.endDate)
      const sessions = []

      console.log(`📡 获取 ${anchor.anchor_name} 的数据...`)

      for (const month of months) {
        try {
          const response = await anchorAPI.getLiveSessions(
            anchor.room_id,
            anchor.union || 'VirtuaReal',
            month.replace('-', '')
          )
          const sessionList = response.sessions || []
          sessionList.forEach(session => {
            sessions.push({
              ...session,
              room_id: anchor.room_id,
              anchor_name: anchor.anchor_name
            })
          })
          console.log(`  ${month}: ${sessionList.length}场`)
        } catch (err) {
          console.warn(`  ${month} 获取失败:`, err.message)
        }
      }

      return sessions
    },

    // 获取单个主播的数据（用于串行获取和刷新）
    async fetchSingleAnchorData(anchor, index) {
      try {
        const months = this.getMonthsBetween(this.startDate, this.endDate)
        const sessions = []
        const failedMonths = []

        for (const month of months) {
          let success = false
          let lastError = null

          // 重试机制：最多重试 3 次
          for (let retry = 0; retry < 3 && !success; retry++) {
            try {
              if (retry > 0) {
                console.log(`  ${month} 第 ${retry} 次重试...`)
                // 重试前等待 2 秒
                await new Promise(resolve => setTimeout(resolve, 2000))
              }

              const response = await anchorAPI.getLiveSessions(
                anchor.room_id,
                anchor.union || 'VirtuaReal',
                month.replace('-', '')
              )
              const sessionList = response.sessions || []
              sessionList.forEach(session => {
                sessions.push({
                  ...session,
                  room_id: anchor.room_id,
                  anchor_name: anchor.anchor_name
                })
              })
              console.log(`  ${month}: ${sessionList.length}场`)
              success = true
            } catch (err) {
              lastError = err
              console.warn(`  ${month} 获取失败 (${retry + 1}/3):`, err.message)
            }
          }

          // 3 次重试后仍然失败，记录失败的月份
          if (!success) {
            failedMonths.push(month)
            console.error(`  ${month} 最终获取失败，已跳过`)
          }
        }

        // 更新数据（Vue 3 直接使用数组索引赋值）
        this.sessionsData[index] = sessions
        console.log(`主播 ${anchor.anchor_name} 数据获取成功，共 ${sessions.length} 条`)

        // 如果有失败的月份，显示警告
        if (failedMonths.length > 0) {
          console.warn(`⚠️ 主播 ${anchor.anchor_name} 以下月份数据获取失败：${failedMonths.join(', ')}`)
        }
      } catch (err) {
        console.error(`获取 ${anchor.anchor_name} 数据失败:`, err)
      }
    },

    // 数据准备好的回调
    onDataReady() {
      this.visibleAnchors = this.selectedAnchors.map(a => a.room_id)
      
      // 等待 DOM 更新后再渲染图表
      this.$nextTick(() => {
        // 再次等待，确保 canvas 元素完全渲染
        setTimeout(() => {
          this.renderBattleChart()
        }, 100)
      })
    },
    
    // 图表渲染
    renderBattleChart() {
      // 1. 检查 canvas 元素
      const canvas = this.$refs.battleChart
      if (!canvas) {
        console.error('❌ Canvas 元素不存在')
        return
      }
      
      // 2. 销毁旧图表
      if (this.battleChart) {
        this.battleChart.destroy()
        this.battleChart = null
      }
      
      // 3. 准备数据
      const metric = this.currentMetric
      const chartData = this.transformChartData(metric)
      
      // 保存 Debug 数据
      this.debugChartData = chartData
      this.debugRawData = JSON.stringify(this.sessions.slice(0, 10), null, 2)
      this.debugDatasetsInfo = JSON.stringify(chartData.datasets.map(ds => ({
        label: ds.label,
        dataPoints: ds.data.filter(v => v !== null).length,
        totalPoints: ds.data.length
      })), null, 2)

      // 4. 设置固定超大 canvas 尺寸
      const chartWidth = 8000
      const chartHeight = 1800

      // 5. 设置 canvas 尺寸（必须在 Chart.js 初始化之前）
      canvas.width = chartWidth
      canvas.height = chartHeight

      console.log('📊 Canvas 尺寸：8000x1800')
      
      // 6. 创建图表
      const ctx = canvas.getContext('2d')
      
      // 不同主播使用不同点形状
      const pointStyles = ['circle', 'rect', 'triangle', 'star', 'diamond', 'cross', 'rectRot', 'line', 'dash']
      
      this.battleChart = new Chart(ctx, {
        type: 'line',
        data: {
          labels: chartData.labels,
          datasets: this.selectedAnchors
            .filter(anchor => this.visibleAnchors.includes(anchor.room_id))
            .map((anchor, index) => ({
              label: anchor.anchor_name,
              data: chartData.datasets.find(ds => ds.label === anchor.anchor_name)?.data || [],
              borderColor: this.getAnchorColor(anchor.room_id),
              backgroundColor: this.getAnchorColor(anchor.room_id) + '20',
              pointStyle: pointStyles[index % pointStyles.length],
              pointRadius: 10,
              pointHoverRadius: 15,
              pointBackgroundColor: this.getAnchorColor(anchor.room_id),
              pointBorderColor: '#fff',
              pointBorderWidth: 3,
              borderWidth: 3,
              fill: false,
              tension: 0.3,
              spanGaps: true
            }))
        },
        options: {
          responsive: false,
          maintainAspectRatio: false,
          animation: {
            duration: 300,
            easing: 'easeOutQuart'
          },
          plugins: {
            title: {
              display: true,
              text: `恶意斗虫 - ${this.metricNameMap[metric]}对比 - 维阿斗虫榜 dc 点 hihivr 点 top`,
              font: { size: 24, weight: 'bold' },
              padding: 30
            },
            legend: {
              display: false
            },
            tooltip: {
              enabled: true,
              mode: 'index',
              intersect: false,
              backgroundColor: 'rgba(0, 0, 0, 0.8)',
              titleFont: { size: 18 },
              bodyFont: { size: 16 },
              padding: 15,
              displayColors: true,
              callbacks: {
                label: (context) => {
                  return `${context.dataset.label}: ${context.parsed.y}`
                }
              }
            }
          },
          scales: {
            x: {
              title: {
                display: true,
                text: '直播场次 (日期 + 主播)',
                font: { size: 20, weight: 'bold' }
              },
              ticks: {
                maxRotation: 45,
                minRotation: 45,
                autoSkip: false,
                font: { size: 14 }
              },
              grid: {
                display: true,
                color: 'rgba(0, 0, 0, 0.05)'
              }
            },
            y: {
              title: {
                display: true,
                text: this.metricNameMap[metric],
                font: { size: 20, weight: 'bold' }
              },
              grid: {
                display: true,
                color: 'rgba(0, 0, 0, 0.05)'
              },
              beginAtZero: true
            }
          },
          interaction: {
            mode: 'nearest',
            axis: 'x',
            intersect: false
          }
        }
      })
      
      console.log('✅ 图表渲染完成')
    },
    
    // 数据转换
    transformChartData(metric) {
      const sortedSessions = [...this.sessions].sort((a, b) => 
        new Date(a.start_time) - new Date(b.start_time)
      )
      
      const labels = sortedSessions.map(session => {
        const date = session.start_time.split(' ')[0]
        return `${date} ${session.anchor_name}`
      })
      
      const datasets = this.selectedAnchors
        .filter(anchor => this.visibleAnchors.includes(anchor.room_id))
        .map((anchor, index) => ({
          label: anchor.anchor_name,
          data: sortedSessions.map(session => 
            session.room_id === anchor.room_id 
              ? this.getMetricValue(session, metric) 
              : null
          ),
          borderColor: this.getAnchorColor(anchor.room_id),
          backgroundColor: this.getAnchorColor(anchor.room_id) + '20',
          fill: false,
          tension: 0.3,
          pointRadius: 5,
          pointHoverRadius: 8,
          pointBackgroundColor: this.getAnchorColor(anchor.room_id),
          pointBorderColor: '#fff',
          pointBorderWidth: 2,
          spanGaps: true
        }))
      
      return { labels, datasets }
    },
    
    getMetricValue(session, metric) {
      const valueMap = {
        duration: () => session.duration_minutes || 0,
        gift: () => parseFloat(session.gift) || 0,
        guard: () => parseFloat(session.guard) || 0,
        superChat: () => parseFloat(session.super_chat) || 0,
        totalRevenue: () => parseFloat(session.total_revenue) || 0,
        newGuard3: () => (session.end_guard_3 || 0) - (session.start_guard_3 || 0),
        newGuard2: () => (session.end_guard_2 || 0) - (session.start_guard_2 || 0),
        newGuard1: () => (session.end_guard_1 || 0) - (session.start_guard_1 || 0),
        newFans: () => (session.end_fans_count || 0) - (session.start_fans_count || 0),
        danmaku: () => session.danmaku_count || 0,
        avgConcurrency: () => session.avg_concurrency || 0,
        maxConcurrency: () => session.max_concurrency || 0,
        newFansCount: () => session.new_fans_count !== undefined ? session.new_fans_count : 0
      }
      
      return (valueMap[metric] || (() => 0))()
    },
    
    // 切换指标
    onMetricChange() {
      console.log(`🔄 切换指标到：${this.currentMetric}`)
      this.$nextTick(() => {
        this.renderBattleChart()
      })
    },
    
    // 切换主播显示
    toggleAnchorVisibility(roomId) {
      const index = this.visibleAnchors.indexOf(roomId)
      if (index > -1) {
        this.visibleAnchors.splice(index, 1)
      } else {
        this.visibleAnchors.push(roomId)
      }
      this.$nextTick(() => {
        this.renderBattleChart()
      })
    },
    
    // 切换 Debug 面板
    toggleDebug() {
      this.showDebug = !this.showDebug
    },
    
    // 辅助方法
    getMonthsBetween(startDate, endDate) {
      const months = []
      const start = new Date(startDate + '-01')
      const end = new Date(endDate + '-01')
      const current = new Date(start)
      while (current <= end) {
        months.push(`${current.getFullYear()}-${String(current.getMonth() + 1).padStart(2, '0')}`)
        current.setMonth(current.getMonth() + 1)
      }
      return months
    },
    
    getCacheKey() {
      return this.selectedAnchors.map(a => a.room_id).sort().join('_') + '_' + this.startDate + '_' + this.endDate
    },
    
    isCacheValid(cacheKey) {
      const cached = this.dataCache.get(cacheKey)
      if (!cached) return false
      const now = Date.now()
      return now - cached.timestamp < cached.expiry
    },
    
    goBack() { 
      this.$emit('close') 
    },
    
    resetSelection() { 
      this.chartVisible = false
      this.showMetricModal = true 
    },
    
    exportChart() {
      console.log('=== 开始导出图表 ===')

      const chart = this.battleChart
      if (!chart) {
        console.error('图表未初始化')
        alert('图表未初始化，请稍后再试')
        return
      }

      try {
        console.log('图表实例存在')

        // 保存原始配置
        const originalTitle = chart.options.plugins.title.text
        const originalFontSize = chart.options.plugins.title.font.size
        const originalPadding = chart.options.plugins.title.padding

        console.log('原始标题:', originalTitle)

        // 修改标题为导出格式（更大字体，带后缀）
        chart.options.plugins.title.text = `恶意斗虫 - ${this.metricNameMap[this.currentMetric]}对比 - 维阿斗虫榜 dc 点 hihivr 点 top`
        chart.options.plugins.title.font.size = 72
        chart.options.plugins.title.padding = 50

        console.log('更新图表...')
        // 更新图表，使用 true 重绘但不触发响应式
        chart.draw()

        console.log('生成图片...')
        // 导出图片
        const link = document.createElement('a')
        link.download = `恶意斗虫-${this.metricNameMap[this.currentMetric]}-${Date.now()}.png`
        link.href = chart.toBase64Image()
        console.log('图片 URL 长度:', link.href.length)
        console.log('下载文件名:', link.download)
        link.click()

        console.log('图片已导出，恢复原始标题...')
        // 恢复原始标题
        chart.options.plugins.title.text = originalTitle
        chart.options.plugins.title.font.size = originalFontSize
        chart.options.plugins.title.padding = originalPadding
        chart.draw()
        console.log('标题已恢复')
      } catch (err) {
        console.error('导出失败:', err)
        console.error('错误堆栈:', err.stack)
        alert('导出失败：' + err.message + '，请重试')
      }
    },

    // 刷新/补全数据
    async refreshData() {
      console.log('=== 开始刷新/补全数据 ===')

      this.isRefreshing = true
      let hasPartialFailure = false
      const failedAnchors = []

      try {
        // 记录缺失数据或数据较少的主播索引
        const missingAnchors = []

        this.selectedAnchors.forEach((anchor, index) => {
          // 数据为空或数据量明显偏少（少于 10 条）都认为需要补全
          if (!this.sessionsData[index] || this.sessionsData[index].length < 10) {
            missingAnchors.push({ anchor, index })
          }
        })

        if (missingAnchors.length === 0) {
          // 没有缺失数据，重新获取所有数据
          console.log('没有缺失数据，重新获取所有数据')
          await this.fetchBattleData()
        } else {
          // 只获取缺失的数据
          console.log(`发现 ${missingAnchors.length} 个主播数据需要补全，开始刷新`)
          for (const { anchor, index } of missingAnchors) {
            this.currentRefreshingAnchor = anchor
            console.log(`正在获取 ${anchor.anchor_name} 的数据...`)
            await this.fetchSingleAnchorData(anchor, index)
            
            // 检查是否获取成功
            if (!this.sessionsData[index] || this.sessionsData[index].length === 0) {
              failedAnchors.push(anchor.anchor_name)
              hasPartialFailure = true
            }
            
            // 延长延迟时间到 1500ms
            await new Promise(resolve => setTimeout(resolve, 1500))
          }
          this.currentRefreshingAnchor = null
        }

        // 更新图表
        this.renderBattleChart()
        
        // 显示结果提示
        if (hasPartialFailure) {
          alert(`数据刷新完成，但以下主播数据仍然缺失：${failedAnchors.join(', ')}\n\n可能是 API 持续超时，请稍后再试或检查网络连接。`)
        } else {
          alert('数据刷新完成！')
        }
      } catch (err) {
        console.error('刷新数据失败:', err)
        alert('刷新数据失败：' + err.message)
      } finally {
        this.isRefreshing = false
      }
    }
  }
}
</script>

<style scoped>
.anchor-battle { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); z-index: 1000; overflow-y: auto }
.modal-overlay { display: flex; align-items: center; justify-content: center; min-height: 100vh }
.modal-content { background: #FFF8E1; border: 2px solid #FFC633; border-radius: 20px; padding: 30px; min-width: 400px; max-width: 500px; box-shadow: 0 10px 40px rgba(0,0,0,0.3) }
.modal-content h3 { color: #FFC633; text-align: center; margin-bottom: 20px }
.form-group { margin-bottom: 15px }
.form-group label { display: block; margin-bottom: 5px; color: #333; font-weight: bold }
.month-input { width: 100%; padding: 10px; border: 2px solid #FFC633; border-radius: 10px; font-size: 1rem }
.hint { color: #f9729a; font-size: 0.85rem; margin: 10px 0; text-align: center }
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
.loading-hint { margin-top: 15px; padding: 10px 20px; background: rgba(255,198,51,0.2); border: 1px solid #FFC633; border-radius: 10px; color: #f9729a; font-size: 0.95rem; font-weight: bold; display: inline-block; animation: pulse 2s infinite }
.error-hint { margin-top: 15px; padding: 10px 20px; background: rgba(255,100,100,0.2); border: 1px solid #FF6384; border-radius: 10px; color: #FF6384; font-size: 0.95rem; font-weight: bold; display: inline-block }
.warning-hint { margin-top: 15px; padding: 10px 20px; background: rgba(255,165,0,0.2); border: 1px solid #FFA500; border-radius: 10px; color: #FF8C00; font-size: 0.95rem; font-weight: bold; display: inline-block }
@keyframes pulse { 0%, 100% { opacity: 1 } 50% { opacity: 0.6 } }
.metric-selector { background: #FFF8E1; padding: 20px 30px; border-radius: 15px; margin: 30px auto; max-width: 1000px; display: flex; align-items: center; gap: 20px; border: 2px solid #FFC633 }
.metric-select-inline { flex: 1; padding: 14px 18px; font-size: 1.1rem; border: 2px solid #FFC633; border-radius: 12px; background: white; cursor: pointer }
.metric-hint { color: #f9729a; font-size: 0.9rem; white-space: nowrap }
.action-buttons { display: flex; gap: 20px; justify-content: center; margin: 30px 0; flex-wrap: wrap }
.btn { padding: 14px 28px; border: none; border-radius: 25px; cursor: pointer; font-size: 1rem; font-weight: bold; transition: all 0.3s ease }
.btn-back { background: linear-gradient(45deg, #6c757d, #5a6268); color: white }
.btn-reset { background: linear-gradient(45deg, #FFC633, #FFA500); color: #333 }
.btn-refresh { background: linear-gradient(45deg, #17a2b8, #138496); color: white }
.btn-refresh:disabled { background: #ccc; cursor: not-allowed; transform: none; box-shadow: none }
.btn-export { background: linear-gradient(45deg, #28a745, #218838); color: white }
.btn-debug { background: linear-gradient(45deg, #9966FF, #8855EE); color: white }
.btn:hover:not(:disabled) { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.3) }

/* 刷新中覆盖层 */
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
  border-top-color: #FFC633;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 20px;
}

@keyframes spin {
  to { transform: rotate(360deg) }
}

.refreshing-overlay p {
  margin: 10px 0;
  color: #FFC633;
  font-weight: bold;
}
.chart-scroll-wrapper { 
  width: 100%;
  overflow-x: auto;
  overflow-y: hidden;
  background: white;
  border-radius: 20px;
  padding: 30px;
  margin: 30px auto;
  border: 2px solid #FFC633;
  box-shadow: 0 8px 24px rgba(255,198,51,0.3);
  position: relative;
}
.chart-scroll-wrapper::-webkit-scrollbar { height: 14px }
.chart-scroll-wrapper::-webkit-scrollbar-track { background: #f1f1f1; border-radius: 10px }
.chart-scroll-wrapper::-webkit-scrollbar-thumb { background: linear-gradient(45deg, #FFC633, #FFA500); border-radius: 10px }
.chart-scroll-wrapper::-webkit-scrollbar-thumb:hover { background: linear-gradient(45deg, #FFA500, #FF8C00) }
.chart-container { 
  height: 1800px !important;
  width: 8000px !important;
  position: relative;
  overflow: hidden;
}
.chart-container canvas {
  width: 8000px !important;
  height: 1800px !important;
  max-width: none !important;
  max-height: none !important;
}
.chart-legend { 
  text-align: center; 
  padding: 20px; 
  background: #FEEFEF; 
  border-radius: 20px; 
  margin: 20px auto; 
  max-width: 1400px; 
  font-size: 1rem;
  order: -1;
}
.chart-legend span { margin: 8px 15px; display: inline-flex; align-items: center; cursor: pointer; font-size: 0.95rem }
.chart-legend input[type="checkbox"] { margin-right: 5px }
.legend-dot { width: 16px; height: 16px; border-radius: 50%; margin: 0 8px; display: inline-block }
.debug-panel { background: #1e1e1e; color: #d4d4d4; border-radius: 15px; padding: 20px; margin: 30px auto; max-width: 1400px; border: 2px solid #4ECDC4 }
.debug-panel h3 { color: #4ECDC4; margin-top: 0; margin-bottom: 20px }
.debug-section { margin-bottom: 20px; border-bottom: 1px solid #333; padding-bottom: 15px }
.debug-section:last-child { border-bottom: none }
.debug-section h4 { color: #FFC633; margin-bottom: 10px }
.debug-info p { margin: 5px 0; font-family: 'Courier New', monospace; font-size: 0.9rem }
.debug-json { background: #2d2d2d; padding: 15px; border-radius: 8px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 0.85rem; max-height: 400px; overflow-y: auto; white-space: pre-wrap; word-wrap: break-word }
@media (max-width: 768px) { .metric-selector { flex-direction: column; align-items: stretch } .metric-hint { text-align: center } }
</style>
