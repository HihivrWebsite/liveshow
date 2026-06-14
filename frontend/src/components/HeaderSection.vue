<template>
  <header class="header" ref="headerRef">
    <div class="header-content">
      <div class="logo-section">
        <div class="logo-images">
          <a href="https://smms.app/image/BvHWJNoL2iscPXl" target="_blank">
            <img
              ref="logoImg1Ref"
              src="https://s2.loli.net/2024/11/16/BvHWJNoL2iscPXl.png"
              alt="h222.png"
              class="logo-img"
            >
          </a>
          <a href="https://smms.app/image/Q2i7IUqjMDb3pev" target="_blank">
            <img
              ref="logoImg2Ref"
              src="https://s2.loli.net/2024/11/16/Q2i7IUqjMDb3pev.png"
              alt="hihi.png"
              class="logo-img"
            >
          </a>
        </div>
        <h1 class="site-title">{{ title }}</h1>
      </div>

      <GlassCard class="cache-stats-card" variant="strong" padding="20px">
        <div class="cache-stats-grid">
          <GlassCard variant="default" padding="16px" class="cache-stat-item" :enter-delay="0">
            <div class="stat-label">缓存命中率</div>
            <div class="stat-value">{{ cacheHitRate }}</div>
          </GlassCard>
          <GlassCard variant="default" padding="16px" class="cache-stat-item" :enter-delay="80">
            <div class="stat-label">缓存占用</div>
            <div class="stat-value">{{ cacheSize }}</div>
            <div class="stat-subvalue">最大容量 {{ cacheMaxSize }}</div>
          </GlassCard>
          <GlassCard variant="default" padding="16px" class="cache-stat-item" :enter-delay="160">
            <div class="stat-label">缓存总条目</div>
            <div class="stat-value">{{ cacheEntryCount }}</div>
          </GlassCard>
          <GlassCard variant="default" padding="16px" class="cache-stat-item" :enter-delay="240">
            <div class="stat-label">Attention缓存条目</div>
            <div class="stat-value">{{ attentionEntries }}</div>
          </GlassCard>
          <GlassCard variant="default" padding="16px" class="cache-stat-item" :enter-delay="320">
            <div class="stat-label">LiveSessions缓存条目</div>
            <div class="stat-value">{{ liveSessionsEntries }}</div>
          </GlassCard>
          <GlassCard variant="default" padding="16px" class="cache-stat-item cache-status-card" :enter-delay="400">
            <div class="stat-label">缓存策略</div>
            <div class="stat-value stat-value--text">{{ cacheStatus }}</div>
          </GlassCard>
        </div>
      </GlassCard>

      <div class="header-actions">
        <div class="main-site-group">
          <GlassButton variant="primary" size="lg" cta @click="goToMainSite" class="btn-main-site">
            进入向阳Hihi粉丝站主站<br>为成为向阳人而骄傲
          </GlassButton>
          <GlassButton variant="info" size="md" cta @click="showGuide = true" class="btn-guide">
            <BookOpen :size="18" class="btn-icon" />
            功能指南
          </GlassButton>
        </div>

        <div class="creator-info">
          <p class="creator-text-large">
            特别感谢某热心小礼猫-千秋紫莹提供的斗虫数据API，感谢其对本项目提供了巨大的帮助
          </p>
          <GlassButton variant="secondary" size="md" cta @click="followCreator">
            <Heart :size="16" class="btn-icon" />
            关注花礼harei喵，关注花礼harei谢谢喵
          </GlassButton>
        </div>
      </div>
    </div>
  </header>

  <GlassDialog v-model:visible="showGuide" title="功能指南" width="700px">
    <template #header>
      <h2 class="guide-dialog-title">
        <BookOpen :size="22" class="guide-title-icon" />
        功能指南
      </h2>
    </template>

    <div class="guide-content">
      <p class="guide-welcome">欢迎使用维阿PSP斗虫榜！以下是网站所有功能的介绍和使用方法。</p>

      <h3>一、核心功能</h3>
      <div class="guide-item guide-highlight">
        <h4><Star :size="14" class="guide-star" /> 1. 导出截图</h4>
        <p><b>位置：</b>导航表格上方按钮区</p>
        <p><b>用途：</b>将选中主播的数据导出为图片，包含网站信息和数据表格</p>
        <p><b>使用：</b>选择主播 → 点击按钮 → 选择时间范围 → 确定导出 → 自动下载PNG</p>
      </div>
      <div class="guide-item">
        <h4>2. 排名对比</h4>
        <p><b>位置：</b>导航表格上方按钮区</p>
        <p><b>用途：</b>查看各主播在指定指标上的排名变化趋势</p>
        <p><b>使用：</b>选择2个以上主播 → 点击按钮 → 选择日期范围 → 选择指标 → 生成排名折线图</p>
      </div>
      <div class="guide-item">
        <h4>3. 恶意斗虫</h4>
        <p><b>位置：</b>导航表格上方按钮区</p>
        <p><b>用途：</b>多主播数据对比分析，支持13种指标折线图</p>
        <p><b>使用：</b>选择2个以上主播 → 点击按钮 → 选择日期范围 → 选择指标 → 生成图表</p>
      </div>
      <div class="guide-item guide-highlight">
        <h4><Star :size="14" class="guide-star" /> 4. Super Chat历史</h4>
        <p><b>位置：</b>详情页操作栏</p>
        <p><b>用途：</b>查看主播的Super Chat（醒目留言）记录</p>
        <p><b>使用：</b>点击按钮 → 进入SC历史页面</p>
      </div>

      <h3>二、数据筛选</h3>
      <div class="guide-item">
        <h4>5. 工会筛选</h4>
        <p><b>位置：</b>页面顶部三个按钮</p>
        <p><b>用途：</b>切换查看维阿PSP全部主播、仅维阿（VirtuaReal）主播、或仅PSPlive主播的数据</p>
        <p><b>使用：</b>点击对应按钮即可切换</p>
      </div>
      <div class="guide-item">
        <h4>6. 切换不同月份</h4>
        <p><b>位置：</b>页面顶部操作栏</p>
        <p><b>用途：</b>查看历史月份的主播数据</p>
        <p><b>使用：</b>点击按钮 → 选择月份 → 确定</p>
      </div>
      <div class="guide-item">
        <h4>7. 多月份共同统计</h4>
        <p><b>位置：</b>页面顶部操作栏</p>
        <p><b>用途：</b>合并多个月份的数据，查看累计排名</p>
        <p><b>使用：</b>点击按钮 → 选择起始和结束月份 → 确定</p>
      </div>

      <h3>三、数据可视化</h3>
      <div class="guide-item">
        <h4>8. 查看营收占比</h4>
        <p><b>位置：</b>页面顶部操作栏</p>
        <p><b>用途：</b>以饼图展示各主播的收入占比，扇形内显示主播头像</p>
        <p><b>使用：</b>点击按钮 → 自动弹出饼图</p>
      </div>
      <div class="guide-item">
        <h4>9. VR PSP对比图</h4>
        <p><b>位置：</b>页面顶部操作栏</p>
        <p><b>用途：</b>对比维阿和PSPlive两个工会的总收入占比</p>
        <p><b>使用：</b>需在"维阿PSP斗虫榜"模式下，点击按钮 → 自动弹出对比饼图</p>
      </div>
      <div class="guide-item">
        <h4>10. 进行回归分析</h4>
        <p><b>位置：</b>页面顶部操作栏</p>
        <p><b>用途：</b>对主播数据进行回归分析，查看变量之间的关系</p>
        <p><b>使用：</b>点击按钮 → 选择变量 → 生成分析结果</p>
      </div>

      <h3>四、导航表格</h3>
      <div class="guide-item">
        <h4>11. 快速导航</h4>
        <p><b>位置：</b>页面中部导航表格</p>
        <p><b>用途：</b>快速查看主播排名、状态、营收，点击跳转到对应卡片</p>
        <p><b>使用：</b>点击"跳转"按钮滚动到对应主播卡片</p>
      </div>
      <div class="guide-item">
        <h4>12. 全选功能</h4>
        <p><b>位置：</b>导航表格左侧复选框</p>
        <p><b>用途：</b>选择多个主播进行对比分析或导出</p>
        <p><b>使用：</b>勾选复选框，表头可全选/全不选</p>
      </div>

      <h3>五、主播卡片</h3>
      <div class="guide-item">
        <h4>13. 查看详细数据</h4>
        <p><b>位置：</b>每个主播卡片底部</p>
        <p><b>用途：</b>进入该主播的直播会话详情页</p>
        <p><b>使用：</b>点击"查看详情"按钮</p>
      </div>

      <h3>六、详情页功能</h3>
      <div class="guide-item">
        <h4>14. 直播会话详情</h4>
        <p><b>位置：</b>详情页</p>
        <p><b>用途：</b>查看主播每场直播的详细数据（时长、收入、弹幕等）</p>
        <p><b>进入：</b>从主页点击主播卡片的"查看详情"</p>
      </div>
      <div class="guide-item">
        <h4>15. 计算新增粉丝数</h4>
        <p><b>位置：</b>详情页操作栏</p>
        <p><b>用途：</b>计算每场直播的新增关注数</p>
        <p><b>使用：</b>点击按钮 → 自动计算并显示</p>
      </div>
      <div class="guide-item">
        <h4>16. 显示直播数据折线图</h4>
        <p><b>位置：</b>详情页操作栏</p>
        <p><b>用途：</b>以折线图展示直播数据趋势</p>
        <p><b>使用：</b>点击按钮 → 自动弹出图表</p>
      </div>
      <div class="guide-item">
        <h4>17. 跳转到直播间</h4>
        <p><b>位置：</b>详情页操作栏</p>
        <p><b>用途：</b>直接跳转到主播的Bilibili直播间</p>
        <p><b>使用：</b>点击按钮 → 新标签页打开直播间</p>
      </div>

      <h3>七、其他功能</h3>
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

    <template #footer>
      <GlassButton variant="primary" size="md" @click="showGuide = false">
        我知道了
      </GlassButton>
    </template>
  </GlassDialog>
