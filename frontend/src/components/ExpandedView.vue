<template>
  <div ref="pageRef" class="expanded-view">
    <div class="controls-section">
      <div class="action-controls">
        <GlassButton variant="default" size="md" @click="goBack">
          返回列表页
        </GlassButton>
      </div>
    </div>

    <div class="info-section">
      <h2 class="page-title">{{ title }}</h2>
      <p class="refresh-time">刷新时间：{{ refreshTime }}</p>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p class="loading-text">加载中...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <GlassButton variant="success" size="md" @click="fetchData">重试</GlassButton>
    </div>

    <div v-else class="data-section">
      <div ref="gridRef" class="grid-container">
        <BaseCard
          v-for="(item, index) in anchors"
          :key="item.room_id || item.id || index"
          :card-type="sourceFromRoute === 'live-sessions' ? 'session' : 'anchor'"
          :rank="index + 1"
          :title="getTitle(item, index)"
          :subtitle="''"
          :default-collapsed="false"
          :fields="getFields(item)"
          :action-button="getActionButton(item)"
          :action-data="item"
          @action-click="handleActionClick(item)"
        >
          <template #actions>
            <GlassButton
              :variant="sourceFromRoute === 'live-sessions' ? 'info' : 'primary'"
              size="sm"
              @click="handleActionClick(item)"
            >
              {{ getActionButtonText(item) }}
            </GlassButton>
          </template>
        </BaseCard>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { anchorAPI } from '@/api'
import BaseCard from '@/components/BaseCard.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import { formatCurrency, formatNumber } from '@/utils/dataProcessor'
import { usePageEnter } from '@/composables/usePageEnter'
import { useStaggerCards } from '@/composables/useCardEnter'

