#!/bin/bash
# ============================================================
# Viya PSP DouChong Bang 启动脚本（带自动更新功能）
# ============================================================

# ====================
# §0  全局设置 & 配置加载
# ====================
set -euo pipefail

# 路径变量
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FRONTEND_DIR="${SCRIPT_DIR}/frontend"
BACKEND_DIR="${SCRIPT_DIR}/rust_backend"
LOG_DIR="${SCRIPT_DIR}/logs"

# 从 Cargo.toml 提取版本号
get_local_version() {
    grep '^version' "${BACKEND_DIR}/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/'
}

# 加载 .deploy.env 配置（如果存在）
if [[ -f "${SCRIPT_DIR}/.deploy.env" ]]; then
    # shellcheck source=/dev/null
    source "${SCRIPT_DIR}/.deploy.env"
fi

# 配置默认值（.deploy.env 可覆盖）
AUTO_UPDATE="${AUTO_UPDATE:-true}"
AUTO_CONFIRM="${AUTO_CONFIRM:-false}"
UPDATE_TIMEOUT="${UPDATE_TIMEOUT:-60}"
SERVER_PORT="${SERVER_PORT:-2992}"
REMOTE="${REMOTE:-origin}"
BRANCH="${BRANCH:-main}"
MIN_DISK_SPACE_MB="${MIN_DISK_SPACE_MB:-2048}"

# 初始化日志目录
mkdir -p "${LOG_DIR}"

# 日志函数：同时输出到终端和日志文件
log() {
    local timestamp
    timestamp="$(date '+%Y-%m-%d %H:%M:%S')"
    echo "[${timestamp}] $*" | tee -a "${LOG_DIR}/update.log"
}

log "========== 启动脚本开始执行 =========="
log "项目目录: ${SCRIPT_DIR}"
log "当前版本: $(get_local_version)"

# ====================
# §1  前置检查
# ====================
log "---------- §1 前置检查 ----------"

# 检查 git
if ! command -v git &> /dev/null; then
    log "错误: git 未安装，请先安装 git"
    exit 1
fi
log "git 已安装: $(git --version)"

# 检查 rustc
if ! command -v rustc &> /dev/null; then
    log "错误: Rust 工具链未安装，请先安装 rustup"
    exit 1
fi
log "Rust 已安装: $(rustc --version)"

# 检查 cargo
if ! command -v cargo &> /dev/null; then
    log "错误: cargo 未安装"
    exit 1
fi
log "cargo 已安装: $(cargo --version)"

# 检查 npm
if ! command -v npm &> /dev/null; then
    log "错误: npm 未安装，请先安装 Node.js"
    exit 1
fi
log "npm 已安装: $(npm --version)"

# 检查网络连接
log "检查网络连接..."
if ! curl -sf --connect-timeout 10 https://github.com > /dev/null 2>&1; then
    log "警告: 无法连接到 github.com（超时10秒），自动更新将不可用"
    NETWORK_OK=false
else
    log "网络连接正常"
    NETWORK_OK=true
fi

# 检查是否在 main 分支
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo 'unknown')"
if [[ "${CURRENT_BRANCH}" != "${BRANCH}" ]]; then
    log "警告: 当前分支为 '${CURRENT_BRANCH}'，不是 '${BRANCH}'，自动更新可能不适用"
fi

# 检查工作区是否有未提交更改
if [[ -n "$(git diff --name-only HEAD 2>/dev/null)" ]]; then
    log "警告: 工作区有未提交的更改，自动更新将跳过"
    HAS_UNCOMMITTED=true
else
    HAS_UNCOMMITTED=false
fi

# 检查磁盘空间（至少 2GB）
AVAILABLE_SPACE_MB="$(df -m "${SCRIPT_DIR}" 2>/dev/null | awk 'NR==2 {print $4}' || echo "0")"
if [[ "${AVAILABLE_SPACE_MB}" -lt "${MIN_DISK_SPACE_MB}" ]]; then
    log "警告: 磁盘可用空间不足 (${AVAILABLE_SPACE_MB}MB < ${MIN_DISK_SPACE_MB}MB)，编译可能失败"
fi

