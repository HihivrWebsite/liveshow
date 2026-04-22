<template>
  <div class="cache-stats-container">
    <div class="cache-stats-panel">
      <h3>缓存统计信息</h3>
      
      <!-- 统计指标卡片 -->
      <div class="stats-grid">
        <!-- 缓存命中率 -->
        <div class="stat-card hit-rate">
          <div class="stat-label">缓存命中率</div>
          <div class="stat-value">
            <span v-if="stats.hit_rate_percentage" class="value-text">{{ stats.hit_rate_percentage }}</span>
            <span v-else class="value-na">N/A</span>
          </div>
          <div class="stat-detail">
            {{ stats.hits }} 命中 / {{ stats.misses }} 未命中
          </div>
        </div>

        <!-- 缓存大小 -->
        <div class="stat-card cache-size">
          <div class="stat-label">缓存大小</div>
          <div class="stat-value">
            <span v-if="stats.current_size_mb" class="value-text">{{ stats.current_size_mb }}</span>
            <span v-else class="value-na">N/A</span>
          </div>
          <div class="stat-detail">
            最大: {{ stats.max_size_mb || 'N/A' }}
          </div>
        </div>

        <!-- 缓存条目数 -->
        <div class="stat-card entry-count">
          <div class="stat-label">缓存条目</div>
          <div class="stat-value">
            <span v-if="stats.entry_count !== undefined" class="value-text">{{ stats.entry_count }}</span>
            <span v-else class="value-na">N/A</span>
          </div>
          <div class="stat-detail">
            内存条目: {{ stats.entry_count || 0 }}
          </div>
        </div>

        <!-- Attention缓存 -->
        <div class="stat-card attention-cache">
          <div class="stat-label">Attention缓存</div>
          <div class="stat-value">
            <span v-if="stats.attention_entries !== undefined" class="value-text">{{ stats.attention_entries }}</span>
            <span v-else class="value-na">N/A</span>
          </div>
          <div class="stat-detail">
            关注数缓存条目
          </div>
        </div>

        <!-- LiveSessions缓存 -->
        <div class="stat-card live-sessions-cache">
          <div class="stat-label">LiveSessions缓存</div>
          <div class="stat-value">
            <span v-if="stats.live_sessions_entries !== undefined" class="value-text">{{ stats.live_sessions_entries }}</span>
            <span v-else class="value-na">N/A</span>
          </div>
          <div class="stat-detail">
            直播会话缓存条目
          </div>
        </div>
      </div>

      <!-- 刷新按钮 -->
      <div class="action-buttons">
        <button @click="refreshStats" :disabled="isLoading" class="btn-refresh">
          <span v-if="!isLoading">🔄 刷新统计</span>
          <span v-else>加载中...</span>
        </button>
        <div class="last-updated" v-if="lastUpdated">
          最后更新: {{ lastUpdated }}
        </div>
      </div>

      <!-- 详细信息 -->
      <div class="detailed-info" v-if="stats.hit_rate_percentage">
        <h4>详细信息</h4>
        <div class="info-row">
          <span class="info-label">总请求数:</span>
          <span class="info-value">{{ (stats.hits + stats.misses) || 0 }}</span>
        </div>
        <div class="info-row">
          <span class="info-label">使用率:</span>
          <span class="info-value">{{ calcUsagePercent() }}%</span>
        </div>
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: calcUsagePercent() + '%' }"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, computed } from 'vue'
import axios from 'axios'

export default {
  name: 'CacheStats',
  setup() {
    const stats = ref({
      hit_rate_percentage: 'N/A',
      current_size_mb: '0 MB',
      entry_count: 0,
      hits: 0,
      misses: 0,
      max_size_mb: '5000.00 MB',
      current_size: 0,
      max_size: 0,
      attention_entries: 0,
      live_sessions_entries: 0
    })
    
    const isLoading = ref(false)
    const lastUpdated = ref(null)

    const refreshStats = async () => {
      isLoading.value = true
      try {
        const response = await axios.get('/cache/stats')
        stats.value = response.data
        lastUpdated.value = new Date().toLocaleTimeString('zh-CN')
        console.log('缓存统计已更新:', stats.value)
      } catch (error) {
        console.error('获取缓存统计失败:', error)
        stats.value = {
          hit_rate_percentage: 'N/A',
          current_size_mb: '0 Bytes',
          entry_count: 0
        }
      } finally {
        isLoading.value = false
      }
    }

    const calcUsagePercent = () => {
      if (stats.value.max_size === 0) return 0
      return Math.round((stats.value.current_size / stats.value.max_size) * 100)
    }

    onMounted(() => {
      refreshStats()
      // 每10秒自动刷新一次
      setInterval(refreshStats, 10000)
    })

    return {
      stats,
      isLoading,
      lastUpdated,
      refreshStats,
      calcUsagePercent
    }
  }
}
</script>

<style scoped>
.cache-stats-container {
  width: 100%;
  padding: 20px;
}

.cache-stats-panel {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  color: white;
}

.cache-stats-panel h3 {
  margin-bottom: 20px;
  font-size: 20px;
  font-weight: 600;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
  margin-bottom: 20px;
}

.stat-card {
  background: rgba(255, 255, 255, 0.15);
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 8px;
  padding: 15px;
  backdrop-filter: blur(10px);
  transition: all 0.3s ease;
}

.stat-card:hover {
  background: rgba(255, 255, 255, 0.25);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.stat-label {
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 1px;
  opacity: 0.8;
  margin-bottom: 8px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  margin-bottom: 8px;
  min-height: 35px;
}

.value-text {
  color: #fff;
}

.value-na {
  color: #ffeb3b;
  font-size: 18px;
}

.stat-detail {
  font-size: 12px;
  opacity: 0.7;
}

.action-buttons {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  flex-wrap: wrap;
  gap: 10px;
}

.btn-refresh {
  background: rgba(255, 255, 255, 0.25);
  border: 1px solid rgba(255, 255, 255, 0.5);
  color: white;
  padding: 10px 20px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.3s ease;
}

.btn-refresh:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.35);
  transform: scale(1.05);
}

.btn-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.last-updated {
  font-size: 12px;
  opacity: 0.7;
}

.detailed-info {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  padding: 15px;
  margin-top: 15px;
}

.detailed-info h4 {
  margin-bottom: 12px;
  font-size: 14px;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 13px;
}

.info-label {
  opacity: 0.8;
}

.info-value {
  font-weight: 600;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  overflow: hidden;
  margin-top: 10px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #4CAF50, #8BC34A);
  border-radius: 4px;
  transition: width 0.3s ease;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .cache-stats-panel {
    padding: 15px;
  }

  .stats-grid {
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 10px;
  }

  .stat-value {
    font-size: 24px;
  }

  .action-buttons {
    flex-direction: column;
    align-items: stretch;
  }

  .btn-refresh {
    width: 100%;
  }
}
</style>
