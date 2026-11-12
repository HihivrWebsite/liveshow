# 数据源配置与 VUP 独立源

## 概述

后端用一个固定注册表描述 VirtuaReal、PSPlive 和 VUP。三个源共享同一套抓取、缓存、清理和统计逻辑，但上游地址、展示名称、筛选别名、历史下界及启用状态均由各自的 `SourceConfig` 决定，不再在业务分支中分别硬编码。

## SourceConfig

```rust
SourceConfig {
    id: SourceId,          // Vr | Psp | Vup
    base_url: String,
    union_name: String,
    filter_alias: String,
    min_month: String,
    official_uid: Option<u64>,
    enabled: bool,
}
```

字段职责：

| 字段 | 用途 |
| --- | --- |
| `id` | 缓存、日志、统计及清理使用的稳定来源身份 |
| `base_url` | 构造上游请求地址 |
| `union_name` | 详情类接口的 `union` 值及响应展示名称 |
| `filter_alias` | 列表接口的 `filter` 值 |
| `min_month` | 该源支持的历史月份下界 |
| `official_uid` | 官方账号 UID，可选 |
| `enabled` | 是否启用该源 |

## 固定映射

| `SourceId` | `base_url` | `union_name` | `filter_alias` | `min_month` | `official_uid` | `enabled` |
| --- | --- | --- | --- | --- | --- | --- |
| `Vr` | `https://vr.qianqiuzy.cn` | `VirtuaReal` | `vr` | `2025-08` | `Some(413748120)` | `true` |
| `Psp` | `https://psp.qianqiuzy.cn` | `PSPlive` | `psp` | `2025-08` | `Some(454673997)` | `true` |
| `Vup` | `https://vup.qianqiuzy.cn` | `VUP` | `vup` | `2025-08` | `None`（待确认） | `true` |

## 选择规则

- 列表请求通过 `filter=vr|psp|vup` 选择单一源；`filter=all` 只聚合 `Vr+Psp`，VUP 不进入该聚合；`filter=everything` 聚合全部三源（VR+PSP+VUP）。
- 详情请求通过 `union=VirtuaReal|PSPlive|VUP` 选择单一源，VUP 使用 `union=VUP`。
- 未知 `filter`/`union` 返回 HTTP 400，不回退 VR；同一请求的 `filter` 与 `union` 不一致返回 400。
- 不新增本地 `source` 查询参数。上游地址按 `base_url + 接口路径 + 业务参数` 构造，也不追加 `source` 查询参数。
- 所选源的 429、503、404 按上游语义返回，不请求其他源，不作为成功空结果写入源数据缓存。

## 缓存、清理与头像

- 源数据键包含 `source + endpoint + room_id + month`，三个源完全隔离；Attention 月份进入键前统一为 `YYYYMM`。
- 不传 `union` 的 `POST /cache/clear` 执行全量清理并包含 VUP；指定 `union=VUP` 只清理 VUP。
- `/cache/stats.upstream_cache.by_endpoint` 以 `vup:` 前缀报告 VUP，顶层合计包含三个源。
- 启动及每 4 小时头像扫描同时遍历三个源；WebP 清单头像键暂仍按 `room_id`。

## 兼容性

- 不传 `filter` 的后端请求仍等价于 `all`（VR+PSP），API 兼容语义不变；前端列表页默认显式请求 `filter=everything`（全部三源），带 `filter=all` 的旧链接仍只显示 VR+PSP。
- 既有 `vr`、`psp` 和 `VirtuaReal`、`PSPlive` 用法保持；`vup` 与 `VUP` 为新增显式入口。
- 旧的未知值静默回退行为取消，统一改为 400，避免错误源被当成 VR 数据展示。
- 磁盘源缓存已包含来源身份，新增 VUP 不需要迁移或删除既有 VR/PSP 缓存。

## 相关文档

- [共享上游缓存](upstream-cache.md)
- [缓存架构](../architecture/cache-system.md)
- [API 接口文档](../api/endpoints.md)
