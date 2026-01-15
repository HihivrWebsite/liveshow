# 维阿PSP斗虫榜前端 (Vue + Vite)

这是一个使用Vue 3和Vite构建的前端项目，用于展示维阿和PSP工会主播的直播数据。

## 项目特性

- 使用Vue 3 Composition API
- Vite构建工具，快速开发体验
- 轻量化设计，优化性能
- 保持原有配色方案 (#FFC633, #33CC99 等)
- 响应式设计
- 图表可视化功能

## 项目结构

```
frontend/
├── index.html
├── package.json
├── vite.config.js
├── src/
│   ├── main.js
│   ├── App.vue
│   ├── api/
│   │   └── index.js
│   ├── components/
│   │   ├── AnchorList.vue
│   │   ├── LiveSessions.vue
│   │   ├── ErrorPage.vue
│   │   └── ChartComponent.vue
│   ├── router/
│   │   └── index.js
│   ├── utils/
│   │   └── dataProcessor.js
│   └── assets/
│       └── style.css
```

## 安装和运行

### 前提条件

- Node.js (版本 >= 14.18.0 或 >= 16.0.0)
- npm 或 yarn

### 安装依赖

```bash
npm install
```

### 开发模式运行

```bash
npm run dev
```

这将在 http://localhost:3000 启动开发服务器。

### 构建生产版本

```bash
npm run build
```

构建后的文件将位于 `dist/` 目录中。

### 预览生产构建

```bash
npm run preview
```

## API 代理配置

开发服务器配置了代理，将 `/api` 请求转发到后端服务器 (http://localhost:2992)。

## 功能说明

1. **主播列表页面** - 显示维阿和PSP工会主播的收入数据
2. **直播详情页面** - 显示特定主播的详细直播会话数据
3. **图表功能** - 营收占比饼图和工会对比图
4. **数据筛选** - 按工会筛选（VR/PSP/全部）
5. **月份选择** - 查看不同月份的历史数据

## 技术栈

- Vue 3 (Composition API)
- Vue Router 4
- Vite 4
- Chart.js 4
- Axios
- CSS3 动画

## 性能优化

- 组件懒加载
- 防抖和节流函数
- 有效的内存管理（图表实例销毁）
- 按需导入依赖

## 与后端集成

前端通过API与Flask后端通信，获取主播数据和直播会话信息。API端点包括：

- `/` - 获取主播列表
- `/by_month` - 按月份获取主播数据
- `/live_sessions` - 获取直播会话详情

## 维护

项目保持轻量化设计，易于维护和扩展。

## 许可证

[项目许可证信息]