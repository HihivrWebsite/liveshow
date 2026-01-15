<template>
  <div class="error-page">
    <div class="error-content">
      <div class="error-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z"/>
        </svg>
      </div>
      <h1 class="error-title">{{ errorCode || '404' }}</h1>
      <p class="error-message">{{ errorMessage || '页面未找到' }}</p>
      <div class="error-actions">
        <button @click="goHome" class="action-btn primary">返回首页</button>
        <button @click="goBack" class="action-btn secondary">返回上一页</button>
      </div>
    </div>
  </div>
</template>

<script>
import { useRouter } from 'vue-router'

export default {
  name: 'ErrorPage',
  props: {
    errorCode: {
      type: String,
      default: '404'
    },
    errorMessage: {
      type: String,
      default: '页面未找到'
    }
  },
  setup() {
    const router = useRouter()

    const goHome = () => {
      router.push('/')
    }

    const goBack = () => {
      router.go(-1)
    }

    return {
      goHome,
      goBack
    }
  }
}
</script>

<style scoped>
.error-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  padding: 20px;
}

.error-content {
  text-align: center;
  background: #FFF8E1;
  border-radius: 15px;
  padding: 40px;
  border: 1px solid #FFC633;
  max-width: 500px;
  width: 100%;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.error-icon {
  width: 80px;
  height: 80px;
  margin: 0 auto 20px;
  color: #FFC633;
}

.error-icon svg {
  width: 100%;
  height: 100%;
}

.error-title {
  font-size: 3rem;
  color: #FFC633;
  margin: 0 0 10px;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
}

.error-message {
  font-size: 1.2rem;
  color: #33CC99;
  margin: 0 0 30px;
  line-height: 1.5;
}

.error-actions {
  display: flex;
  gap: 15px;
  justify-content: center;
  flex-wrap: wrap;
}

.action-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 25px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
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

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .error-content {
    padding: 30px 20px;
    margin: 0 10px;
  }
  
  .error-title {
    font-size: 2.5rem;
  }
  
  .error-message {
    font-size: 1rem;
  }
  
  .error-actions {
    flex-direction: column;
    align-items: center;
  }
  
  .action-btn {
    width: 100%;
    max-width: 250px;
  }
}
</style>