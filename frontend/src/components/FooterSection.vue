<template>
  <footer class="footer" ref="footerRef">
    <div class="footer-content">
      <div class="footer-info">
        <p class="copyright">
          © {{ new Date().getFullYear() }} 为成为向阳人而骄傲
        </p>
        <p class="tech-stack">
          技术栈: Rust + Axum + Vue 3 + Vite + MimoCode
        </p>
      </div>
      <div class="footer-links">
        <GlassButton variant="primary" size="lg" cta class="footer-button" @click="openMainSite">
          向阳Hihi粉丝站
        </GlassButton>
      </div>
    </div>
  </footer>
</template>

<script setup>
import { ref } from 'vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import { usePageEnter } from '@/composables/usePageEnter'

const footerRef = ref(null)

usePageEnter(footerRef, { duration: 0.6, y: 20, delay: 0.2 })

const openMainSite = () => {
  window.open('https://hihivr.top', '_blank')
}
</script>

<style scoped>
.footer {
  background: var(--color-background);
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
  color: var(--color-primary);
  font-size: 1.8rem;
  margin-bottom: 5px;
  font-weight: bold;
}

.tech-stack {
  color: var(--color-accent);
  font-size: 0.8rem;
}

.footer-links {
  display: flex;
  align-items: center;
  gap: 15px;
}

.footer-button {
  font-size: 1.8rem;
  padding: 10px 20px;
  text-decoration: none;
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

  .footer-button {
    font-size: 1rem;
    padding: 8px 20px;
  }
}
</style>
