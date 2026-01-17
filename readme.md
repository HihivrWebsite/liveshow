# 维阿PSP斗虫榜

一个用于展示维阿（VirtuaReal）和PSP（PSPlive）工会主播直播数据的应用。

## 项目概述

维阿PSP斗虫榜是一个用于展示维阿（VirtuaReal）和PSP（PSPlive）工会主播直播数据的应用。该项目采用前后端分离架构，使用Rust/Axum作为后端API服务，Vue 3/Vite作为前端用户界面。

## 快速开始

### 传统部署方式

参考下面的安装与运行部分。

### 主要技术栈
- **后端**: Rust + Axum + Tokio
- **前端**: Vue 3 + Vite + Vue Router + Chart.js
- **API协议**: RESTful API
- **数据来源**: 从外部API获取主播数据

### 核心功能
- 展示维阿和PSP工会主播的收入数据
- 按月份查看历史数据
- 查看特定主播的详细直播会话信息
- 数据可视化（营收占比饼图和工会对比图）
- 按工会筛选功能（VR/PSP/全部）
- Super Chat历史记录查看
- 回归分析和聚类分析功能

## 项目结构

```
liveshow/
├── APIDOC.md                       # API文档
├── readme.md                       # 项目介绍
├── start.bat                       # Windows启动脚本
├── start.sh                        # Linux/Mac启动脚本
├── frontend/                       # Vue前端项目
│   ├── package.json                # 前端依赖配置
│   ├── babel.config.js             # Babel配置文件
│   ├── index.html                  # 前端入口HTML文件
│   ├── jsconfig.json               # JavaScript配置
│   ├── vite.config.js              # Vite构建工具配置
│   ├── vue.config.js               # Vue CLI配置
│   ├── public/                     # 静态资源
│   ├── dist/                       # 构建输出目录
│   ├── src/                        # 前端源代码
│   │   ├── main.js                 # 前端入口文件
│   │   ├── App.vue                 # 根组件
│   │   ├── api/                    # API接口定义
│   │   │   └── index.js            # API请求封装
│   │   ├── assets/                 # 静态资源
│   │   │   └── style.css           # 样式文件
│   │   ├── components/             # Vue组件
│   │   │   ├── AnchorList.vue      # 主播列表组件
│   │   │   ├── SimpleAnchorList.vue # 简化主播列表组件
│   │   │   ├── LiveSessions.vue    # 直播会话组件
│   │   │   ├── ChartComponent.vue  # 图表组件
│   │   │   ├── ErrorPage.vue       # 错误页面组件
│   │   │   ├── HeaderSection.vue   # 页眉组件
│   │   │   ├── FooterSection.vue   # 页脚组件
│   │   │   ├── SuperChatDetail.vue # SC详情组件
│   │   │   └── HelloWorld.vue      # 示例组件
│   │   ├── router/                 # 路由配置
│   │   │   └── index.js            # 路由定义
│   │   └── utils/                  # 工具函数
│   │       └── dataProcessor.js    # 数据处理工具
│   └── node_modules/               # 依赖包目录
└── rust_backend/                   # Rust后端项目
    ├── Cargo.toml                  # Rust依赖配置
    ├── Cargo.lock                  # Rust依赖锁定文件
    ├── README.md                   # 后端项目说明
    ├── src/                        # 后端源代码
    │   └── main.rs                 # 后端主程序
    ├── dist/                       # 分发目录
    └── target/                     # 编译输出目录
```

## 系统架构图

