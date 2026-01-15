<template>
  <div class="super-chat-detail">
    <div class="header-section">
      <h1 class="page-title">SuperChat内容查看</h1>
      <p class="refresh-time">刷新时间：{{ refreshTime }}</p>
      <div class="queried-info">
        房间ID：{{ roomId }} | 工会：{{ union }}
      </div>
      <div class="time-range">
        时间范围：{{ startTime }} 至 {{ endTime }}
      </div>
    </div>

    <div class="controls-section">
      <button @click="goBack" class="action-btn default">返回</button>
      <button @click="fetchData" class="action-btn primary">刷新数据</button>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <button @click="fetchData" class="retry-btn">重试</button>
    </div>

    <div v-else class="sc-content">
      <div v-if="filteredSCList.length === 0" class="no-data">
        <p>在此时间段内没有找到SuperChat数据</p>
      </div>
      <div v-else class="sc-list">
        <div
          v-for="(sc, index) in filteredSCList"
          :key="index"
          :class="['sc-item', getSCClass(sc.price)]"
          :style="{ backgroundColor: getSCColor(sc.price) }"
        >
          <div class="sc-header">
            <span class="sc-name">{{ sc.uname }}</span>
            <span class="sc-uid">UID: {{ sc.uid }}</span>
            <span class="sc-time">{{ sc.send_time }}</span>
          </div>
          <div class="sc-content">
            <div class="sc-message-content">
              {{ sc.message || '无消息内容' }}
            </div>
            <div class="sc-price-container">
              <span class="sc-price">¥{{ formatCurrency(sc.price) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { anchorAPI } from '@/api'

export default {
  name: 'SuperChatDetail',
  setup() {
    const router = useRouter()
    const route = useRoute()
    
    const scList = ref([])
    const filteredSCList = ref([])
    const loading = ref(true)
    const error = ref(null)
    const refreshTime = ref(new Date().toLocaleString())

    const roomId = route.query.room_id
    const startTime = route.query.start_time
    const endTime = route.query.end_time
    const union = route.query.union || 'VirtuaReal'

    // 获取SC数据
    const fetchData = async () => {
      try {
        loading.value = true
        error.value = null

        console.log('正在获取SC数据，房间ID:', roomId, '工会:', union, '时间范围:', startTime, '至', endTime) // 添加调试日志

        // 检查参数是否有效
        if (!roomId) {
          throw new Error('房间ID不能为空')
        }

        const response = await anchorAPI.getSuperChatHistory(roomId, union)
        console.log('API响应:', response) // 添加调试日志

        // 检查API响应结构，后端返回的是 {room_id, month, list} 格式
        console.log('API响应类型:', typeof response, '是否为数组:', Array.isArray(response));
        console.log('API响应内容:', response);

        if (response && response.list && Array.isArray(response.list)) {
          // 如果响应是 {list: [...]} 格式
          scList.value = response.list
          console.log('使用list字段，长度:', response.list.length) // 添加调试日志
        } else if (response && Array.isArray(response)) {
          // 如果响应直接是数组
          scList.value = response
          console.log('直接使用数组响应，长度:', response.length) // 添加调试日志
        } else if (response && response.data && Array.isArray(response.data)) {
          // 如果响应是 {data: [...]} 格式
          scList.value = response.data
          console.log('使用data字段，长度:', response.data.length) // 添加调试日志
        } else if (response && response.sessions && Array.isArray(response.sessions)) {
          // 如果响应是 {sessions: [...]} 格式（兼容现有API）
          scList.value = response.sessions
          console.log('使用sessions字段，长度:', response.sessions.length) // 添加调试日志
        } else {
          // 如果都没有，使用空数组
          scList.value = []
          console.log('使用空数组，响应内容:', response) // 添加调试日志
        }

        // 确保所有SC项都有必需的字段
        scList.value = scList.value.map(sc => ({
          message: sc.message || '',
          price: sc.price || 0,
          send_time: sc.send_time || '',
          uid: sc.uid || 0,
          uname: sc.uname || '未知用户'
        }));

        console.log('处理后的SC列表:', scList.value); // 添加调试日志

        console.log('SC数据加载完成，总数:', scList.value.length) // 添加调试日志

        // 根据时间范围过滤SC数据
        filterSCByTimeRange()
      } catch (err) {
        console.error('获取SC数据失败:', err)
        error.value = `获取数据失败: ${err.message || '未知错误'}`
      } finally {
        loading.value = false
      }
    }

    // 根据时间范围过滤SC数据
    const filterSCByTimeRange = () => {
      console.log('开始时间过滤，原始数据数量:', scList.value.length, '时间范围:', startTime, '至', endTime) // 添加调试日志
      if (!startTime || !scList.value.length) {
        filteredSCList.value = scList.value
        console.log('时间过滤完成，返回全部数据，数量:', filteredSCList.value.length) // 添加调试日志
        return
      }

      // 解析开始和结束时间
      let startDateTime, endDateTime

      try {
        // 尝试解析开始时间
        if (startTime.includes(' ')) {
          startDateTime = new Date(startTime.replace(' ', 'T'))
        } else if (startTime.includes('+')) {
          startDateTime = new Date(startTime.replace('+', ' '))
        } else {
          startDateTime = new Date(startTime)
        }

        // 如果有结束时间，解析结束时间；否则使用开始时间+1小时
        if (endTime && endTime.includes(' ')) {
          endDateTime = new Date(endTime.replace(' ', 'T'))
        } else if (endTime && endTime.includes('+')) {
          endDateTime = new Date(endTime.replace('+', ' '))
        } else if (endTime) {
          endDateTime = new Date(endTime)
        } else {
          // 如果没有结束时间，使用开始时间+1小时
          endDateTime = new Date(startDateTime.getTime() + 60 * 60 * 1000)
        }

        // 添加前后15分钟的缓冲时间
        const bufferTime = 15 * 60 * 1000 // 15分钟的毫秒数
        const adjustedStart = new Date(startDateTime.getTime() - bufferTime)
        const adjustedEnd = new Date(endDateTime.getTime() + bufferTime)

        console.log('时间范围:', adjustedStart, '至', adjustedEnd) // 添加调试日志

        // 过滤SC数据
        filteredSCList.value = scList.value.filter(sc => {
          const scTimeStr = sc.send_time || ''
          if (!scTimeStr) {
            console.log('发现空时间戳的SC:', sc) // 添加调试日志
            return false
          }

          let scTime
          try {
            // 尝试解析时间字符串
            if (scTimeStr.includes(' ')) {
              scTime = new Date(scTimeStr.replace(' ', 'T'))
            } else if (scTimeStr.includes('+')) {
              scTime = new Date(scTimeStr.replace('+', ' '))
            } else {
              scTime = new Date(scTimeStr)
            }

            if (isNaN(scTime.getTime())) {
              console.warn('日期解析失败:', scTimeStr)
              // 如果解析失败，尝试其他格式
              scTime = new Date(scTimeStr)
            }
          } catch (e) {
            console.warn('无法解析时间:', scTimeStr, e)
            return false
          }

          const isInRange = scTime >= adjustedStart && scTime <= adjustedEnd
          if (isInRange) {
            console.log('时间匹配的SC:', sc.send_time, sc.message) // 添加调试日志
          }

          return isInRange
        })

        console.log('时间过滤完成，匹配数据数量:', filteredSCList.value.length) // 添加调试日志
      } catch (e) {
        console.error('时间解析错误:', e)
        // 如果解析失败，返回所有数据
        filteredSCList.value = scList.value
        console.log('时间解析错误，返回全部数据，数量:', filteredSCList.value.length) // 添加调试日志
      }
    }

    // 根据价格获取SC颜色
    const getSCColor = (price) => {
      const p = parseFloat(price) || 0
      if (p < 100) return '#3A5FCD'  // 100以下蓝色
      if (p >= 100 && p < 1000) return '#FFD700'  // 100到1000金色
      if (p >= 1000) return '#FF0000'  // 1000及以上红色
      return '#3A5FCD' // 默认颜色（蓝色）
    }

    // 根据价格获取SC类名
    const getSCClass = (price) => {
      const p = parseFloat(price) || 0
      if (p < 100) return 'sc-blue'  // 100以下蓝色
      if (p >= 100 && p < 1000) return 'sc-gold'  // 100到1000金色
      if (p >= 1000) return 'sc-red'  // 1000及以上红色
      return 'sc-default'
    }

    // 格式化货币
    const formatCurrency = (value) => {
      return parseFloat(value || 0).toFixed(2)
    }

    // 返回上一页
    const goBack = () => {
      router.go(-1)
    }

    onMounted(async () => {
      try {
        await fetchData()
      } catch (err) {
        console.error('初始化SuperChatDetail组件失败:', err)
        error.value = `页面初始化失败: ${err.message || '未知错误'}`
        loading.value = false
      }
    })

    return {
      filteredSCList,
      loading,
      error,
      refreshTime,
      roomId,
      startTime,
      endTime,
      union,
      fetchData,
      goBack,
      getSCColor,
      getSCClass,
      formatCurrency
    }
  }
}
</script>

<style scoped>
.super-chat-detail {
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  border-radius: 30px; /* 超椭圆曲线 */
  padding: 20px;
  margin: 20px 0;
  border: 1px solid rgba(255, 198, 51, 0.3);
}

.header-section {
  text-align: center;
  margin-bottom: 20px;
}

.page-title {
  color: #FFC633;
  font-size: 1.5rem;
  margin-bottom: 10px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
}

.refresh-time {
  color: #33CC99;
  font-size: 0.9rem;
  margin-bottom: 5px;
}

.queried-info {
  color: #f9729a;
  font-size: 0.9rem;
  margin-bottom: 5px;
}

.time-range {
  color: #FFC633;
  font-size: 0.9rem;
  font-weight: bold;
}

.controls-section {
  display: flex;
  justify-content: center;
  gap: 15px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.action-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 30px; /* 超椭圆曲线 */
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.action-btn.default {
  background: linear-gradient(45deg, #6c757d, #5a6268);
  color: white;
}

.action-btn.primary {
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
  font-weight: bold;
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
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

.no-data {
  text-align: center;
  padding: 40px;
  color: #33CC99;
  font-size: 1.1rem;
}

.sc-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.sc-item {
  border-radius: 20px; /* 超椭圆曲线 */
  padding: 15px;
  color: white;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
  min-height: 150px; /* 允许扩展高度 */
  width: 100%; /* 横向铺满 */
}

.sc-item:hover {
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.3);
  position: relative;
  z-index: 10;
}

.sc-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  flex-wrap: wrap;
  gap: 10px;
}

.sc-name {
  font-weight: bold;
  font-size: 1.1rem;
  color: #ffffff; /* 白色 */
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.7); /* 阴影效果 */
}

.sc-uid {
  font-size: 0.9rem;
  color: #cccccc; /* 浅灰色 */
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.7); /* 阴影效果 */
}

.sc-time {
  font-size: 0.8rem;
  color: #aaaaaa; /* 更浅的灰色 */
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.7); /* 阴影效果 */
}

