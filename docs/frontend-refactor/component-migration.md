# 组件迁移指南

## 迁移顺序

### 第一阶段：基础组件（优先级：高）

1. **GlassCard.vue** - 替代 BaseCard.vue
   - 使用毛玻璃效果
   - 统一圆角规范（32px）
   - 统一阴影规范
   - GSAP 入场动画

2. **GlassButton.vue** - 替代所有按钮
   - 超椭圆圆角（999px）
   - 统一 hover 效果
   - 支持多颜色变体：primary/secondary/danger/success/default/info/debug
   - CTA 光带动画仅用于指定按钮（见 README.md 按钮颜色规范）

3. **GlassDialog.vue** - 替代所有弹窗
   - 毛玻璃背景
   - 统一圆角（40px）
   - GSAP 开关动画

4. **GlassInput.vue** - 替代所有输入框
   - 统一圆角（24px）
   - 统一 focus 效果

5. **GlassTable.vue** - 替代 NavigationTable 样式
   - 保留表格结构和逻辑
   - 仅更新视觉样式
   - 统一行高、配色、hover 效果

### 第二阶段：业务组件（优先级：中）

6. **StatCard.vue** - 统计卡片
   - 用于 HeaderSection 的缓存统计
   - 数字增长动画（GSAP）

7. **LiveCounter.vue** - 直播状态计数器
   - 用于显示在线主播数量
   - 呼吸动画效果

8. **AnimatedTag.vue** - 动态标签
   - 用于状态标签（直播中/未开播）
   - 统一样式

9. **RankingRow.vue** - 排名行组件
   - 用于 NavigationTable 的排名显示
   - 排名变化动画

### 第三阶段：页面组件（优先级：低）

10. **HeaderSection.vue** - 顶部导航
    - 使用 GlassCard 包装缓存统计
    - 使用 GlassButton 替换按钮
    - 保留功能指南弹窗逻辑

11. **FooterSection.vue** - 底部信息
    - 使用 GlassButton 替换链接按钮
    - 保留闪光动画（CTA规范）

12. **AnchorList.vue** - 主播列表
    - 使用 GlassButton 替换筛选按钮
    - 使用 GlassCard 包装列表项
    - 保留所有业务逻辑

13. **NavigationTable.vue** - 导航表格
    - 使用 GlassTable 替换表格样式
    - 保留排序、分页、选择逻辑

## 迁移规则

### 样式迁移

1. **删除所有硬编码颜色**
   - 移除 `#FFF8E1`、`#FFC633`、`#FFA500` 等
   - 使用 Design Token 替代

2. **删除所有硬编码圆角**
   - 移除 `border-radius: 20px`、`border-radius: 30px` 等
   - 使用 Token 替代：`rounded-card`、`rounded-button` 等

3. **删除所有硬编码阴影**
   - 移除 `box-shadow: 0 6px 16px rgba(...)` 等
   - 使用 Token 替代：`shadow-default`、`shadow-hover` 等

4. **删除所有 CSS transition**
   - 移除 `transition: all 0.3s ease` 等
   - 使用 GSAP 替代

### 动画迁移

1. **页面进入动画**
   - 使用 GSAP `from` 动画
   - duration: 0.8, ease: "power4.out"

2. **卡片出现动画**
   - 使用 GSAP `from` 动画
   - duration: 0.6, ease: "expo.out"

3. **Hover 动画**
   - 使用 GSAP `to` 动画
   - duration: 0.25, ease: "power2.out"

4. **数字增长动画**
   - 使用 GSAP `to` 动画
   - duration: 1.2, ease: "power3.out"

## 注意事项

1. **保留所有业务逻辑**
   - 不修改 props、emits、methods
   - 不修改数据获取和处理逻辑
   - 不修改路由和导航

2. **保留所有功能入口**
   - 不修改按钮位置
   - 不修改表单布局
   - 不修改弹窗内容

3. **保留所有交互流程**
   - 不修改点击事件
   - 不修改筛选逻辑
   - 不修改排序逻辑

4. **测试要点**
   - 所有按钮可点击
   - 所有表单可提交
   - 所有弹窗可打开/关闭
   - 所有动画流畅
   - 响应式布局正常
