<template>
  <div class="live-sessions">
    <div class="controls-section">
      <div class="action-controls">
        <GlassButton @click="openMonthSelector" variant="secondary">
          切换不同月份
        </GlassButton>
        <GlassButton @click="showSessionChart" variant="primary">
          显示直播数据折线图
        </GlassButton>
        <GlassButton @click="hideChart" variant="danger">
          关闭图表
        </GlassButton>
        <GlassButton @click="goBack" variant="default">
          返回
        </GlassButton>
        <GlassButton @click="goToLiveRoom" variant="success">
          跳转到直播间
        </GlassButton>
        <GlassButton @click="openMultiMonthModal" variant="secondary">
          多月份共同统计
        </GlassButton>
        <GlassButton @click="fetchFansData" variant="primary" :disabled="fansLoading">
          {{ fansLoading ? '计算中...' : '计算新增粉丝数' }}
        </GlassButton>
        <!--
        <button @click="openClusterAnalysisModal" class="action-btn secondary">
          进行聚类分析
        </button>
        -->
      </div>

      <MonthSelector
        :visible="showMonthSelector"
        title="切换月份"
        mode="single"
        @confirm="performMonthSwitch"
        @cancel="closeMonthSelector"
      />

      <MonthSelector
        :visible="showMultiMonthModal"
        title="多月份共同统计"
        mode="range"
        @confirm="performMultiMonthQuery"
        @cancel="closeMultiMonthModal"
      />
    </div>

    <div class="info-section">
      <h1 class="page-title">{{ title }}</h1>
      <p class="refresh-time">刷新时间：{{ refreshTime }}</p>
      <div class="queried-user">
        <img v-if="anchorAvatar" :src="anchorAvatar" class="queried-user-avatar" @error="$event.target.style.display='none'" />
        查询用户：{{ queriedUser }} ({{ union }})
      </div>
    </div>

    <div class="chart-button-container">
      <GlassButton @click="showSessionChart" variant="primary">
        显示直播数据折线图
      </GlassButton>
      <GlassButton @click="hideChart" variant="danger">
        关闭图表
      </GlassButton>
    </div>

    <div class="chart-info" v-if="chartVisible">
      <h3 style="color: var(--color-accent); margin-top: 0;"><BarChart3 :size="20" class="heading-icon" /> 图表交互说明</h3>
      <p><strong>图表功能：</strong></p>
      <ul style="text-align: left; display: inline-block;">
        <li>点击图例可以隐藏/显示对应的数据显示</li>
        <li>鼠标悬停在数据点上可以查看详细数值</li>
        <li>图表支持缩放和拖拽（如果浏览器支持）</li>
        <li>双击图表可以重置缩放</li>
      </ul>
    </div>

    <div :class="['chart-container', { visible: chartVisible }]">
      <div id="chartCanvas" ref="chartCanvas" style="width:100%;height:100%;"></div>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>
    
    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <GlassButton @click="fetchData()" variant="success">重试</GlassButton>
    </div>
    
    <div v-else class="data-section">
      <!-- 导航表格 -->
      <NavigationTable :items="sessions" item-type="session" v-if="sessions.length > 0" />

      <!-- SC历史数据展示 -->
      <div v-if="scHistory && scHistory.list && scHistory.list.length > 0" class="sc-history-section hover-effect">
        <h3 style="color: var(--color-primary); margin-top: 0;"><MessageCircle :size="20" class="heading-icon" /> SC历史记录</h3>
        <div class="sc-history-container">
          <!-- 移动端：网格布局 -->
          <div class="grid-container mobile-grid">
            <div
              v-for="(sc, index) in scHistory.list"
              :key="index"
              class="sc-grid-item"
            >
              <div class="grid-header">
                <div class="grid-time">{{ sc.send_time }}</div>
              </div>
              <div class="grid-fields">
                <div class="field-box">
                  <div class="field-label">用户名</div>
                  <div class="field-value">{{ sc.uname }}</div>
                </div>
                <div class="field-box">
                  <div class="field-label">用户ID</div>
                  <div class="field-value">{{ sc.uid }}</div>
                </div>
                <div class="field-box">
                  <div class="field-label">金额</div>
                  <div class="field-value currency-cell">{{ formatCurrency(sc.price) }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 移动端：卡片布局 -->
      <div class="cards-container mobile-cards" ref="cardsContainer">
        <BaseCard
          v-for="(session, index) in sessions"
          :key="index"
          card-type="session"
          :rank="index + 1"
          :title="session.title"
          :subtitle="''"
          :fields="[
            {
              label: '开始时间',
              value: session.start_time.split(' ')[0] + '<br>' + session.start_time.split(' ')[1],
              type: 'datetime'
            },
            {
              label: '结束时间',
              value: (session.end_time.split(' ')[0] || '-') + '<br>' + (session.end_time.split(' ')[1] || '-'),
              type: 'datetime'
            },
            {
              label: '总直播时间',
              value: formatDurationCell(session.start_time, session.end_time),
              type: 'duration'
            },
            {
              label: '新增总督',
              value: (session.end_guard_3 != null ? session.end_guard_3 : 0) - (session.start_guard_3 != null ? session.start_guard_3 : 0),
              type: 'number'
            },
            {
              label: '新增提督',
              value: session.end_time === null || session.end_time === '' ? '-' : (session.end_guard_2 != null ? session.end_guard_2 : 0) - (session.start_guard_2 != null ? session.start_guard_2 : 0),
              type: 'number'
            },
            {
              label: '新增舰长',
              value: session.end_time === null || session.end_time === '' ? '-' : (session.end_guard_1 != null ? session.end_guard_1 : 0) - (session.start_guard_1 != null ? session.start_guard_1 : 0),
              type: 'number'
            },
            {
              label: '新增粉丝团',
              value: session.end_time === null || session.end_time === '' ? '-' : formatNumber((session.end_fans_count != null ? session.end_fans_count : 0) - (session.start_fans_count != null ? session.start_fans_count : 0)),
              type: 'number'
            },
            {
              label: '新增粉丝数',
              value: session.new_fans_count === undefined || session.new_fans_count === -1 
                ? '-' 
                : formatNumber(session.new_fans_count),
              type: 'number'
            },
            {
              label: '弹幕数',
              value: formatNumber(session.danmaku_count != null ? session.danmaku_count : 0),
              type: 'number'
            },
            {
              label: '礼物收入',
              value: formatCurrency(session.gift) + '<br>(' + calculatePercentage(session.gift, calculateTotalRevenue(session)) + '%)',
              type: 'currency'
            },
            {
              label: '舰长收入',
              value: formatCurrency(session.guard) + '<br>(' + calculatePercentage(session.guard, calculateTotalRevenue(session)) + '%)',
              type: 'currency'
            },
            {
              label: 'SC收入',
              value: formatCurrency(session.super_chat) + '<br>(' + calculatePercentage(session.super_chat, calculateTotalRevenue(session)) + '%)',
              type: 'currency'
            },
            {
              label: '总营收',
              value: formatCurrency(calculateTotalRevenue(session)),
              type: 'currency'
            },
            {
              label: '平均同接',
              value: session.avg_concurrency !== null ? session.avg_concurrency.toFixed(2) : 'N/A',
              type: session.avg_concurrency !== null ? 'number' : 'text'
            },
            {
              label: '最高同接',
              value: session.max_concurrency !== null ? formatNumber(session.max_concurrency) : 'N/A',
              type: session.max_concurrency !== null ? 'number' : 'text'
            },
            {
              label: '即时同接',
              value: session.current_concurrency !== null ? formatNumber(session.current_concurrency) : '未开播',
              type: session.current_concurrency !== null ? 'number' : 'text'
            }
          ]"
          :action-button="{ text: '查看SuperChat详情', className: 'sc-btn hover-effect' }"
          :action-data="session"
          @action-click="viewSuperChatDetails(session.start_time, session.end_time)"
        >
          <template #actions>
            <GlassButton
              @click="viewSuperChatDetails(session.start_time, session.end_time)"
              variant="secondary"
              size="sm"
              cta
            >
              查看SuperChat详情
            </GlassButton>
          </template>
        </BaseCard>
      </div>
      </div>
    </div>
  
