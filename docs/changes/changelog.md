# 版本变更总览

## 版本历史

### v4.2.0 — 2026-06-14 清理死代码 + 提示文字 + 表头改名 + 指南重排

- `api/index.js` 删除未使用的 `getAvatar` 和 `getAvatarProxyUrl` 函数（旧 `/gift/avatar` 和 `/gift/avatar_proxy` 端点）
- `NavigationTable.vue` 标题下方添加橙色提示文字"📸 点击导出截图一键截图"
- `NavigationTable.vue` 表头"多选"改为"全选"
- `HeaderSection.vue` 功能指南重排：导出截图、排名对比、恶意斗虫、Super Chat历史移至最前，新增"核心功能"分类，共七个分类

### v4.1.9 — 2026-06-14 移除未压缩头像回退 + 图例改用压缩头像

- `avatarCache.js` 移除 `getAvatar`/`getAvatarByUid` 对 `/gift/avatar_proxy` 的回退，仅从 localStorage 读取
- 删除 `fetchAvatar` 和 `preloadAllAvatars` 函数
- `loadAvatarAndUpdate` 简化为同步读取 localStorage
- AnchorList 营收饼图图例改用 `getAvatarSync` 获取压缩头像
- AnchorList VR/PSP 图例改用 `getAvatarByUid` 预加载后绑定响应式变量
- AnchorList VR/PSP 饼图扇区头像改用 `scaleAvatar` 圆形裁剪
- RankComparison 图例改用 `getAvatar` 预加载后绑定 `avatarDataUrls`
- AnchorBattle 图例改用 `getAvatar` 预加载后绑定 `avatarDataUrls`

### v4.1.8 — 2026-06-14 后端定时批量下载压缩头像 + 前端简化

- 新增 `compress_avatar` 函数，使用 image crate 将头像压缩为 40x40 JPEG
- 新增 `batch_download_avatars` 定时任务，每 48 小时批量下载压缩所有主播头像
- 新增 `/gift/avatars/batch` 端点，返回所有已缓存头像的 base64 map
- 前端 `avatarCache.js` 移除 `defaultAvatars.js` 依赖，改用批量接口预填充缓存
- 新增 `fetchAllAvatars` 函数，页面加载时一次性获取所有头像 base64
- 简化 `getAvatar` / `getAvatarSync` / `getAvatarByUid`，移除重试和默认头像逻辑
- 删除 `defaultAvatars.js` 文件
- Cargo.toml 新增 `image = "0.25"` 和 `base64 = "0.22"` 依赖

### v4.1.7 — 2026-06-14 默认头像集成 + 导航表格头像同步修复

- avatarCache.js 集成 defaultAvatars.js 内置默认头像，getAvatar/getAvatarByUid 优先返回默认头像（瞬间显示），然后后台刷新
- 新增 getAvatarSync 同步获取头像 base64（用于 img src 绑定）
- 新增 loadAvatarAndUpdate 异步加载并更新回调
- NavigationTable 改用 getAvatarSync + loadAvatarAndUpdate，消除导航表格与卡片头像加载不同步问题

### v4.1.6 — 2026-06-13 头像缓存持久化+重试机制

- AvatarCacheEntry 新增 image_bytes 字段，缓存图片字节避免重复请求 Bilibili CDN
- 新增 save_avatar_to_file / load_avatar_from_file，头像持久化到 cache_data/avatars/ 目录
- 服务器重启后自动从磁盘加载头像缓存
- 前端头像加载添加重试机制（最多3次，间隔1秒）
- 前端 getAvatar/getAvatarByUid 添加重试机制（最多 3 次，间隔 1 秒）

### v4.1.4 — 2026-06-13 头像 localStorage 本地缓存

- 新增 `avatarCache.js` 工具模块，实现头像 localStorage 缓存（版本化、自动刷新）
- `getAvatar(roomId)` / `getAvatarByUid(uid)` 支持缓存优先 + 后台静默刷新
- `preloadAllAvatars(anchors)` 批量预加载主播头像
- `scaleAvatar(img, size)` 圆形裁剪缩放头像
- AnchorList 营收饼图、VR/PSP 对比图、导出截图改用缓存头像
- AnchorBattle / RankComparison 图表头像预加载改用缓存
- NavigationTable 导航表格头像改用缓存 base64 加载
- LiveSessions 主播头像改用缓存加载

### v4.1.3 — 2026-06-13 缓存逻辑全面修复

- 修复 size_estimate 估算失效（改用实际序列化大小）
- 修复 is_past_month 时区不一致（统一使用 Local）
- 修复 clear_live_sessions_cache_for_month 子字符串匹配误删（改用精确键匹配）
- 新增 clear_attention_cache_for_month、clear_avatar_cache 函数
- 新增 POST /cache/clear 端点，支持手动清除指定主播月份缓存
- 修复服务器重启后缓存时间戳重置（使用文件修改时间）
- 修复磁盘加载后冗余写入（只写内存不重复写磁盘）
- CacheManager.hit_count/miss_count 改为 AtomicUsize
- 清理 6 个未使用的死代码函数
- 新增启动时自动清理 3 个月前的历史缓存
- 修复 AnchorBattle 缓存命中时 sessionsData 重置为空
- 修复 AnchorBattle refreshData 不更新 sessions 和缓存
- 修复 RankComparison refreshData 缺 try/finally
- 修复 performExport DOM 节点泄漏
- 修复 CacheStats setInterval 泄漏
- AnchorBattle/RankComparison 模块级颜色状态移入组件 data

### v4.1.3 — 2026-06-13 功能指南弹窗

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
