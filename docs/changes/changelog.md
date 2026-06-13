# 版本变更总览

## 版本历史

### v4.1.6 — 2026-06-13 功能指南弹窗

- 新增功能指南弹窗，介绍网站全部19项功能的用途和使用方法
- 主页每次打开自动弹出功能指南，二级页面不自动弹出
- 功能指南按钮放在"进入向阳Hihi粉丝站主站"按钮下方，添加闪烁效果
- 导出截图按钮添加闪烁效果
- 功能指南中导出截图和Super Chat历史功能着重标记（粉色高亮）
- 修复"关注花礼harei"按钮居中问题

### v4.1.5 — 2026-06-13 饼图头像平铺 + 导出排名修复

- 营收饼图和工会对比图改用 CanvasPattern 平铺头像填充扇形（30x30px）
- 图例添加占比百分比显示
- 导出截图新增排名列，显示主播在全部主播中的总排名（非选中主播内的排名）
- 后端 `/gift/avatar_proxy` 端点支持 `uid` 参数，通过 Bilibili UID 获取工会头像

### 功能指南弹窗 — 2026-06-13

- HeaderSection 新增"📖 功能指南"按钮，点击弹出全站功能介绍弹窗
- 弹窗包含19项功能的详细说明（位置、用途、使用方法）
- 首次访问自动弹出指南，通过 localStorage 记录是否已查看
- 支持"不再显示"复选框，用户可永久关闭自动弹出

### v4.1.4 — 2026-06-13 营收饼图和工会对比图添加头像

- 后端 `/gift/avatar_proxy` 端点新增 `uid` 参数支持，通过 Bilibili UID 获取头像
- 营收饼图（showRevenueChart）扇形中心绘制主播头像（20px 圆形裁剪）
- 工会对比图（showVRPSPComparison）扇形中心绘制工会头像（24px 圆形裁剪）
- 新增公共函数 `scaleAvatar` 用于头像圆形裁剪缩放
- 工会头像通过 UID 获取：VirtuaReal (413748120)、PSPlive (454673997)

### v4.1.3 — 2026-06-13 恶意斗虫头像数据点 + 图例头像

- 恶意斗虫（AnchorBattle）图表数据点改用主播头像作为 pointStyle，圆形裁剪缩放至 20px
- 恶意斗虫图例增加主播头像显示（24px 圆形，金色边框）
- 恶意斗虫 renderBattleChart 改为 async 方法，支持头像预加载
- 排名对比（RankComparison）图例同步增加主播头像显示

### v4.1.2 — 2026-06-13 排名对比功能

- 新增排名对比功能，支持多主播各指标排名变化趋势对比
- 新增 `RankComparison.vue` 组件，基于累计值计算每日排名
- NavigationTable 添加排名对比按钮，与恶意斗虫、导出截图并列
- AnchorList 集成排名对比组件，支持模态框方式打开
- 支持13种指标排名对比（直播时长、礼物收入、舰长收入等）
- 图表使用主播头像作为数据点标记，Y轴反转显示排名
- 支持图例控制、数据缓存（2小时）、图表导出为PNG

### v4.1.1 — 2026-06-13 导出截图功能 + UI 修复

- 导出截图功能：支持多选主播后导出数据截图，包含网站顶栏、致谢、数据表格
- Logo 图片打包到本地静态资源，绕过 CORS 限制
- 图片预加载转 base64，确保 html2canvas 正确渲染
- 后端新增根路径静态文件服务
- 直播中 badge 尺寸调优，与跳转按钮视觉一致
- 卡片头像、名字、URL 完美居中对齐
- 导航表格多选列添加全选/全不选复选框

### 打包 Logo 到本地 + 修复头像压扁 — 2026-06-13

- 下载两张 Logo 图片到 `frontend/public/` 目录，导出截图改用本地路径加载，消除 CORS 和外部图床依赖
- 导出截图头像 HTML 改用 `div + overflow:hidden` 包裹，确保头像在表格中不被压扁

### 导出截图 CORS 修复 — 2026-06-13

- 图片预加载转 base64：Logo 和头像通过 canvas 转为 data URL，绕过 s2.loli.net 的 CORS 限制
- html2canvas 设置 `useCORS: false`，不再需要跨域请求
- 容器定位从 `left:-9999px` 改为 `opacity:0; z-index:-1`，避免渲染异常
- 移除表格 `table-layout:fixed`，列宽自适应内容
- 容器宽度从 1000px 调整为 1200px
- 添加 `requestAnimationFrame` 等待 DOM 渲染后再截取

### 详情页放大 + 主页截图导出 — 2026-06-13

- LiveSessions 详情页查询用户头像从 64px 放大至 80px，用户名文字从 1rem 放大至 1.5rem
- AnchorList 新增截图导出功能：支持选择主播 → 选择月份范围 → 导出 PNG 数据截图
- NavigationTable 新增 `selection-change` 事件，支持多选状态同步
- 导出图片包含顶栏、致谢区、说明区、时间范围和主播数据表格

### 导出截图功能修复 — 2026-06-13

- 导出图片顶部改为与网站 HeaderSection 一致的布局（Logo 图片 + 标题 + 致谢文字）
- 导出容器宽度从 900px 增加至 1400px，修复头像压扁和数据截断问题
- 表格设置 `table-layout: fixed` 确保列宽均匀分配
- 导出按钮从 AnchorList 移至 NavigationTable，与恶意斗虫按钮并列显示
- NavigationTable 表头"多选"添加全选/全不选复选框

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

### 恶意对比功能优化 — 2026-06-13

- 导航表格复选框标签从"对比"改为"多选"
- 移除多选数量 10 个的限制
- 恶意斗虫按钮常驻显示，不足 2 个时禁用
- 添加文字说明"选择多个主播后点击进行数据对比"
- 恶意斗虫按钮文字说明改为金黄色大字
- 按钮未选择时不再变淡
- 未选择时点击按钮显示红色抖动提示

### 恶意斗虫功能 — 2026-03-31

- 实现恶意斗虫（多主播对比分析）功能
- 数据获取优化（30秒超时、自动重试、完整性检测）

### 接口修复 — 2026-04-22

- 修复接口异常调用
- 优化缓存逻辑

---

## 详细变更记录

重要变更的详细记录参见各子文档。
