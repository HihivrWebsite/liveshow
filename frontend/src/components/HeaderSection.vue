<template>
  <header class="header">
    <div class="header-content">
      <div class="logo-section">
        <div class="logo-images">
          <a href="https://smms.app/image/BvHWJNoL2iscPXl" target="_blank">
            <img
              src="https://s2.loli.net/2024/11/16/BvHWJNoL2iscPXl.png"
              alt="h222.png"
              class="logo-img"
            >
          </a>
          <a href="https://smms.app/image/Q2i7IUqjMDb3pev" target="_blank">
            <img
              src="https://s2.loli.net/2024/11/16/Q2i7IUqjMDb3pev.png"
              alt="hihi.png"
              class="logo-img"
            >
          </a>
        </div>
        <h1 class="site-title">{{ title }}</h1>
      </div>

      <div class="cache-stats-grid">
        <div class="cache-stat-item">
          <span class="stat-label">缓存命中率</span>
          <span class="stat-value">{{ cacheHitRateDisplay }}</span>
        </div>
        <div class="cache-stat-item">
          <span class="stat-label">缓存占用</span>
          <span class="stat-value">{{ cacheSizeDisplay }}</span>
          <span class="stat-subvalue">最大容量 {{ cacheMaxSizeDisplay }}</span>
        </div>
        <div class="cache-stat-item">
          <span class="stat-label">缓存总条目</span>
          <span class="stat-value">{{ cacheEntryCount }}</span>
        </div>
        <div class="cache-stat-item">
          <span class="stat-label">Attention缓存条目</span>
          <span class="stat-value">{{ attentionEntries }}</span>
        </div>
        <div class="cache-stat-item">
          <span class="stat-label">LiveSessions缓存条目</span>
          <span class="stat-value">{{ liveSessionsEntries }}</span>
        </div>
        <div class="cache-stat-item cache-status-card">
          <span class="stat-label">缓存策略</span>
          <span class="stat-value">{{ cacheStatus }}</span>
        </div>
      </div>

      <div class="header-actions">
        <div class="main-site-group">
          <button @click="goToMainSite" class="action-btn primary">
            进入向阳Hihi粉丝站主站<br>为成为向阳人而骄傲
          </button>
          <button @click="showGuide = true" class="action-btn guide-btn">
            📖 功能指南
          </button>
        </div>

        <div class="creator-info">
          <p class="creator-text-large">
            特别感谢某热心小礼猫-千秋紫莹提供的斗虫数据API，感谢其对本项目提供了巨大的帮助
          </p>
          <button @click="followCreator" class="action-btn secondary-glowing">
            关注花礼harei喵，关注花礼harei谢谢喵
          </button>
        </div>
      </div>
    </div>
  </header>

  <div v-if="showGuide" class="guide-overlay" @click="showGuide = false">
    <div class="guide-modal" @click.stop>
      <div class="guide-header">
        <h2>📖 功能指南</h2>
        <button @click="showGuide = false" class="guide-close">✕</button>
      </div>
      <div class="guide-content">
        <p class="guide-welcome">欢迎使用维阿PSP斗虫榜！以下是网站所有功能的介绍和使用方法。</p>

        <h3>一、数据筛选</h3>
        <div class="guide-item">
          <h4>1. 工会筛选</h4>
          <p><b>位置：</b>页面顶部三个按钮</p>
          <p><b>用途：</b>切换查看维阿PSP全部主播、仅维阿（VirtuaReal）主播、或仅PSPlive主播的数据</p>
          <p><b>使用：</b>点击对应按钮即可切换</p>
        </div>
        <div class="guide-item">
          <h4>2. 切换不同月份</h4>
          <p><b>位置：</b>页面顶部操作栏</p>
          <p><b>用途：</b>查看历史月份的主播数据</p>
          <p><b>使用：</b>点击按钮 → 选择月份 → 确定</p>
        </div>
        <div class="guide-item">
          <h4>3. 多月份共同统计</h4>
          <p><b>位置：</b>页面顶部操作栏</p>
          <p><b>用途：</b>合并多个月份的数据，查看累计排名</p>
          <p><b>使用：</b>点击按钮 → 选择起始和结束月份 → 确定</p>
        </div>

        <h3>二、数据可视化</h3>
        <div class="guide-item">
          <h4>4. 查看营收占比</h4>
          <p><b>位置：</b>页面顶部操作栏</p>
          <p><b>用途：</b>以饼图展示各主播的收入占比，扇形内显示主播头像</p>
          <p><b>使用：</b>点击按钮 → 自动弹出饼图</p>
        </div>
        <div class="guide-item">
          <h4>5. VR PSP对比图</h4>
          <p><b>位置：</b>页面顶部操作栏</p>
          <p><b>用途：</b>对比维阿和PSPlive两个工会的总收入占比</p>
          <p><b>使用：</b>需在"维阿PSP斗虫榜"模式下，点击按钮 → 自动弹出对比饼图</p>
        </div>
        <div class="guide-item">
          <h4>6. 进行回归分析</h4>
          <p><b>位置：</b>页面顶部操作栏</p>
          <p><b>用途：</b>对主播数据进行回归分析，查看变量之间的关系</p>
          <p><b>使用：</b>点击按钮 → 选择变量 → 生成分析结果</p>
        </div>

        <h3>三、导航表格</h3>
        <div class="guide-item">
          <h4>7. 快速导航</h4>
          <p><b>位置：</b>页面中部导航表格</p>
          <p><b>用途：</b>快速查看主播排名、状态、营收，点击跳转到对应卡片</p>
          <p><b>使用：</b>点击"跳转"按钮滚动到对应主播卡片</p>
        </div>
        <div class="guide-item">
          <h4>8. 多选功能</h4>
          <p><b>位置：</b>导航表格左侧复选框</p>
          <p><b>用途：</b>选择多个主播进行对比分析或导出</p>
          <p><b>使用：</b>勾选复选框，表头可全选/全不选</p>
        </div>
        <div class="guide-item">
          <h4>9. 恶意斗虫</h4>
          <p><b>位置：</b>导航表格上方按钮区</p>
          <p><b>用途：</b>多主播数据对比分析，支持13种指标折线图</p>
          <p><b>使用：</b>选择2个以上主播 → 点击按钮 → 选择日期范围 → 选择指标 → 生成图表</p>
        </div>
        <div class="guide-item">
          <h4>10. 排名对比</h4>
          <p><b>位置：</b>导航表格上方按钮区</p>
          <p><b>用途：</b>查看各主播在指定指标上的排名变化趋势</p>
          <p><b>使用：</b>选择2个以上主播 → 点击按钮 → 选择日期范围 → 选择指标 → 生成排名折线图</p>
        </div>
        <div class="guide-item guide-highlight">
          <h4>⭐ 11. 导出截图</h4>
          <p><b>位置：</b>导航表格上方按钮区</p>
          <p><b>用途：</b>将选中主播的数据导出为图片，包含网站信息和数据表格</p>
          <p><b>使用：</b>选择主播 → 点击按钮 → 选择时间范围 → 确定导出 → 自动下载PNG</p>
        </div>

        <h3>四、主播卡片</h3>
        <div class="guide-item">
          <h4>12. 查看详细数据</h4>
          <p><b>位置：</b>每个主播卡片底部</p>
          <p><b>用途：</b>进入该主播的直播会话详情页</p>
          <p><b>使用：</b>点击"查看详情"按钮</p>
        </div>

        <h3>五、详情页功能</h3>
        <div class="guide-item">
          <h4>13. 直播会话详情</h4>
          <p><b>位置：</b>详情页</p>
          <p><b>用途：</b>查看主播每场直播的详细数据（时长、收入、弹幕等）</p>
          <p><b>进入：</b>从主页点击主播卡片的"查看详情"</p>
        </div>
        <div class="guide-item">
          <h4>14. 计算新增粉丝数</h4>
          <p><b>位置：</b>详情页操作栏</p>
          <p><b>用途：</b>计算每场直播的新增关注数</p>
          <p><b>使用：</b>点击按钮 → 自动计算并显示</p>
        </div>
        <div class="guide-item">
          <h4>15. 显示直播数据折线图</h4>
          <p><b>位置：</b>详情页操作栏</p>
          <p><b>用途：</b>以折线图展示直播数据趋势</p>
          <p><b>使用：</b>点击按钮 → 自动弹出图表</p>
        </div>
        <div class="guide-item guide-highlight">
          <h4>⭐ 16. Super Chat历史</h4>
          <p><b>位置：</b>详情页操作栏</p>
          <p><b>用途：</b>查看主播的Super Chat（醒目留言）记录</p>
          <p><b>使用：</b>点击按钮 → 进入SC历史页面</p>
        </div>
        <div class="guide-item">
          <h4>17. 跳转到直播间</h4>
          <p><b>位置：</b>详情页操作栏</p>
          <p><b>用途：</b>直接跳转到主播的Bilibili直播间</p>
          <p><b>使用：</b>点击按钮 → 新标签页打开直播间</p>
        </div>

        <h3>六、其他功能</h3>
        <div class="guide-item">
          <h4>18. 主播头像</h4>
          <p><b>位置：</b>主播卡片、导航表格、详情页</p>
          <p><b>用途：</b>显示主播的Bilibili头像，帮助快速识别</p>
        </div>
        <div class="guide-item">
          <h4>19. 缓存统计</h4>
          <p><b>位置：</b>页面顶部信息区</p>
          <p><b>用途：</b>显示后端缓存的命中率和占用情况</p>
        </div>

        <p class="guide-tip">提示：所有月份选择器最早只能选择到2025年8月。</p>
      </div>
    <div class="guide-footer">
      <button @click="showGuide = false" class="guide-ok-btn">我知道了</button>
    </div>
    </div>
  </div>