# ====================
# §2  自动更新
# ====================
log "---------- §2 自动更新 ----------"

UPDATED=false

# 如果 AUTO_UPDATE != true，跳过
if [[ "${AUTO_UPDATE}" != "true" ]]; then
    log "自动更新已禁用 (AUTO_UPDATE=false)，跳过更新检查"
elif [[ "${HAS_UNCOMMITTED}" == "true" ]]; then
    log "工作区有未提交更改，跳过自动更新"
elif [[ "${NETWORK_OK}" != "true" ]]; then
    log "网络不可用，跳过自动更新"
else
    # git fetch origin main（带超时）
    log "正在获取远程更新..."
    if ! timeout "${UPDATE_TIMEOUT}" git fetch "${REMOTE}" "${BRANCH}" 2>&1 | tee -a "${LOG_DIR}/update.log"; then
        log "警告: git fetch 失败或超时，跳过更新"
    else
        # 比较 HEAD 和 origin/main
        LOCAL_HEAD="$(git rev-parse HEAD)"
        REMOTE_HEAD="$(git rev-parse "${REMOTE}/${BRANCH}")"

        if [[ "${LOCAL_HEAD}" == "${REMOTE_HEAD}" ]]; then
            log "代码已是最新，无需更新"
        else
            log "发现新版本，更新日志："
            git log HEAD.."${REMOTE}/${BRANCH}" --oneline | tee -a "${LOG_DIR}/update.log"

            # 如果 AUTO_CONFIRM != true，提示用户确认
            if [[ "${AUTO_CONFIRM}" != "true" ]]; then
                read -r -p "是否执行更新？(y/N) " response
                if [[ ! "${response}" =~ ^[Yy]$ ]]; then
                    log "用户取消更新"
                else
                    DO_PULL=true
                fi
            else
                DO_PULL=true
            fi

            if [[ "${DO_PULL:-false}" == "true" ]]; then
                # 检测更新前的文件状态，用于智能跳过
                CHANGED_FILES="$(git diff --name-only HEAD "${REMOTE}/${BRANCH}" 2>/dev/null || echo "")"

                # 执行 git pull
                log "正在拉取更新..."
                if ! timeout "${UPDATE_TIMEOUT}" git pull "${REMOTE}" "${BRANCH}" 2>&1 | tee -a "${LOG_DIR}/update.log"; then
                    log "错误: git pull 失败"
                    exit 1
                fi

                UPDATED=true
                NEW_VERSION="$(get_local_version)"
                log "更新完成，当前版本: ${NEW_VERSION}"

                # 智能判断是否需要重编译
                NEED_FRONTEND_BUILD=false
                NEED_BACKEND_BUILD=false

                if echo "${CHANGED_FILES}" | grep -q "^frontend/"; then
                    NEED_FRONTEND_BUILD=true
                    log "检测到前端文件变化，需要重新编译前端"
                fi
                if echo "${CHANGED_FILES}" | grep -q "^rust_backend/"; then
                    NEED_BACKEND_BUILD=true
                    log "检测到后端文件变化，需要重新编译后端"
                fi
                if echo "${CHANGED_FILES}" | grep -q "^start.sh\|^\.deploy\.env"; then
                    log "检测到部署配置变化，请重新运行脚本"
                fi
            fi
        fi
    fi
fi

# ====================
# §3  依赖安装 & 编译
# ====================
log "---------- §3 依赖安装 & 编译 ----------"

# --- 前端构建 ---
NEED_FRONTEND_BUILD="${NEED_FRONTEND_BUILD:-true}"

if [[ "${UPDATED}" == "false" && -d "${FRONTEND_DIR}/dist" ]]; then
    log "未检测到更新且前端 dist/ 已存在，跳过前端构建"
