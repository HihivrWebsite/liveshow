<template>
  <div class="live-sessions">
    <div class="controls-section">
      <div class="action-controls">
        <button @click="openMonthSelector" class="action-btn secondary">
          切换不同月份
        </button>
        <button @click="showSessionChart" class="action-btn primary">
          显示直播数据折线图
        </button>
        <button @click="hideChart" class="action-btn danger">
          关闭图表
        </button>
        <button @click="goBack" class="action-btn default">
          返回
        </button>
        <button @click="goToLiveRoom" class="action-btn success">
          跳转到直播间
        </button>
        <button @click="openMultiMonthModal" class="action-btn secondary">
          多月份共同统计
        </button>
        <!--
        <button @click="openClusterAnalysisModal" class="action-btn secondary">
          进行聚类分析
        </button>
        -->
      </div>

      <!-- 月份选择器模态框 -->
      <div v-if="showMonthSelector" class="modal-overlay" @click="closeMonthSelector">
        <div class="modal-content" @click.stop>
          <h3>切换月份</h3>
          <div class="modal-form">
            <div class="form-group">
              <label>选择月份:</label>
              <input type="month" v-model="monthSelection" class="month-input">
            </div>
            <div class="button-group">
              <button @click="performMonthSwitch" class="confirm-btn">确定</button>
              <button @click="closeMonthSelector" class="cancel-btn">取消</button>
            </div>
          </div>
        </div>
      </div>

      <!-- 多月份统计模态框 -->
      <div v-if="showMultiMonthModal" class="modal-overlay" @click="closeMultiMonthModal">
        <div class="modal-content" @click.stop>
          <h3>多月份共同统计</h3>
          <div class="modal-form">
            <div class="form-group">
              <label>起始月份:</label>
              <input type="month" v-model="startMonth" class="month-input">
            </div>
            <div class="form-group">
              <label>结束月份:</label>
              <input type="month" v-model="endMonth" class="month-input">
            </div>
            <div class="button-group">
              <button @click="performMultiMonthQuery" class="confirm-btn">确定</button>
              <button @click="closeMultiMonthModal" class="cancel-btn">取消</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="info-section">
      <h1 class="page-title">{{ title }}</h1>
      <p class="refresh-time">刷新时间：{{ refreshTime }}</p>
      <div class="queried-user">
        查询用户：{{ queriedUser }} ({{ union }})
      </div>
    </div>

    <div class="chart-button-container">
      <button @click="showSessionChart" class="action-btn primary">
        显示直播数据折线图
      </button>
      <button @click="hideChart" class="action-btn danger">
        关闭图表
      </button>
    </div>

    <div class="chart-info" v-if="chartVisible">
      <h3 style="color: #f9729a; margin-top: 0;">📊 图表交互说明</h3>
      <p><strong>图表功能：</strong></p>
      <ul style="text-align: left; display: inline-block;">
        <li>点击图例可以隐藏/显示对应的数据显示</li>
        <li>鼠标悬停在数据点上可以查看详细数值</li>
        <li>图表支持缩放和拖拽（如果浏览器支持）</li>
        <li>双击图表可以重置缩放</li>
      </ul>
    </div>

    <div :class="['chart-container', { visible: chartVisible }]">
      <canvas id="chartCanvas" ref="chartCanvas"></canvas>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>
    
    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <button @click="fetchData()" class="retry-btn">重试</button>
    </div>
    
    <div v-else class="data-section">
      <!-- SC历史数据展示 -->
      <div v-if="scHistory && scHistory.list && scHistory.list.length > 0" class="sc-history-section hover-effect">
        <h3 style="color: #FFC633; margin-top: 0;">💬 SC历史记录</h3>
        <div class="sc-history-container">
          <table class="sc-history-table">
            <thead>
              <tr>
                <th>发送时间</th>
                <th>用户名</th>
                <th>用户ID</th>
                <th>金额</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(sc, index) in scHistory.list" :key="index" class="hover-effect">
                <td>{{ sc.send_time }}</td>
                <td>{{ sc.uname }}</td>
                <td>{{ sc.uid }}</td>
                <td class="currency-cell">{{ formatCurrency(sc.price) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div class="table-container hover-effect">
        <table class="sessions-table">
          <thead>
            <tr>
              <th>编号</th>
              <th>时间</th>
              <th>总直播时间</th>
              <th class="bold-header">新增总督</th>
              <th class="bold-header">新增提督</th>
              <th class="bold-header">新增舰长</th>
              <th class="bold-header">新增粉丝团</th>
              <th class="bold-header">弹幕数</th>
              <th class="bold-header">礼物收入<br><span class="sub-label">(占比%)</span></th>
              <th class="bold-header">舰长收入<br><span class="sub-label">(占比%)</span></th>
              <th class="bold-header">SC收入<br><span class="sub-label">(占比%)</span></th>
              <th>总营收</th>
              <th>标题</th>
              <th>查看SuperChat详情</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(session, index) in sessions" :key="index" class="live-session-row hover-effect">
              <td class="index-cell">{{ index + 1 }}</td>
              <td class="datetime-cell">
                <div class="start-time">{{ session.start_time.split(' ')[0] }}</div>
                <div class="start-time">{{ session.start_time.split(' ')[1] }}</div>
                <div class="end-time">{{ session.end_time.split(' ')[0] || '-' }}</div>
                <div class="end-time">{{ session.end_time.split(' ')[1] || '-' }}</div>
              </td>
              <td class="duration-cell" v-html="formatDurationCell(session.start_time, session.end_time)"></td>
              <td class="number-cell">{{ (session.end_guard_3 != null ? session.end_guard_3 : 0) - (session.start_guard_3 != null ? session.start_guard_3 : 0) }}</td>
              <td class="number-cell">{{ (session.end_guard_2 != null ? session.end_guard_2 : 0) - (session.start_guard_2 != null ? session.start_guard_2 : 0) }}</td>
              <td class="number-cell">{{ (session.end_guard_1 != null ? session.end_guard_1 : 0) - (session.start_guard_1 != null ? session.start_guard_1 : 0) }}</td>
              <td class="number-cell">{{ formatNumber((session.end_fans_count != null ? session.end_fans_count : 0) - (session.start_fans_count != null ? session.start_fans_count : 0)) }}</td>
              <td class="number-cell">{{ formatNumber(session.danmaku_count != null ? session.danmaku_count : 0) }}</td>
              <td class="revenue-cell">
                <span class="amount">{{ formatCurrency(session.gift) }}</span>
                <span class="percentage">({{ calculatePercentage(session.gift, calculateTotalRevenue(session)) }}%)</span>
              </td>
              <td class="revenue-cell">
                <span class="amount">{{ formatCurrency(session.guard) }}</span>
                <span class="percentage">({{ calculatePercentage(session.guard, calculateTotalRevenue(session)) }}%)</span>
              </td>
              <td class="revenue-cell">
                <span class="amount">{{ formatCurrency(session.super_chat) }}</span>
                <span class="percentage">({{ calculatePercentage(session.super_chat, calculateTotalRevenue(session)) }}%)</span>
              </td>
              <td class="total-revenue">{{ formatCurrency(calculateTotalRevenue(session)) }}</td>
              <td class="title-cell" style="white-space: normal; word-break: break-word; max-width: 150px;">{{ session.title }}</td>
              <td class="action-cell">
                <button
                  @click="viewSuperChatDetails(session.start_time, session.end_time)"
                  class="sc-btn hover-effect"
                >
                  查看SuperChat详情
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, watch, nextTick } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Chart, registerables } from 'chart.js'
import { anchorAPI } from '@/api'

