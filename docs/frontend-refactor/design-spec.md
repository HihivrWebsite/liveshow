# Liveshow 前端设计规范 v2.0

> 本规范是前端视觉重构的完整指南，所有前端修改必须遵守本规范。

---

## 一、设计目标

### 风格定位

**"温暖的毛玻璃 SaaS"**

参考：
- HyperOS（超椭圆圆角）
- Apple VisionOS（毛玻璃效果）
- 现代 SaaS Dashboard（层次感）

### 必须保留

- 米黄色背景（温暖感）
- 金黄色主色（品牌识别）
- 粉色强调色（二次元氛围）
- 所有功能布局和交互流程

### 禁止改成

- 蓝色后台风
- 深色科技风
- Element Plus 默认风格
- DataV 大屏风格

---

## 二、技术栈

| 技术 | 用途 | 版本 |
|------|------|------|
| Vue 3 | 框架 | - |
| Vite | 构建工具 | - |
| TailwindCSS | 样式系统 | - |
| shadcn-vue | 基础组件系统 | - |
| GSAP | 动画系统 | - |
| figma-squircle | 超椭圆圆角 | - |
| lucide-vue-next | 图标库 | - |
| Apache ECharts | 图表 | - |

### 禁止事项

- 禁止直接使用大量第三方现成主题
- 禁止使用 emoji 替代图标
- 禁止混用不同图标库
- 禁止使用默认 ECharts 配色

---

## 三、颜色系统

### 核心色板（6色）

| 色值 | 用途 | Tailwind 类名 |
|------|------|---------------|
| #F7F1DF | 页面背景 | `bg-background` |
| #F6B100 | 主色（金黄色） | `text-primary` / `bg-primary` |
| #FF6B9D | 强调色（粉色） | `text-accent` / `bg-accent` |
| #5D4B24 | 主文字 | `text-text-main` |
| #8E7B50 | 次文字 | `text-text-secondary` |
| #FFF3D0 | 卡片背景 | `bg-card` |

### 派生色规范

所有边框、阴影、hover状态等，必须从上述6色派生，禁止引入新色值。

| 派生用途 | 规则 |
|----------|------|
| 边框 | 主色 20% 透明度 → `rgba(246, 177, 0, 0.2)` |
| 阴影 | 主色 10% 透明度 → `rgba(246, 177, 0, 0.1)` |
| Hover背景 | 卡片背景 + 主色 5% → `rgba(246, 177, 0, 0.05)` |
| 禁用状态 | 次文字 50% 透明度 |

### SC 卡片颜色（B站原色保留）

| 价格区间 | 颜色 | 色值 |
|----------|------|------|
| < ¥10 | 银色 | #C0C0C0 |
| ¥10-100 | 蓝色 | #3A5FCD |
| ¥100-1000 | 金色 | #FFD700 |
| ≥ ¥1000 | 红色 | #FF0000 |

---

## 四、圆角系统（超椭圆）

使用 `figma-squircle` 实现 HyperOS 风格圆角。

### 统一规范

| 元素 | 圆角值 | Tailwind 类名 |
|------|--------|---------------|
| Button | 999px | `rounded-button` |
| Input | 24px | `rounded-input` |
| Card | 32px | `rounded-card` |
| Dialog | 40px | `rounded-dialog` |

### 禁止事项

- 禁止使用不统一的 border-radius
- 禁止使用普通 CSS border-radius（必须使用 figma-squircle）

---

## 五、阴影系统

| 状态 | 阴影值 | Tailwind 类名 |
|------|--------|---------------|
| 默认 | `0 8px 24px rgba(0,0,0,.04)` | `shadow-default` |
| Hover | `0 12px 32px rgba(0,0,0,.08)` | `shadow-hover` |
| Dialog | `0 20px 60px rgba(0,0,0,.12)` | `shadow-dialog` |

---

## 六、毛玻璃系统

### 规范

| 属性 | 值 |
|------|------|
| blur | 24px |
| background | `rgba(255,255,255,.55)` |
| border | `1px solid rgba(255,255,255,.35)` |

### 适用元素

- Card（卡片）
- Dialog（弹窗）
- Sidebar（侧边栏）
- Dropdown（下拉菜单）

### 注意事项

毛玻璃效果在米黄色背景上需测试实际效果，必要时调整透明度。

---

## 七、动画系统

使用 GSAP 统一管理所有动画，禁止大量使用 CSS transition。

