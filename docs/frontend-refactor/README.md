# Liveshow UI V2 重构规范

## 项目背景

这是一个直播数据展示平台（Liveshow）。

当前项目已经具备完整功能和成熟的数据逻辑。

本次重构目标：

* 不修改业务逻辑
* 不修改数据结构
* 不修改页面布局
* 不修改导航结构
* 不修改操作流程

仅升级视觉设计体系和交互体验。

---

## 设计目标

整体风格参考：

* HyperOS
* Apple VisionOS
* 现代 SaaS Dashboard

但保留当前项目的品牌识别。

### 视觉风格描述

**"温暖的毛玻璃 SaaS"**

想象一个阳光透过磨砂玻璃照进来的房间——温暖、柔和、有层次感。不是冰冷的科技蓝，而是带着金色暖意的现代感。

#### 视觉层次

```
┌─────────────────────────────────────────────┐
│  页面背景 #F7F1DF（米黄色，像旧报纸的温暖感）  │
│                                             │
│  ┌─────────────────────────────────────┐    │
│  │  卡片 #FFF3D0（浅金色，略深于背景）   │    │
│  │  + 毛玻璃效果（半透明白色 + 模糊）    │    │
│  │  + 轻微阴影（0 8px 24px）           │    │
│  │                                     │    │
│  │  ┌─────────────────────────────┐    │    │
│  │  │  按钮 #F6B100（金黄色）      │    │    │
│  │  │  超椭圆圆角（999px）         │    │    │
│  │  │  + 光带扫过动画（4-6秒）     │    │    │
│  │  └─────────────────────────────┘    │    │
│  │                                     │    │
│  │  文字 #5D4B24（深棕色）              │    │
│  │  辅助文字 #8E7B50（浅棕色）          │    │
│  └─────────────────────────────────────┘    │
│                                             │
│  ┌─────────────────────────────────────┐    │
│  │  状态标签 #FF6B9D（粉色）            │    │
│  │  用于"直播中"等状态标识              │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

#### 组件效果

**卡片（GlassCard）**：
- 半透明白色背景（rgba(255,255,255,0.55)）
- 背后模糊效果（blur 24px）
- 细微白色边框（rgba(255,255,255,0.35)）
- 轻微阴影（0 8px 24px rgba(0,0,0,0.04)）
- 圆角 32px（超椭圆风格）
- Hover 时轻微上浮 + 阴影加深

**按钮（GlassButton）**：
- 主按钮：金黄色 #F6B100 + 超椭圆圆角（999px）
- 次按钮：粉色 #FF6B9D
- 危险按钮：红色 #E74C3C
- 成功按钮：绿色 #27AE60
- 默认按钮：灰色 #8E7B50
- 信息按钮：青色 #00BCD4
- 调试按钮：紫色 #9C27B0
- Hover 时轻微上浮（y: -4px）
- CTA 按钮：4-6秒一次的光带扫过效果（仅指定按钮）

**表格（GlassTable）**：
- 表头：半透明白色 + 毛玻璃效果
- 行：透明背景，hover 时显示轻微毛玻璃效果
- 圆角：行圆角 16px，整体圆角 24px
- 分隔线：极细的半透明线（rgba(246,177,0,0.1)）

**弹窗（GlassDialog）**：
- 半透明白色背景 + 毛玻璃效果
- 圆角 40px
- 阴影更重（0 20px 60px rgba(0,0,0,0.12)）
- 背景遮罩：半透明黑色（rgba(0,0,0,0.3)）
- 打开时从下方滑入 + 淡入动画

**统计数字（StatCard）**：
- 数字从 0 增长到目标值（1.2秒动画）
- 金黄色高亮
- 配合 LiveCounter 呼吸动画（3-5秒周期）

#### 与当前风格的对比

| 维度 | 当前 | 新版 |
|------|------|------|
| 背景 | 实心米黄 #FFF8E1 | 实心米黄 #F7F1DF（略深） |
| 卡片 | 实心黄色 + 渐变 | 毛玻璃半透明 |
| 按钮 | 实心橙色 + 闪光 | 金黄色 + 光带扫过（仅指定按钮） |
| 圆角 | 不统一（10-30px） | 统一超椭圆（32px/40px） |
| 阴影 | 橙色阴影 | 中性灰色阴影 |
| 动画 | CSS transition | GSAP 统一管理 |
| 层次 | 扁平 | 有深度感（毛玻璃 + 阴影） |

#### 保留的元素

- 米黄色背景（温暖感）
- 金黄色主色（品牌识别）
- 粉色强调色（二次元氛围）
- 所有功能布局和交互流程
- 现有按钮颜色种类（7种变体）

#### 一句话总结

**"HyperOS 的圆润 + VisionOS 的毛玻璃 + 暖色调的粉丝站氛围"**

像一个阳光房里的现代控制面板——温暖、清晰、有质感。

### 必须保留

当前项目的：

* 米黄色背景
* 金黄色主色
* 粉色强调色
* 二次元粉丝站氛围

禁止改成：

* 蓝色后台风
* 深色科技风
* Element Plus 默认风格
* DataV 大屏风格

---

## 技术栈要求

### UI基础

引入：

shadcn-vue

作为基础组件系统。

禁止直接使用大量第三方现成主题。

需要基于 shadcn-vue 二次封装。

---

### 样式系统

引入：

TailwindCSS

建立统一 Design Token。

所有颜色、圆角、阴影必须来自 Token。

禁止页面内硬编码样式。

---

### 动画系统

引入：

GSAP

统一使用 GSAP 实现：

* 页面进入动画
* 卡片出现动画
* 数字增长动画
* 排名变化动画
* Hover动画

禁止大量使用简单 CSS transition。

统一使用：

ease: "power4.out"

作为主动画曲线。

---

### 超椭圆圆角

引入：

figma-squircle

用于实现 HyperOS 风格圆角。

禁止使用不统一的 border-radius。

统一规范：

Button:
999px

Input:
24px

Card:
32px

Dialog:
40px

---

### 图标

引入：

lucide-vue-next

统一替换旧图标。

---

### 图表

保留：

Apache ECharts

建立统一主题。

禁止使用默认 ECharts 配色。

---

## 颜色规范

### 核心色板（6色）

| 用途 | 色值 | 说明 |
|------|------|------|
| 背景 | #F7F1DF | 米黄色页面背景 |
| 主色 | #F6B100 | 金黄色，用于主要按钮、高亮、品牌色 |
| 强调色 | #FF6B9D | 粉色，用于状态标识、次要强调 |
| 主文字 | #5D4B24 | 深棕色，用于标题、正文 |
| 次文字 | #8E7B50 | 浅棕色，用于辅助信息、说明文字 |
| 卡片背景 | #FFF3D0 | 浅金色，用于卡片、输入框等容器背景 |

#### 派生色规范

所有边框、阴影、hover状态等，必须从上述6色派生，禁止引入新色值。

派生规则：
- 边框：主色 20% 透明度 → rgba(246, 177, 0, 0.2)
- 阴影：主色 10% 透明度 → rgba(246, 177, 0, 0.1)
- Hover背景：卡片背景 + 主色 5% → rgba(246, 177, 0, 0.05)
- 禁用状态：次文字 50% 透明度

---

## Design Token 体系

### TailwindCSS 配置

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        background: '#F7F1DF',
        primary: '#F6B100',
        accent: '#FF6B9D',
        'text-main': '#5D4B24',
        'text-secondary': '#8E7B50',
        'card': '#FFF3D0',
      },
      borderRadius: {
        'button': '999px',
        'input': '24px',
        'card': '32px',
        'dialog': '40px',
      },
      boxShadow: {
        'default': '0 8px 24px rgba(0,0,0,.04)',
        'hover': '0 12px 32px rgba(0,0,0,.08)',
        'dialog': '0 20px 60px rgba(0,0,0,.12)',
      },
      backdropBlur: {
        'glass': '24px',
      },
    },
  },
}
```

