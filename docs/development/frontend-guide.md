# 前端开发指南

## 技术栈

- Vue 3（Composition API）
- Vite 构建工具
- Vue Router 路由管理
- Chart.js 数据可视化
- Axios HTTP 请求

## 目录结构

```
frontend/src/
├── api/
│   └── index.js            # API 接口封装
├── components/             # Vue 组件
│   ├── MonthSelector.vue   # 统一月份选择器
│   ├── AnchorList.vue      # 主播列表
│   ├── AnchorBattle.vue    # 恶意斗虫
│   ├── BaseCard.vue        # 基础卡片
│   ├── BaseAnchorCard.vue  # 主播卡片
│   ├── CacheStats.vue      # 缓存统计
│   ├── ChartComponent.vue  # 图表
│   ├── ErrorPage.vue       # 错误页面
│   ├── ExpandedView.vue    # 全展开视图
│   ├── FooterSection.vue   # 页脚
│   ├── HeaderSection.vue   # 页眉
│   ├── LiveSessions.vue    # 直播会话
│   ├── NavigationTable.vue # 导航表格
│   ├── SimpleAnchorList.vue# 简化主播列表
│   └── SuperChatDetail.vue # SC详情
├── composables/
│   └── useGlobalCardState.js # 全局卡片状态管理
├── router/
│   └── index.js            # 路由定义
├── utils/
│   └── dataProcessor.js    # 数据处理工具
├── assets/
│   └── style.css           # 全局样式
├── App.vue                 # 根组件
└── main.js                 # 入口文件
```

## 组件规范

### 组件命名

- 使用 PascalCase 命名组件文件
- 组件文件名与组件名一致

### Composition API

所有组件使用 Vue 3 Composition API（`<script setup>`）。

### 组件通信

- 父子组件：props / emit
- 跨组件状态：`useGlobalCardState` composable
- 全局数据：provide / inject

## API 调用

所有 API 请求通过 `api/index.js` 封装：

```javascript
import { anchorAPI } from '@/api'

// 获取主播列表
const data = await anchorAPI.getAnchors(filter, month)

// 获取直播会话
const sessions = await anchorAPI.getLiveSessions(roomId, union, month)

// 获取 SC 历史
const scData = await anchorAPI.getSuperChatHistory(roomId, union)
```

## 路由定义

| 路径 | 组件 | 说明 |
|------|------|------|
| `/` | AnchorList | 主播排行榜 |
| `/by-month` | AnchorList | 按月查看 |
| `/live-sessions` | LiveSessions | 直播会话详情 |
| `/superchat-detail` | SuperChatDetail | SC 详情 |
| `/expanded-view` | ExpandedView | 全展开视图 |
| `/expanded-view/:source` | ExpandedView | 特定来源展开视图 |

## 全局卡片状态管理

`composables/useGlobalCardState.js` 提供：

- `provideGlobalCardState()` — 提供全局卡片状态
- `toggleAllCards()` — 切换所有卡片的展开/收起状态

## 数据处理

`utils/dataProcessor.js` 提供格式化和计算函数：

- `formatCurrency(value)` — 货币格式化
- `formatNumber(value)` — 数字格式化（支持千位分隔符）
- `calculatePercentage(value, total)` — 百分比计算
- `calculateDuration(startTime, endTime)` — 时长计算
- `calculateTotalRevenue(item)` — 总收入计算
- `debounce(func, delay)` — 防抖
- `throttle(func, limit)` — 节流
- `formatDate(date, format)` — 日期格式化

`utils/monthUtils.js` 提供月份工具函数：

- `parseYearMonth(ymString)` — 安全解析 YYYY-MM，避免时区偏移
- `getMonthRange(startYm, endYm)` — 获取月份范围（返回 YYYYMM 数组）
- `getCurrentYearMonth()` — 获取当前 YYYY-MM
- `MIN_MONTH` — 最早可选月份常量（'2025-08'）

## 代码风格

- 遵循 ESLint 标准
- 使用 Composition API
- 保持项目现有风格一致