### 动画规范

| 动画类型 | duration | ease | 适用场景 |
|----------|----------|------|----------|
| 页面进入 | 0.8s | power4.out | 页面加载 |
| 卡片出现 | 0.6s | expo.out | 列表渲染 |
| Hover | 0.25s | power2.out | 所有可交互元素 |
| 数字增长 | 1.2s | power3.out | 统计数字 |
| CTA光带 | 1.5s | power2.inOut | 指定CTA按钮 |

### 动画实现方式

#### 1. 页面进入动画

```javascript
// composables/usePageEnter.js
import { gsap } from 'gsap'
import { onMounted } from 'vue'

export function usePageEnter(selector) {
  onMounted(() => {
    gsap.from(selector, {
      y: 30,
      opacity: 0,
      duration: 0.8,
      ease: 'power4.out',
      stagger: 0.1
    })
  })
}
```

#### 2. 卡片出现动画

```javascript
// composables/useCardEnter.js
import { gsap } from 'gsap'

export function useCardEnter(element) {
  gsap.from(element, {
    y: 20,
    scale: 0.95,
    opacity: 0,
    duration: 0.6,
    ease: 'expo.out'
  })
}
```

#### 3. Hover 动画

```javascript
// composables/useHover.js
import { gsap } from 'gsap'

export function useHover(element) {
  const enter = () => {
    gsap.to(element, {
      y: -4,
      scale: 1.02,
      boxShadow: '0 12px 32px rgba(0, 0, 0, 0.08)',
      duration: 0.25,
      ease: 'power2.out'
    })
  }
  
  const leave = () => {
    gsap.to(element, {
      y: 0,
      scale: 1,
      boxShadow: '0 8px 24px rgba(0, 0, 0, 0.04)',
      duration: 0.25,
      ease: 'power2.out'
    })
  }
  
  element.addEventListener('mouseenter', enter)
  element.addEventListener('mouseleave', leave)
  
  return () => {
    element.removeEventListener('mouseenter', enter)
    element.removeEventListener('mouseleave', leave)
  }
}
```

#### 4. 数字增长动画

```javascript
// composables/useCountUp.js
import { gsap } from 'gsap'

export function useCountUp(element, targetValue) {
  const obj = { value: 0 }
  
  gsap.to(obj, {
    value: targetValue,
    duration: 1.2,
    ease: 'power3.out',
    onUpdate: () => {
      element.textContent = obj.value.toFixed(2)
    }
  })
}
```

#### 5. CTA 光带扫过动画

```javascript
// composables/useCTAShine.js
import { gsap } from 'gsap'

export function useCTAShine(element) {
  const shine = document.createElement('div')
  shine.style.cssText = `
    position: absolute;
    top: 0;
    left: -100%;
    width: 50%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.4), transparent);
    pointer-events: none;
  `
  element.style.position = 'relative'
  element.style.overflow = 'hidden'
  element.appendChild(shine)
  
  gsap.to(shine, {
    left: '200%',
    duration: 1.5,
    ease: 'power2.inOut',
    repeat: -1,
    repeatDelay: 3
  })
}
```

### CTA 动画规范

仅以下按钮保留光效动画（光带扫过 4~6秒一次）：

- HeaderSection 的「进入主站」按钮
- HeaderSection 的「关注」按钮
- HeaderSection 的「功能指南」按钮
- FooterSection 的「粉丝站」按钮

其他按钮不使用光效动画，仅使用标准 hover 效果。

### 禁止事项

- 禁止高频闪烁
- 禁止整体透明度闪烁
- 禁止快速缩放
- 禁止彩虹变色

---

## 八、按钮系统

### 颜色变体

| 变体 | 用途 | 颜色 |
|------|------|------|
| primary | 主要操作 | 金黄色 #F6B100 |
| secondary | 次要操作 | 粉色 #FF6B9D |
| danger | 危险操作 | 红色 #E74C3C |
| success | 成功/跳转操作 | 绿色 #27AE60 |
| default | 默认/返回操作 | 灰色 #8E7B50 |
| info | 信息操作 | 青色 #00BCD4 |
| debug | 调试操作 | 紫色 #9C27B0 |

### 样式规范

- 圆角：999px（超椭圆）
- Hover：轻微上浮（y: -4px）+ 阴影加深
- 动画：GSAP，duration 0.25s，ease power2.out

---

## 九、组件清单

### 基础组件（src/components/ui/）

