<template>
  <div ref="pageRef" class="super-chat-detail">
    <GlassCard variant="strong" padding="28px">
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
        <GlassButton variant="default" size="md" @click="goBack">返回</GlassButton>
        <GlassButton variant="primary" size="md" @click="fetchData">刷新数据</GlassButton>
      </div>
    </GlassCard>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p class="loading-text">加载中...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <GlassButton variant="success" size="md" @click="fetchData">重试</GlassButton>
    </div>

    <div v-else class="sc-content">
      <div v-if="filteredSCList.length === 0" class="no-data">
        <p>在此时间段内没有找到SuperChat数据</p>
      </div>
      <div ref="scListRef" class="sc-list">
        <div
          v-for="(sc, index) in filteredSCList"
          :key="index"
          :class="['sc-item', getSCClass(sc.price)]"
        >
          <div class="sc-color-bar" :style="{ backgroundColor: getSCColor(sc.price) }"></div>
          <div class="sc-body">
            <div class="sc-header">
              <span class="sc-additional-text">https<em><strong>斜杠dc点hihivr点top</strong></em></span>
              <span class="sc-name">{{ sc.uname }}</span>
              <span class="sc-uid">UID: {{ sc.uid }}</span>
              <span class="sc-time">{{ sc.send_time }}</span>
            </div>
            <div class="sc-content-row">
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
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { anchorAPI } from '@/api'
import GlassCard from '@/components/ui/GlassCard.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import { usePageEnter } from '@/composables/usePageEnter'
import { useStaggerCards } from '@/composables/useCardEnter'