Chart.register(...registerables)

export default {
  name: 'LiveSessions',
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
    let sessionChart = null
    const chartCanvas = ref(null)

    // 从路由参数获取数据
    const room_id = route.query.room_id
    union.value = route.query.union || 'VirtuaReal'
    const month = route.query.month || new Date().toISOString().slice(0, 7).replace('-', '').substring(0, 6)

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
    const monthSelection = ref('')

    const openMonthSelector = () => {
      // 设置默认值为当前年月
      const now = new Date()
      const currentYear = now.getFullYear()
      const currentMonth = String(now.getMonth() + 1).padStart(2, '0')
      monthSelection.value = `${currentYear}-${currentMonth}`
      showMonthSelector.value = true
    }

    const closeMonthSelector = () => {
      showMonthSelector.value = false
    }

    const performMonthSwitch = () => {
      if (!monthSelection.value) {
        alert('请选择月份')
        return
      }

      // 验证月份格式
      const selectedDate = new Date(monthSelection.value)
      if (isNaN(selectedDate.getTime())) {
        alert('无效的月份')
        return
      }

      const selectedMonth = monthSelection.value.replace('-', '')

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
        const start = new Date(startTime.replace(' ', 'T'));
        const end = new Date(endTime.replace(' ', 'T'));

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
      return parseInt(value || 0).toLocaleString()
    }

    const formatDurationCell = (startTime, endTime) => {
      let durationText = calculateDuration(startTime, endTime);
      // 在括号前插入换行标记
      return durationText.replace(/\s\(/, '<br>(');
    }

    const showSessionChart = async () => {
      console.log('准备显示图表，会话数据数量:', sessions.value.length) // 添加调试日志
      chartVisible.value = true

      await nextTick()

      if (sessionChart) {
        console.log('销毁现有图表实例') // 添加调试日志
        sessionChart.destroy()
      }

      const labels = []
      const giftData = []
      const guardData = []
      const superChatData = []
      const totalRevenueData = []
      const durationData = []
      const newGuard3Data = []  // 新增总督
      const newGuard2Data = []  // 新增提督
      const newGuard1Data = []  // 新增舰长
      const newFansData = []    // 新增粉丝团
      const danmakuData = []    // 弹幕数

      console.log('开始处理会话数据，共', sessions.value.length, '个会话') // 添加调试日志
      sessions.value.forEach((session, index) => {
        console.log(`处理第${index+1}个会话:`, session) // 添加调试日志
        const startTime = session.start_time
        // API返回中没有duration_minutes字段，需要在前端计算
        const durationMinutes = parseFloat(calculateDuration(session.start_time, session.end_time).split('分钟')[0]) || 0
        const gift = parseFloat(session.gift) || 0
        const guard = parseFloat(session.guard) || 0
        const superChat = parseFloat(session.super_chat) || 0
        const totalRevenue = parseFloat(calculateTotalRevenue(session)) || 0
        // 计算新增数量
        const newGuard3 = (session.end_guard_3 != null ? session.end_guard_3 : 0) - (session.start_guard_3 != null ? session.start_guard_3 : 0)
        const newGuard2 = (session.end_guard_2 != null ? session.end_guard_2 : 0) - (session.start_guard_2 != null ? session.start_guard_2 : 0)
        const newGuard1 = (session.end_guard_1 != null ? session.end_guard_1 : 0) - (session.start_guard_1 != null ? session.start_guard_1 : 0)
        const newFans = (session.end_fans_count != null ? session.end_fans_count : 0) - (session.start_fans_count != null ? session.start_fans_count : 0)
        const danmakuCount = session.danmaku_count != null ? session.danmaku_count : 0

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
        console.log(`会话${index+1}处理完成，数据:`, {
          durationMinutes,
          gift,
          guard,
          superChat,
          totalRevenue,
          newGuard3,
          newGuard2,
          newGuard1,
          newFans,
          danmakuCount
        }) // 添加调试日志
      })

      console.log('数据处理完成，标签数量:', labels.length) // 添加调试日志

      if (labels.length === 0) {
        console.log('没有可用的数据来生成图表') // 添加调试日志
        alert('没有可用的数据来生成图表')
        hideChart()
        return
      }

      if (!chartCanvas.value) {
        console.error('图表画布不存在') // 添加调试日志
        return
      }

      const ctx = chartCanvas.value.getContext('2d')
      console.log('准备创建图表实例') // 添加调试日志
      sessionChart = new Chart(ctx, {
        type: 'line',
        data: {
          labels: labels,
          datasets: [
            {
              label: '直播时长',
              data: durationData,
              borderColor: '#FF6384',
              backgroundColor: 'rgba(255, 99, 132, 0.1)',
              yAxisID: 'y',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'circle'  // 圆形
            },
            {
              label: '礼物收入',
              data: giftData,
              borderColor: '#36A2EB',
              backgroundColor: 'rgba(54, 162, 235, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'triangle'  // 三角形
            },
            {
              label: '舰长收入',
              data: guardData,
              borderColor: '#FFCE56',
              backgroundColor: 'rgba(255, 206, 86, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'rect'  // 矩形
            },
            {
              label: 'SC收入',
              data: superChatData,
              borderColor: '#4BC0C0',
              backgroundColor: 'rgba(75, 192, 192, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'rectRot'  // 旋转矩形
            },
            {
              label: '新增总督',
              data: newGuard3Data,
              borderColor: '#FF6B6B',
              backgroundColor: 'rgba(255, 107, 107, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'cross'  // 十字形
            },
            {
              label: '新增提督',
              data: newGuard2Data,
              borderColor: '#4ECDC4',
              backgroundColor: 'rgba(78, 205, 196, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'crossRot'  // 旋转十字形
            },
            {
              label: '新增舰长',
              data: newGuard1Data,
              borderColor: '#45B7D1',
              backgroundColor: 'rgba(69, 183, 209, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'star'  // 星形
            },
            {
              label: '新增粉丝团',
              data: newFansData,
              borderColor: '#96CEB4',
              backgroundColor: 'rgba(150, 206, 180, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'line'  // 线形
            },
            {
              label: '弹幕数',
              data: danmakuData,
              borderColor: '#FFEAA7',
              backgroundColor: 'rgba(255, 234, 167, 0.1)',
              yAxisID: 'y1',
              fill: true,
              pointRadius: 5,
              pointHoverRadius: 8,
              tension: 0.4,
              pointStyle: 'dash'  // 虚线形
            },
            {
              label: '总营收',
              data: totalRevenueData,
              borderColor: '#9966FF',
              backgroundColor: 'rgba(153, 102, 255, 0.1)',
              yAxisID: 'y1',
              fill: true,
              borderWidth: 3,
              pointRadius: 6,
              pointHoverRadius: 10,
              tension: 0.4,
              pointStyle: 'circle'  // 圆形，加粗显示
            }
          ]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            title: {
              display: true,
              text: '直播数据趋势图',
              font: {
                size: 16
              }
            },
            legend: {
              position: 'top',
            }
          },
          scales: {
            y: {
              type: 'linear',
              display: true,
              position: 'left',
              title: {
                display: true,
                text: '直播时长 (分钟)'
              }
            },
            y1: {
              type: 'linear',
              display: true,
              position: 'right',
              title: {
                display: true,
                text: '收入 (元)'
              },
              grid: {
                drawOnChartArea: false,
              },
            }
          },
          interaction: {
            mode: 'index',
            intersect: false
          },
          plugins: {
            tooltip: {
              enabled: true,
              mode: 'index',
              intersect: false
            }
          }
        }
      })
      console.log('图表创建完成') // 添加调试日志
    }

    const hideChart = () => {
      chartVisible.value = false
      if (sessionChart) {
        sessionChart.destroy()
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

        if (response.queried_user) {
          queriedUser.value = response.queried_user
        } else if (response.queried_user) {  // 修正拼写错误
          queriedUser.value = response.queried_user
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
    const startMonth = ref('')
    const endMonth = ref('')

    const openMultiMonthModal = () => {
      // 设置默认值为当前年月
      const now = new Date()
      const currentYear = now.getFullYear()
      const currentMonth = String(now.getMonth() + 1).padStart(2, '0')
      startMonth.value = `${currentYear}-${currentMonth}`
      endMonth.value = `${currentYear}-${currentMonth}`
      showMultiMonthModal.value = true
    }

    const closeMultiMonthModal = () => {
      showMultiMonthModal.value = false
    }

    const performMultiMonthQuery = async () => {
      if (!startMonth.value || !endMonth.value) {
        alert('请选择起始和结束月份')
        return
      }

      // 验证月份格式
      const start = new Date(startMonth.value)
      const end = new Date(endMonth.value)

      if (start > end) {
        alert('起始月份不能晚于结束月份')
        return
      }

      try {
        loading.value = true
        error.value = null

        // 将月份格式转换为 YYYYMM 格式
        const startMonthFormatted = startMonth.value.replace('-', '')
        const endMonthFormatted = endMonth.value.replace('-', '')

        // 获取起始和结束月份之间的所有月份
        const months = []
        const startDate = new Date(startMonth.value)
        const endDate = new Date(endMonth.value)

        // 设置为月初，避免日期问题
        startDate.setDate(1)
        endDate.setDate(1)

        const current = new Date(startDate)
        while (current <= endDate) {
          const year = current.getFullYear()
          const month = String(current.getMonth() + 1).padStart(2, '0')
          months.push(`${year}${month}`)
          current.setMonth(current.getMonth() + 1)
        }

        // 获取所有月份的数据并合并
        let combinedSessions = []
        for (const month of months) {
          try {
            const response = await anchorAPI.getLiveSessions(room_id, union.value, month)
            const sessionsForMonth = response.sessions || []

            // 累加每个会话的数据
            sessionsForMonth.forEach(session => {
              // 将数值字段转换为数字
              session.gift = parseFloat(session.gift) || 0
              session.guard = parseFloat(session.guard) || 0
              session.super_chat = parseFloat(session.super_chat) || 0
              session.start_guard_1 = parseInt(session.start_guard_1) || 0
              session.start_guard_2 = parseInt(session.start_guard_2) || 0
              session.start_guard_3 = parseInt(session.start_guard_3) || 0
              session.end_guard_1 = parseInt(session.end_guard_1) || 0
              session.end_guard_2 = parseInt(session.end_guard_2) || 0
              session.end_guard_3 = parseInt(session.end_guard_3) || 0
              session.start_fans_count = parseInt(session.start_fans_count) || 0
              session.end_fans_count = parseInt(session.end_fans_count) || 0
              session.danmaku_count = parseInt(session.danmaku_count) || 0

              combinedSessions.push(session)
            })
          } catch (err) {
            console.error(`获取${month}月份数据失败:`, err)
            // 继续处理下一个月份
          }
        }

        // 更新会话数据
        sessions.value = combinedSessions

        // 更新标题
        const startYear = startMonth.value.substring(0, 4)
        const startMon = startMonth.value.substring(5, 7)
        const endYear = endMonth.value.substring(0, 4)
        const endMon = endMonth.value.substring(5, 7)
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
      monthSelection,
      closeMonthSelector,
      performMonthSwitch,
      // 多月份统计相关
      showMultiMonthModal,
      startMonth,
      endMonth,
      openMultiMonthModal,
      closeMultiMonthModal,
      performMultiMonthQuery
    }
  }
}
</script>

<style scoped>
.live-sessions {
  background: #FFF8E1;
  border-radius: 30px; /* 超椭圆曲线 */
  padding: 20px;
  margin: 20px 0;
  border: 1px solid #FFC633;
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
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 120px; /* 最小宽度确保圆形效果 */
}

.action-btn.primary {
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
  font-weight: bold;
}

.action-btn.secondary {
  background: linear-gradient(45deg, #f9729a, #f75982);
  color: white;
}

.action-btn.danger {
  background: linear-gradient(45deg, #dc3545, #c82333);
  color: white;
}

.action-btn.default {
  background: linear-gradient(45deg, #6c757d, #5a6268);
  color: white;
}

.action-btn.success {
  background: linear-gradient(45deg, #28a745, #218838);
  color: white;
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.info-section {
  text-align: center;
  margin-bottom: 20px;
}

.page-title {
  color: #FFC633;
  font-size: 1.5rem;
  margin-bottom: 5px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
}

.refresh-time {
  color: #f9729a;
  font-size: 0.9rem;
  margin-bottom: 10px;
}

.queried-user {
  color: #FFC633;
  font-size: 1rem;
  font-weight: bold;
}

.chart-info {
  background: #FEEFEF;
  border: 2px solid #f9729a;
  border-radius: 30px; /* 超椭圆曲线 */
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
  border-radius: 30px; /* 超椭圆曲线 */
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
  border: 4px solid rgba(255, 198, 51, 0.3);
  border-top: 4px solid #FFC633;
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
  background: linear-gradient(45deg, #33CC99, #28a745);
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

.retry-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(40, 167, 69, 0.3);
}

.table-container {
  overflow-x: auto;
  border-radius: 30px; /* 使用与表格相同的超椭圆曲线 */
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
}

.sessions-table {
  width: 100%;
  border-collapse: collapse;
  background: #FFF8E1;
  border-radius: 30px; /* 添加超椭圆曲线 */
  overflow: hidden; /* 确保圆角生效 */
}

.sessions-table th:first-child {
  border-top-left-radius: 30px; /* 左上角圆角 */
}

.sessions-table th:last-child {
  border-top-right-radius: 30px; /* 右上角圆角 */
}

.sessions-table th {
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
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
  border-bottom: 1px solid #FFC633;
  color: #333;
}

.sessions-table tbody tr {
  transition: background-color 0.3s ease;
}

.sessions-table tbody tr:nth-child(even) {
  background: #FFE5B4; /* 橙色略微变深的背景 */
}

.sessions-table tbody tr:hover {
  background: #FFD580; /* 橙色变浅的悬停效果 */
  color: #333;
}

.index-cell {
  color: #FF6600;
  font-weight: bold;
  text-align: center;
  background-color: #FFF3CD;
}

.date-cell {
  color: #f9729a;
  font-family: 'Courier New', monospace;
  white-space: nowrap;
  font-weight: bold;
}

.duration-cell {
  color: #f9729a;
  font-weight: bold;
  text-align: center;
}

.revenue-cell {
  text-align: right;
}

.amount {
  display: block;
  color: #333;  /* 改为黑色 */
  font-weight: bold;
}

.percentage {
  display: block;
  font-size: 0.8rem;
  color: #f9729a;
  font-weight: bold;  /* 加粗 */
}

.total-revenue {
  color: #f9729a;
  font-weight: bold;
  text-align: right;
}

.title-cell {
  color: #f9729a;
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: bold;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .sessions-table th,
  .sessions-table td {
    padding: 8px 6px;
    font-size: 0.85rem;
  }
  
  .title-cell {
    max-width: 150px;
  }
}

@media (max-width: 768px) {
  .live-sessions {
    padding: 15px;
    margin: 10px 0;
  }
  
  .action-controls {
    flex-direction: column;
    align-items: center;
  }
  
  .action-btn {
    width: 100%;
    max-width: 280px;
    margin: 5px 0;
  }
  
  .page-title {
    font-size: 1.3rem;
  }
  
  .sessions-table {
    font-size: 0.8rem;
  }
  
  .sessions-table th,
  .sessions-table td {
    padding: 6px 4px;
  }
  
  .title-cell {
    max-width: 100px;
  }
}

.sc-btn {
  padding: 6px 12px;
  background: linear-gradient(45deg, #f9729a, #f75982);
  color: white;
  border: none;
  border-radius: 30px; /* 超椭圆形状 */
  cursor: pointer;
  font-size: 0.8rem;
  transition: all 0.3s ease;
  font-weight: bold;
  min-width: 120px; /* 最小宽度确保圆形效果 */
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
    box-shadow: 0 0 5px rgba(249, 114, 154, 0.5); /* 洋红色 */
  }
  50% {
    box-shadow: 0 0 15px rgba(249, 114, 154, 0.8); /* 洋红色 */
  }
  100% {
    box-shadow: 0 0 25px rgba(249, 114, 154, 1); /* 洋红色 */
  }
}

/* 表格行也不使用闪光效果 */
.sessions-table tbody tr {
  animation: none;
}

.sc-btn:hover {
  background: linear-gradient(45deg, #ff88ad, #f06a8a); /* 变亮效果 */
  box-shadow: 0 2px 8px rgba(249, 114, 154, 0.3);
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
  transition: all 0.3s ease;
  position: relative;
  z-index: 1;
}

.hover-effect:hover {
  transform: translateY(-3px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.3);
  z-index: 10;
}

/* 为表格行添加悬停效果 */
.sessions-table tbody tr {
  transition: all 0.3s ease;
}

.sessions-table tbody tr:hover {
  background-color: rgba(255, 255, 255, 0.1);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
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
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.action-btn:hover,
.sc-btn:hover,
.view-btn:hover,
.follow-btn:hover,
.retry-btn:hover,
.default:hover,
.primary:hover,
.success:hover,
.danger:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.3);
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
  color: #4CAF50;
  font-weight: bold;
}

.end-time {
  color: #F44336;
  font-weight: bold;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px dashed #ccc;
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
  background: #FFF8E1;
  border-radius: 20px;
  padding: 25px;
  width: 400px;
  max-width: 90%;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.modal-content h3 {
  margin-top: 0;
  margin-bottom: 20px;
  color: #f9729a;
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
  color: #333;
}

.month-input {
  padding: 10px;
  border: 2px solid #f9729a;
  border-radius: 10px;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.8);
  transition: all 0.3s ease;
}

.month-input:focus {
  outline: none;
  border-color: #e0658a;
  box-shadow: 0 0 10px rgba(249, 114, 154, 0.3);
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
  border-radius: 10px;
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
}

.confirm-btn {
  background: linear-gradient(45deg, #f9729a, #f75982);
  color: white;
}

.confirm-btn:hover {
  background: linear-gradient(45deg, #e0658a, #d05572);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.cancel-btn {
  background: #ccc;
  color: white;
}

.cancel-btn:hover {
  background: #bbb;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}
</style>