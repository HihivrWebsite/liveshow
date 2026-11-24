# 弹幕分级统计

## 功能概述

主播直播数据新增弹幕分级统计，按用户身份分为普通弹幕、舰长弹幕、提督弹幕、总督弹幕；另接入表情包弹幕数、当月/当场去重发言人数，以及**弹幕前排占比**（`danmaku_dependency`）。

## 数据来源

上游 API（`vr/psp/vup.qianqiuzy.cn`）的 `danmaku` 二级对象、顶层 `danmaku_user_count`、`danmaku_dependency` 对象；场次接口为 `emoticon_danmaku_count` + 同名 `danmaku_dependency`。

### 弹幕分级与表情包（`danmaku` / 场次扁平字段）

| 字段（上游 → 本项目后端字段） | 说明 |
|------|------|
| `danmaku.total` → `danmaku_total` | 月总弹幕数 |
| `danmaku.emoticon` → `danmaku_emoticon` | 表情包弹幕数（整条表情包发送 `dm_type=1`；带内嵌表情的文本按文本计） |
| `danmaku.captain` → `danmaku_captain` | 舰长弹幕数 |
| `danmaku.admiral` → `danmaku_admiral` | 提督弹幕数 |
| `danmaku.governor` → `danmaku_governor` | 总督弹幕数 |
| `danmaku.normal` → `danmaku_normal` | 普通弹幕数 |
| `danmaku_user_count` → `danmaku_user_count` | 当月/当场去重发言人数（旧数据或部分覆盖为 `null`） |
| 场次 `emoticon_danmaku_count` → `emoticon_danmaku_count` | 整场表情包弹幕数，口径同月度 `danmaku.emoticon` |

### 弹幕前排占比（`danmaku_dependency`，与金主独立）

| 后端字段（Anchor / LiveSession） | 上游 | 说明 |
|------|------|------|
| `danmaku_top1` / `danmaku_top5` / `danmaku_top10` / `danmaku_top1_percent` | `danmaku_dependency.top*` | float 0–1，最多 4 位小数；**弹幕按有效 UID 发言条数排序的占比，不是金主付费占比** |
| `danmaku_dependency_status` | `danmaku_dependency.status` | `live` 实时 / `archived` 已归档 / `partial` 仅部分覆盖 / `unavailable` 无数据 |

**与金主占比的区分（硬约束）：**

- 项目既有 `top1` / `top5` / `top10` / `top1_percent` 来自 **`whale_dependency`（金主/付费金额）**，语义与字段名不得改动。
- 弹幕占比一律使用 `danmaku_top*` + `danmaku_dependency_status` 独立命名，禁止复用或覆盖金主 `top*`。
- UI 标签必须带「弹幕」字样（如「弹幕Top1占比」），与「Top1金主」等标签并列区分。

## 展示位置

- 主播卡片字段：总弹幕 + 分级弹幕 + 表情包弹幕 + 弹幕发言人数 + 弹幕Top1/5/10/1%占比
- 导出截图：上述字段与金主占比字段分列
- 直播会话详情：弹幕数 + 分级弹幕 + 表情包弹幕 + 弹幕发言人数 + 弹幕前排占比 + 趋势图
- 展开详情（主播/场次）：表情包弹幕、弹幕发言人数、弹幕前排占比；金主占比保持原标签
- 恶意斗虫 / 排名对比：既有 4 个分级弹幕指标（本次未新增弹幕占比指标）

## 判空规则（status）

历史月大量 `null` / `status=unavailable|partial`：

- **必须按 `danmaku_dependency_status` 判空**，不能只看数值 `!= null`。
- `status` 为 `unavailable` 或缺失时，占比显示 `—`；`partial` 且数值非空时可展示（仅覆盖已采集部分）。
- 数值格式化为百分比、保留 1 位小数（与金主占比展示一致）。
- 表情包弹幕数 / 发言人数为 `null` 时显示 `—`。