.sc-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  flex-grow: 1;
  width: 100%; /* 横向铺满 */
}

.sc-message-content {
  font-size: 1.5rem; /* 放大1.5倍 */
  line-height: 1.4;
  word-wrap: break-word;
  white-space: pre-wrap;
  background: rgba(255, 255, 255, 0.1); /* 浅色背景 */
  color: #FFFFFF; /* 白色字体 */
  padding: 10px;
  border-radius: 10px;
  flex-grow: 1; /* 允许扩展 */
  min-height: 40px; /* 最小高度 */
  overflow-y: visible; /* 可用滚动，让内容自然扩展 */
  margin-right: 10px; /* 为价格留出空间 */
  text-shadow:
    -1px -1px 0 #000,
     1px -1px 0 #000,
    -1px  1px 0 #000,
     1px  1px 0 #000; /* 黑色描边效果 */
  min-width: 0; /* 允许收缩 */
}

.sc-price-container {
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  min-width: 80px; /* 为价格留出最小宽度 */
}

.sc-price {
  font-weight: bold;
  font-size: 2.8rem; /* 放大一倍（从1.4到2.8） */
  color: #FFD700; /* 默认金色 */
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.7); /* 添加阴影确保可读性 */
}

/* 金色背景时金额文字变为红色 */
.sc-gold .sc-price {
  color: #FF0000; /* 红色 */
}