### CSS Variables（备选方案）

```css
:root {
  --color-background: #F7F1DF;
  --color-primary: #F6B100;
  --color-accent: #FF6B9D;
  --color-text-main: #5D4B24;
  --color-text-secondary: #8E7B50;
  --color-card: #FFF3D0;
  
  --radius-button: 999px;
  --radius-input: 24px;
  --radius-card: 32px;
  --radius-dialog: 40px;
  
  --shadow-default: 0 8px 24px rgba(0,0,0,.04);
  --shadow-hover: 0 12px 32px rgba(0,0,0,.08);
  --shadow-dialog: 0 20px 60px rgba(0,0,0,.12);
  
  --blur-glass: 24px;
}
```

---

## 毛玻璃规范

建立统一 Glass System。

所有：

* Card
* Dialog
* Sidebar
* Dropdown

使用：

backdrop-filter: blur(24px)

background:
rgba(255,255,255,.55)

border:
1px solid rgba(255,255,255,.35)

注意：毛玻璃效果在米黄色背景上需测试实际效果，必要时调整透明度。

---

## 阴影规范

默认：

0 8px 24px rgba(0,0,0,.04)

Hover：

0 12px 32px rgba(0,0,0,.08)

Dialog：

0 20px 60px rgba(0,0,0,.12)

