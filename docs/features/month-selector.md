# 月份选择器组件

## 概述

统一的月份选择器组件，替代各页面内联的月份选择模态框，统一管理月份范围限制和选择逻辑。

## 组件

- `MonthSelector.vue` — 统一月份选择器组件
- `utils/monthUtils.js` — 月份工具函数

## MonthSelector 组件

### Props

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| visible | Boolean | false | 控制模态框显示 |
| title | String | '选择月份' | 模态框标题 |
| mode | String | 'single' | 模式：`single`（单月）或 `range`（月份范围） |
| min | String | '2025-08' | 最早可选月份 |

### Events

| 事件 | 参数 | 说明 |
|------|------|------|
| confirm | `{ selectedMonth }` 或 `{ startMonth, endMonth }` | 确认选择，YYYYMM 格式 |
| cancel | - | 取消选择 |

### 使用示例

```vue
<MonthSelector
  :visible="showSelector"
  title="切换月份"
  mode="single"
  @confirm="onMonthConfirm"
  @cancel="showSelector = false"
/>
```

### 特性

- 所有 `<input type="month">` 自动添加 `:min` 属性限制
- 月份范围模式自动验证起始不晚于结束
- 打开时自动设置默认值为当前年月
- Scoped 样式，不污染其他组件

## monthUtils 工具函数

### parseYearMonth(ymString)

安全解析 `YYYY-MM` 格式为 `{ year, month }` 对象，避免 `new Date()` 时区偏移问题。

### getMonthRange(startYm, endYm)

获取两个 `YYYY-MM` 格式月份之间的所有月份列表，返回 `YYYYMM` 格式数组。

```javascript
getMonthRange('2026-03', '2026-05')
// 返回: ['202603', '202604', '202605']
```

使用纯整数运算，不依赖 `new Date()`，彻底避免时区偏移问题。

### getCurrentYearMonth()

返回当前 `YYYY-MM` 格式字符串。

### MIN_MONTH

常量 `'2025-08'`，项目最早可选月份。

## 修复的 Bug

### 多月份统计偏移一个月

**问题**：`new Date("2026-03")` 在 UTC+8 时区下解析为 2 月 28 日下午，导致 `getMonth()` 返回 1（2 月）而非 2（3 月）。用户选择 3-5 月时实际请求的是 2-4 月数据。

**修复**：使用 `getMonthRange()` 替代 `new Date()` 进行月份迭代，采用纯整数运算不涉及时区。

**影响文件**：
- `AnchorList.vue` — `performMultiMonthQuery` 函数
- `LiveSessions.vue` — `performMultiMonthQuery` 函数
- `AnchorBattle.vue` — `getMonthsBetween` 方法
