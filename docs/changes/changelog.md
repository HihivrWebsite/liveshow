# 版本变更总览

## 版本历史

### UI 细节修复 — 2026-06-13

- 修复导航表格"直播中"按钮垂直偏上问题
- 页底技术栈 Qwen Code 改为 MimoCode
- SC 低于 10 元改为银色背景 + 深色字体

### 修复粉丝数计算逻辑 — 2026-06-13

- 修复 `new_fans_count` 计算逻辑，改为直接从外部 API 的 `start_attention`/`end_attention` 计算
- 移除 `calculate_session_fans_change` 函数及其端点 `/gift/session_fans_change`
- 简化 `fetch_live_session_data_with_fans_calc` 函数
- 更新文档，补充 `start_attention` 和 `end_attention` 字段说明

### 新增粉丝数直接从外部 API 获取 — 2026-06-13

- `LiveSession` 结构体新增 `start_attention` 和 `end_attention` 字段
- `new_fans_count` 改为直接从外部 API 的 `start_attention`/`end_attention` 计算，不再调用 `/gift/attention` 每日快照
- 移除 `calculate_session_fans_change` 函数及其端点 `/gift/session_fans_change`
- 简化 `fetch_live_session_data_with_fans_calc` 函数，移除 `calculate_fans_change` 参数

### 新增粉丝数功能 + 缓存清理 — 2026-06-13

- 删除后端 4 个未使用的 Attention 缓存辅助函数（`is_attention_cache_valid`、`get_attention_cache`、`put_attention_cache`、`clear_attention_cache_for_month`）
- 前端 API 层新增 `getLiveSessionsWithFans` 和 `getSessionFansChange` 方法
- LiveSessions 页面新增"计算新增粉丝数"按钮，按需调用 `/gift/live_sessions_with_fans`
- 修复新增粉丝数未计算时显示为 `-1` 的问题，改为显示 `-`
- 更新 API 文档，补充 `/gift/live_sessions_with_fans` 端点（`/gift/session_fans_change` 已移除）

### 月份选择器统一 + Bug 修复 — 2026-06-13

- 创建 `MonthSelector.vue` 统一月份选择器组件，替代各页面内联模态框
- 创建 `monthUtils.js` 工具函数，安全处理月份解析
- 修复多月份统计偏移一个月的 bug（`new Date("YYYY-MM")` 时区问题）
- 所有月份选择器添加 `min="2025-08"` 限制
- 整理文档结构，创建 `docs/` 目录，归档旧文档

### v1.0.0 — 初始版本

- 添加 PSPlive 斗虫数据
- 添加 PSP/VR 数据单独/混合显示模式

### v2.0.0 — 2026-01-15

- 整合 WASM 高精度计算功能，移除单独高级模式
- 直播间跳转功能和图表百分比显示
- 修复 Flask async 错误
- 添加 Docker 部署功能并优化响应式设计

### v3.5.0 — 2026-01-17

- 移除 Docker 部署功能
- 优化前端界面

### v3.5.1 — 2026-01-17

- 添加 `.gitignore` 文件以排除编译产物

### v3.5.2 — 2026-01-17

- 更新 README 文件，添加详细系统架构图

### v3.5.5 — 2026-01-17

- 更新 README 文件
- 修复 Mermaid 图表语法兼容性问题

### v3.5.6 — 2026-01-24

- 实现 NavigationTable 导航表格组件
- 实现 BaseCard 基础卡片组件
- 优化前端架构

### v3.5.7 — 2026-01-24

- 实现后端智能缓存机制（5GB 上限，LRU 算法）
- 前端交互功能增强
- Attention 数据批量缓存
- 懒加载模式实现

### v3.5.8 — 2026-01-26

- 修复数值格式化函数无法解析带千位分隔符字符串的问题

### v3.5.9 — 2026-03-06

- 修复缓存统计显示问题
- 优化直播数据展示

### 恶意斗虫功能 — 2026-03-31

- 实现恶意斗虫（多主播对比分析）功能
- 数据获取优化（30秒超时、自动重试、完整性检测）

### 接口修复 — 2026-04-22

- 修复接口异常调用
- 优化缓存逻辑

---

## 详细变更记录

重要变更的详细记录参见各子文档。