---

## 必须创建的基础组件

src/components/ui/

GlassCard.vue

GlassButton.vue

GlassDialog.vue

GlassInput.vue

GlassTable.vue

StatCard.vue

LiveCounter.vue

AnimatedTag.vue

RankingRow.vue

---

## 表格改造要求

保留：

* 表格结构
* 排序逻辑
* 分页逻辑
* 选择逻辑

仅优化：

* 行高
* 配色
* Hover
* 动画
* 圆角

禁止改成卡片瀑布流。

---

## 动画规范

页面进入：

duration: 0.8

ease: power4.out

卡片出现：

duration: 0.6

ease: expo.out

Hover：

duration: 0.25

ease: power2.out

数字增长：

duration: 1.2

ease: power3.out

---

## 按钮颜色规范

### 颜色变体

| 变体 | 用途 | 颜色 |
|------|------|------|
| primary | 主要操作（查看营收、恶意斗虫等） | 金黄色 #F6B100 |
| secondary | 次要操作（切换月份、多月份统计等） | 粉色 #FF6B9D |
| danger | 危险操作（关闭图表等） | 红色 #E74C3C |
| success | 成功/跳转操作（跳转直播间等） | 绿色 #27AE60 |
| default | 默认/返回操作（返回主页等） | 灰色 #8E7B50 |
| info | 信息操作（功能指南等） | 青色 #00BCD4 |
| debug | 调试操作（显示Debug等） | 紫色 #9C27B0 |

### CTA动画规范

仅以下按钮保留光效动画（光带扫过 4~6秒一次）：

- HeaderSection 的「进入主站」按钮（primary）
- HeaderSection 的「关注」按钮（secondary-glowing）
- HeaderSection 的「功能指南」按钮（guide-btn）
- FooterSection 的「粉丝站」按钮

其他按钮不使用光效动画，仅使用标准 hover 效果。

禁止：

✗ 高频闪烁

✗ 整体透明度闪烁

✗ 快速缩放

✗ 彩虹变色

目标：

吸引注意力，而非打扰用户。

---

## 重构原则

本项目是视觉重构。

禁止：

* 修改业务逻辑
* 修改接口
* 修改数据结构
* 修改页面布局
* 修改功能入口

允许：

* 重构组件
* 重构样式
* 重构动画
* 重构 Design Token
* 重构视觉层

最终目标：

在保持当前用户使用习惯完全不变的前提下，使项目达到：

"HyperOS 风格 + 现代 SaaS 产品级视觉质量"

---

## 快速参考

### 核心色板

| 色值 | 用途 |
|------|------|
| #F7F1DF | 页面背景 |
| #F6B100 | 主色（金黄色） |
| #FF6B9D | 强调色（粉色） |
| #5D4B24 | 主文字 |
| #8E7B50 | 次文字 |
| #FFF3D0 | 卡片背景 |

### 圆角规范

| 元素 | 圆角 |
|------|------|
| Button | 999px |
| Input | 24px |
| Card | 32px |
| Dialog | 40px |

### 阴影规范

| 状态 | 阴影 |
|------|------|
| 默认 | 0 8px 24px rgba(0,0,0,.04) |
| Hover | 0 12px 32px rgba(0,0,0,.08) |
| Dialog | 0 20px 60px rgba(0,0,0,.12) |

### 毛玻璃规范

| 属性 | 值 |
|------|------|
| blur | 24px |
| background | rgba(255,255,255,.55) |
| border | 1px solid rgba(255,255,255,.35) |

### 动画规范

| 动画类型 | duration | ease |
|----------|----------|------|
| 页面进入 | 0.8s | power4.out |
| 卡片出现 | 0.6s | expo.out |
| Hover | 0.25s | power2.out |
| 数字增长 | 1.2s | power3.out |

### 按钮变体

| 变体 | 颜色 |
|------|------|
| primary | #F6B100 |
| secondary | #FF6B9D |
| danger | #E74C3C |
| success | #27AE60 |
| default | #8E7B50 |
| info | #00BCD4 |
| debug | #9C27B0 |
