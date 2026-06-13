# 图表可视化功能

## 概述

基于 Chart.js 实现的数据可视化，提供多种图表类型用于数据分析。

## 组件

- `ChartComponent.vue` — 图表组件

## 图表类型

### 营收占比饼图

展示各主播在选定月份中的收入占比分布。

### VR/PSP 工会对比图

对比 VirtuaReal 和 PSPlive 两个工会的整体数据。

### 直播数据折线图

展示特定主播的直播数据趋势（在直播会话详情页面）。

### 恶意斗虫对比图

超大尺寸（8000x1800px）多主播多指标对比折线图。

## 技术实现

- 基于 Chart.js 库
- 支持响应式设计
- 支持导出为 PNG 图片
- 自定义点形状区分不同数据系列

## 数据处理工具

`utils/dataProcessor.js` 提供：

- `formatCurrency(value)` — 货币格式化
- `formatNumber(value)` — 数字格式化（支持千位分隔符）
- `calculatePercentage(value, total)` — 百分比计算
- `calculateDuration(startTime, endTime)` — 时长计算
- `calculateTotalRevenue(item)` — 总收入计算
- `debounce(func, delay)` — 防抖函数
- `throttle(func, limit)` — 节流函数
- `formatDate(date, format)` — 日期格式化
