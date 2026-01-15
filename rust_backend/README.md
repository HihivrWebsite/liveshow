# 维阿PSP斗虫榜 - Rust后端

这是一个使用Rust和Axum框架构建的高性能后端服务，用于提供维阿和PSP工会主播的直播数据。

## 技术架构

- **后端**: Rust + Axum + Tokio
- **前端**: Vue 3 + Vite (位于 ../frontend 目录)

## 环境要求

- Rust (推荐1.70+版本)
- Cargo (随Rust一起安装)

## 安装与运行

### 1. 安装Rust

访问 [Rust官网](https://www.rust-lang.org/tools/install) 下载并安装Rust工具链。

### 2. 安装依赖

```bash
cargo build
```

### 3. 运行开发服务器

```bash
cargo run
```

服务器将在 http://0.0.0.0:2992 上启动。

### 4. 构建生产版本

```bash
cargo build --release
```

构建后的可执行文件位于 `target/release/liveshow-backend`。

## API 端点

- `GET /api/` - 获取主播列表数据
- `GET /api/by_month` - 按月份获取主播数据
- `GET /api/live_sessions` - 获取直播会话详情

## 特性

- 高性能异步处理
- 内部缓存机制
- CORS支持
- 错误处理
- 日志记录

## 项目结构

```
rust_backend/
├── Cargo.toml      # 依赖配置
├── src/
│   └── main.rs     # 主应用文件
```

## 部署

在生产环境中，建议使用进程管理器（如systemd、supervisor）来管理服务，或使用容器化部署。

## 许可证

[项目许可证信息]