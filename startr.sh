#!/bin/bash
# ============================================================
# Viya PSP DouChong Bang 强制更新启动脚本
# 用法: ./startr.sh
# 说明: 强制拉取最新代码、重新编译前后端、重启服务
# ============================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FRONTEND_DIR="${SCRIPT_DIR}/frontend"
BACKEND_DIR="${SCRIPT_DIR}/rust_backend"
LOG_DIR="${SCRIPT_DIR}/logs"
RELEASE_BINARY="${BACKEND_DIR}/target/release/liveshow-backend"

# 配置
SERVER_PORT="${SERVER_PORT:-2992}"
UPDATE_TIMEOUT="${UPDATE_TIMEOUT:-60}"

mkdir -p "${LOG_DIR}"

log() {
    local ts
    ts="$(date '+%Y-%m-%d %H:%M:%S')"
    echo "[${ts}] $*" | tee -a "${LOG_DIR}/update.log"
}

# ====================
# §1  前置检查
# ====================
log "=========================================="
log "  强制更新模式"
log "=========================================="

for cmd in git rustc cargo npm; do
    if ! command -v "$cmd" &> /dev/null; then
        log "错误: $cmd 未安装"
        exit 1
    fi
done

# ====================
# §2  强制拉取更新
# ====================
log "---------- §2 拉取更新 ----------"

LOCAL_HEAD="$(git rev-parse HEAD 2>/dev/null || echo 'unknown')"

# stash 本地更改
if [[ -n "$(git diff --name-only HEAD 2>/dev/null)" ]]; then
    git stash push -m "startr.sh auto-stash $(date '+%Y-%m-%d %H:%M:%S')" 2>/dev/null || true
    log "已暂存本地更改 (git stash)"
fi

if curl -sf --connect-timeout 10 --max-time 15 "https://github.com" > /dev/null 2>&1; then
    if timeout "${UPDATE_TIMEOUT}" git pull --rebase origin main 2>&1 | tee -a "${LOG_DIR}/update.log"; then
        REMOTE_HEAD="$(git rev-parse HEAD)"
        if [[ "${LOCAL_HEAD}" != "${REMOTE_HEAD}" ]]; then
            log "更新成功！最近提交:"
            git log --oneline -5 | tee -a "${LOG_DIR}/update.log"
        else
            log "已是最新版本"
        fi
    else
        log "警告: git pull 失败，使用本地版本继续"
        git rebase --abort 2>/dev/null || true
    fi
else
    log "警告: 无法连接 GitHub，跳过更新"
fi

VERSION="$(grep '^version' "${BACKEND_DIR}/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/')"
log "当前版本: ${VERSION}"

# ====================
# §3  强制编译前端
# ====================
log "---------- §3 编译前端 ----------"

pushd "${FRONTEND_DIR}" > /dev/null

# npm install（node_modules 不存在或 package.json 有更新时）
if [[ ! -d "node_modules" || "package.json" -nt "node_modules" ]]; then
    log "安装前端依赖..."
    npm install 2>&1 | tee -a "${LOG_DIR}/update.log"
fi

log "编译前端..."
npm run build 2>&1 | tee -a "${LOG_DIR}/update.log"
log "前端编译完成"

popd > /dev/null

# ====================
# §4  强制编译后端
# ====================
log "---------- §4 编译后端 ----------"

# 备份旧二进制
if [[ -f "${RELEASE_BINARY}" ]]; then
    mkdir -p "${BACKEND_DIR}/.backup"
    cp "${RELEASE_BINARY}" "${BACKEND_DIR}/.backup/liveshow-backend.bak"
fi

pushd "${BACKEND_DIR}" > /dev/null

log "编译后端 (release 模式，可能需要几分钟)..."
if ! cargo build --release 2>&1 | tee -a "${LOG_DIR}/update.log"; then
    log "错误: 后端编译失败，尝试回退到备份..."
    if [[ -f "${BACKEND_DIR}/.backup/liveshow-backend.bak" ]]; then
        cp "${BACKEND_DIR}/.backup/liveshow-backend.bak" "${RELEASE_BINARY}"
        log "已回退到备份版本"
    else
        log "没有可用的备份，退出"
        exit 1
    fi
fi
log "后端编译完成"

popd > /dev/null

# ====================
# §5  停止旧进程并重启
# ====================
log "---------- §5 停止旧进程并重启 ----------"

# 方法1: 通过端口查找 PID
OLD_PID=""
if command -v lsof &> /dev/null; then
    OLD_PID="$(lsof -ti :"${SERVER_PORT}" 2>/dev/null || echo "")"
elif command -v ss &> /dev/null; then
    OLD_PID="$(ss -tlnp "sport = :${SERVER_PORT}" 2>/dev/null | grep -oP 'pid=\K[0-9]+' || echo "")"
fi

# 方法2: 通过进程名查找（兜底）
if [[ -z "${OLD_PID}" ]]; then
    OLD_PID="$(pgrep -f 'liveshow-backend' 2>/dev/null || echo "")"
fi

if [[ -n "${OLD_PID}" ]]; then
    log "发现旧进程 (PID: ${OLD_PID})，正在停止..."
    # 先优雅停止
    kill ${OLD_PID} 2>/dev/null || true
    sleep 2
    # 检查是否还活着，强制杀死
    for pid in ${OLD_PID}; do
        if kill -0 "${pid}" 2>/dev/null; then
            log "进程 ${pid} 未响应，强制终止..."
            kill -9 "${pid}" 2>/dev/null || true
        fi
    done
    sleep 1
    log "旧进程已清除"
else
    log "未发现旧进程"
fi

# 清理可能残留的旧备份
rm -rf "${BACKEND_DIR}/.backup" 2>/dev/null || true

# 版本 banner
echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║   Viya PSP DouChong Bang                        ║"
echo "║   版本: ${VERSION}                                    ║"
echo "║   端口: ${SERVER_PORT}                                    ║"
echo "║   模式: Release                                   ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""
log "启动服务中... 按 Ctrl+C 停止"

cd "${BACKEND_DIR}"
exec "${RELEASE_BINARY}"