export default {
  name: 'ExpandedView',
  components: {
    BaseCard,
    GlassButton
  },
  setup() {
    const router = useRouter()
    const route = useRoute()
    const pageRef = ref(null)
    const gridRef = ref(null)
    const anchors = ref([])
    const title = ref('维阿PSP斗虫榜（全部展开视图）')
    const refreshTime = ref(new Date().toLocaleString())
    const loading = ref(true)
    const error = ref(null)

    const filterFromRoute = route.query.filter || 'all'
    const monthFromRoute = route.query.month || null
    const sourceFromRoute = route.query.source || 'anchor'
    const roomId = route.query.room_id
    const union = route.query.union
    const anchorName = route.query.anchor_name

    usePageEnter(pageRef)
    useStaggerCards(gridRef, '.base-card')

    if (monthFromRoute) {
      const year = monthFromRoute.substring(0, 4)
      const month = parseInt(monthFromRoute.substring(4, 6)).toString().padStart(2, '0')
      title.value = `维阿PSP斗虫榜_${year}年${month}月记录数据（全部展开视图）`
    } else {
      title.value = filterFromRoute === 'vr' ? '维阿斗虫榜（全部展开视图）' :
                   filterFromRoute === 'psp' ? 'PSPlive斗虫榜（全部展开视图）' : '维阿PSP斗虫榜（全部展开视图）'
    }

    const fetchData = async () => {
      try {
        loading.value = true
        error.value = null
        let response;

        if (sourceFromRoute === 'live-sessions') {
          if (roomId && union) {
            response = await anchorAPI.getLiveSessions(roomId, union, monthFromRoute);
            anchors.value = response.sessions || response.data || [];
            title.value = `${anchorName || '主播'} 的直播会话详情（全部展开视图）`;
          } else {
            anchors.value = [];
            title.value = '直播会话详情（缺少必要参数）';
            refreshTime.value = new Date().toLocaleString();
            return;
          }
        } else {
          if (monthFromRoute) {
            response = await anchorAPI.getAnchorsByMonth(monthFromRoute, filterFromRoute);
          } else {
            response = await anchorAPI.getAnchors(filterFromRoute);
          }
          anchors.value = response.anchors || response.data || []
        }

        refreshTime.value = response.refresh_time || new Date().toLocaleString()
      } catch (err) {
        let errorMessage = '获取数据失败，请稍后重试';
        if (err.response) {
          errorMessage = `服务器错误 (${err.response.status}): ${err.response.data?.message || '请求失败'}`;
        } else if (err.request) {
          errorMessage = '网络连接失败，请检查网络连接';
        } else {
          errorMessage = err.message || '发生未知错误';
        }
        error.value = errorMessage;
      } finally {
        loading.value = false
      }
    }

    const calculateTotalRevenue = (anchor) => {
      const gift = parseFloat(anchor.gift) || 0
      const guard = parseFloat(anchor.guard) || 0
      const superChat = parseFloat(anchor.super_chat) || 0
      return gift + guard + superChat
    }

    const viewLiveSessions = (roomId, union) => {
      const currentMonth = route.query.month || new Date().toISOString().slice(0, 7).replace('-', '');
      router.push(`/live-sessions?room_id=${roomId}&union=${union}&month=${currentMonth}`)
    }

    const goBack = () => {
      router.go(-1)
    }

    const getTitle = (item, index) => {
      if (sourceFromRoute === 'live-sessions') {
        return item.title || `直播会话 ${index + 1}`;
      } else {
        return `${item.anchor_name || item.name} [${item.union || item.group}]`;
      }
    };

    const getFields = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        return [
          { label: '标题', value: item.title || '无标题' },
          { label: '开始时间', value: item.start_time },
          { label: '结束时间', value: item.end_time || '未结束' },
          { label: '开播时长', value: item.duration || 'N/A' },
          { label: '舰长数量', value: item.guard_num || 0 },
          { label: 'SC数量', value: item.sc_num || 0 },
          { label: '弹幕数', value: formatNumber(item.danmaku_count || 0), type: 'number' },
          { label: '礼物收入', value: formatCurrency(item.gift_income || 0), type: 'currency' },
          { label: '舰长收入', value: formatCurrency(item.guard_income || 0), type: 'currency' },
          { label: 'SC收入', value: formatCurrency(item.sc_income || 0), type: 'currency' },
          { label: '总营收', value: formatCurrency(parseFloat(item.gift_income || 0) + parseFloat(item.guard_income || 0) + parseFloat(item.sc_income || 0)), type: 'currency' }
        ];
      } else {
        return [
          { label: '关注数', value: formatNumber(item.attention || 0), type: 'number' },
          { label: '有效天', value: item.effective_days || 0 },
          { label: '开播时长', value: item.live_duration || '0小时0分钟', type: 'duration' },
          { label: '开播状态', value: item.status === 1 ? '正在直播' : '未开播' },
          { label: '总督', value: item.guard_3 || 0 },
          { label: '提督', value: item.guard_2 || 0 },
          { label: '舰长', value: item.guard_1 || 0 },
          { label: '粉丝团', value: formatNumber(item.fans_count || 0), type: 'number' },
          { label: '礼物收入', value: formatCurrency(item.gift || 0), type: 'currency' },
          { label: '舰长收入', value: formatCurrency(item.guard || 0), type: 'currency' },
          { label: 'SC收入', value: formatCurrency(item.super_chat || 0), type: 'currency' },
          { label: '总营收', value: formatCurrency(calculateTotalRevenue(item)), type: 'currency' }
        ];
      }
    };

    const getActionButton = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        return { text: '查看直播间', className: 'view-btn' };
      } else {
        return { text: '查看详细数据', className: 'view-btn' };
      }
    };

    const getActionButtonText = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        return '查看直播间';
      } else {
        return '查看详细数据';
      }
    };

    const getActionButtonClass = (item) => {
      return sourceFromRoute === 'live-sessions' ? 'live-room-btn' : 'view-btn';
    };

    const handleActionClick = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        if (item.room_id) {
          window.open(`https://live.bilibili.com/${item.room_id}`, '_blank');
        }
      } else {
        viewLiveSessions(item.room_id, item.union);
      }
    };

    onMounted(() => {
      fetchData()
    })

    return {
      pageRef,
      gridRef,
      anchors,
      title,
      refreshTime,
      loading,
      error,
      sourceFromRoute,
      calculateTotalRevenue,
      formatCurrency,
      formatNumber,
      viewLiveSessions,
      goBack,
      getTitle,
      getFields,
      getActionButton,
      getActionButtonText,
      getActionButtonClass,
      handleActionClick
    }
  }
}
</script>

<style scoped>
.expanded-view {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.controls-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  flex-wrap: wrap;
  gap: 10px;
}

.action-controls {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.info-section {
  text-align: center;
  margin-bottom: 20px;
}

.page-title {
  color: var(--color-text-main);
  margin-bottom: 8px;
  font-size: 1.4rem;
}

.refresh-time {
  color: var(--color-text-secondary);
  font-size: 0.9rem;
}

.loading-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  text-align: center;
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

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  width: 100%;
}

@media (max-width: 768px) {
  .grid-container {
    grid-template-columns: 1fr;
  }

  .controls-section {
    flex-direction: column;
    align-items: stretch;
  }

  .action-controls {
    justify-content: center;
  }

  .page-title {
    font-size: 1.2rem;
  }
}
</style>