</template>

<script>
import { ref, onMounted, watch, nextTick, provide } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import * as echarts from 'echarts'
import '@/utils/echartsTheme.js'
import { anchorAPI } from '@/api'
import BaseCard from '@/components/BaseCard.vue'
import NavigationTable from '@/components/NavigationTable.vue'
import MonthSelector from '@/components/MonthSelector.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import { BarChart3, MessageCircle } from 'lucide-vue-next'
import { getMonthRange } from '@/utils/monthUtils'
import { provideGlobalCardState } from '@/composables/useGlobalCardState'
import { getAvatar, getAvatarSync } from '@/utils/avatarCache'
import { staggerEnter } from '@/composables/useGSAP'
import gsap from 'gsap'

export default {
  name: 'LiveSessions',
  components: {
    BaseCard,
    NavigationTable,
    MonthSelector,
    GlassButton,
    BarChart3,
    MessageCircle
  },
  setup() {
    const router = useRouter()
    const route = useRoute()
    const chartVisible = ref(false)
    const sessions = ref([])
    const queriedUser = ref('未知主播')
    const title = ref('')
    const refreshTime = ref(new Date().toLocaleString())
    const union = ref('VirtuaReal')
    const loading = ref(true)
    const error = ref(null)
    const fansLoading = ref(false)
    const anchorAvatar = ref('')
    let sessionChart = null
    const chartCanvas = ref(null)
    const cardsContainer = ref(null)

    // 创建并提供全局卡片状态
    const globalCardState = provideGlobalCardState()
    provide('globalCardState', globalCardState)

    // 从路由参数获取数据
    const room_id = route.query.room_id
    union.value = route.query.union || 'VirtuaReal'
    const currentDate = new Date()
    const currentYear = currentDate.getFullYear()
    const currentMonth = String(currentDate.getMonth() + 1).padStart(2, '0')
    const month = route.query.month || `${currentYear}${currentMonth}`

    title.value = `${month.substring(0, 4)}年${parseInt(month.substring(4, 6)).toString().padStart(2, '0')}月直播数据`

    const goToMainSite = () => {
      window.open('https://hihivr.top', '_blank')
    }

    const followCreator = () => {
      window.open('https://space.bilibili.com/1048135385', '_blank')
    }

    const goToLiveRoom = () => {
      window.open(`https://live.bilibili.com/${room_id}`, '_blank')
    }

    const goBack = () => {
      router.go(-1)
    }


    // 月份选择器相关
    const showMonthSelector = ref(false)

    const openMonthSelector = () => {
      showMonthSelector.value = true
    }

    const closeMonthSelector = () => {
      showMonthSelector.value = false
    }

    const performMonthSwitch = ({ selectedMonth } = {}) => {
      if (!selectedMonth) {
        alert('请选择月份')
        return
      }

      if (room_id) {
        router.push(`/live-sessions?room_id=${room_id}&union=${union.value}&month=${selectedMonth}`)
        fetchData(selectedMonth)
        closeMonthSelector()
      } else {
        alert("无法切换月份，因为没有有效的 room_id。")
      }
    }

    const calculatePercentage = (value, total) => {
      if (!total || total <= 0) return '0.0'
      return ((parseFloat(value || 0) / total) * 100).toFixed(1)
    }

    const calculateDuration = (startTime, endTime) => {
      if (!startTime || !endTime) return 'N/A'

      try {
        // 处理时间格式，确保能正确解析
        let startStr = startTime;
        let endStr = endTime;

        // 如果时间格式包含空格，替换为T以便Date.parse能正确解析
        if (startStr.includes(' ') && !startStr.includes('T')) {
          startStr = startStr.replace(' ', 'T');
        }
        if (endStr.includes(' ') && !endStr.includes('T')) {
          endStr = endStr.replace(' ', 'T');
        }

        const start = new Date(startStr);
        const end = new Date(endStr);

        // 检查日期是否有效
        if (isNaN(start.getTime()) || isNaN(end.getTime())) {
          console.error('无效的日期格式:', startTime, endTime);
          return 'N/A';
        }

        const diffMs = end.getTime() - start.getTime();
        const diffMins = Math.round(diffMs / 60000);

        const hours = Math.floor(diffMins / 60);
        const minutes = diffMins % 60;

        return `${diffMins}分钟 (${hours}小时${minutes}分钟)`;
      } catch (e) {
        console.error('计算持续时间时出错:', e);
        return 'N/A';
      }
    }

    const calculateTotalRevenue = (session) => {
      const gift = parseFloat(session.gift) || 0;
      const guard = parseFloat(session.guard) || 0;
      const superChat = parseFloat(session.super_chat) || 0;

      return (gift + guard + superChat).toFixed(2);
    }

    const formatCurrency = (value) => {
      return parseFloat(value || 0).toFixed(2)
    }

    const formatNumber = (value) => {
      // 确保处理大数值时不会发生精度丢失
      const num = Number(value);
      // 检查是否为有效数字，如果不是则返回0
      if (isNaN(num) || num === null || num === undefined) {
        return '0';
      }
      // 使用 toLocaleString 进行格式化，这是更安全的方式
      return num.toLocaleString('en-US', {
        maximumFractionDigits: 0  // 不显示小数部分
      });
    }

    const formatDurationCell = (startTime, endTime) => {
      let durationText = calculateDuration(startTime, endTime);
      // 在括号前插入换行标记
      return durationText.replace(/\s\(/, '<br>(');
    }

    const showSessionChart = async () => {
      console.log('准备显示图表，会话数据数量:', sessions.value.length)
      chartVisible.value = true

      await nextTick()

      if (sessionChart) {
        console.log('销毁现有图表实例')
        sessionChart.dispose()
      }

      const labels = []
      const giftData = []
      const guardData = []
      const superChatData = []
      const totalRevenueData = []
      const durationData = []
      const newGuard3Data = []
      const newGuard2Data = []
      const newGuard1Data = []
      const newFansData = []
      const danmakuData = []
      const avgConcurrencyData = []
      const maxConcurrencyData = []
      const newFansCountData = []

      console.log('开始处理会话数据，共', sessions.value.length, '个会话')
      sessions.value.forEach((session, index) => {
        console.log(`处理第${index+1}个会话:`, session)
        const startTime = session.start_time
        const durationMinutes = session.duration_minutes || parseFloat(calculateDuration(session.start_time, session.end_time).split('分钟')[0]) || 0
        const gift = parseFloat(session.gift) || 0
        const guard = parseFloat(session.guard) || 0
        const superChat = parseFloat(session.super_chat) || 0
        const totalRevenue = parseFloat(calculateTotalRevenue(session)) || 0
        const newGuard3 = (session.end_guard_3 != null ? Number(session.end_guard_3) : 0) - (session.start_guard_3 != null ? Number(session.start_guard_3) : 0)
        const newGuard2 = session.end_time === null || session.end_time === '' ? 0 : (session.end_guard_2 != null ? Number(session.end_guard_2) : 0) - (session.start_guard_2 != null ? Number(session.start_guard_2) : 0)
        const newGuard1 = session.end_time === null || session.end_time === '' ? 0 : (session.end_guard_1 != null ? Number(session.end_guard_1) : 0) - (session.start_guard_1 != null ? Number(session.start_guard_1) : 0)
        const newFans = session.end_time === null || session.end_time === '' ? 0 : (session.end_fans_count != null ? Number(session.end_fans_count) : 0) - (session.start_fans_count != null ? Number(session.start_fans_count) : 0)
        const danmakuCount = session.danmaku_count != null ? Number(session.danmaku_count) : 0
        const avgConcurrency = session.avg_concurrency !== null ? Number(session.avg_concurrency) : 0
        const maxConcurrency = session.max_concurrency !== null ? Number(session.max_concurrency) : 0
        const newFansCount = session.new_fans_count !== undefined && session.new_fans_count !== -1
          ? Number(session.new_fans_count) : 0

        labels.push((startTime.split(' ')[0] || '直播场次'))
        giftData.push(gift)
        guardData.push(guard)
        superChatData.push(superChat)
        totalRevenueData.push(totalRevenue)
        durationData.push(durationMinutes)
        newGuard3Data.push(newGuard3)
        newGuard2Data.push(newGuard2)
        newGuard1Data.push(newGuard1)
        newFansData.push(newFans)
        danmakuData.push(danmakuCount)
        avgConcurrencyData.push(avgConcurrency)
        maxConcurrencyData.push(maxConcurrency)
        newFansCountData.push(newFansCount)
        console.log(`会话${index+1}处理完成，数据:`, {
          durationMinutes, gift, guard, superChat, totalRevenue,
          newGuard3, newGuard2, newGuard1, newFans, danmakuCount,
          avgConcurrency, maxConcurrency, newFansCount
        })
      })

      console.log('数据处理完成，标签数量:', labels.length)

      if (labels.length === 0) {
        console.log('没有可用的数据来生成图表')
        alert('没有可用的数据来生成图表')
        hideChart()
        return
      }

      if (!chartCanvas.value) {
        console.error('图表画布不存在')
        return
      }

      console.log('准备创建图表实例')
      sessionChart = echarts.init(chartCanvas.value, 'liveshow')
      sessionChart.setOption({
        title: {
          text: '直播数据趋势图',
          left: 'center',
          textStyle: { fontSize: 16 }
        },
        tooltip: {
          trigger: 'axis',
          axisPointer: { type: 'cross' }
        },
        legend: {
          top: 30,
          type: 'scroll'
        },
        grid: {
          left: 80,
          right: 80,
          top: 80,
          bottom: 60
        },
        xAxis: {
          type: 'category',
          data: labels,
          boundaryGap: false
        },
        yAxis: [
          {
            type: 'value',
            name: '直播时长 (分钟)',
            position: 'left'
          },
          {
            type: 'value',
            name: '收入 (元)',
            position: 'right',
            splitLine: { show: false }
          },
          {
            type: 'value',
            name: '人数',
            position: 'right',
            offset: 60,
            splitLine: { show: false }
          }
        ],
        series: [
          {
            name: '直播时长',
            type: 'line',
            data: durationData,
            yAxisIndex: 0,
            smooth: 0.4,
            symbol: 'circle',
            symbolSize: 5,
            itemStyle: { color: '#FF6384' },
            areaStyle: { color: 'rgba(255, 99, 132, 0.1)' }
          },
          {
            name: '礼物收入',
            type: 'line',
            data: giftData,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'triangle',
            symbolSize: 5,
            itemStyle: { color: '#36A2EB' },
            areaStyle: { color: 'rgba(54, 162, 235, 0.1)' }
          },
          {
            name: '舰长收入',
            type: 'line',
            data: guardData,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'rect',
            symbolSize: 5,
            itemStyle: { color: '#FFCE56' },
            areaStyle: { color: 'rgba(255, 206, 86, 0.1)' }
          },
          {
            name: 'SC收入',
            type: 'line',
            data: superChatData,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'diamond',
            symbolSize: 5,
            itemStyle: { color: '#4BC0C0' },
            areaStyle: { color: 'rgba(75, 192, 192, 0.1)' }
          },
          {
            name: '新增总督',
            type: 'line',
            data: newGuard3Data,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'cross',
            symbolSize: 5,
            itemStyle: { color: '#FF6B6B' },
            areaStyle: { color: 'rgba(255, 107, 107, 0.1)' }
          },
          {
            name: '新增提督',
            type: 'line',
            data: newGuard2Data,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'arrow',
            symbolSize: 5,
            itemStyle: { color: '#4ECDC4' },
            areaStyle: { color: 'rgba(78, 205, 196, 0.1)' }
          },
          {
            name: '新增舰长',
            type: 'line',
            data: newGuard1Data,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'star',
            symbolSize: 5,
            itemStyle: { color: '#45B7D1' },
            areaStyle: { color: 'rgba(69, 183, 209, 0.1)' }
          },
          {
            name: '新增粉丝团',
            type: 'line',
            data: newFansData,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'rect',
            symbolSize: 5,
            itemStyle: { color: '#96CEB4' },
            areaStyle: { color: 'rgba(150, 206, 180, 0.1)' }
          },
          {
            name: '弹幕数',
            type: 'line',
            data: danmakuData,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'none',
            itemStyle: { color: '#FFEAA7' },
            areaStyle: { color: 'rgba(255, 234, 167, 0.1)' }
          },
          {
            name: '总营收',
            type: 'line',
            data: totalRevenueData,
            yAxisIndex: 1,
            smooth: 0.4,
            symbol: 'circle',
            symbolSize: 6,
            lineStyle: { width: 3 },
            itemStyle: { color: '#9966FF' },
            areaStyle: { color: 'rgba(153, 102, 255, 0.1)' }
          },
          {
            name: '平均同接',
            type: 'line',
            data: avgConcurrencyData,
            yAxisIndex: 2,
            smooth: 0.4,
            symbol: 'circle',
            symbolSize: 5,
            itemStyle: { color: '#2E86AB' },
            areaStyle: { color: 'rgba(46, 134, 171, 0.1)' }
          },
          {
            name: '最高同接',
            type: 'line',
            data: maxConcurrencyData,
            yAxisIndex: 2,
            smooth: 0.4,
            symbol: 'triangle',
            symbolSize: 5,
            itemStyle: { color: '#A23B72' },
            areaStyle: { color: 'rgba(162, 59, 114, 0.1)' }
          },
          {
            name: '新增粉丝数',
            type: 'line',
            data: newFansCountData,
            yAxisIndex: 2,
            smooth: 0.4,
            symbol: 'rect',
            symbolSize: 5,
            itemStyle: { color: '#F18F01' },
            areaStyle: { color: 'rgba(241, 143, 1, 0.1)' }
          }
        ],
        dataZoom: [
          { type: 'inside', start: 0, end: 100 },
          { type: 'slider', start: 0, end: 100 }
        ]
      })
      console.log('图表创建完成')
    }

    const hideChart = () => {
      chartVisible.value = false
      if (sessionChart) {
        sessionChart.dispose()
        sessionChart = null
      }
    }

    const fetchData = async (selectedMonth = month) => {
      try {
        console.log('开始获取直播会话数据，参数:', { room_id, union: union.value, selectedMonth }) // 添加调试日志
        loading.value = true
        error.value = null
        const response = await anchorAPI.getLiveSessions(room_id, union.value, selectedMonth)
        console.log('API响应:', response) // 添加调试日志

        sessions.value = response.sessions || []
        console.log('设置会话数据，数量:', sessions.value.length) // 添加调试日志

        // 输出第一个会话的详细数据，用于调试
        if (sessions.value.length > 0) {
          console.log('第一个会话的详细数据:', sessions.value[0]);
          console.log('第一个会话的关键字段值:', {
            start_time: sessions.value[0].start_time,
            end_time: sessions.value[0].end_time,
            duration_minutes: sessions.value[0].duration_minutes,
            start_fans_count: sessions.value[0].start_fans_count,
            end_fans_count: sessions.value[0].end_fans_count,
            danmaku_count: sessions.value[0].danmaku_count,
            max_concurrency: sessions.value[0].max_concurrency,
            start_guard_1: sessions.value[0].start_guard_1,
            end_guard_1: sessions.value[0].end_guard_1,
            gift: sessions.value[0].gift,
            guard: sessions.value[0].guard,
            super_chat: sessions.value[0].super_chat
          });

          // 额外调试：检查特定会话（如泽音Melody的周年庆）
          const meloAnniversary = sessions.value.find(session =>
            session.title.includes('周年') && session.title.includes('泽音Melody')
          );
          if (meloAnniversary) {
            console.log('泽音Melody周年庆会话数据:', meloAnniversary);
            console.log('danmaku_count:', meloAnniversary.danmaku_count, typeof meloAnniversary.danmaku_count);
            console.log('max_concurrency:', meloAnniversary.max_concurrency, typeof meloAnniversary.max_concurrency);
          }
        }

        if (response.queried_user) {
          queriedUser.value = response.queried_user
        } else if (response.queried_user) {
          queriedUser.value = response.queried_user
        }

        if (room_id) {
          anchorAvatar.value = getAvatarSync(room_id)
        }
        console.log('设置查询用户:', queriedUser.value) // 添加调试日志

        const year = selectedMonth.substring(0, 4)
        const monthNum = parseInt(selectedMonth.substring(4, 6)).toString().padStart(2, '0')
        title.value = `${year}年${monthNum}月直播数据`
        refreshTime.value = response.refresh_time || new Date().toLocaleString()
        console.log('设置标题和刷新时间完成') // 添加调试日志

      } catch (err) {
        console.error('获取直播会话数据失败:', err)
        error.value = '获取数据失败，请稍后重试'
      } finally {
        loading.value = false
        nextTick(() => {
          if (cardsContainer.value) {
            const cards = cardsContainer.value.querySelectorAll(':scope > *')
            if (cards.length) staggerEnter(cards)
          }
        })
      }
    }

    const fetchFansData = async () => {
      if (!room_id) return
      fansLoading.value = true
      try {
        const monthValue = route.query.month || month
        const response = await anchorAPI.getLiveSessionsWithFans(room_id, union.value, monthValue)
        if (response.sessions && response.sessions.length > 0) {
          sessions.value = response.sessions
        }
      } catch (err) {
        console.error('获取粉丝数据失败:', err)
      } finally {
        fansLoading.value = false
      }
    }

    // 获取SC历史数据
    const fetchSCHistory = async (roomId) => {
      try {
        const response = await anchorAPI.getSuperChatHistory(roomId)
        scHistory.value = response
      } catch (err) {
        console.error('获取SC历史数据失败:', err)
        // 不设置错误，因为SC历史数据是可选的
      }
    }

    // 查看SuperChat详情
    const viewSuperChatDetails = (startTime, endTime) => {
      // 将直播时间段传递给SuperChat详情页面
      router.push({
        name: 'SuperChatDetail',
        query: {
          room_id: room_id,
          start_time: startTime,
          end_time: endTime,
          union: union.value
        }
      })
    }

    // 多月份统计相关
    const showMultiMonthModal = ref(false)

    const openMultiMonthModal = () => {
      showMultiMonthModal.value = true
    }

    const closeMultiMonthModal = () => {
      showMultiMonthModal.value = false
    }

    const performMultiMonthQuery = async ({ startMonth: sMonth, endMonth: eMonth } = {}) => {
      if (!sMonth || !eMonth) {
        alert('请选择起始和结束月份')
        return
      }

      try {
        loading.value = true
        error.value = null

        const months = getMonthRange(sMonth, eMonth)

        // 获取所有月份的数据并合并
        let combinedSessions = []
        for (const m of months) {
          try {
            const response = await anchorAPI.getLiveSessions(room_id, union.value, m)
            const sessionsForMonth = response.sessions || []

            // 累加每个会话的数据
            sessionsForMonth.forEach(session => {
              // 将数值字段转换为数字，使用Number()处理大数值
              session.gift = parseFloat(session.gift) || 0
              session.guard = parseFloat(session.guard) || 0
              session.super_chat = parseFloat(session.super_chat) || 0
              session.start_guard_1 = Number(session.start_guard_1) || 0
              session.start_guard_2 = Number(session.start_guard_2) || 0
              session.start_guard_3 = Number(session.start_guard_3) || 0
              session.end_guard_1 = Number(session.end_guard_1) || 0
              session.end_guard_2 = Number(session.end_guard_2) || 0
              session.end_guard_3 = Number(session.end_guard_3) || 0
              session.start_fans_count = Number(session.start_fans_count) || 0
              session.end_fans_count = Number(session.end_fans_count) || 0
              session.danmaku_count = Number(session.danmaku_count) || 0
              session.duration_minutes = Number(session.duration_minutes) || 0

              combinedSessions.push(session)
            })
          } catch (err) {
            console.error(`获取${m}月份数据失败:`, err)
            // 继续处理下一个月份
          }
        }

        // 更新会话数据
        sessions.value = combinedSessions

        // 更新标题
        const startYear = sMonth.substring(0, 4)
        const startMon = sMonth.substring(4, 6)
        const endYear = eMonth.substring(0, 4)
        const endMon = eMonth.substring(4, 6)
        title.value = `${startYear}年${startMon}月-${endYear}年${endMon}月直播数据`

        refreshTime.value = new Date().toLocaleString()
      } catch (err) {
        console.error('多月份统计查询失败:', err)
        error.value = '多月份统计查询失败，请稍后重试'
      } finally {
        loading.value = false
        closeMultiMonthModal()
      }
    }

    // 监听路由变化
    watch(
      () => route.query,
      (newQuery) => {
        // 页面变化时关闭图表
        hideChart()

        const newMonth = newQuery.month || month
        fetchData(newMonth)

        // 检查是否有scrollTo参数，如果有则跳转到指定元素
        if (newQuery.scrollTo) {
          nextTick(() => {
            const targetElement = document.getElementById(newQuery.scrollTo)
            if (targetElement) {
              targetElement.scrollIntoView({ behavior: 'smooth', block: 'start' })

              gsap.set(targetElement, { backgroundColor: 'rgba(249, 114, 154, 0.3)' })
              gsap.to(targetElement, { backgroundColor: 'transparent', duration: 0.5, delay: 1.5 })
            }
          })
        }
      }
    )

    onMounted(() => {
      fetchData()
    })

    return {
      sessions,
      queriedUser,
      title,
      refreshTime,
      union,
      loading,
      error,
      chartVisible,
      chartCanvas,
      anchorAvatar,
      goToMainSite,
      followCreator,
      viewSuperChatDetails,
      goToLiveRoom,
      goBack,
      openMonthSelector,
      calculatePercentage,
      calculateDuration,
      calculateTotalRevenue,
      formatCurrency,
      formatNumber,
      formatDurationCell,
      showSessionChart,
      hideChart,
      // 月份选择器相关
      showMonthSelector,
      closeMonthSelector,
      performMonthSwitch,
      // 多月份统计相关
      showMultiMonthModal,
      openMultiMonthModal,
      closeMultiMonthModal,
      performMultiMonthQuery,
      // 粉丝数计算相关
      fansLoading,
      fetchFansData,
      cardsContainer
    }
  }
}
</script>

<style scoped>
.heading-icon { vertical-align: middle; margin-right: 4px }
.live-sessions {
  background: var(--color-card);
  border-radius: var(--radius-card);
  padding: 20px;
  margin: 20px 0;
  border: 1px solid var(--color-primary);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.controls-section {
  margin-bottom: 20px;
}

.chart-button-container {
  display: flex;
  gap: 20px; /* 增加按钮间距 */
  justify-content: center;
  flex-wrap: wrap;
  margin: 30px 0 10px 0; /* 增加上下间距，让按钮离下面更远，但离表格有一定距离 */
}

.action-controls {
  display: flex;
  gap: 20px; /* 增加按钮间距 */
  justify-content: center;
  flex-wrap: wrap;
  margin: 30px 0; /* 增加上下间距，让按钮离下面更远 */
}

.action-btn {
  padding: 8px 16px;
  border: none;
  border-radius: var(--radius-button);
  cursor: pointer;
  font-size: 0.85rem;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 120px;
}

.action-btn.primary {
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: var(--color-text-main);
  font-weight: bold;
}

.action-btn.secondary {
  background: linear-gradient(45deg, var(--color-accent), var(--color-accent));
  color: white;
}

.action-btn.danger {
  background: linear-gradient(45deg, #E74C3C, #c0392b);
  color: white;
}

.action-btn.default {
  background: linear-gradient(45deg, var(--color-text-secondary), #7a6940);
  color: white;
}

.action-btn.success {
  background: linear-gradient(45deg, #27AE60, #219653);
  color: white;
}

.info-section {
  text-align: center;
  margin-bottom: 20px;
}

.page-title {
  color: var(--color-primary);
  font-size: 1.5rem;
  margin-bottom: 5px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
}

.refresh-time {
  color: var(--color-accent);
  font-size: 0.9rem;
  margin-bottom: 10px;
}

.queried-user {
  color: var(--color-primary);
  font-size: 1.5rem;
  font-weight: bold;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.queried-user-avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid var(--color-primary);
}

.chart-info {
  background: rgba(255, 107, 157, 0.1);
  border: 2px solid var(--color-accent);
  border-radius: var(--radius-card);
  padding: 15px;
  margin: 20px 0;
  text-align: center;
}

.chart-container {
  display: none;
  text-align: center;
  margin: 20px 0;
  height: 500px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: var(--radius-card);
  padding: 20px;
}

.chart-container.visible {
  display: block;
}

#chartCanvas {
  width: 100% !important;
  height: 100% !important;
}

.loading-state, .error-state {
  text-align: center;
  padding: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(246, 177, 0, 0.3);
  border-top: 4px solid var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 15px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-message {
  color: #ff6b6b;
  font-size: 1.1rem;
  margin-bottom: 15px;
}

.retry-btn {
  padding: 10px 20px;
  background: linear-gradient(45deg, #27AE60, #219653);
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-size: 0.9rem;
}

.table-container {
  overflow-x: auto;
  border-radius: 0; /* 移除圆角 */
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  /* 确保在窄屏设备上表格容器可以横向滚动 */
  display: block;
  white-space: nowrap;
  width: 100%; /* 使用正常宽度，修复截断问题 */
  max-width: 100%; /* 限制表格容器最大宽度为容器宽度 */
  margin-left: 0; /* 确保紧贴左侧 */
  margin-right: 0; /* 确保紧贴右侧 */
}

.sessions-table {
  width: 100%;
  border-collapse: collapse;
  background: var(--color-card);
  border-radius: var(--radius-card);
  overflow: hidden;
  table-layout: auto;
}

.sessions-table th:first-child {
  border-top-left-radius: 30px; /* 左上角圆角 */
}

.sessions-table th:last-child {
  border-top-right-radius: 30px; /* 右上角圆角 */
}

.sessions-table th {
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: var(--color-text-main);
  padding: 12px 8px;
  text-align: left;
  font-weight: bold;
  position: sticky;
  top: 0;
  z-index: 10;
}

.bold-header {
  font-weight: bolder !important;
  font-size: 1.1em;
}

.sessions-table th .sub-label {
  display: block;
  font-weight: normal;
  font-size: 0.75rem;
  margin-top: 2px;
}

.sessions-table td {
  padding: 10px 8px;
  border-bottom: 1px solid var(--color-primary);
  color: var(--color-text-main);
}

.sessions-table tbody tr {
}

.sessions-table tbody tr:nth-child(even) {
  background: rgba(246, 177, 0, 0.15); /* 橙色略微变深的背景 */
}

.sessions-table tbody tr:hover {
  background: rgba(246, 177, 0, 0.2); /* 橙色变浅的悬停效果 */
  color: var(--color-text-main);
}

.index-cell {
  color: var(--color-primary);
  font-weight: bold;
  text-align: center;
  background-color: rgba(246, 177, 0, 0.15);
}

.date-cell {
  color: var(--color-accent);
  font-family: 'Courier New', monospace;
  white-space: nowrap;
  font-weight: bold;
}

.duration-cell {
  color: var(--color-accent);
  font-weight: bold;
  text-align: center;
}

.revenue-cell {
  text-align: right;
}

.amount {
  display: block;
  color: var(--color-text-main);  /* 改为黑色 */
  font-weight: bold;
}

.percentage {
  display: block;
  font-size: 0.8rem;
  color: var(--color-accent);
  font-weight: bold;
}

.total-revenue {
  color: var(--color-accent);
  font-weight: bold;
  text-align: right;
}

.title-cell {
  color: var(--color-accent);
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: bold;
}

/* SC历史记录表格样式 */
.sc-history-container {
  overflow-x: auto;
  border-radius: 0; /* 移除圆角 */
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  display: block;
  white-space: nowrap;
  width: 100%; /* 使用正常宽度，修复截断问题 */
  max-width: 100%; /* 限制表格容器最大宽度为容器宽度 */
  margin-left: 0; /* 确保紧贴左侧 */
  margin-right: 0; /* 确保紧贴右侧 */
}

.sc-history-table {
  width: 100%;
  border-collapse: collapse;
  background: var(--color-card);
  border-radius: var(--radius-card);
  overflow: hidden;
  min-width: auto;
  table-layout: auto;
}

.sc-history-table th {
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: var(--color-text-main);
  padding: 12px 8px;
  text-align: left;
  font-weight: bold;
  position: sticky;
  top: 0;
  z-index: 10;
}

.sc-history-table td {
  padding: 10px 8px;
  border-bottom: 1px solid var(--color-primary);
  color: var(--color-text-main);
}

/* SC历史记录表格单元格内容截断处理 */
.sc-history-table .uname-cell {
  max-width: 100px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sc-history-table .uid-cell {
  max-width: 80px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sc-history-table .send-time-cell {
  max-width: 120px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* SC历史记录表格的响应式媒体查询 */
@media (max-width: 1024px) {
  .sc-history-table {
    font-size: 0.75rem;
  }

  .sc-history-table th,
  .sc-history-table td {
    padding: 6px 3px;
  }
}

@media (max-width: 600px) {
  .sc-history-table {
    font-size: 0.7rem;
  }

  .sc-history-table th,
  .sc-history-table td {
    padding: 8px 6px;
    min-width: 60px;
  }

  .sc-history-table td {
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

@media (max-width: 600px) {
  .sc-history-table {
    font-size: 0.65rem;
  }

  .sc-history-table th,
  .sc-history-table td {
    padding: 5px 4px;
  }
}

@media (max-width: 480px) {
  .sc-history-table {
    font-size: 0.6rem;
  }

  .sc-history-table th,
  .sc-history-table td {
    padding: 4px 3px;
    min-width: 40px;
  }

  .sc-history-table td {
    max-width: 80px;
  }
}

@media (max-width: 360px) {
  .sc-history-table {
    font-size: 0.55rem;
  }

  .sc-history-table th,
  .sc-history-table td {
    padding: 3px 2px;
  }
}

/* 桌面端表格显示 */
.desktop-table {
  display: table;
}

/* 移动端网格容器 */
.grid-container {
  display: none; /* 默认隐藏网格布局 */
}

/* 网格布局样式 */
.session-grid-item {
  background: var(--color-card);
  border: 1px solid var(--color-primary);
  border-radius: 15px;
  padding: 12px;
  margin-bottom: 12px;
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.1);
}

.sc-grid-item {
  background: var(--color-card);
  border: 1px solid var(--color-primary);
  border-radius: 15px;
  padding: 12px;
  margin-bottom: 12px;
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.1);
}

.grid-header {
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: white;
  padding: 8px;
  border-radius: 10px;
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.grid-index {
  font-weight: bold;
  font-size: 1.1em; /* 正常大小 */
}

.grid-title {
  font-weight: bold;
  margin: 5px 0;
  font-size: 1.2em; /* 增大字号 */
  text-align: center; /* 居中对齐 */
  color: var(--color-text-main); /* 设置颜色 */
  flex-grow: 1; /* 占据剩余空间 */
  padding: 0 10px; /* 添加左右内边距 */
}

.grid-time {
  font-weight: bold;
  text-align: center;
  font-size: 1.1em; /* 正常大小 */
}

.grid-fields {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 自适应网格 */
  gap: 8px;
  margin-bottom: 10px;
}

.field-box {
  background: rgba(255, 248, 225, 0.7);
  border: 1px solid var(--color-primary);
  border-radius: 10px;
  padding: 12px;
  min-width: 120px;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  text-align: left;
  margin-bottom: 6px;
}

.field-label {
  font-weight: bold;
  color: var(--color-primary);
  font-size: 1.1em;
  word-break: break-word;
  margin-bottom: 4px;
  flex-shrink: 0;
  background-color: rgba(246, 177, 0, 0.15);
  padding: 4px 8px;
  border-radius: 8px;
}

.field-value {
  color: var(--color-text-main);
  font-size: 1.1em;
  word-break: break-word;
  text-align: left;
  margin-left: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  align-self: flex-start;
  width: 100%;
}

/* 高亮重要数值 */
.total-revenue {
  color: var(--color-accent) !important;
  font-weight: bold;
}

.currency-cell {
  color: var(--color-accent) !important;
  font-weight: bold;
}

.grid-footer {
  text-align: center;
  margin-top: 8px;
}

/* 旧的卡片布局样式（保留用于可能的回退） */
.session-card {
  background: linear-gradient(135deg, var(--color-card), var(--color-card));
  border: 1px solid var(--color-primary);
  border-radius: 20px;
  padding: 15px;
  margin-bottom: 15px;
  box-shadow: 0 6px 16px rgba(246, 177, 0, 0.2);
  position: relative;
  overflow: hidden; /* 确保内容不会溢出 */
  will-change: transform; /* 优化性能 */
  transform: translateZ(0); /* 启用硬件加速 */
}


.sc-card {
  background: linear-gradient(135deg, var(--color-card), var(--color-card));
  border: 1px solid var(--color-primary);
  border-radius: 20px;
  padding: 15px;
  margin-bottom: 15px;
  box-shadow: 0 6px 16px rgba(246, 177, 0, 0.2);
  position: relative;
  overflow: hidden;
  will-change: transform;
  transform: translateZ(0);
}


.card-header {
  display: flex;
  justify-content: center;
  align-items: center;
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: white;
  padding: 10px;
  border-radius: 10px;
  margin-bottom: 8px;
}

.card-index {
  font-weight: bold;
  color: var(--color-primary);
  text-align: center;
  background-color: rgba(246, 177, 0, 0.15);
  border-radius: 50%;
  width: 35px;
  height: 35px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2em;
}

.card-title {
  color: var(--color-text-main);
  font-weight: bold;
  text-align: center;
  font-size: 1.2em;
}

.card-time {
  color: #27AE60;
  font-weight: bold;
  text-align: center;
  font-size: 1.1em;
}

.card-body {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 自动填充，最小140px宽的列，与AnchorList一致 */
  gap: 8px; /* 优化间距 */
}

/* 大屏优化：在大屏幕上显示更多列，与AnchorList一致 */
@media (min-width: 1024px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 宽屏下使用与AnchorList相同的最小宽度 */
    gap: 8px; /* 与AnchorList一致的间距 */
  }
}

/* 中等屏幕：显示固定列数 */
@media (min-width: 769px) and (max-width: 1023px) {
  .card-body {
    grid-template-columns: repeat(2, 1fr); /* 中等屏幕固定2列，与AnchorList一致 */
    gap: 8px; /* 与AnchorList一致的间距 */
  }
}

.field-label {
  font-weight: bold;
  color: var(--color-primary);
  font-size: 1em;
  word-break: break-word;
  margin-bottom: 4px;
  align-self: flex-start;
  background-color: rgba(246, 177, 0, 0.15);
  padding: 4px 8px;
  border-radius: 8px;
  min-width: 80px;
}

.field-label:hover {
  color: var(--color-primary);
  background-color: rgba(255, 165, 0, 0.25);
}

.field-value {
  text-align: left;
  color: var(--color-text-main);
  font-size: 1.1em;
  word-break: break-word;
  overflow: hidden;
  text-overflow: ellipsis;
  align-self: flex-start;
  width: 100%;
}

.field-value:hover {
  color: var(--color-accent);
}

.card-footer {
  margin-top: 10px;
  text-align: center;
}

/* 高亮重要数值 */
.total-revenue {
  color: var(--color-accent) !important;
  font-weight: bold;
}

.currency-cell {
  color: var(--color-accent) !important;
  font-weight: bold;
}

/* 响应式设计 */
@media (max-width: 1300px) {
  .sessions-table th,
  .sessions-table td {
    padding: 8px 5px;
    font-size: 0.8rem;
  }

  .table-container {
    overflow-x: auto;
  }
}

@media (max-width: 1200px) {
  .sessions-table th,
  .sessions-table td {
    padding: 7px 4px;
    font-size: 0.75rem;
  }

  .title-cell {
    max-width: 150px;
  }
}

@media (max-width: 1024px) {
  .live-sessions {
    padding: 15px 10px;
    margin: 10px 5px;
  }

  .page-title {
    font-size: 1.4rem;
    text-align: center;
  }

  .sessions-table {
    font-size: 0.75rem;
    min-width: auto; /* 移除固定最小宽度，让表格适应屏幕 */
    width: 100%; /* 让表格占满容器宽度 */
  }

  .sessions-table th,
  .sessions-table td {
    padding: 6px 3px;
  }

  .table-container {
    overflow-x: auto;
  }

  .title-cell {
    max-width: 120px;
  }
}

@media (max-width: 1024px) {
  .live-sessions {
    padding: 15px 8px;
    margin: 8px 0;
  }

  .action-controls {
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .action-btn {
    width: 100%;
    max-width: 280px;
    margin: 5px 0;
    padding: 10px 15px;
  }

  .page-title {
    font-size: 1.3rem;
    text-align: center;
  }

  /* 移动端使用卡片布局 */
  .desktop-table {
    display: none; /* 隐藏桌面表格 */
  }

  .cards-container {
    display: block; /* 显示卡片布局 */
    width: 100%;
  }

  .table-container {
    overflow-x: visible; /* 移动端不需要横向滚动 */
  }

  .session-card {
    margin-bottom: 15px; /* 调整间距 */
    padding: 12px; /* 调整内边距 */
  }

  .sc-card {
    margin-bottom: 15px; /* 调整间距 */
    padding: 12px; /* 调整内边距 */
  }

  .grid-fields {
    gap: 6px; /* 调整间距 */
  }

  .field-box {
    min-width: 110px; /* 调整最小宽度 */
    padding: 6px; /* 调整内边距 */
    flex-direction: column; /* 移动端改为垂直布局 */
    text-align: center; /* 文字居中 */
  }

  .field-label {
    font-size: 0.85em; /* 调整字体大小 */
    margin-bottom: 2px;
    margin-right: 0; /* 移动端移除右边距 */
    text-align: center; /* 文字居中 */
  }

  .field-value {
    font-size: 0.95em; /* 调整字体大小 */
    margin-left: 0; /* 移动端移除左边距 */
    text-align: center; /* 文字居中 */
  }

  .grid-header {
    padding: 6px; /* 调整内边距 */
  }

  .grid-index {
    font-size: 1em; /* 调整字体大小 */
  }

  .grid-title {
    font-size: 1em; /* 调整字体大小 */
  }

  .card-body {
    gap: 5px; /* 调整间距 */
  }
}

@media (max-width: 600px) {
  .live-sessions {
    padding: 12px 6px;
    margin: 6px 0;
  }

  .action-btn {
    max-width: 100%;
    padding: 8px 12px;
    font-size: 0.85rem;
  }

  .page-title {
    font-size: 1.2rem;
  }

  .refresh-time {
    font-size: 0.75rem;
    text-align: center;
  }

  .queried-user {
    font-size: 0.9rem;
    text-align: center;
  }

  .sessions-table {
    font-size: 0.65rem;
    min-width: auto;
    width: 100%;
  }

  .sessions-table th,
  .sessions-table td {
    padding: 6px 4px;
    min-width: 50px;
  }

  .title-cell {
    max-width: 80px;
    white-space: normal; /* 允许标题换行 */
  }

  .revenue-cell .amount,
  .revenue-cell .percentage,
  .total-revenue {
    font-size: 0.75em;
    text-align: right;
  }

  /* 仅在移动网格中保留换行处理 */
  .mobile-grid .revenue-cell .percentage {
    word-break: break-word;
  }

  .sc-btn {
    padding: 6px 8px;
    font-size: 0.7rem;
    min-width: 90px;
    width: 100%;
  }
}

@media (max-width: 480px) {
  .live-sessions {
    padding: 10px 4px;
  }

  .action-btn {
    padding: 7px 10px;
    font-size: 0.8rem;
    margin: 4px 0;
  }

  .page-title {
    font-size: 1.1rem;
  }

  .refresh-time {
    font-size: 0.7rem;
  }

  .queried-user {
    font-size: 0.9rem;
  }

  .sessions-table {
    font-size: 0.6rem;
    min-width: auto;
    width: 100%;
  }

  .sessions-table th,
  .sessions-table td {
    padding: 5px 3px;
    min-width: 40px;
  }

  .revenue-cell {
    text-align: center;
  }

  .total-revenue {
    text-align: center;
  }

  .title-cell {
    max-width: 60px;
    white-space: normal; /* 允许标题换行 */
  }

  .action-cell {
    text-align: center;
    min-width: 100px;
  }

  .sc-btn {
    padding: 5px 6px;
    font-size: 0.65rem;
    min-width: 80px;
    width: 100%;
  }
}

@media (max-width: 360px) {
  .live-sessions {
    padding: 8px 2px;
  }

  .page-title {
    font-size: 1.0rem;
  }

  .sessions-table {
    font-size: 0.55rem;
    min-width: auto; /* 移除固定最小宽度，让表格适应屏幕 */
    width: 100%;
  }

  .sessions-table th,
  .sessions-table td {
    padding: 4px 2px;
    min-width: 35px;
  }

  .sc-btn {
    padding: 4px 5px;
    font-size: 0.6rem;
    min-width: 70px;
    width: 100%;
  }
}

.sc-btn {
  padding: 6px 12px;
  background: linear-gradient(45deg, var(--color-accent), var(--color-accent));
  color: white;
  border: none;
  border-radius: var(--radius-button);
  cursor: pointer;
  font-size: 0.8rem;
  font-weight: bold;
  min-width: 120px;
  animation: buttonGlow 2s infinite alternate;
}

/* 特定按钮不使用闪光效果 */
.action-btn.secondary,
.action-btn.primary,
.action-btn.danger,
.action-btn.default,
.action-btn.success {
  animation: none;
}

/* sc-btn 使用洋红色闪光效果 */
.sc-btn {
  animation: magentaGlow 2s infinite alternate;
}

@keyframes magentaGlow {
  0% {
    box-shadow: 0 0 5px rgba(255, 107, 157, 0.5);
  }
  50% {
    box-shadow: 0 0 15px rgba(255, 107, 157, 0.8);
  }
  100% {
    box-shadow: 0 0 25px rgba(255, 107, 157, 1);
  }
}

/* 表格行也不使用闪光效果 */
.sessions-table tbody tr {
  animation: none;
}

.sc-btn:hover {
  background: linear-gradient(45deg, #ff88ad, #f06a8a);
}

@keyframes buttonGlow {
  0% {
    box-shadow: 0 0 5px rgba(255, 215, 0, 0.5);
  }
  50% {
    box-shadow: 0 0 15px rgba(255, 215, 0, 0.8);
  }
  100% {
    box-shadow: 0 0 25px rgba(255, 215, 0, 1);
  }
}

.datetime-cell {
  text-align: center;
  vertical-align: middle;
  font-family: 'Courier New', monospace;
  font-size: 0.8rem;
  line-height: 1.2;
  padding: 8px 4px;
}

/* 与主界面保持一致的悬停效果 */
.hover-effect {
  position: relative;
  z-index: 1;
}

/* 为表格行添加悬停效果 */
.sessions-table tbody tr {
}

/* 为按钮添加统一的悬停效果 */
.action-btn,
.sc-btn,
.view-btn,
.follow-btn,
.retry-btn,
.default,
.primary,
.success,
.danger {
  position: relative;
  overflow: hidden;
}

/* 按钮闪烁效果 */
.action-btn,
.sc-btn,
.view-btn,
.follow-btn,
.retry-btn,
.default,
.primary,
.success,
.danger {
  animation: buttonGlow 2s infinite alternate;
}

/* 主页开播主播行闪烁效果 */
.live-session-row {
  animation: subtleGlow 3s infinite alternate;
}

@keyframes subtleGlow {
  0% {
    background-color: rgba(255, 255, 255, 0.05);
  }
  50% {
    background-color: rgba(255, 215, 0, 0.1);
  }
  100% {
    background-color: rgba(255, 165, 0, 0.1);
  }
}

.start-time {
  color: #27AE60;
  font-weight: bold;
}

.end-time {
  color: #F44336;
  font-weight: bold;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px dashed rgba(142, 123, 80, 0.3);
}

.action-cell {
  text-align: center;
  vertical-align: middle;
}

@media (max-width: 480px) {
  .live-sessions {
    padding: 10px;
  }

  .page-title {
    font-size: 1.1rem;
  }

  .refresh-time {
    font-size: 0.8rem;
  }

  .queried-user {
    font-size: 0.9rem;
  }

  .sessions-table {
    font-size: 0.7rem;
  }

  .sessions-table th,
  .sessions-table td {
    padding: 4px 2px;
  }

  .revenue-cell {
    text-align: center;
  }

  .total-revenue {
    text-align: center;
  }

  .title-cell {
    max-width: 80px;
  }

  .action-cell {
    text-align: center;
  }

  .sc-btn {
    padding: 4px 8px;
    font-size: 0.7rem;
    min-width: 100px;
  }
}

/* 多月份统计模态框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: var(--color-card);
  border-radius: var(--radius-card);
  padding: 25px;
  width: 400px;
  max-width: 90%;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
}

.modal-content h3 {
  margin-top: 0;
  margin-bottom: 20px;
  color: var(--color-accent);
  text-align: center;
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

.form-group label {
  font-weight: bold;
  color: var(--color-text-main);
}

.month-input {
  padding: 10px;
  border: 2px solid var(--color-accent);
  border-radius: 10px;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.8);
}

.month-input:focus {
  outline: none;
  border-color: color-mix(in srgb, var(--color-accent) 80%, black);
  box-shadow: 0 0 10px rgba(255, 107, 157, 0.3);
}

.button-group {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.confirm-btn, .cancel-btn {
  flex: 1;
  padding: 10px;
  border: none;
  border-radius: var(--radius-button);
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
}

.confirm-btn {
  background: linear-gradient(45deg, var(--color-accent), var(--color-accent));
  color: white;
}

.cancel-btn {
  background: var(--color-text-secondary);
  color: white;
}

/* 宽屏优化：在大屏幕上显示更多列 */
@media (min-width: 1025px) {
  .grid-container {
    display: grid !important;
    grid-template-columns: repeat(auto-fill, minmax(420px, 1fr)); /* 自动填充，最小420px宽的列 */
    gap: 20px; /* 卡片间距 */
    padding: 15px; /* 内边距 */
  }

  .session-grid-item, .sc-grid-item {
    margin-bottom: 0; /* 在网格布局中不需要底部边距 */
    height: fit-content; /* 高度自适应内容 */
  }
}

/* 宽屏优化：在大屏幕上显示更多列，与AnchorList一致 */
@media (min-width: 1025px) {
  .cards-container {
    display: grid !important;
    grid-template-columns: repeat(auto-fill, minmax(380px, 1fr)); /* 自动填充，最小380px宽的列，与AnchorList一致 */
    gap: 20px; /* 卡片间距，与AnchorList一致 */
    padding: 15px; /* 内边距，与AnchorList一致 */
  }

  .session-card, .sc-card {
    margin-bottom: 0; /* 在网格布局中不需要底部边距 */
    height: fit-content; /* 高度自适应内容，与AnchorList一致 */
  }
}

/* 移动端优化：在小屏幕上优化显示，与AnchorList一致 */
@media (max-width: 768px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 保持网格布局，与AnchorList一致 */
    gap: 6px; /* 调整间距 */
  }

  .field-label, .field-value {
    text-align: left; /* 移动端统一左对齐，与AnchorList一致 */
    margin: 2px 0; /* 调整间距 */
  }

  .field-label {
    font-weight: bold;
    color: var(--color-text-secondary);
  }

  .card-header {
    flex-direction: column; /* 移动端改为垂直布局 */
    gap: 5px; /* 调整间距 */
    text-align: center; /* 文字居中 */
  }

  .card-index, .card-title {
    text-align: center; /* 移动端文字居中 */
  }
}

/* 更窄屏幕优化 */
@media (max-width: 600px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); /* 保持网格布局，调整最小宽度 */
    gap: 5px; /* 调整间距 */
  }
}

@media (max-width: 480px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); /* 保持网格布局，调整最小宽度 */
    gap: 5px; /* 调整间距 */
  }

  .field-label {
    font-size: 0.9em; /* 调整字体大小 */
  }

  .field-value {
    font-size: 0.95em; /* 调整字体大小 */
  }
}

@media (max-width: 360px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(80px, 1fr)); /* 保持网格布局，调整最小宽度 */
    gap: 4px; /* 调整间距 */
  }

  .field-label {
    font-size: 0.85em; /* 调整字体大小 */
  }

  .field-value {
    font-size: 0.9em; /* 调整字体大小 */
  }
}

.queried-anchor-card {
  margin: 20px 0;
  display: flex;
  justify-content: center;
}

/* 触屏设备优化 */
@media (hover: none) and (pointer: coarse) {
  .session-card,
  .sc-card {
    /* 为触屏设备添加点击反馈 */
    tap-highlight-color: transparent;
    -webkit-tap-highlight-color: transparent;
  }

  .session-card:active,
  .sc-card:active {
    transform: scale(0.98); /* 点击时轻微缩小 */
    box-shadow: 0 4px 16px rgba(255, 198, 51, 0.3); /* 减弱阴影 */
  }

  .field-label:active,
  .field-value:active {
    transform: scale(0.99); /* 点击时轻微缩小 */
  }
}
</style>