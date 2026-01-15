<template>
  <footer class="footer">
    <div class="footer-content">
      <div class="footer-info">
        <p class="copyright">
          © {{ new Date().getFullYear() }} 为成为向阳人而骄傲
        </p>
        <p class="tech-stack">
          技术栈: Rust + Axum + Vue 3 + Vite + Qwen Code
        </p>
      </div>
      <div class="footer-links">
        <a href="https://hihivr.top" target="_blank" class="footer-button">
          向阳Hihi粉丝站
        </a>
      </div>
    </div>
  </footer>
</template>

<script>
export default {
  name: 'FooterSection'
}
</script>

<style scoped>
.footer {
  background: #FFF8E1;
  padding: 20px 0;
  margin-top: auto;
}

.footer-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 15px;
}

.footer-info {
  text-align: left;
}

.copyright {
  color: #FFC633;
  font-size: 1.8rem; /* 放大一倍 */
  margin-bottom: 5px;
  font-weight: bold;
}

.tech-stack {
  color: #f9729a;
  font-size: 0.8rem;
}

.footer-links {
  display: flex;
  align-items: center;
  gap: 15px;
}

.footer-button {
  color: white;
  text-decoration: none;
  font-size: 1.8rem; /* 放大一倍 */
  padding: 10px 20px;
  background: linear-gradient(45deg, #FFC633, #FFA500);
  border: none;
  border-radius: 25px;
  cursor: pointer;
  font-weight: bold;
  transition: all 0.3s ease;
  display: inline-block;
  animation: shine 3s infinite; /* 添加闪光动画 */
}

.footer-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.separator {
  color: #666;
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
  .footer-content {
    flex-direction: column;
    text-align: center;
    gap: 10px;
  }
  
  .footer-links {
    justify-content: center;
  }
  
  .copyright,
  .tech-stack {
    font-size: 0.8rem;
  }
  
  .footer-link {
    font-size: 0.8rem;
  }
}
</style>