</template>

<script setup>
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { anchorAPI } from '@/api'
import { BookOpen, Heart, Star } from 'lucide-vue-next'
import GlassCard from '@/components/ui/GlassCard.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import GlassDialog from '@/components/ui/GlassDialog.vue'
import { usePageEnter } from '@/composables/usePageEnter'
import { useHover } from '@/composables/useHover'
import gsap from 'gsap'

const route = useRoute()

const headerRef = ref(null)
const logoImg1Ref = ref(null)
const logoImg2Ref = ref(null)

const cacheHitRate = ref('N/A')
const cacheSize = ref('N/A')
const cacheEntryCount = ref('N/A')
const cacheMaxSize = ref('N/A')
const attentionEntries = ref('N/A')
const liveSessionsEntries = ref('N/A')
const cacheStatus = ref('加载中...')

const showGuide = ref(false)

usePageEnter(headerRef, { duration: 0.6, y: 30 })

onMounted(() => {
  ;[logoImg1Ref.value, logoImg2Ref.value].forEach((el) => {
    if (!el) return
    el.addEventListener('mouseenter', () => {
      gsap.to(el, { scale: 1.05, duration: 0.25, ease: 'power2.out' })
    })
    el.addEventListener('mouseleave', () => {
      gsap.to(el, { scale: 1, duration: 0.25, ease: 'power2.out' })
    })
  })
})

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

