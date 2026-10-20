# 测试指南

## 测试工具

项目包含多个测试和验证工具：

### 响应式测试

| 文件 | 用途 |
|------|------|
| `responsive_test.html` | 手动响应式测试页面 |
| `responsive_test_optimized.html` | 优化版响应式测试页面 |
| `scroll_test.html` | 横向滚动功能测试页面 |
| `test_responsive.js` | 自动化响应式测试脚本 |

### 功能验证

| 文件 | 用途 |
|------|------|
| `verify_changes.js` | 验证代码修改对功能的影响 |
| `verify_changes_fixed.js` | 确认问题已被解决 |
| `snapshot.html` | 捕获和比较 UI 状态 |

### 服务器测试

| 文件 | 用途 |
|------|------|
| `test_server_only.bat` | 仅测试后端服务状态 |
| `test_cache.bat` | 测试缓存功能 |

### 缓存监控

| 文件 | 用途 |
|------|------|
| `cache_monitor.py` | 实时缓存监控脚本 |

## 测试流程

### 1. 启动服务

```bash
.\start.bat  # Windows
./start.sh   # Linux/Mac
```

### 2. 响应式测试

访问 `http://localhost:3000`，使用浏览器开发者工具：

1. 模拟不同设备尺寸
2. 测试 320px、360px、480px 等窄屏尺寸
3. 验证表格横向滚动
4. 确认所有信息完整显示

### 3. 功能测试

1. 主播列表加载和筛选
2. 月份切换
3. 直播会话详情查看
4. SC 历史查看
5. 恶意斗虫对比功能
6. 图表导出功能

### 4. 缓存测试

#### 4.1 既有底层缓存

```bash
# 第一次请求（缓存未命中）
curl "http://localhost:2992/gift/attention?room_id=1713546334&month=202604&union=VirtuaReal"

# 第二次请求（应缓存命中）
curl "http://localhost:2992/gift/attention?room_id=1713546334&month=202604&union=VirtuaReal"

# 查看缓存统计
curl "http://localhost:2992/cache/stats"
```

#### 4.2 Gift 10 秒响应缓存

执行静态检查和回归测试，失败时定位并修复任务范围内的问题后重跑：

```bash
cd rust_backend
cargo fmt --check
cargo check
cargo test
```

使用完整服务进行运行时验证：

```bash
AUTO_UPDATE=false bash start.sh
```

对 10 个 `/gift*` 路由逐个检查首次、重复和过期请求：

```bash
# 首次应为 MISS；10 秒内重复应为 HIT；超过 10 秒再次请求应为 MISS
curl -i "http://localhost:2992/gift?filter=all"
sleep 1
curl -i "http://localhost:2992/gift?filter=all"
sleep 10
curl -i "http://localhost:2992/gift?filter=all"
```

覆盖路由：

```text
/gift
/gift/by_month
/gift/live_sessions
/gift/live_sessions_with_fans
/gift/sc
/gift/attention
/gift/avatar
/gift/avatar_proxy
/gift/avatars/batch
/gift/fans_overlap
```

必测场景：

1. **同键并发**：mock handler 记录执行次数，50 个完全相同请求冷启动并发只执行一次；一个 `leader` 加其余等待者，响应字节一致，诊断头出现 `MISS` 和 `COALESCED`。
2. **10 秒过期**：TTL 内命中，等待 10 秒后执行 handler 并再次 `MISS`。
3. **键隔离**：`room_id`、`month`、`filter`、`union` 或未知参数不同绝不串键；唯一参数名换序可归并，重复参数名换序不归并。
4. **响应类型**：200 JSON、400、500、JPEG 和批量头像均保留状态码、正文与 `Content-Type`；错误响应 10 秒内同样命中。
5. **压缩兼容**：分别发送 `Accept-Encoding: gzip` 与 `identity`，均可正确解码，证明缓存位于 Compression 内侧。
6. **单飞取消**：取消 leader future 后锁自动释放，等待者能够接管或完成，不悬挂。
7. **容量与绕过**：验证 512 条和 64 MiB LRU 淘汰、单响应 16 MiB 上限，以及 `no-store`、非空 `Vary`、缺少可用长度（`Content-Length` 或精确 body size hint）的响应不写入。
8. **诊断统计**：`GET /cache/stats` 包含 `gift_response_cache` 的 TTL、命中、合并命中、miss、过期、淘汰、绕过、缓冲错误、条目、字节、在途键和命中率；`misses` 不得解释为远程 API 调用数。

清除与 generation 验证：

```bash
# 首次填充短缓存
curl -i "http://localhost:2992/gift/attention?room_id=1713546334&month=202604&union=VirtuaReal"

# 有效清除后下一次必须 MISS，且慢请求完成后不能把旧结果写回
curl -i -X POST "http://localhost:2992/cache/clear?room_id=1713546334&month=202604"
curl -i "http://localhost:2992/gift/attention?room_id=1713546334&month=202604&union=VirtuaReal"

# 无参数/无效参数应保持 ok:false，且不清短缓存
curl -i -X POST "http://localhost:2992/cache/clear"
```

回归首页筛选、月份切换、直播会话、SC、头像、粉丝重合度以及 `/cache/stats` 原有字段。

#### 4.3 共享上游缓存与真实刷新

在 `rust_backend` 执行 `cargo test --locked`；在 `frontend` 执行 `npm run test:cache` 和 `npm run build`。前端缓存测试使用 Node 22+，不需要连接生产接口。

新增回归覆盖：不同调用入口及筛选共享源缓存、显式当月缓存、VR/PSP 隔离、成功空结果落盘与过期、上游失败不污染缓存、历史 24 小时到期、每日 01:30 失效后旧条目不复活、同键并发/取消、LRU、内存和磁盘清理以及清理时的在途请求。HTTP 测试只连接随机回环端口，检查状态码及调用方请求头。

前端测试直接执行两个 Vue 组件的刷新方法，验证先清服务器再请求、范围去重与串行执行、服务端拒绝时保留本地数据。源指标及上线采样步骤见[共享缓存说明](../features/upstream-cache.md)。

### 5. 性能测试

观察日志中的缓存命中情况：

```bash
# 统计缓存命中次数
grep "🎯 \[缓存命中\]" server.log | wc -l

# 统计外部API调用次数
grep "🌐 \[API调用\]" server.log | wc -l
```

## 测试报告

`RESPONSIVE_TEST_REPORT.md` 记录了响应式设计测试的结果和发现的问题。

## 常见测试问题

1. **API 请求失败** — 检查外部 API 服务是否正常
2. **跨域问题** — 后端已配置 CORS
3. **构建错误** — 确保所有依赖已安装
4. **缓存未命中** — 检查是否传入 union 参数，服务器是否重启
