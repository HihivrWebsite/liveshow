<template>
  <div class="anchor-battle">
    <!-- 日期选择模态框 -->
    <GlassDialog :visible="showDateModal" title="选择日期范围" width="500px" @close="closeDateModal">
        <div class="form-group">
          <label>起始月份:</label>
          <input type="month" v-model="startDate" class="month-input" min="2025-08">
        </div>
        <div class="form-group">
          <label>结束月份:</label>
          <input type="month" v-model="endDate" class="month-input" min="2025-08">
        </div>
        <p class="hint">可选择任意月份范围</p>
        <template #footer>
          <GlassButton @click="confirmDateRange" variant="secondary">确定</GlassButton>
          <GlassButton @click="closeDateModal" variant="default">取消</GlassButton>
        </template>
    </GlassDialog>

    <!-- 指标选择模态框 -->
    <GlassDialog :visible="showMetricModal" title="选择对比项目" width="500px" @close="closeMetricModal">
        <select v-model="selectedMetric" class="metric-select">
          <option value="duration">直播时长</option>
          <option value="gift">礼物收入</option>
          <option value="guard">舰长收入</option>
          <option value="superChat">SC 收入</option>
          <option value="totalRevenue">总营收</option>
          <option value="newGuard3">新增总督</option>
          <option value="newGuard2">新增提督</option>
          <option value="newGuard1">新增舰长</option>
          <option value="newFans">新增粉丝团</option>
          <option value="danmaku">弹幕数</option>
          <option value="avgConcurrency">平均同接</option>
          <option value="maxConcurrency">最高同接</option>
          <option value="newFansCount">新增粉丝数</option>
        </select>
        <template #footer>
          <GlassButton @click="confirmMetric" variant="secondary">确定</GlassButton>
          <GlassButton @click="closeMetricModal" variant="default">取消</GlassButton>
        </template>
    </GlassDialog>

    <!-- 图表页面 -->
    <div v-if="chartVisible" class="chart-page">
      <div class="chart-header">
        <h2><Target :size="24" class="heading-icon" /> 恶意斗虫 - 对比分析</h2>
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
        <label>对比项目：</label>
        <select v-model="currentMetric" @change="onMetricChange" class="metric-select-inline">
          <option value="duration">直播时长</option>
          <option value="gift">礼物收入</option>
          <option value="guard">舰长收入</option>
          <option value="superChat">SC 收入</option>
          <option value="totalRevenue">总营收</option>
          <option value="newGuard3">新增总督</option>
          <option value="newGuard2">新增提督</option>
          <option value="newGuard1">新增舰长</option>
          <option value="newFans">新增粉丝团</option>
          <option value="danmaku">弹幕数</option>
          <option value="avgConcurrency">平均同接</option>
          <option value="maxConcurrency">最高同接</option>
          <option value="newFansCount">新增粉丝数</option>
        </select>
        <span class="metric-hint">下拉快速切换对比指标</span>
      </div>

      <!-- 人物选择（图例）移到上面 -->
      <div class="chart-legend">
        <span v-for="anchor in selectedAnchors" :key="anchor.room_id">
          <input 
            type="checkbox" 
            :checked="visibleAnchors.includes(anchor.room_id)" 
            @change="toggleAnchorVisibility(anchor.room_id)">
          <img :src="avatarDataUrls[anchor.room_id] || ''" 
            class="legend-avatar">
          <span class="legend-dot" :style="{ backgroundColor: getAnchorColor(anchor.room_id) }"></span>
          {{ anchor.anchor_name }}
        </span>
      </div>

      <!-- 操作按钮 -->
      <div class="action-buttons">
        <GlassButton @click="goBack" variant="default">返回主页</GlassButton>
        <GlassButton @click="resetSelection" variant="primary">重新选择</GlassButton>
        <GlassButton @click="refreshData" variant="info" :disabled="isRefreshing">刷新/补全数据</GlassButton>
        <GlassButton @click="exportChart" variant="success">导出图表</GlassButton>
        <GlassButton @click="toggleDebug" variant="debug">{{ showDebug ? '隐藏 Debug' : '显示 Debug' }}</GlassButton>
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
        <div class="chart-container" ref="battleChart"></div>
      </div>

      <!-- Debug 面板 -->
      <div v-if="showDebug" class="debug-panel">
        <h3><Search :size="20" class="heading-icon" /> Debug 数据</h3>

        <div class="debug-section">
          <h4><BarChart3 :size="16" class="section-icon" /> 数据概览</h4>
          <div class="debug-info">
            <p>总会话数：{{ sessions.length }}</p>
            <p>主播数：{{ selectedAnchors.length }}</p>
            <p>可见主播数：{{ visibleAnchors.length }}</p>
            <p>当前指标：{{ currentMetric }} ({{ metricNameMap[currentMetric] }})</p>
          </div>
        </div>

        <div class="debug-section">
          <h4><TrendingUp :size="16" class="section-icon" /> 图表数据</h4>
          <div class="debug-info">
            <p>标签数：{{ debugChartData.labels ? debugChartData.labels.length : 0 }}</p>
            <p>数据集数：{{ debugChartData.series ? debugChartData.series.length : 0 }}</p>
          </div>
        </div>

        <div class="debug-section">
          <h4><ClipboardList :size="16" class="section-icon" /> 原始会话数据 (前 10 条)</h4>
          <pre class="debug-json">{{ debugRawData }}</pre>
        </div>

        <div class="debug-section">
          <h4><BarChart3 :size="16" class="section-icon" /> 图表数据集详情</h4>
          <pre class="debug-json">{{ debugDatasetsInfo }}</pre>
        </div>
      </div>
    </div>
    
    <!-- 页脚 -->
    <FooterSection />
  </div>
