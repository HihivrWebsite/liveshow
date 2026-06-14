# 技术实现指南

## 依赖安装

### 1. TailwindCSS

```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

### 2. shadcn-vue

```bash
npx shadcn-vue@latest init
```

配置选项：
- Style: New York
- Base color: Neutral
- CSS variables: Yes

### 3. GSAP

```bash
npm install gsap
```

### 4. lucide-vue-next

```bash
npm install lucide-vue-next
```

### 5. figma-squircle

```bash
npm install figma-squircle
```

---

## 项目结构

```
frontend/src/
├── assets/
│   └── styles/
│       ├── tokens.css          # Design Token 定义
│       ├── glass.css           # 毛玻璃样式
│       └── animations.css      # GSAP 动画预设
├── components/
│   ├── ui/                     # 基础 UI 组件
│   │   ├── GlassCard.vue
│   │   ├── GlassButton.vue
│   │   ├── GlassDialog.vue
│   │   ├── GlassInput.vue
│   │   ├── GlassTable.vue
│   │   ├── StatCard.vue
│   │   ├── LiveCounter.vue
│   │   ├── AnimatedTag.vue
│   │   └── RankingRow.vue
│   ├── HeaderSection.vue       # 业务组件
│   ├── FooterSection.vue
│   ├── AnchorList.vue
│   ├── NavigationTable.vue
│   └── ...
├── composables/
│   ├── useGSAP.js              # GSAP 动画 composable
│   └── useGlass.js             # 毛玻璃效果 composable
└── utils/
    └── theme.ts                # 主题配置
```

---

## Design Token 实现

### tokens.css

```css
@layer base {
  :root {
    /* 颜色 */
    --color-background: #F7F1DF;
    --color-primary: #F6B100;
    --color-accent: #FF6B9D;
    --color-text-main: #5D4B24;
    --color-text-secondary: #8E7B50;
    --color-card: #FFF3D0;
    
    /* 派生色 */
    --color-border: rgba(246, 177, 0, 0.2);
    --color-shadow: rgba(246, 177, 0, 0.1);
    --color-hover: rgba(246, 177, 0, 0.05);
    --color-disabled: rgba(142, 123, 80, 0.5);
    
    /* 圆角 */
    --radius-button: 999px;
    --radius-input: 24px;
    --radius-card: 32px;
    --radius-dialog: 40px;
    
    /* 阴影 */
    --shadow-default: 0 8px 24px rgba(0, 0, 0, 0.04);
    --shadow-hover: 0 12px 32px rgba(0, 0, 0, 0.08);
    --shadow-dialog: 0 20px 60px rgba(0, 0, 0, 0.12);
    
    /* 毛玻璃 */
    --blur-glass: 24px;
    --glass-bg: rgba(255, 255, 255, 0.55);
    --glass-border: rgba(255, 255, 255, 0.35);
    
    /* 动画 */
    --ease-main: power4.out;
    --ease-card: expo.out;
    --ease-hover: power2.out;
    --ease-counter: power3.out;
  }
}
```

---

## GSAP 动画实现

### composables/useGSAP.js

```javascript
import { gsap } from 'gsap'

export function useGSAP() {
  // 页面进入动画
  const pageEnter = (element) => {
    gsap.from(element, {
      opacity: 0,
      y: 30,
      duration: 0.8,
      ease: 'power4.out'
    })
  }
  
  // 卡片出现动画
  const cardEnter = (element) => {
    gsap.from(element, {
      opacity: 0,
      y: 20,
      scale: 0.95,
      duration: 0.6,
      ease: 'expo.out'
    })
  }
  
  // Hover 动画
  const hoverIn = (element) => {
    gsap.to(element, {
      y: -4,
      scale: 1.02,
      boxShadow: '0 12px 32px rgba(0, 0, 0, 0.08)',
      duration: 0.25,
      ease: 'power2.out'
    })
  }
  
  const hoverOut = (element) => {
    gsap.to(element, {
      y: 0,
      scale: 1,
      boxShadow: '0 8px 24px rgba(0, 0, 0, 0.04)',
      duration: 0.25,
      ease: 'power2.out'
    })
  }
  
  // 数字增长动画
  const countUp = (element, target) => {
    gsap.to(element, {
      innerText: target,
      duration: 1.2,
      ease: 'power3.out',
      snap: { innerText: 1 }
    })
  }
  
  return {
    pageEnter,
    cardEnter,
    hoverIn,
    hoverOut,
    countUp
  }
}
```

---

## 毛玻璃组件实现

### GlassCard.vue

```vue
<template>
  <div class="glass-card" ref="cardRef">
    <slot />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useGSAP } from '@/composables/useGSAP'

const cardRef = ref(null)
const { cardEnter, hoverIn, hoverOut } = useGSAP()

onMounted(() => {
  cardEnter(cardRef.value)
})
</script>

<style scoped>
.glass-card {
  background: var(--glass-bg);
  backdrop-filter: blur(var(--blur-glass));
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-default);
  transition: none; /* 使用 GSAP 替代 */
}

.glass-card:hover {
  /* 通过 GSAP hoverIn/hoverOut 控制 */
}
</style>
```

---

## 迁移步骤

### 步骤 1：安装依赖

```bash
cd frontend
npm install -D tailwindcss postcss autoprefixer
npm install gsap lucide-vue-next figma-squircle
npx tailwindcss init -p
npx shadcn-vue@latest init
```

### 步骤 2：配置 TailwindCSS

更新 `tailwind.config.js`，添加 Design Token。

### 步骤 3：创建 Design Token 文件

创建 `src/assets/styles/tokens.css`。

### 步骤 4：创建基础组件

在 `src/components/ui/` 下创建 9 个基础组件。

### 步骤 5：创建 Composables

创建 `useGSAP.js` 和 `useGlass.js`。

### 步骤 6：迁移业务组件

按照 `component-migration.md` 中的顺序逐个迁移。

### 步骤 7：测试和优化

- 功能测试
- 性能测试
- 响应式测试
- 动画流畅度测试