const fetchCacheStats = async () => {
  try {
    const response = await anchorAPI.getCacheStats()
    const stats = response

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

let cacheStatsInterval
onMounted(() => {
  fetchCacheStats()
  cacheStatsInterval = setInterval(fetchCacheStats, 30000)
})

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
</script>

<style scoped>
.header {
  background: var(--color-background);
  padding: 20px 0;
  position: static;
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
  will-change: transform;
}

.site-title {
  color: var(--color-accent);
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

.creator-text-large {
  color: var(--color-accent);
  font-size: 1.2rem;
  margin-bottom: 10px;
  line-height: 1.4;
  font-weight: bold;
}

.cache-stats-card {
  margin: 15px 0;
}

.cache-stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 14px;
  justify-items: center;
  align-items: stretch;
  overflow: hidden;
}

.cache-stat-item {
  min-width: 140px;
  width: 100%;
  text-align: center;
}

.cache-stat-item.cache-status-card {
  min-width: 200px;
}

.stat-label {
  font-size: 0.95rem;
  margin-bottom: 10px;
  color: var(--color-text-secondary);
}

.stat-subvalue {
  display: block;
  margin-top: 10px;
  font-size: 0.8rem;
  color: var(--color-text-secondary);
}

.stat-value {
  font-size: 1.1rem;
  font-weight: bold;
  color: var(--color-primary);
}

.stat-value--text {
  font-size: 0.9rem;
  color: var(--color-text-main);
}

.main-site-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.btn-main-site {
  font-size: 1.8rem;
  padding: 20px 40px;
}

.btn-guide {
  font-size: 1.2rem;
  padding: 12px 30px;
}

.btn-icon {
  vertical-align: middle;
  margin-right: 4px;
}

.guide-dialog-title {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--color-primary);
  margin: 0;
  font-size: 1.5rem;
}

.guide-title-icon {
  color: var(--color-primary);
}

.guide-content {
  color: var(--color-text-main);
}

.guide-welcome {
  color: var(--color-accent);
  font-size: 1.1rem;
  font-weight: bold;
  margin-bottom: 20px;
}

.guide-content h3 {
  color: var(--color-primary);
  font-size: 1.2rem;
  margin: 20px 0 10px;
  border-bottom: 1px solid var(--color-primary);
  padding-bottom: 5px;
}

.guide-item {
  margin-bottom: 15px;
  padding: 10px;
  background: var(--color-card);
  border-radius: 10px;
  border: 1px solid rgba(246, 177, 0, 0.2);
}

.guide-highlight {
  background: linear-gradient(135deg, #FFF0F5, #FFE4E1);
  border: 2px solid var(--color-accent);
  box-shadow: 0 0 10px rgba(255, 107, 157, 0.3);
}

.guide-highlight h4 {
  color: var(--color-accent);
}

.guide-item h4 {
  color: var(--color-primary);
  margin: 0 0 8px;
  font-size: 1rem;
  display: flex;
  align-items: center;
  gap: 6px;
}

.guide-star {
  color: var(--color-primary);
  vertical-align: middle;
}

.guide-item p {
  margin: 4px 0;
  color: var(--color-text-main);
  font-size: 0.9rem;
  line-height: 1.5;
}

.guide-tip {
  margin-top: 20px;
  padding: 10px;
  background: rgba(255, 107, 157, 0.08);
  border-radius: 10px;
  color: var(--color-accent);
  font-weight: bold;
  text-align: center;
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

  .btn-main-site {
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

  .btn-main-site {
    font-size: 1.2rem;
    padding: 12px 24px;
    width: 100%;
    max-width: 300px;
  }

  .btn-guide {
    font-size: 0.9rem;
    padding: 10px 20px;
    width: 100%;
    max-width: 300px;
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

  .btn-main-site {
    font-size: 1rem;
    padding: 10px 20px;
  }

  .btn-guide {
    font-size: 0.8rem;
    padding: 8px 16px;
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

  .btn-main-site {
    font-size: 0.9rem;
    padding: 8px 16px;
  }

  .creator-text-large {
    font-size: 0.75rem;
  }
}
</style>