else
    if [[ "${NEED_FRONTEND_BUILD}" == "false" && -d "${FRONTEND_DIR}/dist" ]]; then
        log "前端文件无变化且 dist/ 已存在，跳过前端构建"
    else
        log "开始构建前端..."
        pushd "${FRONTEND_DIR}" > /dev/null

        # npm install（如果 node_modules 不存在或 package.json 有更新）
        if [[ ! -d "node_modules" || "package.json" -nt "node_modules" ]]; then
            log "安装前端依赖..."
            if ! npm install 2>&1 | tee -a "${LOG_DIR}/update.log"; then
                log "警告: npm install 失败，尝试继续..."
            fi
        else
            log "前端依赖已是最新，跳过 npm install"
        fi

        # npm run build
        log "编译前端..."
        if ! npm run build 2>&1 | tee -a "${LOG_DIR}/update.log"; then
            log "错误: 前端编译失败"
            popd > /dev/null
            exit 1
        fi
        log "前端编译完成"
        popd > /dev/null
    fi
fi

# --- 后端构建 ---
NEED_BACKEND_BUILD="${NEED_BACKEND_BUILD:-true}"

# release 二进制路径
RELEASE_BINARY="${BACKEND_DIR}/target/release/liveshow-backend"

if [[ "${UPDATED}" == "false" && -f "${RELEASE_BINARY}" ]]; then
    # 检查源码是否比二进制新（手动 git pull 后的情况）
    NEWEST_SRC="$(find "${BACKEND_DIR}/src" -name '*.rs' -newer "${RELEASE_BINARY}" 2>/dev/null | head -1)"
    if [[ -n "${NEWEST_SRC}" ]]; then
        log "检测到源文件比二进制新，需要重新编译"
        UPDATED="true"
    else
        log "未检测到更新且 release 二进制已存在，跳过后端构建"
    fi
else
    if [[ "${NEED_BACKEND_BUILD}" == "false" && -f "${RELEASE_BINARY}" ]]; then
        log "后端文件无变化且 release 二进制已存在，跳过后端构建"
    else
        log "开始构建后端 (release 模式)..."
        pushd "${BACKEND_DIR}" > /dev/null

        # 备份旧二进制
        BACKUP_DIR="${BACKEND_DIR}/.backup"
        if [[ -f "${RELEASE_BINARY}" ]]; then
            mkdir -p "${BACKUP_DIR}"
            cp "${RELEASE_BINARY}" "${BACKUP_DIR}/liveshow-backend.bak"
            log "已备份旧二进制到 ${BACKUP_DIR}/"
        fi

        # cargo build --release
        if ! cargo build --release 2>&1 | tee -a "${LOG_DIR}/update.log"; then
            log "错误: 后端编译失败"
            # 尝试回退到备份
            if [[ -f "${BACKUP_DIR}/liveshow-backend.bak" ]]; then
                log "正在回退到备份二进制..."
                cp "${BACKUP_DIR}/liveshow-backend.bak" "${RELEASE_BINARY}"
                log "已回退到备份版本"
            else
                log "没有可用的备份，退出"
                popd > /dev/null
                exit 1
            fi
        else
            log "后端编译完成"
        fi

        popd > /dev/null
    fi
fi

# ====================
# §4  启动服务
# ====================
log "---------- §4 启动服务 ----------"

# 检查端口占用
if command -v lsof &> /dev/null; then
    PORT_PID="$(lsof -ti :${SERVER_PORT} 2>/dev/null || echo "")"
elif command -v ss &> /dev/null; then
    PORT_PID="$(ss -tlnp "sport = :${SERVER_PORT}" 2>/dev/null | grep -oP 'pid=\K[0-9]+' || echo "")"
else
    PORT_PID=""
fi

if [[ -n "${PORT_PID}" ]]; then
    log "警告: 端口 ${SERVER_PORT} 已被进程 ${PORT_PID} 占用"
    log "请先停止该进程或更改 SERVER_PORT 配置"
    exit 1
fi

# 版本信息 banner
VERSION="$(get_local_version)"
echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║   Viya PSP DouChong Bang                        ║"
echo "║   版本: ${VERSION}                                    ║"
echo "║   端口: ${SERVER_PORT}                                    ║"
echo "║   模式: Release                                   ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""
log "启动服务中... 按 Ctrl+C 停止"

# 切换到后端目录执行（与 cargo run 行为一致，确保 ./dist/index.html 路径正确）
cd "${BACKEND_DIR}"
exec "${RELEASE_BINARY}"
