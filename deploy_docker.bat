@echo off
REM 维阿PSP斗虫榜一键部署脚本 (Docker版 - Windows)

echo ===========================================
echo 维阿PSP斗虫榜一键部署脚本 (Docker版 - Windows)
echo ===========================================

REM 检查是否安装了Docker
docker --version >nul 2>&1
if errorlevel 1 (
    echo 错误: 未找到Docker。请先安装Docker Desktop。
    echo Docker Desktop下载地址: https://www.docker.com/products/docker-desktop
    pause
    exit /b 1
)

REM 检查是否安装了Docker Compose
docker-compose --version >nul 2>&1
if errorlevel 1 (
    echo 警告: 未找到docker-compose命令。
    echo 尝试使用docker compose (Docker Desktop v2.0+内置)...
    docker compose version >nul 2>&1
    if errorlevel 1 (
        echo 错误: 未找到Docker Compose。请确保Docker Desktop已启用Compose功能。
        echo Docker Compose安装指南: https://docs.docker.com/compose/install/
        pause
        exit /b 1
    )
    set COMPOSE_CMD=docker compose
) else (
    set COMPOSE_CMD=docker-compose
)

echo Docker和Docker Compose检查通过。

REM 检查当前目录是否有docker-compose.yml
if not exist "docker-compose.yml" (
    echo 错误: 当前目录缺少docker-compose.yml文件。
    echo 请确保在项目根目录下运行此脚本。
    pause
    exit /b 1
)

echo.
echo 即将开始部署维阿PSP斗虫榜应用...
echo 应用将在 http://localhost:2992 上运行
echo.

echo 按任意键继续部署，或按Ctrl+C取消...
pause >nul

echo.
echo 正在构建并启动服务...
%COMPOSE_CMD% up -d --build

echo.
echo 等待服务启动...
timeout /t 10 /nobreak >nul

REM 检查服务状态
echo 服务状态:
%COMPOSE_CMD% ps

echo.
echo ===========================================
echo 部署完成！
echo.
echo 维阿PSP斗虫榜已成功部署。
echo 访问地址: http://localhost:2992
echo.
echo 常用管理命令:
echo   查看日志: %COMPOSE_CMD% logs
echo   停止服务: %COMPOSE_CMD% down
echo   重启服务: %COMPOSE_CMD% restart
echo ===========================================

pause