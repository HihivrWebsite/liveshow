# 维阿PSP斗虫榜

一个用于展示维阿（VirtuaReal）和PSP（PSPlive）工会主播直播数据的应用。

## 项目概述

维阿PSP斗虫榜是一个用于展示维阿（VirtuaReal）和PSP（PSPlive）工会主播直播数据的应用。该项目采用前后端分离架构，使用Rust/Axum作为后端API服务，Vue 3/Vite作为前端用户界面。

### 主要技术栈
- **后端**: Rust + Axum + Tokio
- **前端**: Vue 3 + Vite + Vue Router + Chart.js
- **API协议**: RESTful API
- **数据来源**: 从外部API（vr.qianqiuzy.cn 和 psp.qianqiuzy.cn）获取主播数据

### 核心功能
- 展示维阿和PSP工会主播的收入数据
- 按月份查看历史数据
- 查看特定主播的详细直播会话信息
- 数据可视化（营收占比饼图和工会对比图）
- 按工会筛选功能（VR/PSP/全部）

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

### 参数说明
- `/gift` 和 `/gift/by_month`:
  - `filter` (可选): `all`(默认), `vr`, `psp`

- `/gift/by_month`:
  - `month` (必需): 月份，格式为 YYYYMM (如: 202509)

- `/gift/live_sessions`:
  - `room_id` (必需): 直播间ID
  - `union` (必需): `VirtuaReal` 或 `PSPlive`
  - `month` (可选): 月份，格式为 YYYYMM

## 前端路由

- `/` - 主播排行榜页面
- `/by-month` - 按月查看主播数据
- `/live-sessions` - 直播会话详情页面

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

### Docker部署（可选）
项目可以使用Docker容器化部署，分别构建前端和后端镜像。

## 维护与扩展

### 当前特性
- 高性能异步后端处理
- 实时主播数据展示
- 收入统计和可视化
- 工会对比分析
- 月份历史数据查询
- 响应式设计

### 可能的改进方向
- 添加更多数据维度的分析
- 实现数据缓存机制以减少外部API调用
- 增加用户认证和个性化功能
- 添加数据导出功能
- 优化移动端用户体验

## 故障排除

### 常见问题
1. **API请求失败**: 检查外部API服务是否正常运行
2. **跨域问题**: 后端已配置CORS，确保前端请求正确
3. **构建错误**: 确保所有依赖项都已正确安装

### 调试信息
- 后端启动时会在控制台输出调试信息
- API请求失败时会有相应的错误日志
- 前端控制台会显示API响应和错误信息
