# v4.7.21 2026-11-10 新增 VUP 独立数据源

## 原因

VUP 使用独立上游 `https://vup.qianqiuzy.cn`，需要作为独立工会选项接入，但不能混入现有 VR+PSP 默认聚合，避免榜单人数过多。

## 涉及文件

- `rust_backend/src/upstream_cache.rs`
- `rust_backend/src/main.rs`
- `frontend/src/api/index.js`
- `frontend/src/components/AnchorList.vue`
- `frontend/src/components/HeaderSection.vue`
- `frontend/src/components/SimpleAnchorList.vue`
- `frontend/src/utils/guildConfig.js`
- `docs/features/data-sources.md`
- `docs/features/upstream-cache.md`
- `docs/api/endpoints.md`
- `docs/architecture/cache-system.md`

## 涉及模块

- 后端来源注册、严格筛选解析、上游 URL 构造、源缓存键、缓存清理和头像扫描。
- 前端工会配置、榜单筛选、标题、URL 状态、详情来源传递和错误提示。
- API 与缓存架构文档。

## 兼容策略

- `filter=all` 仍仅聚合 VR 与 PSP；VUP 使用 `filter=vup`。
- 详情来源使用 `union=VUP`，不新增本地 `source` 查询参数。
- 未知 `filter`/`union` 返回 400，不回退 VR。
- 三个来源使用独立缓存键；全量清理包含 VUP。
- VUP 官方 UID 暂未配置，头像任务通过 VUP 榜单房间扫描刷新。

## 测试结果

- `cargo test --locked`：通过，30 个测试全部成功，退出码 0。
- `npm run test:cache`：通过，7 个测试全部成功，退出码 0。
- `npm run build`：通过，退出码 0；仅保留既有的 chunk 大小警告。
- 完整服务浏览器验收：尚未执行。

## 版本说明

本文档按 `v4.7.21` 记录；`frontend/package.json`、`frontend/package-lock.json`、`rust_backend/Cargo.toml`、`rust_backend/Cargo.lock` 已同步为 `4.7.21`。
