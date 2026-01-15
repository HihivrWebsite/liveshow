<template>
  <div class="simple-anchor-list">
    <div class="controls-section">
      <div class="filter-controls">
        <button 
          @click="switchFilter('all')" 
          :class="['filter-btn', { active: currentFilter === 'all' }]"
        >
          维阿PSP斗虫榜
        </button>
        <button 
          @click="switchFilter('vr')" 
          :class="['filter-btn', { active: currentFilter === 'vr' }]"
        >
          维阿斗虫榜
        </button>
        <button 
          @click="switchFilter('psp')" 
          :class="['filter-btn', { active: currentFilter === 'psp' }]"
        >
          PSPlive斗虫榜
        </button>
      </div>

      <div class="action-controls">
        <button @click="openMonthSelector" class="action-btn secondary">
          切换不同月份
        </button>
      </div>
    </div>

    <div class="info-section">
      <h2 class="page-title">{{ title }}</h2>
      <p class="refresh-time">刷新时间：{{ refreshTime }}</p>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>
    
    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <button @click="fetchData" class="retry-btn">重试</button>
    </div>
    
    <div v-else class="data-section">
      <div class="table-container">
        <table class="anchor-table">
          <thead>
            <tr>
              <th>排名</th>
              <th>主播名称</th>
              <th>工会</th>
              <th class="bold-header">关注数</th>
              <th class="bold-header">有效天</th>
              <th>开播时长</th>
              <th class="bold-header">开播状态</th>
              <th class="bold-header">礼物收入</th>
              <th class="bold-header">舰长收入</th>
              <th class="bold-header">SC收入</th>
              <th>总营收</th>
              <th>操作</th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="(anchor, index) in anchors" 
              :key="anchor.room_id || index"
              :class="{ 'live-row': anchor.status === 1 }"
            >
              <td class="rank-cell">{{ index + 1 }}</td>
              <td class="name-cell">{{ anchor.anchor_name }}</td>
              <td class="union-cell">{{ anchor.union }}</td>
              <td class="number-cell">{{ formatNumber(anchor.attention) }}</td>
              <td class="number-cell">{{ anchor.effective_days }}</td>
              <td class="duration-cell">{{ anchor.live_duration }}</td>
              <td class="status-cell">
                <span 
                  :class="[
                    'status-badge', 
                    { live: anchor.status === 1, offline: anchor.status !== 1 }
                  ]"
                >
                  <template v-if="anchor.status === 1">
                    <a 
                      :href="`https://live.bilibili.com/${anchor.room_id}`" 
                      target="_blank" 
                      class="live-link"
                      :title="`点击跳转到 ${anchor.anchor_name} 的 Bilibili 直播间`"
                    >
                      正在直播
                    </a>
                  </template>
                  <template v-else>
                    未开播
                  </template>
                </span>
              </td>
              <td class="number-cell">{{ formatCurrency(anchor.gift) }}</td>
              <td class="number-cell">{{ formatCurrency(anchor.guard) }}</td>
              <td class="number-cell">{{ formatCurrency(anchor.super_chat) }}</td>
              <td class="number-cell total-revenue">
                {{ formatCurrency(calculateTotalRevenue(anchor)) }}
              </td>
              <td class="action-cell">
                <button
                  @click="viewLiveSessions(anchor.room_id, anchor.union)"
                  class="view-btn"
                >
                  查看详细数据和详细SuperChat
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
import { ref, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { anchorAPI } from '@/api'
import { formatCurrency, formatNumber, calculateTotalRevenue } from '@/utils/dataProcessor'

export default {
  name: 'SimpleAnchorList',
  setup() {
    const router = useRouter()
    const route = useRoute()
    const anchors = ref([])
    const title = ref('维阿PSP斗虫榜')
    const refreshTime = ref(new Date().toLocaleString())
    const currentFilter = ref('all')
    const loading = ref(true)
    const error = ref(null)

    // 从路由参数获取初始值
    const filterFromRoute = route.query.filter || 'all'
    const monthFromRoute = route.query.month || null
    currentFilter.value = filterFromRoute

    // 更新标题
    if (monthFromRoute) {
      const year = monthFromRoute.substring(0, 4)
      const month = parseInt(monthFromRoute.substring(4, 6)).toString().padStart(2, '0')
      title.value = `维阿PSP斗虫榜_${year}年${month}月记录数据（点击"正在直播"跳转到对应直播间）`
    } else {
      title.value = filterFromRoute === 'vr' ? '维阿斗虫榜' :
                   filterFromRoute === 'psp' ? 'PSPlive斗虫榜' : '维阿PSP斗虫榜'
    }

    // 获取数据
    const fetchData = async () => {
      try {
        loading.value = true
        error.value = null
        let response;
        const currentMonth = route.query.month || null;
        if (currentMonth) {
          response = await anchorAPI.getAnchorsByMonth(currentMonth, currentFilter.value);
        } else {
          response = await anchorAPI.getAnchors(currentFilter.value, currentMonth);
        }
        anchors.value = response.anchors || response.data || []
        refreshTime.value = response.refresh_time || new Date().toLocaleString()
      } catch (err) {
        console.error('获取主播数据失败:', err)
        error.value = '获取数据失败，请稍后重试'
      } finally {
        loading.value = false
      }
    }

    const goToMainSite = () => {
      window.open('https://hihivr.top', '_blank')
    }

    const followCreator = () => {
      window.open('https://space.bilibili.com/1048135385', '_blank')
    }

    const viewLiveSessions = (roomId, union) => {
      const currentMonth = route.query.month || new Date().toISOString().slice(0, 7).replace('-', '');
      router.push(`/live-sessions?room_id=${roomId}&union=${union}&month=${currentMonth}`)
    }

    const switchFilter = (filterType) => {
      router.push({ 
        path: '/simple', 
        query: { ...route.query, filter: filterType } 
      })
    }

    const openMonthSelector = () => {
      let year = prompt("请输入年份 (例如: 2025):")
      let month = prompt("请输入月份 (例如: 09):")

      if (!year || !month) {
        alert("请输入有效的年份和月份！")
        return
      }

      year = year.trim()
      month = month.trim()

      if (!/^\d{4}$/.test(year)) {
        alert("年份必须为4位纯数字（2000-2099），例如：2025")
        return
      }

      if (!/^(0[1-9]|1[0-2])$/.test(month)) {
        alert("月份必须为01-12之间的2位数字，例如：09")
        return
      }

      const safeYear = encodeURIComponent(year)
      const safeMonth = encodeURIComponent(month)

      if (parseInt(month) > 12) {
        alert("请输入有效的月份（01-12）")
        return
      }

      router.push({
        path: '/by-month',
        query: { month: `${safeYear}${safeMonth}`, filter: currentFilter.value }
      })
    }

    // 监听路由变化
    watch(
      () => route.query,
      (newQuery) => {
        // 页面变化时关闭所有图表
        // 注意：SimpleAnchorList组件没有图表功能，所以不需要关闭图表

        currentFilter.value = newQuery.filter || 'all'
        const newMonth = newQuery.month || null
        if (newMonth) {
          const year = newMonth.substring(0, 4)
          const monthNum = parseInt(newMonth.substring(4, 6)).toString().padStart(2, '0')
          title.value = `维阿PSP斗虫榜_${year}年${monthNum}月记录数据（点击"正在直播"跳转到对应直播间）`
        } else {
          title.value = currentFilter.value === 'vr' ? '维阿斗虫榜' :
                       currentFilter.value === 'psp' ? 'PSPlive斗虫榜' : '维阿PSP斗虫榜'
        }
        fetchData()
      },
      { immediate: true }
    )

    onMounted(() => {
      fetchData()
    })

    return {
      anchors,
      title,
      refreshTime,
      currentFilter,
      loading,
      error,
      goToMainSite,
      followCreator,
      viewLiveSessions,
      switchFilter,
      openMonthSelector,
      formatCurrency,
      formatNumber,
      calculateTotalRevenue
    }
  }
}
</script>

<style scoped>
.simple-anchor-list {
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

.filter-controls {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
  flex-wrap: wrap;
  justify-content: center;
}

.filter-btn {
  padding: 10px 20px;
  border: 2px solid #FFC633;
  border-radius: 25px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  background: linear-gradient(45deg, #FFC633, #FFA500); /* 添加渐变背景 */
  color: #333;
  font-weight: bold;
}

.filter-btn.active {
  background: linear-gradient(45deg, #f9729a, #f75982);
  border-color: #f9729a;
  color: white;
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.filter-btn:hover:not(.active) {
  background: linear-gradient(45deg, #FFDB58, #FFC633); /* 悬停时的渐变 */
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
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

.action-btn.secondary {
  background: linear-gradient(45deg, #f9729a, #f75982);
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
  color: #33CC99;
  font-size: 0.9rem;
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

.anchor-table {
  width: 100%;
  border-collapse: collapse;
  background: #FFF8E1;
  border-radius: 30px; /* 添加超椭圆曲线 */
  overflow: hidden; /* 确保圆角生效 */
}

.anchor-table th:first-child {
  border-top-left-radius: 30px; /* 左上角圆角 */
}

.anchor-table th:last-child {
  border-top-right-radius: 30px; /* 右上角圆角 */
}

.anchor-table th {
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

.anchor-table td {
  padding: 10px 8px;
  border-bottom: 1px solid #FFC633;
  color: #333;
}

.anchor-table tbody tr {
  transition: background-color 0.3s ease;
}

.anchor-table tbody tr:nth-child(even) {
  background: #FFE5B4; /* 橙色略微变深的背景 */
}

.anchor-table tbody tr:hover {
  background: #FFD580; /* 橙色变浅的悬停效果 */
  color: #333;
}

.live-row {
  background: #FFF8E1; /* 浅黄色背景 */
  color: #333;
}

.rank-cell {
  font-weight: bold;
  color: #FFC633;
  text-align: center;
}

.name-cell {
  color: #fff;
  font-weight: 500;
}

.union-cell {
  color: #f9729a;
  font-weight: 500;
}

.number-cell {
  text-align: right;
  font-family: 'Courier New', monospace;
  color: #fff;
}

.total-revenue {
  color: #f9729a;
  font-weight: bold;
}

.duration-cell {
  color: #f9729a;
}

.status-cell {
  text-align: center;
}

.status-badge {
  display: inline-block;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: bold;
}

.status-badge.live {
  background: #f9729a; /* 实心洋红色背景，与查看详细数据按钮相同 */
  color: white; /* 白色文字，与查看详细数据按钮相同 */
  border: 2px solid #f9729a; /* 洋红色边框，与查看详细数据按钮相同 */
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  padding: 4px 8px; /* 内边距，与查看详细数据按钮相似 */
  cursor: pointer; /* 鼠标指针 */
  font-size: 0.8rem; /* 字体大小，与查看详细数据按钮相同 */
  transition: all 0.3s ease; /* 过渡效果 */
  font-weight: bold; /* 加粗 */
  animation: glow 2s infinite; /* 添加闪光动画 */
  text-decoration: none; /* 去除下划线 */
  display: inline-block; /* 行内块显示 */
  min-width: 80px; /* 最小宽度确保圆形效果 */
}

/* 闪光动画 */
@keyframes glow {
  0% {
    box-shadow: 0 0 5px #f9729a;
  }
  50% {
    box-shadow: 0 0 20px #f9729a;
  }
  100% {
    box-shadow: 0 0 5px #f9729a;
  }
}

.status-badge.offline {
  background: rgba(255, 255, 255, 0.2);
  color: #ccc;
}

.live-link {
  color: white;
  text-decoration: none;
}

.live-link:hover {
  text-decoration: underline;
}

.view-btn {
  padding: 6px 12px;
  background: #f9729a; /* 实心洋红色背景 */
  color: white; /* 白色文字 */
  border: 2px solid #f9729a; /* 洋红色边框 */
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  cursor: pointer;
  font-size: 0.8rem;
  transition: all 0.3s ease;
  font-weight: bold;
  min-width: 120px; /* 最小宽度确保圆形效果 */
}

.view-btn:hover {
  background: #e0658a; /* 悬停时更深的洋红色 */
  color: white; /* 悬停时文字保持白色 */
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(249, 114, 154, 0.3);
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .anchor-table th,
  .anchor-table td {
    padding: 8px 6px;
    font-size: 0.85rem;
  }
}

@media (max-width: 768px) {
  .simple-anchor-list {
    padding: 15px;
    margin: 10px 0;
  }
  
  .filter-controls,
  .action-controls {
    flex-direction: column;
    align-items: center;
  }
  
  .filter-btn,
  .action-btn {
    width: 100%;
    max-width: 280px;
    margin: 5px 0;
  }
  
  .page-title {
    font-size: 1.3rem;
  }
  
  .anchor-table {
    font-size: 0.8rem;
  }
  
  .anchor-table th,
  .anchor-table td {
    padding: 6px 4px;
  }
}

@media (max-width: 480px) {
  .simple-anchor-list {
    padding: 10px;
  }
  
  .page-title {
    font-size: 1.1rem;
  }
  
  .refresh-time {
    font-size: 0.8rem;
  }
  
  .anchor-table {
    font-size: 0.7rem;
  }
  
  .anchor-table th,
  .anchor-table td {
    padding: 4px 2px;
  }
  
  .number-cell,
  .duration-cell {
    text-align: center;
  }
  
  .action-cell {
    text-align: center;
  }
  
  .view-btn {
    padding: 4px 8px;
    font-size: 0.7rem;
  }
}
</style>