</template>

<script>
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { anchorAPI } from '@/api'

export default {
  name: 'HeaderSection',
  setup() {
    const route = useRoute()

    // 缓存统计信息
    const cacheHitRate = ref('N/A')
    const cacheSize = ref('N/A')
    const cacheEntryCount = ref('N/A')
    const cacheMaxSize = ref('N/A')
    const attentionEntries = ref('N/A')
    const liveSessionsEntries = ref('N/A')
    const cacheStatus = ref('加载中...')

    const showGuide = ref(false)

    const isMainPage = computed(() => {
      return route.path === '/' || route.path === '/by-month'
    })

    const title = computed(() => {
      const filter = route.query.filter || 'all'
      const month = route.query.month

      if (month) {
        const year = month.substring(0, 4)
        const monthNum = parseInt(month.substring(4, 6)).toString().padStart(2, '0')
        return `维阿PSP斗虫榜_${year}年${monthNum}月记录数据（点击"正在直播"跳转到对应直播间）_https:斜杠hihivr点top`
      } else {
        return (filter === 'vr' ? '维阿斗虫榜' :
               filter === 'psp' ? 'PSPlive斗虫榜' : '维阿PSP斗虫榜') + '_https:斜杠hihivr点top'
      }
    })

    // 获取缓存统计信息
    const fetchCacheStats = async () => {
      try {
        const response = await anchorAPI.getCacheStats()
        const stats = response

        // 更新缓存统计信息
        cacheHitRate.value = typeof stats.hit_rate === 'number' ? (stats.hit_rate * 100).toFixed(2) + '%' : 'N/A'
        cacheSize.value = stats.current_size_mb || '0 MB'
        cacheMaxSize.value = stats.max_size_mb || '0 MB'
        cacheEntryCount.value = stats.entry_count !== undefined ? String(stats.entry_count) : 'N/A'
        attentionEntries.value = stats.attention_entries !== undefined ? String(stats.attention_entries) : 'N/A'
        liveSessionsEntries.value = stats.live_sessions_entries !== undefined ? String(stats.live_sessions_entries) : 'N/A'
        cacheStatus.value = '历史月永久缓存，当前月每日1:30刷新'
      } catch (error) {
        console.error('获取缓存统计信息失败:', error)
        cacheStatus.value = '缓存统计获取失败'
      }
    }

    // 每30秒更新一次缓存统计信息
    let cacheStatsInterval
    onMounted(() => {
      fetchCacheStats()
      cacheStatsInterval = setInterval(fetchCacheStats, 30000)

      // 每次打开主页时自动弹出功能指南
      if (isMainPage.value) {
        showGuide.value = true
      }
    })

    // 组件卸载时清除定时器
    onUnmounted(() => {
      if (cacheStatsInterval) {
        clearInterval(cacheStatsInterval)
      }
    })

    const goToMainSite = () => {
      window.open('https://hihivr.top', '_blank')
    }

    const followCreator = () => {
      window.open('https://space.bilibili.com/1048135385', '_blank')
    }

    return {
      title,
      goToMainSite,
      followCreator,
      cacheHitRateDisplay: cacheHitRate,
      cacheSizeDisplay: cacheSize,
      cacheEntryCount,
      cacheMaxSizeDisplay: cacheMaxSize,
      attentionEntries,
      liveSessionsEntries,
      cacheStatus,
      showGuide,
      isMainPage,
    }
  }
}
</script>