| 组件 | 说明 | 圆角 |
|------|------|------|
| GlassCard.vue | 毛玻璃卡片 | 32px |
| GlassButton.vue | 超椭圆按钮 | 999px |
| GlassDialog.vue | 毛玻璃弹窗 | 40px |
| GlassInput.vue | 统一输入框 | 24px |
| GlassTable.vue | 统一表格 | 24px |
| StatCard.vue | 统计卡片 | 32px |
| LiveCounter.vue | 直播计数器 | 32px |
| AnimatedTag.vue | 动态标签 | 999px |
| RankingRow.vue | 排名行组件 | 16px |

### 业务组件

| 组件 | 说明 |
|------|------|
| HeaderSection.vue | 顶部导航 |
| FooterSection.vue | 底部信息 |
| AnchorList.vue | 主播列表 |
| NavigationTable.vue | 导航表格 |
| MonthSelector.vue | 月份选择器 |
| ChartComponent.vue | 图表组件 |
| ExpandedView.vue | 展开视图 |
| AnchorBattle.vue | 斗虫对比 |
| RankComparison.vue | 排名对比 |
| SuperChatDetail.vue | SC详情 |
| LiveSessions.vue | 直播会话 |

---

## 十、SC 卡片设计

### 布局结构

```
┌─────────────────────────────────────────────────────────┐
│  [左侧彩色条]  [内容区域]                                 │
│                ┌─────────────────────────────────────┐  │
│                │  用户名  UID  时间                    │  │
│                ├─────────────────────────────────────┤  │
│                │  消息内容                    ¥价格   │  │
│                └─────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 样式规范

- 背景：毛玻璃（rgba(255,255,255,0.55) + blur(24px)）
- 左侧彩色条：6px，颜色对应价格等级
- 圆角：32px（超椭圆）
- 文字：深棕色 #5D4B24，无描边
- 价格：2rem，颜色跟随价格等级
- Hover：上浮 4px + 阴影加深

---

## 十一、响应式设计

### 断点

| 断点 | 宽度 | 说明 |
|------|------|------|
| sm | 640px | 手机横屏 |
| md | 768px | 平板 |
| lg | 1024px | 小桌面 |
| xl | 1280px | 桌面 |
| 2xl | 1536px | 大桌面 |

### 最大内容宽度

- 最大宽度：1400px
- 居中：`margin: 0 auto`

---

## 十二、ECharts 主题

### 自定义主题色

```javascript
const theme = {
  color: ['#F6B100', '#FF6B9D', '#3A5FCD', '#27AE60', '#E74C3C', '#9C27B0'],
  backgroundColor: 'transparent',
  textStyle: {
    color: '#5D4B24'
  },
  title: {
    textStyle: {
      color: '#5D4B24'
    }
  },
  legend: {
    textStyle: {
      color: '#8E7B50'
    }
  }
}
```

---

## 十三、重构原则

### 禁止

- 修改业务逻辑
- 修改接口
- 修改数据结构
- 修改页面布局
- 修改功能入口

### 允许

- 重构组件
- 重构样式
- 重构动画
- 重构 Design Token
- 重构视觉层

---

## 十四、快速参考

### 颜色

| 色值 | 用途 |
|------|------|
| #F7F1DF | 背景 |
| #F6B100 | 主色 |
| #FF6B9D | 强调色 |
| #5D4B24 | 主文字 |
| #8E7B50 | 次文字 |
| #FFF3D0 | 卡片背景 |

### 圆角

| 元素 | 圆角 |
|------|------|
| Button | 999px |
| Input | 24px |
| Card | 32px |
| Dialog | 40px |

### 阴影

| 状态 | 阴影 |
|------|------|
| 默认 | 0 8px 24px rgba(0,0,0,.04) |
| Hover | 0 12px 32px rgba(0,0,0,.08) |
| Dialog | 0 20px 60px rgba(0,0,0,.12) |

### 毛玻璃

| 属性 | 值 |
|------|------|
| blur | 24px |
| background | rgba(255,255,255,.55) |
| border | 1px solid rgba(255,255,255,.35) |

### 动画

| 类型 | duration | ease |
|------|----------|------|
| 页面进入 | 0.8s | power4.out |
| 卡片出现 | 0.6s | expo.out |
| Hover | 0.25s | power2.out |
| 数字增长 | 1.2s | power3.out |

---

*文档版本：v2.0*
*最后更新：2026-06-14*