```mermaid
graph TB
    subgraph Frontend ["前端应用 (Vue 3)"]
        app[App.vue - 根组件]
        header[HeaderSection.vue - 页眉组件]
        anchorlist[AnchorList.vue - 主播列表组件\n- 营收占比分析\n- VR/PSP对比图\n- 回归分析\n- 聚类分析]
        livesessions[LiveSessions.vue - 直播会话组件\n- 直播数据折线图\n- SC历史记录\n- 多月份统计]
        footer[FooterSection.vue - 页脚组件]
        superchatdetail[SuperChatDetail.vue - SC详情组件\n- SC历史详情\n- 时间段筛选]
        errorpage[ErrorPage.vue - 错误页面组件]
        chartcomp[ChartComponent.vue - 图表组件\n- Chart.js实现\n- 响应式图表]

        router[Vue Router - 路由管理\n- / 路由到AnchorList\n- /by-month 路由到AnchorList\n- /live-sessions 路由到LiveSessions\n- /superchat-detail 路由到SuperChatDetail]
        axios[Axios API - HTTP客户端\n- 请求拦截\n- 响应拦截\n- 错误处理]
        dataproc[dataProcessor.js - 数据处理工具\n- formatCurrency: 格式化货币\n- formatNumber: 格式化数字\n- calculatePercentage: 计算百分比\n- calculateDuration: 计算持续时间\n- calculateTotalRevenue: 计算总收入\n- debounce: 防抖函数\n- throttle: 节流函数]
        api_def[api/index.js - API接口定义\n- getAnchors: 获取主播列表\n- getAnchorsByMonth: 按月份获取主播数据\n- getLiveSessions: 获取直播会话详情\n- getSuperChatHistory: 获取SC历史数据]
    end

    subgraph Backend ["后端应用 (Rust/Axum)"]
        main_rs[main.rs - 主程序入口\n- tokio异步运行时\n- 全局HTTP客户端\n- 请求计数器]
        axum_router[Axum Router - 路由处理器\n- GET /gift\n- GET /gift/by_month\n- GET /gift/live_sessions\n- GET /gift/sc]

        subgraph Middleware ["HTTP中间件"]
            cors[CorsLayer - 跨域处理\n- 多域名支持\n- 预检缓存]
            compress[CompressionLayer - 响应压缩\n- br/gzip压缩\n- 性能优化]
            trace[TraceLayer - 请求跟踪\n- 日志记录\n- 性能监控]
        end

        subgraph BusinessLogic ["业务逻辑层"]
            get_anc[get_anchors - 获取主播列表\n- 参数过滤\n- 并发请求VR/PSP]
            get_anc_month[get_anchors_by_month - 按月获取主播数据\n- 月份参数处理]
            get_sess[get_live_sessions - 获取直播会话详情\n- 房间ID验证\n- 工会识别]
            get_sc_hist[get_sc_history - 获取SC历史数据\n- 数据清洗\n- 格式转换]
            fetch_anc[fetch_anchor_data - 获取主播数据\n- VR/PSP过滤\n- 数据合并\n- 总营收计算]
            fetch_ext[fetch_external_api - 获取外部API数据\n- JSON解析\n- 错误处理\n- 数据映射]
            fetch_sess[fetch_live_session_data - 获取直播会话数据\n- 月份参数\n- 工会判断]
        end

        subgraph DataModels ["数据模型"]
            anchor_model[Anchor - 主播数据模型\n- anchor_name: 主播名\n- attention: 关注数\n- effective_days: 有效天\n- fans_count: 粉丝团\n- gift: 礼物收入\n- guard: 舰长收入\n- guard_1, guard_2, guard_3: 舰长数量\n- live_duration: 直播时长\n- room_id: 房间ID\n- status: 状态\n- super_chat: SC收入\n- total_revenue: 总营收\n- union: 工会]
            session_model[LiveSession - 直播会话模型\n- start_time: 开始时间\n- end_time: 结束时间\n- duration_minutes: 持续分钟\n- 舰长变化: start/end_guard_1/2/3\n- 粉丝团变化: start/end_fans_count\n- danmaku_count: 弹幕数\n- gift: 礼物收入\n- guard: 舰长收入\n- super_chat: SC收入\n- total_revenue: 总营收\n- title: 标题]
            sc_model[SuperChat - SC消息模型\n- send_time: 发送时间\n- uname: 用户名\n- uid: 用户ID\n- price: 价格\n- message: 消息内容]
            api_resp[ApiResponse - API响应模型\n- anchors: 主播列表\n- refresh_time: 刷新时间\n- filter: 过滤条件]
        end
    end

    subgraph ExternalAPI ["外部API数据源"]
        vr_api[VR API - 虚拟主播工会]
        psp_api[PSP API - PSP工会]
    end

    subgraph ProcessFlow ["数据处理流程"]
        user_req[用户请求]
        front_call[前端API调用]
        back_route[后端路由处理]
        ext_api[外部API获取]
        data_proc[数据处理]
        resp_back[响应返回]
        front_render[前端渲染]
    end

    %% 连接关系
    app --> header
    app --> anchorlist
    app --> livesessions
    app --> footer
    app --> superchatdetail
    app --> errorpage
    app --> chartcomp
    app --> router

    anchorlist --> axios
    livesessions --> axios
    superchatdetail --> axios

    axios --> front_call
    front_call --> back_route
    back_route --> get_anc
    back_route --> get_anc_month
    back_route --> get_sess
    back_route --> get_sc_hist

    get_anc --> fetch_anc
    get_anc_month --> fetch_anc
    get_sess --> fetch_sess
    get_sc_hist --> fetch_ext

    fetch_anc --> ext_api
    fetch_sess --> ext_api
    fetch_ext --> ext_api

    ext_api --> vr_api
    ext_api --> psp_api

    vr_api --> data_proc
    psp_api --> data_proc

    data_proc --> resp_back
    resp_back --> front_render

    front_render --> anchorlist
    front_render --> livesessions
    front_render --> superchatdetail
```