export default {
  name: 'SuperChatDetail',
  components: { GlassCard, GlassButton },
  setup() {
    const router = useRouter()
    const route = useRoute()
    const pageRef = ref(null)
    const scListRef = ref(null)

    const scList = ref([])
    const filteredSCList = ref([])
    const loading = ref(true)
    const error = ref(null)
    const refreshTime = ref(new Date().toLocaleString())

    const roomId = route.query.room_id
    const startTime = route.query.start_time
    const endTime = route.query.end_time
    const union = route.query.union || 'VirtuaReal'

    usePageEnter(pageRef)
    useStaggerCards(scListRef, '.sc-item')

    const fetchData = async () => {
      try {
        loading.value = true
        error.value = null

        if (!roomId) {
          throw new Error('房间ID不能为空')
        }

        const response = await anchorAPI.getSuperChatHistory(roomId, union)

        if (response && response.list && Array.isArray(response.list)) {
          scList.value = response.list
        } else if (response && Array.isArray(response)) {
          scList.value = response
        } else if (response && response.data && Array.isArray(response.data)) {
          scList.value = response.data
        } else if (response && response.sessions && Array.isArray(response.sessions)) {
          scList.value = response.sessions
        } else {
          scList.value = []
        }

        scList.value = scList.value.map(sc => ({
          message: sc.message || '',
          price: sc.price || 0,
          send_time: sc.send_time || '',
          uid: sc.uid || 0,
          uname: sc.uname || '未知用户'
        }))

        filterSCByTimeRange()
      } catch (err) {
        error.value = `获取数据失败: ${err.message || '未知错误'}`
      } finally {
        loading.value = false
      }
    }

    const filterSCByTimeRange = () => {
      if (!startTime || !scList.value.length) {
        filteredSCList.value = scList.value
        return
      }

      let startDateTime, endDateTime

      try {
        if (startTime.includes(' ')) {
          startDateTime = new Date(startTime.replace(' ', 'T'))
        } else if (startTime.includes('+')) {
          startDateTime = new Date(startTime.replace('+', ' '))
        } else {
          startDateTime = new Date(startTime)
        }

        if (endTime && endTime.includes(' ')) {
          endDateTime = new Date(endTime.replace(' ', 'T'))
        } else if (endTime && endTime.includes('+')) {
          endDateTime = new Date(endTime.replace('+', ' '))
        } else if (endTime) {
          endDateTime = new Date(endTime)
        } else {
          endDateTime = new Date(startDateTime.getTime() + 60 * 60 * 1000)
        }

        const bufferTime = 15 * 60 * 1000
        const adjustedStart = new Date(startDateTime.getTime() - bufferTime)
        const adjustedEnd = new Date(endDateTime.getTime() + bufferTime)

        filteredSCList.value = scList.value.filter(sc => {
          const scTimeStr = sc.send_time || ''
          if (!scTimeStr) return false

          let scTime
          try {
            if (scTimeStr.includes(' ')) {
              scTime = new Date(scTimeStr.replace(' ', 'T'))
            } else if (scTimeStr.includes('+')) {
              scTime = new Date(scTimeStr.replace('+', ' '))
            } else {
              scTime = new Date(scTimeStr)
            }
            if (isNaN(scTime.getTime())) {
              scTime = new Date(scTimeStr)
            }
          } catch (e) {
            return false
          }

          return scTime >= adjustedStart && scTime <= adjustedEnd
        })
      } catch (e) {
        filteredSCList.value = scList.value
      }
    }

    const getSCColor = (price) => {
      const p = parseFloat(price) || 0
      if (p < 10) return '#A8A8A8'
      if (p < 100) return '#3A5FCD'
      if (p < 1000) return '#F6B100'
      return '#E74C3C'
    }

    const getSCClass = (price) => {
      const p = parseFloat(price) || 0
      if (p < 10) return 'sc-silver'
      if (p < 100) return 'sc-blue'
      if (p < 1000) return 'sc-gold'
      return 'sc-red'
    }

    const formatCurrency = (value) => {
      return parseFloat(value || 0).toFixed(2)
    }

    const goBack = () => {
      router.go(-1)
    }

    onMounted(async () => {
      try {
        await fetchData()
      } catch (err) {
        error.value = `页面初始化失败: ${err.message || '未知错误'}`
        loading.value = false
      }
    })

    return {
      pageRef,
      scListRef,
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
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  width: 100%;
}

.header-section {
  text-align: center;
  margin-bottom: 16px;
}

.page-title {
  color: var(--color-text-main);
  font-size: 1.5rem;
  margin-bottom: 8px;
}

.refresh-time {
  color: var(--color-text-secondary);
  font-size: 0.9rem;
  margin-bottom: 4px;
}

.queried-info {
  color: var(--color-accent);
  font-size: 0.9rem;
  margin-bottom: 4px;
}

.time-range {
  color: var(--color-primary);
  font-size: 0.9rem;
  font-weight: bold;
}

.controls-section {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-top: 16px;
  flex-wrap: wrap;
}

.loading-state,
.error-state {
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
  border: 4px solid rgba(246, 177, 0, 0.2);
  border-top: 4px solid var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 12px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.loading-text {
  color: var(--color-text-secondary);
}

.error-message {
  color: var(--color-accent);
  font-size: 1.05rem;
  margin-bottom: 16px;
}

.no-data {
  text-align: center;
  padding: 40px;
  color: var(--color-text-secondary);
  font-size: 1.1rem;
}

.sc-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-top: 20px;
}

.sc-item {
  display: flex;
  border-radius: var(--radius-card);
  background: var(--glass-bg);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
  border: 1px solid var(--glass-border);
  box-shadow: var(--shadow-default);
  overflow: hidden;
  min-height: 120px;
  transition: box-shadow var(--duration-normal) var(--ease-out);
}

.sc-item:hover {
  box-shadow: var(--shadow-hover);
}

.sc-color-bar {
  width: 16.666%;
  min-width: 40px;
  max-width: 80px;
  min-height: 100%;
  flex-shrink: 0;
}

.sc-body {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
}

.sc-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  flex-wrap: wrap;
  gap: 8px;
}

.sc-name {
  font-weight: bold;
  font-size: 1.1rem;
  color: var(--color-text-main);
}

.sc-uid {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
}

.sc-time {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
}

.sc-additional-text {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  font-style: italic;
  font-weight: bold;
}

.sc-content-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  flex-grow: 1;
  width: 100%;
  flex-wrap: wrap;
}

.sc-message-content {
  font-size: 1.3rem;
  line-height: 1.4;
  word-wrap: break-word;
  white-space: pre-wrap;
  color: var(--color-text-main);
  flex: 1;
  min-width: 0;
  flex-basis: 60%;
  margin-right: 10px;
}

.sc-price-container {
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  min-width: 80px;
  flex-basis: 35%;
  text-align: right;
}

.sc-price {
  font-weight: bold;
  font-size: 2rem;
  color: var(--color-primary);
  word-break: break-all;
  overflow-wrap: break-word;
}

.sc-silver .sc-price {
  color: var(--color-text-secondary);
}

.sc-blue .sc-price {
  color: #3A5FCD;
}

.sc-gold .sc-price {
  color: var(--color-primary);
}

.sc-red .sc-price {
  color: #E74C3C;
}

@media (max-width: 768px) {
  .super-chat-detail {
    padding: 12px;
  }

  .page-title {
    font-size: 1.2rem;
  }

  .sc-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .sc-content-row {
    flex-direction: column;
    align-items: stretch;
  }

  .sc-message-content {
    flex-basis: auto;
    margin-right: 0;
    margin-bottom: 10px;
    font-size: 1.1rem;
  }

  .sc-price-container {
    flex-basis: auto;
    text-align: center;
  }

  .sc-price {
    font-size: 1.6rem;
  }
}

@media (max-width: 480px) {
  .sc-message-content {
    font-size: 1rem;
  }

  .sc-price {
    font-size: 1.3rem;
  }
}
</style>
