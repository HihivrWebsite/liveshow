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

```bash
# 第一次请求（缓存未命中）
curl "http://localhost:2992/gift/attention?room_id=1713546334&month=202604&union=VirtuaReal"

# 第二次请求（应缓存命中）
curl "http://localhost:2992/gift/attention?room_id=1713546334&month=202604&union=VirtuaReal"

# 查看缓存统计
curl "http://localhost:2992/cache/stats"
```

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
