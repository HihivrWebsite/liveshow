# 使用官方Node.js运行时作为基础镜像
FROM node:18-alpine AS frontend-builder

# 设置工作目录
WORKDIR /app/frontend

# 复制前端项目文件
COPY frontend/package*.json ./
COPY frontend/vite.config.js ./

# 安装前端依赖
RUN npm ci --only=production

# 复制前端源代码
COPY frontend/src ./src
COPY frontend/public ./public
COPY frontend/index.html ./

# 构建前端应用
RUN npm run build

# 使用官方Rust镜像作为基础镜像
FROM rust:1.75-alpine AS backend-builder

# 安装构建依赖
RUN apk add --no-cache musl-dev

# 设置工作目录
WORKDIR /app/backend

# 复制Cargo文件
COPY rust_backend/Cargo.toml .
COPY rust_backend/Cargo.lock .

# 创建一个虚拟的src目录以允许Cargo下载依赖
RUN mkdir -p src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制后端源代码
COPY rust_backend/src ./src

# 构建后端应用
RUN cargo build --release

# 运行时阶段
FROM alpine:latest

# 安装必要的运行时库
RUN apk --no-cache add ca-certificates

# 创建非root用户
RUN addgroup -g 1001 -S appuser && \
    adduser -u 1001 -S appuser -G appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制前端构建产物
COPY --from=frontend-builder --chown=appuser:appuser /app/frontend/dist ./dist

# 从构建阶段复制后端可执行文件
COPY --from=backend-builder --chown=appuser:appuser /app/backend/target/release/liveshow-backend ./server

# 更改权限
RUN chmod +x ./server

# 切换到非root用户
USER appuser

# 暴露端口
EXPOSE 2992

# 启动命令
CMD ["./server"]