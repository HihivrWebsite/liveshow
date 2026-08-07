<template>
  <div id="app" class="app">
    <CursorProvider />
    <HeaderSection />
    <main class="main-content">
      <router-view v-slot="{ Component }">
        <transition name="page" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
    <FooterSection />

    <!-- 右侧抽屉 -->
    <div :class="['side-drawer', { collapsed: !showDrawer }]">
      <!-- 收起状态：只显示一个小箭头 -->
      <div class="drawer-toggle" @click="showDrawer = !showDrawer">
        <span>{{ showDrawer ? '▶' : '◀' }}</span>
      </div>

      <!-- 展开状态：显示完整内容 -->
      <div class="drawer-content" v-show="showDrawer">
        <h3 class="drawer-title">快捷操作</h3>

        <div class="drawer-item">
          <button class="drawer-btn battle-btn" @click="emitBattle">
            🎯 恶意斗虫 ({{ selectedCount }})
          </button>
          <p class="drawer-desc">选择多个主播后点击进行数据对比</p>
        </div>

        <div class="drawer-item">
          <button class="drawer-btn export-btn export-btn-flash" @click="emitExport">
            📸 导出截图 ({{ selectedCount }})
          </button>
          <p class="drawer-desc">将选中主播数据导出为图片</p>
        </div>

        <div class="drawer-item">
          <button class="drawer-btn rank-btn" @click="emitRank">
            📊 排名对比 ({{ selectedCount }})
          </button>
          <p class="drawer-desc">选择多个主播后进行排名对比</p>
        </div>

        <p class="drawer-footer">选择多个主播后点击进行数据对比</p>
      </div>
    </div>
  </div>
</template>

<script>
import { defineAsyncComponent } from 'vue'
import HeaderSection from './components/HeaderSection.vue'
import FooterSection from './components/FooterSection.vue'
import CursorProvider from './components/ui/cursor/CursorProvider.vue'

export default {
  name: 'App',
  components: {
    HeaderSection,
    FooterSection,
    CursorProvider
  },
  data() {
    return {
      showDrawer: true,
      selectedCount: 0
    }
  },
  mounted() {
    window.addEventListener('selection-change', (e) => {
      this.selectedCount = e.detail.count || 0
    })
  },
  methods: {
    emitBattle() {
      window.dispatchEvent(new CustomEvent('popup-open-battle'))
    },
    emitExport() {
      window.dispatchEvent(new CustomEvent('popup-open-export'))
    },
    emitRank() {
      window.dispatchEvent(new CustomEvent('popup-open-rank'))
    }
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background-color: #FFF8E1; /* 淡黄色背景 */
  color: #333;
  min-height: 100vh;
  position: relative;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
  /* 确保内容可以横向滚动而不是被裁剪 */
  overflow-x: auto;
}

/* 页面过渡动画 */
.page-enter-active,
.page-leave-active {
  transition: opacity 0.3s ease;
}

.page-enter-from,
.page-leave-to {
  opacity: 0;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .main-content {
    padding: 15px;
    max-width: 100%;
  }
}

@media (max-width: 768px) {
  .main-content {
    padding: 10px;
  }
}

@media (max-width: 480px) {
  .main-content {
    padding: 8px;
  }
}

/* 右侧抽屉样式 */
.side-drawer {
  position: fixed;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  z-index: 9999;
  display: flex;
  align-items: center;
  transition: all 0.3s ease;
}

.side-drawer.collapsed {
  right: 0;
}

.drawer-toggle {
  background: #FFC633;
  color: #333;
  width: 24px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 8px 0 0 8px;
  font-size: 12px;
  font-weight: bold;
  box-shadow: -2px 0 8px rgba(0,0,0,0.15);
}

.drawer-content {
  background: #FFF8E1;
  border: 2px solid #FFC633;
  border-right: none;
  border-radius: 12px 0 0 12px;
  padding: 16px;
  width: 200px;
  box-shadow: -4px 0 15px rgba(0,0,0,0.1);
}

.drawer-title {
  color: #FF6600;
  font-size: 14px;
  margin: 0 0 12px 0;
  text-align: center;
  border-bottom: 1px solid #FFC633;
  padding-bottom: 8px;
}

.drawer-item {
  margin-bottom: 12px;
}

.drawer-btn {
  width: 100%;
  padding: 8px 12px;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.2s;
}

.battle-btn {
  background: #FF6B6B;
  color: white;
}

.export-btn {
  background: #4ECDC4;
  color: white;
}

.rank-btn {
  background: #45B7D1;
  color: white;
}

.drawer-btn:hover {
  opacity: 0.85;
  transform: scale(1.02);
}

.drawer-desc {
  font-size: 11px;
  color: #666;
  margin: 4px 0 0 0;
  padding-left: 4px;
}

.drawer-footer {
  font-size: 11px;
  color: #999;
  text-align: center;
  margin: 8px 0 0 0;
  border-top: 1px solid #FFC633;
  padding-top: 8px;
}

/* 导出截图按钮闪烁效果 */
.export-btn-flash {
  animation: flash-color 1.5s ease-in-out infinite;
}

@keyframes flash-color {
  0%, 100% {
    background: #FF6B6B;
    box-shadow: 0 0 15px rgba(255, 107, 107, 0.6);
  }
  50% {
    background: #4ECDC4;
    box-shadow: 0 0 15px rgba(78, 205, 196, 0.6);
  }
}
</style>