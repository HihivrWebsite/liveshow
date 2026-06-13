# 开发环境搭建

## 环境要求

### 后端（Rust）

- Rust 1.70+
- Cargo（随 Rust 一起安装）

安装方式：访问 [Rust 官网](https://www.rust-lang.org/tools/install) 下载安装。

### 前端（Vue）

- Node.js 14.18+ 或 16.0+
- npm 或 yarn

## 获取项目

```bash
git clone https://github.com/HihivrWebsite/liveshow.git
cd liveshow
```

## 安装依赖

### 后端

```bash
cd rust_backend
cargo build
```

### 前端

```bash
cd frontend
npm install
```

## 启动开发服务器

### 方法一：使用启动脚本（推荐）

Windows：
```bash
.\start.bat
```

Linux/Mac：
```bash
chmod +x ./start.sh
./start.sh
```

### 方法二：手动启动

后端：
```bash
cd rust_backend
cargo run
```
服务器将在 `http://0.0.0.0:2992` 启动。

前端：
```bash
cd frontend
npm run dev
```
开发服务器将在 `http://localhost:3000` 启动。

## 构建生产版本

后端：
```bash
cd rust_backend
cargo build --release
```
生成的可执行文件位于 `target/release/liveshow-backend`。

前端：
```bash
cd frontend
npm run build
```
构建结果位于 `dist/` 目录。

## 升级项目

```bash
cd liveshow
git pull origin main
cd frontend && npm install && cd ..
cd rust_backend && cargo build && cd ..
```
