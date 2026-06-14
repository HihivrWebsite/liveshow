# 实施计划

## 阶段划分

### 阶段 1：基础设施搭建（预计 2-3 天）

**目标**：建立新的技术栈和 Design Token 体系

**任务清单**：
- [ ] 安装 TailwindCSS、GSAP、lucide-vue-next、figma-squircle
- [ ] 配置 TailwindCSS，集成 Design Token
- [ ] 创建 `tokens.css` 颜色和变量定义
- [ ] 创建 `useGSAP.js` composable
- [ ] 创建 `useGlass.js` composable
- [ ] 配置 ECharts 自定义主题

**验收标准**：
- TailwindCSS 正常工作
- Design Token 可全局使用
- GSAP 动画 composable 可正常调用
- ECharts 使用自定义主题

---

### 阶段 2：基础组件开发（预计 3-4 天）

**目标**：完成 9 个基础 UI 组件

**任务清单**：
- [ ] GlassCard.vue - 毛玻璃卡片
- [ ] GlassButton.vue - 超椭圆按钮
- [ ] GlassDialog.vue - 毛玻璃弹窗
- [ ] GlassInput.vue - 统一输入框
- [ ] GlassTable.vue - 统一表格
- [ ] StatCard.vue - 统计卡片
- [ ] LiveCounter.vue - 直播计数器
- [ ] AnimatedTag.vue - 动态标签
- [ ] RankingRow.vue - 排名行组件

**验收标准**：
- 所有组件可正常渲染
- 毛玻璃效果正常显示
- GSAP 动画流畅
- 组件 props 接口设计合理

---

### 阶段 3：页面组件迁移（预计 4-5 天）

**目标**：完成所有业务组件的视觉迁移

**任务清单**：
- [ ] HeaderSection.vue - 顶部导航
- [ ] FooterSection.vue - 底部信息
- [ ] AnchorList.vue - 主播列表
- [ ] NavigationTable.vue - 导航表格
- [ ] BaseCard.vue → GlassCard 迁移
- [ ] MonthSelector.vue - 月份选择器
- [ ] ChartComponent.vue - 图表组件
- [ ] ExpandedView.vue - 展开视图
- [ ] AnchorBattle.vue - 斗虫对比
- [ ] RankComparison.vue - 排名对比
- [ ] SuperChatDetail.vue - SC详情
- [ ] LiveSessions.vue - 直播会话

**验收标准**：
- 所有页面正常显示
- 所有功能可正常使用
- 所有动画流畅
- 响应式布局正常

---

### 阶段 4：细节优化和测试（预计 2-3 天）

**目标**：优化细节，确保质量

**任务清单**：
- [ ] 动画性能优化
- [ ] 响应式断点调整
- [ ] 毛玻璃效果在不同浏览器测试
- [ ] 移动端适配优化
- [ ] 无障碍访问优化
- [ ] 代码清理和重构

**验收标准**：
- 所有动画 60fps
- 移动端体验良好
- 主流浏览器兼容
- 代码整洁无冗余

---

## 总预计时间

**11-15 天**（按每天 4-6 小时计算）

---

## 风险点

1. **毛玻璃性能**
   - 风险：大量使用 backdrop-filter 可能影响性能
   - 方案：限制同时显示的毛玻璃元素数量，使用 will-change 优化

2. **GSAP 内存泄漏**
   - 风险：动画未正确清理可能导致内存泄漏
   - 方案：在 onUnmounted 中正确清理动画

3. **TailwindCSS 冲突**
   - 风险：与现有样式可能产生冲突
   - 方案：使用 @layer 隔离，逐步迁移

4. **浏览器兼容性**
   - 风险：backdrop-filter 在部分浏览器不支持
   - 方案：提供降级方案（纯背景色）

---

## 成功指标

1. **视觉一致性**
   - 所有颜色来自 Design Token
   - 所有圆角来自 Design Token
   - 所有阴影来自 Design Token
   - 毛玻璃效果统一

2. **动画流畅度**
   - 页面进入动画 60fps
   - 卡片出现动画 60fps
   - Hover 动画 60fps
   - 数字增长动画 60fps

3. **代码质量**
   - 无硬编码样式
   - 组件职责清晰
   - Composable 复用率高
   - 无冗余代码

4. **用户体验**
   - 视觉风格统一现代
   - 交互反馈及时
   - 加载性能良好
   - 移动端体验良好