## 详细功能模块说明

### 前端模块

#### 1. 主要组件
- **App.vue**: 根组件，包含头部、主内容区和底部
  - 使用defineAsyncComponent异步加载组件
  - 集成HeaderSection和FooterSection组件
  - 使用Vue Router进行页面路由管理

- **HeaderSection.vue**: 页眉组件，包含站点Logo和标题
  - 包含两个Logo图片链接
  - 动态标题显示，根据路由参数变化
  - 包含跳转到主站和关注创建者的按钮

- **FooterSection.vue**: 页脚组件，包含版权信息和技术栈
  - 显示版权信息和当前年份
  - 显示技术栈信息
  - 包含跳转到主站的按钮

- **AnchorList.vue**: 主播列表组件，展示维阿和PSP工会主播数据
  - 支持按工会筛选（all/vr/psp）
  - 支持按月份查看数据
  - 提供营收占比饼图分析
  - 提供VR/PSP工会对比图
  - 支持回归分析和聚类分析
  - 包含查看详细数据按钮，可跳转到直播会话页面
  - 响应式设计，适配不同屏幕尺寸

- **LiveSessions.vue**: 直播会话组件，展示特定主播的详细直播数据
  - 显示指定主播的直播会话详情
  - 提供直播数据折线图
  - 显示SC历史记录
  - 支持多月份统计数据
  - 包含跳转到直播间按钮

- **SuperChatDetail.vue**: SC详情组件，展示Super Chat历史记录
  - 显示Super Chat历史消息
  - 按时间排序显示
  - 包含发送者信息和金额

- **ErrorPage.vue**: 错误页面组件，处理页面错误和404
  - 显示错误信息
  - 提供返回首页按钮

#### 2. 功能组件
- **ChartComponent.vue**: 图表组件，用于数据可视化
  - 基于Chart.js实现
  - 支持多种图表类型
  - 响应式设计

- **SimpleAnchorList.vue**: 简化主播列表组件
  - 简化版的主播列表
  - 适用于嵌入其他页面

#### 3. 工具模块
- **api/index.js**: API接口定义，包含与后端通信的方法
  - anchorAPI.getAnchors(filter, month): 获取主播列表
  - anchorAPI.getAnchorsByMonth(month, filter): 按月份获取主播数据
  - anchorAPI.getLiveSessions(roomId, union, month): 获取直播会话详情
  - anchorAPI.getSuperChatHistory(roomId, union): 获取SC历史数据
  - 包含请求和响应拦截器

- **utils/dataProcessor.js**: 数据处理工具，包含格式化和计算函数
  - formatCurrency(value): 格式化货币数值
  - formatNumber(value): 格式化数字
  - calculatePercentage(value, total): 计算百分比
  - calculateDuration(startTime, endTime): 计算持续时间
  - calculateTotalRevenue(item): 计算总收入
  - debounce(func, delay): 防抖函数
  - throttle(func, limit): 节流函数
  - formatDate(date, format): 日期格式化

- **router/index.js**: 路由配置，定义页面路由
  - / 路由到AnchorList组件
  - /by-month 路由到AnchorList组件
  - /live-sessions 路由到LiveSessions组件
  - /superchat-detail 路由到SuperChatDetail组件
  - /error 路由到ErrorPage组件
  - 通配符路由处理404错误

### 后端模块

#### 1. HTTP路由
- **GET /gift**: 获取主播列表数据
  - 处理函数: get_anchors
  - 参数: filter (可选, all/vr/psp)
  - 返回: ApiResponse结构

- **GET /gift/by_month**: 按月份获取主播数据
  - 处理函数: get_anchors_by_month
  - 参数: month (必需), filter (可选)
  - 返回: ByMonthResponse结构

- **GET /gift/live_sessions**: 获取直播会话详情
  - 处理函数: get_live_sessions
  - 参数: room_id (必需), union (必需), month (可选)
  - 返回: LiveSessionResponse结构

