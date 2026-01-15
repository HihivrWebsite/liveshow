#!/bin/bash

# 维阿PSP斗虫榜一键部署脚本 (Docker版)

set -e

echo "==========================================="
echo "维阿PSP斗虫榜一键部署脚本 (Docker版)"
echo "==========================================="

# 检查是否安装了Docker
if ! command -v docker &> /dev/null; then
    echo "错误: 未找到Docker。请先安装Docker。"
    echo "Docker安装指南: https://docs.docker.com/get-docker/"
    exit 1
fi

# 检查是否安装了Docker Compose
if ! command -v docker-compose &> /dev/null; then
    echo "警告: 未找到docker-compose命令。"
    echo "尝试使用docker compose (Docker Desktop v2.0+内置)..."
    if ! docker compose version &> /dev/null; then
        echo "错误: 未找到Docker Compose。请先安装Docker Compose。"
        echo "Docker Compose安装指南: https://docs.docker.com/compose/install/"
        exit 1
    fi
    COMPOSE_CMD="docker compose"
else
    COMPOSE_CMD="docker-compose"
fi

echo "Docker和Docker Compose检查通过。"

# 检查当前目录是否有docker-compose.yml
if [ ! -f "docker-compose.yml" ]; then
    echo "错误: 当前目录缺少docker-compose.yml文件。"
    echo "请确保在项目根目录下运行此脚本。"
    exit 1
fi

echo ""
echo "即将开始部署维阿PSP斗虫榜应用..."
echo "应用将在 http://localhost:2992 上运行"
echo ""

read -p "按任意键继续部署，或按Ctrl+C取消..."

echo ""
echo "正在构建并启动服务..."
$COMPOSE_CMD up -d --build

echo ""
echo "等待服务启动..."
sleep 10

# 检查服务状态
echo "服务状态:"
$COMPOSE_CMD ps

echo ""
echo "==========================================="
echo "部署完成！"
echo ""
echo "维阿PSP斗虫榜已成功部署。"
echo "访问地址: http://localhost:2992"
echo ""
echo "常用管理命令:"
echo "  查看日志: $COMPOSE_CMD logs -f"
echo "  停止服务: $COMPOSE_CMD down"
echo "  重启服务: $COMPOSE_CMD restart"
echo "==========================================="