<style scoped>
.header {
  background: #FFF8E1;
  padding: 20px 0;
  position: static; /* 不再固定在顶部 */
  z-index: 1000;
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 20px;
}

.logo-section {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 20px;
  margin-bottom: 15px;
  flex-wrap: wrap;
}

.logo-images {
  display: flex;
  gap: 10px;
  align-items: center;
}

.logo-img {
  height: 120px;
  transition: transform 0.3s ease;
}

.logo-img:hover {
  transform: scale(1.05);
}

.site-title {
  color: #FF6600;
  font-size: 2.5rem;
  font-weight: bold;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
  margin: 0;
  text-align: center;
}

.header-actions {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
}

.creator-info {
  margin-top: 15px;
  text-align: center;
}

.creator-text {
  color: #f9729a;
  font-size: 1.2rem;
  margin-bottom: 10px;
  line-height: 1.4;
  font-weight: bold;
}

.creator-text-large {
  color: #f9729a;
  font-size: 1.2rem; /* 缩小一倍 */
  margin-bottom: 10px;
  line-height: 1.4;
  font-weight: bold;
}

.cache-stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 14px;
  justify-items: center;
  align-items: stretch;
  margin: 15px 0;
}

.cache-stat-item {
  background: #FFF4DD;
  border: 1px solid rgba(255, 165, 0, 0.25);
  border-radius: 18px;
  padding: 18px 16px;
  min-width: 180px;
  width: 100%;
  max-width: 280px;
  box-shadow: 0 10px 25px rgba(255, 165, 0, 0.12);
  text-align: center;
}

