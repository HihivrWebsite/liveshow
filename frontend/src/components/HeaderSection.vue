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
      
      <div class="header-actions">
        <button @click="goToMainSite" class="action-btn primary">
          进入向阳Hihi粉丝站主站
        </button>
        
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
</template>

<script>
import { computed } from 'vue'
import { useRoute } from 'vue-router'

export default {
  name: 'HeaderSection',
  setup() {
    const route = useRoute()
    
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

    const goToMainSite = () => {
      window.open('https://hihivr.top', '_blank')
    }

    const followCreator = () => {
      window.open('https://space.bilibili.com/1048135385', '_blank')
    }

    return {
      title,
      goToMainSite,
      followCreator
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
  text-align: center;
}

.creator-info {
  margin-top: 15px;
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
@media (max-width: 768px) {
  .header-content {
    padding: 0 10px;
  }
  
  .logo-section {
    flex-direction: column;
    gap: 10px;
  }
  
  .logo-img {
    height: 80px;
  }
  
  .site-title {
    font-size: 1.4rem;
  }
  
  .action-btn {
    padding: 8px 16px;
    font-size: 0.8rem;
  }
  
  .creator-text {
    font-size: 0.8rem;
  }
}

@media (max-width: 480px) {
  .logo-img {
    height: 60px;
  }
  
  .site-title {
    font-size: 1.2rem;
  }
  
  .action-btn {
    padding: 6px 12px;
    font-size: 0.75rem;
    margin: 5px 2px;
    display: block;
    width: 100%;
    max-width: 280px;
    margin-left: auto;
    margin-right: auto;
  }
}
</style>