- **GET /gift/sc**: 获取SC历史数据
  - 处理函数: get_sc_history
  - 参数: room_id (必需), union (可选)
  - 返回: SCResponse结构

- **GET /assets/***: 静态资源服务
  - 使用ServeDir提供静态资源

- **GET /favicon.ico**: 网站图标
  - 返回网站图标文件

- **fallback**: 首页处理
  - 返回index.html文件

#### 2. 业务逻辑
- **fetch_anchor_data(filter, month)**: 获取主播数据，支持VR/PSP/all过滤
  - 根据过滤条件决定请求哪个工会数据
  - 支持按月份获取数据
  - 计算总营收
  - 按总营收排序

- **fetch_anchor_data_by_url(url)**: 根据URL获取主播数据
  - 从指定URL获取主播数据
  - 解析JSON响应
  - 映射到Anchor结构

- **fetch_external_api(client, api_url)**: 从外部API获取数据
  - 发送HTTP请求到外部API
  - 解析JSON响应
  - 映射到Anchor结构
  - 错误处理

- **fetch_live_session_data(room_id, union, month)**: 获取直播会话数据
  - 根据房间ID、工会和月份获取直播会话
  - 构造外部API请求URL
  - 解析返回的会话数据

- **fetch_live_session_from_api(client, api_url)**: 从API获取直播会话数据
  - 发送HTTP请求到外部API
  - 解析JSON响应
  - 映射到LiveSession结构

- **fetch_sc_history**: 获取SC历史数据
  - 根据房间ID和工会获取SC历史
  - 解析返回的SC消息列表

#### 3. 数据模型
- **Anchor**: 主播数据模型
  - anchor_name: String - 主播名称
  - attention: i64 - 关注数
  - effective_days: i32 - 有效天数
  - fans_count: i32 - 粉丝团数量
  - gift: f64 - 礼物收入
  - guard: f64 - 舰长收入
  - guard_1/2/3: i32 - 舰长数量（总督/提督/舰长）
  - live_duration: String - 直播时长
  - live_time: String - 直播时间
  - month: String - 月份
  - room_id: u64 - 房间ID
  - status: i32 - 状态（0-未开播，1-直播中）
  - super_chat: f64 - SC收入
  - title: String - 标题
  - total_revenue: f64 - 总营收
  - union: String - 工会

- **LiveSession**: 直播会话模型
  - start_time: String - 开始时间
  - end_time: String - 结束时间
  - duration_minutes: i32 - 持续分钟数
  - start_guard_1/2/3: i32 - 开始时舰长数量
  - end_guard_1/2/3: i32 - 结束时舰长数量
  - start_fans_count: i32 - 开始时粉丝团数量
  - end_fans_count: i32 - 结束时粉丝团数量
  - danmaku_count: i32 - 弹幕数
  - gift: f64 - 礼物收入
  - guard: f64 - 舰长收入
  - super_chat: f64 - SC收入
  - total_revenue: f64 - 总营收
  - title: String - 标题

- **SuperChat**: SC消息模型
  - send_time: String - 发送时间
  - uname: String - 用户名
  - uid: u64 - 用户ID
  - price: f64 - 价格
  - message: String - 消息内容

- **ApiResponse**: API响应模型
  - anchors: Vec<Anchor> - 主播列表
  - refresh_time: String - 刷新时间
  - filter: String - 过滤条件

- **ByMonthResponse**: 按月响应模型
  - anchors: Vec<Anchor> - 主播列表
  - refresh_time: String - 刷新时间
  - filter: String - 过滤条件
  - month: String - 月份

- **LiveSessionResponse**: 直播会话响应模型
  - sessions: Vec<LiveSession> - 会话列表
  - room_id: String - 房间ID
  - queried_user: String - 查询用户
  - union: String - 工会
  - title: String - 标题
  - refresh_time: String - 刷新时间

- **SCResponse**: SC历史响应模型
  - room_id: u64 - 房间ID
  - month: String - 月份
  - list: Vec<SuperChat> - SC列表

## 项目获取与安装

### 从GitHub获取项目

```bash
# 克隆项目到本地
git clone https://github.com/HihivrWebsite/liveshow.git
cd liveshow

# 或者如果项目在私有仓库中
git clone git@github.com:HihivrWebsite/liveshow.git
cd liveshow
```

### 如何升级项目

如果您已经克隆了项目并希望升级到最新版本，请按照以下步骤操作：

```bash
# 进入项目目录
cd liveshow

# 拉取最新的代码
git pull origin main

# 更新前端依赖
cd frontend
npm install

# 返回项目根目录
cd ..

# 如果需要，更新后端依赖并重新构建
cd rust_backend
cargo build
```

### 环境要求

#### 后端（Rust）
- Rust 1.70+
- Cargo（随Rust一起安装）

#### 前端（Vue）
- Node.js 14.18+ 或 16.0+
- npm 或 yarn

### 安装与运行

#### 方法一：使用启动脚本（推荐）

项目根目录下提供了便捷的启动脚本：

##### Windows:
```bash
# 在项目根目录运行
.\start.bat
```

##### Linux/Mac:
```bash
# 在项目根目录运行
chmod +x ./start.sh
./start.sh
```

#### 方法二：手动启动

##### 后端启动:
```bash
cd rust_backend
cargo run
```
后端服务器将在 http://0.0.0.0:2992 启动。

##### 前端启动:
```bash
cd frontend
npm install
npm run dev
```
前端开发服务器将在 http://localhost:3000 启动。

## API端点

### 主播数据相关
- `GET /gift` - 获取主播列表数据
- `GET /gift/by_month` - 按月份获取主播数据
- `GET /gift/live_sessions` - 获取直播会话详情
- `GET /gift/sc` - 获取SC历史数据

### 参数说明
- `/gift` 和 `/gift/by_month`:
  - `filter` (可选): `all`(默认), `vr`, `psp`

- `/gift/by_month`:
  - `month` (必需): 月份，格式为 YYYYMM (如: 202509)

- `/gift/live_sessions`:
  - `room_id` (必需): 直播间ID
  - `union` (必需): `VirtuaReal` 或 `PSPlive`
  - `month` (可选): 月份，格式为 YYYYMM

- `/gift/sc`:
  - `room_id` (必需): 直播间ID
  - `union` (可选): `VirtuaReal` 或 `PSPlive`

## 前端路由

- `/` - 主播排行榜页面
- `/by-month` - 按月查看主播数据
- `/live-sessions` - 直播会话详情页面
- `/superchat-detail` - Super Chat详情页面

## 构建生产版本

### 后端构建:
```bash
cd rust_backend
cargo build --release
```
生成的可执行文件位于 `target/release/liveshow-backend`

### 前端构建:
```bash
cd frontend
npm run build
```
构建结果位于 `dist/` 目录

## 特性

- 高性能异步后端处理
- 实时主播数据展示
- 收入统计和可视化
- 工会对比分析
- 月份历史数据查询
- 响应式设计
- 数据图表可视化
- 直播会话详情
- Super Chat历史记录
- 回归分析功能
- 聚类分析功能

## 开发约定

### 代码风格
- Rust代码遵循Rust官方风格指南
- Vue组件使用Composition API
- JavaScript代码遵循ESLint标准

### API交互
- 前端通过Axios与后端API进行数据交互
- 所有API请求都有适当的错误处理
- 使用防抖和节流优化用户体验

### 数据处理
- 后端负责从外部API获取原始数据并进行预处理
- 前端负责数据展示和可视化
- 收入数据自动计算总营收（礼物收入 + 舰长收入 + SC收入）

## 部署说明

### 生产环境部署
1. 构建前端静态文件 (`npm run build`)
2. 构建后端可执行文件 (`cargo build --release`)
3. 配置Web服务器（如Nginx）以提供静态文件服务并代理API请求到后端

## 维护与扩展

### 当前特性
- 高性能异步后端处理
- 实时主播数据展示
- 收入统计和可视化
- 工会对比分析
- 月份历史数据查询
- 响应式设计
- 数据图表可视化
- 直播会话详情
- Super Chat历史记录
- 回归分析功能
- 聚类分析功能

### 可能的改进方向
- 添加更多数据维度的分析
- 实现数据缓存机制以减少外部API调用
- 增加用户认证和个性化功能
- 添加数据导出功能
- 优化移动端用户体验
- 增加实时数据推送功能

## 故障排除

### 常见问题
1. **API请求失败**: 检查外部API服务是否正常运行
2. **跨域问题**: 后端已配置CORS，确保前端请求正确
3. **构建错误**: 确保所有依赖项都已正确安装

### 调试信息
- 后端启动时会在控制台输出调试信息
- API请求失败时会有相应的错误日志
- 前端控制台会显示API响应和错误信息