.cache-stat-item.cache-status-card {
  min-width: 240px;
}

.stat-label {
  font-size: 0.95rem;
  margin-bottom: 10px;
  color: #555;
}

.stat-subvalue {
  display: block;
  margin-top: 10px;
  font-size: 0.8rem;
  color: #777;
}

.stat-value {
  font-size: 1.1rem;
  font-weight: bold;
  color: #FF7A00;
}

/* Cache statistics orange styling for all screen sizes */
.stat-orange {
  color: #FFA500 !important; /* 橙色文字 */
  font-weight: bold;
}

.stat-orange-full {
  color: #FFA500 !important; /* 橙色文字 */
  font-weight: bold;
}

.action-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-block;
  margin: 0 5px;
  font-weight: bold;
  min-width: 120px; /* 最小宽度确保圆形效果 */
}

.action-btn.primary {
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
  font-weight: bold;
  font-size: 1.8rem; /* 放大一倍 */
  padding: 20px 40px; /* 放大内边距 */
  animation: shine 3s infinite; /* 添加闪光动画 */
}

.action-btn.secondary {
  background: linear-gradient(45deg, #f9729a, #f75982); /* 改为新颜色 */
  color: white;
}

.action-btn.secondary-glowing {
  background: linear-gradient(45deg, #f9729a, #f75982); /* 改为新颜色 */
  color: white;
  font-size: 0.9rem; /* 缩小0.5倍 */
  padding: 10px 20px; /* 缩小内边距 */
  animation: secondary-shine 2s infinite; /* 添加闪光动画 */
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

/* 第二个按钮的特殊闪光动画 */
@keyframes secondary-shine {
  0% {
    background-position: 0% 50%;
    box-shadow: 0 0 15px #f9729a;
  }
  50% {
    background-position: 100% 50%;
    box-shadow: 0 0 25px #f75982;
  }
  100% {
    background-position: 0% 50%;
    box-shadow: 0 0 15px #f9729a;
  }
}

/* 闪光动画 */
@keyframes shine {
  0% {
    background-position: 0% 50%;
    box-shadow: 0 0 15px #FFC633;
  }
  50% {
    background-position: 100% 50%;
    box-shadow: 0 0 25px #FFA500;
  }
  100% {
    background-position: 0% 50%;
    box-shadow: 0 0 15px #FFC633;
  }
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .header-content {
    padding: 0 15px;
  }

  .logo-section {
    gap: 15px;
  }

  .logo-img {
    height: 100px;
  }

  .site-title {
    font-size: 2rem;
  }

  .action-btn.primary {
    font-size: 1.5rem;
    padding: 15px 30px;
  }

  .creator-text-large {
    font-size: 1rem;
  }
}

@media (max-width: 768px) {
  .header-content {
    padding: 0 10px;
  }

  .logo-section {
    flex-direction: column;
    gap: 10px;
    align-items: center;
  }

  .logo-images {
    flex-direction: column;
    align-items: center;
  }

  .logo-img {
    height: 80px;
    max-width: 100%;
  }

  .site-title {
    font-size: 1.4rem;
    text-align: center;
    padding: 0 10px;
  }

  .header-actions {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 15px;
  }

  .action-btn {
    padding: 8px 16px;
    font-size: 0.8rem;
    width: 100%;
    max-width: 300px;
    margin: 0 auto 5px auto;
  }

  .action-btn.primary {
    font-size: 1.2rem;
    padding: 12px 24px;
  }

  .creator-info {
    text-align: center;
  }

  .creator-text-large {
    font-size: 0.9rem;
    margin-bottom: 10px;
  }
}

@media (max-width: 480px) {
  .logo-img {
    height: 60px;
  }

  .site-title {
    font-size: 1.2rem;
    padding: 0 5px;
  }

  .action-btn {
    padding: 6px 12px;
    font-size: 0.75rem;
    margin: 5px auto;
    display: block;
    width: 100%;
    max-width: 280px;
  }

  .action-btn.primary {
    font-size: 1rem;
    padding: 10px 20px;
  }

  .creator-text-large {
    font-size: 0.8rem;
  }
}

@media (max-width: 360px) {
  .logo-img {
    height: 50px;
  }

  .site-title {
    font-size: 1.0rem;
  }

  .action-btn.primary {
    font-size: 0.9rem;
    padding: 8px 16px;
  }

  .creator-text-large {
    font-size: 0.75rem;
  }
}

.main-site-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.guide-btn {
  background: linear-gradient(45deg, #4ECDC4, #45B7D1);
  color: white;
  font-weight: bold;
  font-size: 1.2rem;
  padding: 12px 30px;
  animation: guide-glow 2s ease-in-out infinite;
}

@keyframes guide-glow {
  0%, 100% { box-shadow: 0 0 5px rgba(78, 205, 196, 0.5); }
  50% { box-shadow: 0 0 20px rgba(78, 205, 196, 0.8), 0 0 40px rgba(78, 205, 196, 0.4); }
}

.guide-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.guide-modal {
  background: #FFF8E1;
  border: 2px solid #FFC633;
  border-radius: 20px;
  width: 90%;
  max-width: 700px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
}

.guide-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 25px;
  border-bottom: 1px solid #FFC633;
}

.guide-header h2 {
  color: #FF6600;
  margin: 0;
  font-size: 1.5rem;
}

.guide-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #999;
  padding: 5px;
}

.guide-close:hover {
  color: #333;
}

.guide-content {
  padding: 20px 25px;
  overflow-y: auto;
  flex: 1;
}

.guide-welcome {
  color: #f9729a;
  font-size: 1.1rem;
  font-weight: bold;
  margin-bottom: 20px;
}

.guide-content h3 {
  color: #FF6600;
  font-size: 1.2rem;
  margin: 20px 0 10px;
  border-bottom: 1px solid #FFC633;
  padding-bottom: 5px;
}

.guide-item {
  margin-bottom: 15px;
  padding: 10px;
  background: #FFF5C2;
  border-radius: 10px;
  border: 1px solid #FFE5B4;
}

.guide-highlight {
  background: linear-gradient(135deg, #FFF0F5, #FFE4E1);
  border: 2px solid #FF69B4;
  box-shadow: 0 0 10px rgba(255, 105, 180, 0.3);
}

.guide-highlight h4 {
  color: #FF1493;
}

.guide-item h4 {
  color: #FF6600;
  margin: 0 0 8px;
  font-size: 1rem;
}

.guide-item p {
  margin: 4px 0;
  color: #333;
  font-size: 0.9rem;
  line-height: 1.5;
}

.guide-tip {
  margin-top: 20px;
  padding: 10px;
  background: #FEEFEF;
  border-radius: 10px;
  color: #f9729a;
  font-weight: bold;
  text-align: center;
}

.guide-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 25px;
  border-top: 1px solid #FFC633;
}

.guide-checkbox {
  color: #666;
  font-size: 0.9rem;
  cursor: pointer;
}

.guide-checkbox input {
  margin-right: 5px;
}

.guide-ok-btn {
  padding: 10px 30px;
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
  border: none;
  border-radius: 20px;
  font-weight: bold;
  cursor: pointer;
  font-size: 1rem;
}

.guide-ok-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}
</style>