# SuperChat 卡片设计规范

## 设计原则

保留B站 SuperChat 的标志性四色（银/蓝/金/红），同时融入新的毛玻璃视觉体系。

---

## SC 价格分层

| 价格区间 | 颜色名称 | 色值 | 用途 |
|----------|----------|------|------|
| < ¥10 | 银色 | #C0C0C0 | 小额打赏 |
| ¥10-100 | 蓝色 | #3A5FCD | 中等打赏 |
| ¥100-1000 | 金色 | #FFD700 | 大额打赏 |
| ≥ ¥1000 | 红色 | #FF0000 | 豪华打赏 |

---

## SC 卡片样式

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

```scss
.sc-item {
  // 基础样式（毛玻璃）
  background: rgba(255, 255, 255, 0.55);
  backdrop-filter: blur(24px);
  border: 1px solid rgba(255, 255, 255, 0.35);
  border-radius: 32px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.04);
  
  // 布局
  display: flex;
  padding: 0; // 内部 padding 由子元素控制
  overflow: hidden; // 裁剪左侧彩色条
  
  // Hover 效果（GSAP）
  &:hover {
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.08);
  }
}

// 左侧彩色条
.sc-color-bar {
  width: 6px;
  flex-shrink: 0;
}

// 价格等级对应彩色条颜色
.sc-silver .sc-color-bar { background: #C0C0C0; }
.sc-blue .sc-color-bar { background: #3A5FCD; }
.sc-gold .sc-color-bar { background: #FFD700; }
.sc-red .sc-color-bar { background: #FF0000; }

// 内容区域
.sc-body {
  flex: 1;
  padding: 16px 20px;
}

// 头部信息
.sc-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

// 用户名
.sc-name {
  font-weight: 600;
  font-size: 1rem;
  color: #5D4B24; // 主文字色
}

// UID
.sc-uid {
  font-size: 0.85rem;
  color: #8E7B50; // 次文字色
}

// 时间
.sc-time {
  font-size: 0.8rem;
  color: #8E7B50;
}

// 消息内容区域
.sc-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 16px;
}

// 消息文本
.sc-message {
  flex: 1;
  font-size: 1.1rem;
  line-height: 1.6;
  color: #5D4B24;
  word-wrap: break-word;
  white-space: pre-wrap;
}

// 价格容器
.sc-price-container {
  flex-shrink: 0;
  text-align: right;
}

// 价格数字
.sc-price {
  font-weight: 700;
  font-size: 2rem;
  
  // 根据价格等级变色
  .sc-silver & { color: #8E7B50; } // 银色用次文字色
  .sc-blue & { color: #3A5FCD; }
  .sc-gold & { color: #F6B100; } // 金色用主色
  .sc-red & { color: #E74C3C; } // 红色用危险色
}
```

---

## 与旧版对比

| 维度 | 旧版 | 新版 |
|------|------|------|
| 背景 | 实心彩色 | 毛玻璃半透明 |
| 颜色区分 | 整块背景色 | 左侧6px彩色条 |
| 文字 | 白色+黑色描边 | 深棕色，无描边 |
| 圆角 | 20px | 32px |
| 阴影 | 橙色阴影 | 中性灰色阴影 |
| 价格 | 金色/红色 | 对应等级颜色 |
| 布局 | 垂直堆叠 | 水平+垂直混合 |

---

## 响应式设计

### 平板（≤1024px）
- 价格字号：1.8rem
- 消息字号：1rem

### 手机（≤768px）
- 卡片圆角：24px
- 价格字号：1.5rem
- 消息字号：0.95rem
- SC内容改为垂直堆叠

### 小屏（≤480px）
- 价格字号：1.2rem
- 消息字号：0.9rem
- 左侧彩色条：4px

---

## 动画规范

### SC 卡片出现动画
```javascript
// 使用 GSAP，逐个错开出现
gsap.from('.sc-item', {
  opacity: 0,
  y: 20,
  duration: 0.6,
  ease: 'expo.out',
  stagger: 0.05 // 每张卡片间隔 0.05 秒
})
```

### SC 卡片 Hover
```javascript
// 使用 GSAP
gsap.to('.sc-item', {
  y: -4,
  boxShadow: '0 12px 32px rgba(0, 0, 0, 0.08)',
  duration: 0.25,
  ease: 'power2.out'
})
```

### 价格数字动画
```javascript
// 价格从 0 增长到目标值
gsap.to('.sc-price', {
  innerText: targetPrice,
  duration: 1.2,
  ease: 'power3.out',
  snap: { innerText: 0.01 }
})
```

---

## Design Token 变量

```css
:root {
  /* SC 颜色 */
  --sc-silver: #C0C0C0;
  --sc-blue: #3A5FCD;
  --sc-gold: #FFD700;
  --sc-red: #FF0000;
  
  /* SC 圆角 */
  --sc-radius: 32px;
  
  /* SC 彩色条宽度 */
  --sc-bar-width: 6px;
  
  /* SC 价格字号 */
  --sc-price-font-size: 2rem;
}
```

---

## 注意事项

1. **保留B站原色**：银/蓝/金/红四色必须保留，这是B站SuperChat的标志性设计
2. **颜色仅用于区分**：新设计中，颜色通过左侧彩色条体现，而非整块背景
3. **文字可读性**：深棕色文字在毛玻璃背景上可读性优于白色描边文字
4. **价格数字**：保持大字号（2rem），但颜色跟随价格等级
5. **Hover效果**：统一使用GSAP，不使用CSS transition
