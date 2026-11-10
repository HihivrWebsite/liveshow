# liveshow — 维阿 PSP 斗虫榜

Bilibili VirtuaReal/PSPlive 直播数据展示。Rust/Axum 后端（端口 2992）+ Vue 3/Vite 前端（端口 3000）。

继承 `~/.codex/AGENTS.md` 全局约定。

## 硬性规则

- 文档驱动：改代码前查 `docs/`，同步更新 `docs/changes|features|api|architecture/`
- **禁止自动 git 操作**；版本号每次提交递增 patch（feature/bugfix/style 均 patch，大功能可用 minor）
- 前端改动须 `start.bat`（或 npm install → npm run build → cargo build → cargo run）起完整服务后验证

## 前端设计规范（`docs/frontend-refactor/design-spec.md`）

- 6 色色板：`#F7F1DF` `#F6B100` `#FF6B9D` `#5D4B24` `#8E7B50` `#FFF3D0`，**禁止新色值**
- 圆角 figma-squircle：Button 999px / Input 24px / Card 32px / Dialog 40px
- 毛玻璃 blur 24px，bg `rgba(246,177,0,0.08)`，border `rgba(246,177,0,0.15)`
- 动画 GSAP：页面 0.8s/power4.out，卡片 0.6s/expo.out，Hover 0.25s/power2.out
- 图标 lucide-vue-next；SC 卡片保留 B 站原色 + 左侧 6px 彩条

## 架构要点

- Vue3 + Vite + Tailwind + shadcn-vue + GSAP + figma-squircle + ECharts
- 月份选择统一 `MonthSelector.vue` + `monthUtils.js`，**纯整数数学**，避免 `new Date()` 时区坑
- Cursor System：CursorProvider/Dot/Ring/Ripple + useCursor.ts，移动端禁用
- 头像：后端 `manifest.json` 统一索引 40×40、<100KiB WebP；启动及每 4 小时强制刷新，批量接口返回 WebP data URL，`defaultAvatars.js` 保留离线 JPEG 回退
- 缓存：`cache_data/current_month/`（当月过期检查）+ `previous_months/`（永久）

## 已知坑

- GSAP 3.15 + Vite 4.x：`gsap.ticker.add()` 打包后可能不触发
- GSAP `to({scale})` 会覆盖 Vue `transform: translate()` — transform 只让一方控制
- `ts` 单元格用 `display:flex` 会丢 `vertical-align`
- Canvas 2D 不支持 CSS `var()`
- ECharts：`inverse: true` 排名倒序；pattern fill 用 `{ image: HTMLCanvasElement }`
- CDN hdslb.com 防盗链，头像须后端代理

## 外部接口

- 头像：`GET /live_user/v1/UserInfo/get_anchor_in_room?roomid=`
- 用户卡：`GET /x/web-interface/card?mid={uid}`（无需 WBI）
- 斗虫 API 文档：`https://github.com/QianQiuZy/VR_douchong/blob/main/api.md`
- VirtuaReal UID:413748120，PSPlive UID:454673997
