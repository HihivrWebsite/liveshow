<template>
  <div class="expanded-view">
    <div class="controls-section">
      <div class="action-controls">
        <button @click="goBack" class="action-btn secondary">
          返回列表页
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
      <div class="grid-container">
        <BaseCard
          v-for="(item, index) in anchors"
          :key="item.room_id || item.id || index"
          :card-type="sourceFromRoute === 'live-sessions' ? 'session' : 'anchor'"
          :rank="index + 1"
          :title="getTitle(item, index)"
          :subtitle="''"
          :default-collapsed="false"  <!-- 所有卡片默认展开 -->
          :fields="getFields(item)"
          :action-button="getActionButton(item)"
          :action-data="item"
          @action-click="handleActionClick(item)"
        >
          <template #actions>
            <button
              @click="handleActionClick(item)"
              :class="getActionButtonClass(item)"
            >
              {{ getActionButtonText(item) }}
            </button>
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
import { formatCurrency, formatNumber } from '@/utils/dataProcessor'

export default {
  name: 'ExpandedView',
  components: {
    BaseCard
  },
  setup() {
    const router = useRouter()
    const route = useRoute()
    const anchors = ref([])
    const title = ref('维阿PSP斗虫榜（全部展开视图）')
    const refreshTime = ref(new Date().toLocaleString())
    const loading = ref(true)
    const error = ref(null)

    // 从路由参数获取数据
    const filterFromRoute = route.query.filter || 'all'
    const monthFromRoute = route.query.month || null

    // 从路由参数获取原始页面来源
    const sourceFromRoute = route.query.source || 'anchor'

    // 从路由参数获取房间ID和工会信息（用于直播会话页面）
    const roomId = route.query.room_id
    const union = route.query.union
    const anchorName = route.query.anchor_name

    // 更新标题
    if (monthFromRoute) {
      const year = monthFromRoute.substring(0, 4)
      const month = parseInt(monthFromRoute.substring(4, 6)).toString().padStart(2, '0')
      title.value = `维阿PSP斗虫榜_${year}年${month}月记录数据（全部展开视图）`
    } else {
      title.value = filterFromRoute === 'vr' ? '维阿斗虫榜（全部展开视图）' :
                   filterFromRoute === 'psp' ? 'PSPlive斗虫榜（全部展开视图）' : '维阿PSP斗虫榜（全部展开视图）'
    }

    // 获取数据
    const fetchData = async () => {
      try {
        loading.value = true
        error.value = null
        let response;

        // 根据来源页面获取数据
        if (sourceFromRoute === 'live-sessions') {
          // 从LiveSessions页面跳转来的，需要获取直播会话数据
          if (roomId && union) {
            response = await anchorAPI.getLiveSessions(roomId, union, monthFromRoute);
            // 将会话数据转换为适合显示的格式
            anchors.value = response.sessions || response.data || [];
            title.value = `${anchorName || '主播'} 的直播会话详情（全部展开视图）`;
          } else {
            // 如果缺少必要参数，显示提示信息而不是抛出错误
            console.warn('缺少房间ID或工会信息，将显示空数据');
            anchors.value = [];
            title.value = '直播会话详情（缺少必要参数）';
            refreshTime.value = new Date().toLocaleString();
            return; // 提前返回，避免后续处理
          }
        } else {
          // 从AnchorList页面跳转来的，获取主播数据
          if (monthFromRoute) {
            response = await anchorAPI.getAnchorsByMonth(monthFromRoute, filterFromRoute);
          } else {
            response = await anchorAPI.getAnchors(filterFromRoute);
          }
          anchors.value = response.anchors || response.data || []
        }

        refreshTime.value = response.refresh_time || new Date().toLocaleString()
      } catch (err) {
        console.error('获取数据失败:', err)
        console.error('错误详情:', err.response || err.message || err)

        // 更详细的错误信息处理
        let errorMessage = '获取数据失败，请稍后重试';
        if (err.response) {
          // 服务器响应了错误状态码
          errorMessage = `服务器错误 (${err.response.status}): ${err.response.data?.message || '请求失败'}`;
        } else if (err.request) {
          // 请求已发出但没有收到响应
          errorMessage = '网络连接失败，请检查网络连接';
        } else {
          // 其他错误
          errorMessage = err.message || '发生未知错误';
        }

        error.value = errorMessage;
      } finally {
        loading.value = false
      }
    }

    // 计算总营收
    const calculateTotalRevenue = (anchor) => {
      const gift = parseFloat(anchor.gift) || 0
      const guard = parseFloat(anchor.guard) || 0
      const superChat = parseFloat(anchor.super_chat) || 0
      return gift + guard + superChat
    }

    // 查看直播会话
    const viewLiveSessions = (roomId, union) => {
      const currentMonth = route.query.month || new Date().toISOString().slice(0, 7).replace('-', '');
      router.push(`/live-sessions?room_id=${roomId}&union=${union}&month=${currentMonth}`)
    }

    // 返回上一页
    const goBack = () => {
      router.go(-1)
    }

    // 获取卡片标题 - 需要通过作用域插槽传递索引
    // 实际上在模板中使用 (item, index) 来获取索引
    const getTitle = (item, index) => {
      if (sourceFromRoute === 'live-sessions') {
        // 直播会话数据
        return item.title || `直播会话 ${index + 1}`;
      } else {
        // 主播数据
        return `${item.anchor_name || item.name} [${item.union || item.group}]`;
      }
    };

    // 获取卡片字段
    const getFields = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        // 直播会话数据字段
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
        // 主播数据字段
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

    // 获取行动按钮配置
    const getActionButton = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        return { text: '查看直播间', className: 'view-btn' };
      } else {
        return { text: '查看详细数据', className: 'view-btn' };
      }
    };

    // 获取行动按钮文本
    const getActionButtonText = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        return '查看直播间';
      } else {
        return '查看详细数据';
      }
    };

    // 获取行动按钮类名
    const getActionButtonClass = (item) => {
      return sourceFromRoute === 'live-sessions' ? 'live-room-btn' : 'view-btn';
    };

    // 处理行动点击
    const handleActionClick = (item) => {
      if (sourceFromRoute === 'live-sessions') {
        // 如果是直播会话，跳转到直播间
        if (item.room_id) {
          window.open(`https://live.bilibili.com/${item.room_id}`, '_blank');
        }
      } else {
        // 如果是主播数据，查看详细直播数据
        viewLiveSessions(item.room_id, item.union);
      }
    };

    onMounted(() => {
      fetchData()
    })

    return {
      anchors,
      title,
      refreshTime,
      loading,
      error,
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

.action-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.3s ease;
  min-width: 100px;
  text-align: center;
}

.action-btn.secondary {
  background: #f0f0f0;
  color: #333;
}

.action-btn.secondary:hover {
  background: #e0e0e0;
}

.info-section {
  text-align: center;
  margin-bottom: 20px;
}

.page-title {
  color: #333;
  margin-bottom: 10px;
}

.refresh-time {
  color: #666;
  font-size: 14px;
}

.loading-state, .error-state {
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
  border: 4px solid #f3f3f3;
  border-top: 4px solid #ff6b6b;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 10px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.retry-btn {
  padding: 8px 16px;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  margin-top: 10px;
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
}
</style>