/* 蓝色背景时保持金额不变 */
.sc-blue .sc-price {
  color: #FFD700; /* 保持金色 */
}

/* 不同价格区间的样式 */
.sc-blue { background-color: #3A5FCD; }
.sc-gold { background-color: #FFD700; color: white; }
.sc-red { background-color: #FF0000; color: white; }

/* 响应式设计 */
@media (max-width: 768px) {
  .super-chat-detail {
    padding: 15px;
    margin: 10px 0;
  }
  
  .page-title {
    font-size: 1.3rem;
  }
  
  .sc-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 5px;
  }
  
  .action-btn {
    width: 100%;
    max-width: 200px;
  }
  
  .sc-message {
    font-size: 0.9rem;
  }
}

/* SC按钮不使用闪烁效果，保持静态洋红色样式 */
.sc-btn {
  background: linear-gradient(45deg, #f9729a, #f75982);
  box-shadow: 0 0 10px rgba(249, 114, 154, 0.3);
  animation: none !important;
}

/* 确保SC按钮样式优先级最高，且不使用动画 */
.action-btn.sc-btn,
.sc-btn.hover-effect {
  background: linear-gradient(45deg, #f9729a, #f75982);
  box-shadow: 0 0 10px rgba(249, 114, 154, 0.3);
  animation: none !important;
}

/* 特定按钮不使用闪光效果 */
.default,
.retry-btn,
.action-btn.primary {
  animation: none;
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
</style>