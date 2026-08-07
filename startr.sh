#!/bin/bash
# ============================================================
# Viya PSP DouChong Bang 强制更新启动脚本
# 用法: ./startr.sh
# 说明: 即使工作区有未提交更改，也会强制拉取最新代码并启动
# ============================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

echo "=========================================="
echo "  强制更新模式"
echo "=========================================="

# 检查 git
if ! command -v git &> /dev/null; then
    echo "错误: git 未安装"
    exit 1
fi

# 检查网络
if ! curl -sf --connect-timeout 10 --max-time 15 "https://github.com" > /dev/null 2>&1; then
    echo "警告: 无法连接 GitHub，跳过更新，直接启动"
    exec bash start.sh
fi

# 保存当前 HEAD
LOCAL_HEAD="$(git rev-parse HEAD 2>/dev/null || echo 'unknown')"

# 强制拉取更新（stash 本地更改）
echo "正在拉取最新代码..."
if git stash push -m "startr.sh auto-stash $(date '+%Y-%m-%d %H:%M:%S')" 2>/dev/null; then
    echo "已暂存本地更改 (git stash)"
fi

if timeout 60 git pull --rebase origin main 2>&1; then
    REMOTE_HEAD="$(git rev-parse HEAD)"
    if [[ "${LOCAL_HEAD}" != "${REMOTE_HEAD}" ]]; then
        echo ""
        echo "更新成功！最近提交:"
        git log --oneline -5
        echo ""
    else
        echo "已是最新版本"
    fi
else
    echo "警告: git pull 失败，使用本地版本继续"
fi

# 启动 start.sh（此时工作区已干净，自动更新不会被跳过）
echo ""
echo "=========================================="
echo "  启动服务..."
echo "=========================================="
exec bash start.sh