</template>

<script>
import * as echarts from 'echarts'
import '@/utils/echartsTheme.js'
import { anchorAPI } from '@/api'
import { getMonthRange } from '@/utils/monthUtils'
import { getAvatar, scaleAvatar } from '@/utils/avatarCache'
import FooterSection from '@/components/FooterSection.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import GlassDialog from '@/components/ui/GlassDialog.vue'
import { Target, Search, BarChart3, TrendingUp, ClipboardList } from 'lucide-vue-next'

const colorPalette = [
  'var(--color-accent)', '#00BCD4', 'var(--color-primary)', '#00BCD4', '#9C27B0',
  '#FF9F40', '#FF6B6B', '#00BCD4', '#45B7D1', '#96CEB4',
  '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE'
]

export default {
  name: 'AnchorBattle',
  components: {
    FooterSection,
    GlassButton,
    GlassDialog,
    Target,
    Search,
    BarChart3,
    TrendingUp,
    ClipboardList
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
      sessionsData: [],
      visibleAnchors: [],
      dataCache: new Map(),
      loading: false,
      error: null,
      battleChart: null,
      showDebug: false,
      debugChartData: { labels: [], series: [] },
      debugRawData: '[]',
      debugDatasetsInfo: '[]',
      isRefreshing: false,
      currentRefreshingAnchor: null,
      anchorColors: {},
      colorIndex: 0,
      avatarDataUrls: {}
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
      return this.sessionsData.some((sessions, index) => {
        return !sessions || sessions.length < 10
      })
    }
  },
  watch: {
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
    window.addEventListener('resize', this.handleResize)
  },
  beforeDestroy() {
    window.removeEventListener('resize', this.handleResize)
    if (this.battleChart) {
      this.battleChart.dispose()
      this.battleChart = null
    }
  },
  methods: {
    handleResize() {
      if (this.battleChart) {
        this.battleChart.resize()
      }
    },

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
    },
    
    async fetchBattleData() {
      this.loading = true
      this.error = null

      try {
        const cacheKey = this.getCacheKey()

        if (this.isCacheValid(cacheKey)) {
          console.log('✅ 使用缓存数据')
          const cached = this.dataCache.get(cacheKey)
          this.sessions = cached.sessions
          this.sessionsData = cached.sessionsData || this.selectedAnchors.map(() => [])
          this.onDataReady()
          return
        }

        console.log('🔄 开始获取数据...')

        this.sessionsData = new Array(this.selectedAnchors.length).fill(null).map(() => [])

        for (let i = 0; i < this.selectedAnchors.length; i++) {
          const anchor = this.selectedAnchors[i]
          console.log(`[Radio] 正在获取 ${anchor.anchor_name} 的数据... (${i + 1}/${this.selectedAnchors.length})`)
          await this.fetchSingleAnchorData(anchor, i)
          if (i < this.selectedAnchors.length - 1) {
            await new Promise(resolve => setTimeout(resolve, 600))
          }
        }

        const allSessions = this.sessionsData.flat()

        allSessions.sort((a, b) =>
          new Date(a.start_time) - new Date(b.start_time)
        )

        this.sessions = allSessions

        console.log(`✅ 获取 ${allSessions.length} 场直播数据`)

        this.dataCache.set(cacheKey, {
          sessions: allSessions,
          sessionsData: this.sessionsData,
          timestamp: Date.now(),
          expiry: 2 * 60 * 60 * 1000
        })

        this.onDataReady()

      } catch (err) {
        console.error('❌ 获取数据失败:', err)
        this.error = err.message || '获取数据失败，请稍后重试'
      } finally {
        this.loading = false
      }
    },
    
    async fetchAnchorData(anchor) {
      const months = getMonthRange(this.startDate, this.endDate)
      const sessions = []

      console.log(`📡 获取 ${anchor.anchor_name} 的数据...`)

      for (const month of months) {
        try {
          const response = await anchorAPI.getLiveSessions(
            anchor.room_id,
            anchor.union || 'VirtuaReal',
            month
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

    async fetchSingleAnchorData(anchor, index) {
      try {
        const months = getMonthRange(this.startDate, this.endDate)
        const sessions = []
        const failedMonths = []

        for (const month of months) {
          let success = false
          let lastError = null

          for (let retry = 0; retry < 3 && !success; retry++) {
            try {
              if (retry > 0) {
                console.log(`  ${month} 第 ${retry} 次重试...`)
                await new Promise(resolve => setTimeout(resolve, 2000))
              }

              const response = await anchorAPI.getLiveSessions(
                anchor.room_id,
                anchor.union || 'VirtuaReal',
                month
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

          if (!success) {
            failedMonths.push(month)
            console.error(`  ${month} 最终获取失败，已跳过`)
          }
        }

        this.sessionsData[index] = sessions
        console.log(`主播 ${anchor.anchor_name} 数据获取成功，共 ${sessions.length} 条`)

        if (failedMonths.length > 0) {
          console.warn(`⚠️ 主播 ${anchor.anchor_name} 以下月份数据获取失败：${failedMonths.join(', ')}`)
        }
      } catch (err) {
        console.error(`获取 ${anchor.anchor_name} 数据失败:`, err)
      }
    },

    onDataReady() {
      this.visibleAnchors = this.selectedAnchors.map(a => a.room_id)
      
      this.$nextTick(() => {
        setTimeout(() => {
          this.renderBattleChart().catch(() => {})
        }, 100)
      })
    },
    
    async renderBattleChart() {
      const container = this.$refs.battleChart
      if (!container) {
        console.error('❌ 图表容器不存在')
        return
      }
      
      if (this.battleChart) {
        this.battleChart.dispose()
        this.battleChart = null
      }
      
      const metric = this.currentMetric

      const avatarDataUrls = {}
      for (const anchor of this.selectedAnchors) {
        const img = await getAvatar(anchor.room_id)
        if (img) {
          const circularCanvas = scaleAvatar(img, 36)
          if (circularCanvas) {
            avatarDataUrls[anchor.room_id] = circularCanvas.toDataURL('image/png')
          }
        }
      }
      this.avatarDataUrls = avatarDataUrls

      const chartData = this.transformChartData(metric, avatarDataUrls)
      
      this.debugChartData = chartData
      this.debugRawData = JSON.stringify(this.sessions.slice(0, 10), null, 2)
      this.debugDatasetsInfo = JSON.stringify(chartData.series.map(s => ({
        name: s.name,
        dataPoints: s.data.filter(v => v !== null && v !== undefined).length,
        totalPoints: s.data.length
      })), null, 2)

      const chart = echarts.init(container, 'liveshow')

      const option = {
        animation: true,
        animationDuration: 300,
        title: {
          text: `恶意斗虫 - ${this.metricNameMap[metric]}对比 - 维阿斗虫榜 dc 点 hihivr 点 top`,
          left: 'center',
          top: 10,
          textStyle: {
            fontSize: 24,
            fontWeight: 'bold'
          }
        },
        legend: {
          show: false
        },
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            type: 'cross'
          },
          backgroundColor: 'rgba(0, 0, 0, 0.8)',
          textStyle: {
            fontSize: 16,
            color: '#fff'
          },
          padding: 15
        },
        grid: {
          left: '3%',
          right: '4%',
          bottom: '15%',
          top: 80,
          containLabel: true
        },
        xAxis: {
          type: 'category',
          data: chartData.labels,
          name: '直播场次 (日期 + 主播)',
          nameLocation: 'middle',
          nameGap: 120,
          nameTextStyle: {
            fontSize: 20,
            fontWeight: 'bold'
          },
          axisLabel: {
            rotate: 45,
            fontSize: 14
          },
          axisTick: {
            alignWithLabel: true
          }
        },
        yAxis: {
          type: 'value',
          name: this.metricNameMap[metric],
          nameTextStyle: {
            fontSize: 20,
            fontWeight: 'bold'
          },
          min: 0
        },
        series: chartData.series
      }

      chart.setOption(option)
      this.battleChart = chart
      
      console.log('✅ 图表渲染完成')
    },
    
    transformChartData(metric, avatarMap) {
      const sortedSessions = [...this.sessions].sort((a, b) => 
        new Date(a.start_time) - new Date(b.start_time)
      )
      
      const labels = sortedSessions.map(session => {
        const date = session.start_time.split(' ')[0]
        return `${date} ${session.anchor_name}`
      })
      
      const pointStyles = ['circle', 'rect', 'triangle', 'diamond', 'roundRect', 'pin', 'arrow', 'none']
      const avatarDataUrls = avatarMap || this.avatarDataUrls || {}
      
      const series = this.selectedAnchors
        .filter(anchor => this.visibleAnchors.includes(anchor.room_id))
        .map((anchor, index) => {
          const hasAvatar = !!avatarDataUrls[anchor.room_id]
          return {
            name: anchor.anchor_name,
            type: 'line',
            data: sortedSessions.map(session => 
              session.room_id === anchor.room_id 
                ? this.getMetricValue(session, metric) 
                : null
            ),
            connectNulls: true,
            smooth: 0.3,
            symbolSize: hasAvatar ? 36 : 10,
            symbol: hasAvatar ? 'image://' + avatarDataUrls[anchor.room_id] : pointStyles[index % pointStyles.length],
            lineStyle: {
              color: this.getAnchorColor(anchor.room_id),
              width: 3
            },
            itemStyle: {
              color: this.getAnchorColor(anchor.room_id),
              borderColor: '#fff',
              borderWidth: 2
            },
            emphasis: {
              itemStyle: {
                symbolSize: hasAvatar ? 44 : 16
              }
            }
          }
        })
      
      return { labels, series }
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
    
    onMetricChange() {
      console.log(`🔄 切换指标到：${this.currentMetric}`)
      this.$nextTick(() => {
        this.renderBattleChart().catch(() => {})
      })
    },
    
    toggleAnchorVisibility(roomId) {
      const index = this.visibleAnchors.indexOf(roomId)
      if (index > -1) {
        this.visibleAnchors.splice(index, 1)
      } else {
        this.visibleAnchors.push(roomId)
      }
      this.$nextTick(() => {
        this.renderBattleChart().catch(() => {})
      })
    },
    
    toggleDebug() {
      this.showDebug = !this.showDebug
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

        const metric = this.currentMetric
        const chartData = this.transformChartData(metric)

        chart.setOption({
          title: {
            text: `恶意斗虫 - ${this.metricNameMap[metric]}对比 - 维阿斗虫榜 dc 点 hihivr 点 top`,
            textStyle: {
              fontSize: 72,
              fontWeight: 'bold'
            },
            top: 20
          },
          grid: {
            top: 120
          }
        })

        setTimeout(() => {
          const dataURL = chart.getDataURL({
            type: 'png',
            pixelRatio: 2,
            backgroundColor: '#fff'
          })

          const link = document.createElement('a')
          link.download = `恶意斗虫-${this.metricNameMap[metric]}-${Date.now()}.png`
          link.href = dataURL
          console.log('图片 URL 长度:', dataURL.length)
          console.log('下载文件名:', link.download)
          link.click()

          chart.setOption({
            title: {
              text: `恶意斗虫 - ${this.metricNameMap[metric]}对比 - 维阿斗虫榜 dc 点 hihivr 点 top`,
              textStyle: {
                fontSize: 24,
                fontWeight: 'bold'
              },
              top: 10
            },
            grid: {
              top: 80
            }
          })
          console.log('标题已恢复')
        }, 100)
      } catch (err) {
        console.error('导出失败:', err)
        console.error('错误堆栈:', err.stack)
        alert('导出失败：' + err.message + '，请重试')
      }
    },

    async refreshData() {
      console.log('=== 开始刷新/补全数据 ===')

      const cacheKey = this.getCacheKey()
      this.dataCache.delete(cacheKey)

      this.isRefreshing = true
      let hasPartialFailure = false
      const failedAnchors = []

      try {
        const missingAnchors = []

        this.selectedAnchors.forEach((anchor, index) => {
          if (!this.sessionsData[index] || this.sessionsData[index].length < 10) {
            missingAnchors.push({ anchor, index })
          }
        })

        if (missingAnchors.length === 0) {
          console.log('没有缺失数据，重新获取所有数据')
          await this.fetchBattleData()
        } else {
          console.log(`发现 ${missingAnchors.length} 个主播数据需要补全，开始刷新`)
          for (const { anchor, index } of missingAnchors) {
            this.currentRefreshingAnchor = anchor
            console.log(`正在获取 ${anchor.anchor_name} 的数据...`)
            await this.fetchSingleAnchorData(anchor, index)

            if (!this.sessionsData[index] || this.sessionsData[index].length === 0) {
              failedAnchors.push(anchor.anchor_name)
              hasPartialFailure = true
            }

            await new Promise(resolve => setTimeout(resolve, 1500))
          }
          this.currentRefreshingAnchor = null

          this.sessions = this.sessionsData.flat().sort((a, b) =>
            (a.start_time || '').localeCompare(b.start_time || '')
          )

          this.dataCache.set(cacheKey, {
            sessions: this.sessions,
            sessionsData: this.sessionsData,
            timestamp: Date.now(),
            expiry: 2 * 60 * 60 * 1000
          })
        }

        await this.renderBattleChart()

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
.heading-icon { vertical-align: middle; margin-right: 6px }
.section-icon { vertical-align: middle; margin-right: 4px }
.anchor-battle { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.7); z-index: 1000; overflow-y: auto }
.modal-overlay { display: flex; align-items: center; justify-content: center; min-height: 100vh }
.modal-content { background: var(--color-card); border: 2px solid var(--color-primary); border-radius: var(--radius-card); padding: 30px; min-width: 400px; max-width: 500px; box-shadow: 0 10px 40px rgba(0,0,0,0.3) }
.modal-content h3 { color: var(--color-primary); text-align: center; margin-bottom: 20px }
.form-group { margin-bottom: 15px }
.form-group label { display: block; margin-bottom: 5px; color: var(--color-text-main); font-weight: bold }
.month-input { width: 100%; padding: 10px; border: 2px solid var(--color-primary); border-radius: 10px; font-size: 1rem }
.hint { color: var(--color-accent); font-size: 0.85rem; margin: 10px 0; text-align: center }
.metric-select { width: 100%; padding: 12px; border: 2px solid var(--color-primary); border-radius: 10px; font-size: 1rem; margin-bottom: 15px; background: white }
.button-group { display: flex; gap: 10px; justify-content: center }
.confirm-btn, .cancel-btn { padding: 10px 20px; border: none; border-radius: var(--radius-button); cursor: pointer; font-weight: bold; transition: all 0.3s ease }
.confirm-btn { background: linear-gradient(45deg, var(--color-accent), #f75982); color: white }
.cancel-btn { background: linear-gradient(45deg, var(--color-text-secondary), #7a6940); color: white }
.confirm-btn:hover, .cancel-btn:hover { transform: translateY(-2px); box-shadow: 0 4px 12px rgba(0,0,0,0.3) }
.chart-page { background: var(--color-card); min-height: 100vh; padding: 30px }
.chart-header { text-align: center; margin-bottom: 30px }
.chart-header h2 { color: var(--color-primary); font-size: 2rem; margin-bottom: 15px }
.battle-info { color: var(--color-accent); font-size: 1rem }
.battle-info span { margin: 0 20px }
.loading-hint { margin-top: 15px; padding: 10px 20px; background: rgba(246,177,0,0.2); border: 1px solid var(--color-primary); border-radius: 10px; color: var(--color-accent); font-size: 0.95rem; font-weight: bold; display: inline-block; animation: pulse 2s infinite }
.error-hint { margin-top: 15px; padding: 10px 20px; background: rgba(255,100,100,0.2); border: 1px solid var(--color-accent); border-radius: 10px; color: var(--color-accent); font-size: 0.95rem; font-weight: bold; display: inline-block }
.warning-hint { margin-top: 15px; padding: 10px 20px; background: rgba(255,165,0,0.2); border: 1px solid var(--color-primary); border-radius: 10px; color: var(--color-primary); font-size: 0.95rem; font-weight: bold; display: inline-block }
@keyframes pulse { 0%, 100% { opacity: 1 } 50% { opacity: 0.6 } }
.metric-selector { background: var(--color-card); padding: 20px 30px; border-radius: 15px; margin: 30px auto; max-width: 1000px; display: flex; align-items: center; gap: 20px; border: 2px solid var(--color-primary) }
.metric-select-inline { flex: 1; padding: 14px 18px; font-size: 1.1rem; border: 2px solid var(--color-primary); border-radius: 12px; background: white; cursor: pointer }
.metric-hint { color: var(--color-accent); font-size: 0.9rem; white-space: nowrap }
.action-buttons { display: flex; gap: 20px; justify-content: center; margin: 30px 0; flex-wrap: wrap }

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
  to { transform: rotate(360deg) }
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
  background: white;
  border-radius: 20px;
  padding: 30px;
  margin: 30px auto;
  border: 2px solid var(--color-primary);
  box-shadow: 0 8px 24px rgba(255,198,51,0.3);
  position: relative;
}
.chart-scroll-wrapper::-webkit-scrollbar { height: 14px }
.chart-scroll-wrapper::-webkit-scrollbar-track { background: #f1f1f1; border-radius: 10px }
.chart-scroll-wrapper::-webkit-scrollbar-thumb { background: linear-gradient(45deg, var(--color-primary), var(--color-primary)); border-radius: 10px }
.chart-scroll-wrapper::-webkit-scrollbar-thumb:hover { background: linear-gradient(45deg, var(--color-primary), var(--color-primary)) }
.chart-container { 
  height: 1800px !important;
  width: 8000px !important;
  position: relative;
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
.legend-avatar { width: 24px; height: 24px; border-radius: 50%; object-fit: cover; vertical-align: middle; margin-right: 4px; border: 1px solid var(--color-primary) }
.debug-panel { background: #1e1e1e; color: #d4d4d4; border-radius: 15px; padding: 20px; margin: 30px auto; max-width: 1400px; border: 2px solid #00BCD4 }
.debug-panel h3 { color: #00BCD4; margin-top: 0; margin-bottom: 20px }
.debug-section { margin-bottom: 20px; border-bottom: 1px solid var(--color-text-main); padding-bottom: 15px }
.debug-section:last-child { border-bottom: none }
.debug-section h4 { color: var(--color-primary); margin-bottom: 10px }
.debug-info p { margin: 5px 0; font-family: 'Courier New', monospace; font-size: 0.9rem }
.debug-json { background: #2d2d2d; padding: 15px; border-radius: 8px; overflow-x: auto; font-family: 'Courier New', monospace; font-size: 0.85rem; max-height: 400px; overflow-y: auto; white-space: pre-wrap; word-wrap: break-word }
@media (max-width: 768px) { .metric-selector { flex-direction: column; align-items: stretch } .metric-hint { text-align: center } }
</style>
