<template>
  <div :class="['anchor-grid-item', { 'live-grid-item': anchor.status === 1 }]">
    <div class="grid-header">
      <div class="grid-rank">#{{ rank }}</div>
      <div class="grid-name">{{ anchor.anchor_name }}</div>
      <div class="grid-union">{{ anchor.union }}</div>
    </div>
    <div class="grid-fields">
      <div class="field-box">
        <div class="field-label">关注数</div>
        <div class="field-value">{{ formatNumber(anchor.attention) }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">有效天</div>
        <div class="field-value">{{ anchor.effective_days }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">开播时长</div>
        <div class="field-value duration-value" v-html="formatDurationWithBreak(anchor.live_duration)"></div>
      </div>
      <div class="field-box">
        <div class="field-label">开播状态</div>
        <div class="field-value status-field">
          <span
            :class="[
              'status-badge',
              { live: anchor.status === 1, offline: anchor.status !== 1 }
            ]"
          >
            <template v-if="anchor.status === 1">
              <a
                :href="`https://live.bilibili.com/${anchor.room_id}`"
                target="_blank"
                class="live-link"
                :title="`点击跳转到 ${anchor.anchor_name} 的 Bilibili 直播间`"
              >
                正在直播
              </a>
            </template>
            <template v-else>
              未开播
            </template>
          </span>
        </div>
      </div>
      <div class="field-box">
        <div class="field-label">总督</div>
        <div class="field-value">{{ anchor.guard_3 || 0 }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">提督</div>
        <div class="field-value">{{ anchor.guard_2 || 0 }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">舰长</div>
        <div class="field-value">{{ anchor.guard_1 || 0 }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">粉丝团</div>
        <div class="field-value">{{ formatNumber(anchor.fans_count || 0) }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">礼物收入</div>
        <div class="field-value">{{ formatCurrency(anchor.gift) }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">舰长收入</div>
        <div class="field-value">{{ formatCurrency(anchor.guard) }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">SC收入</div>
        <div class="field-value">{{ formatCurrency(anchor.super_chat) }}</div>
      </div>
      <div class="field-box">
        <div class="field-label">总营收</div>
        <div class="field-value total-revenue">
          {{ formatCurrency(calculateTotalRevenue(anchor)) }}
        </div>
      </div>
    </div>
    <div class="grid-footer">
      <slot name="actions">
        <button
          @click="$emit('view-details', anchor)"
          class="view-btn"
        >
          查看详细数据
        </button>
      </slot>
    </div>
  </div>
</template>

<script>
import { formatCurrency, formatNumber, calculateTotalRevenue } from '@/utils/dataProcessor'

export default {
  name: 'BaseAnchorCard',
  props: {
    anchor: {
      type: Object,
      required: true
    },
    rank: {
      type: Number,
      default: 0
    }
  },
  emits: ['view-details'],
  setup() {
    const formatDurationWithBreak = (durationStr) => {
      // 格式化时长并在括号前添加换行
      const formatted = formatLiveDuration(durationStr);
      // 在括号前添加换行
      const parts = formatted.split(' (');
      if (parts.length > 1) {
        return `${parts[0]}<br>(${parts.slice(1).join('(')}`;
      }
      return formatted;
    }

    const formatLiveDuration = (durationStr) => {
      // 解析 API 返回的 HH:MM:SS 格式
      if (!durationStr) return '0小时0分钟 (0分钟)'

      // 尝试解析 HH:MM:SS 格式
      const timeParts = durationStr.split(':')
      if (timeParts.length >= 2) {
        const hours = parseInt(timeParts[0]) || 0
        const minutes = parseInt(timeParts[1]) || 0

        // 计算总分钟数
        const totalMinutes = hours * 60 + minutes

        let result = `${hours}小时${minutes}分钟 (${totalMinutes}分钟)`
        // 在括号前插入换行标记
        return result.replace(/\s\(/, '<br>(')
      }

      // 如果不是 HH:MM:SS 格式，返回原值
      return durationStr
    }

    return {
      formatCurrency,
      formatNumber,
      calculateTotalRevenue,
      formatDurationWithBreak
    }
  }
}
</script>

<style scoped>
/* 完全复制 AnchorList.vue 中的卡片样式 */
.anchor-grid-item {
  background: linear-gradient(135deg, #FFF8E1, #FFF5C2); /* 添加轻微渐变背景 */
  border: 1px solid #FFC633;
  border-radius: 20px; /* 增加圆角 */
  padding: 15px; /* 增加内边距 */
  margin-bottom: 15px; /* 增加外边距 */
  box-shadow: 0 6px 16px rgba(255, 198, 51, 0.2); /* 添加更柔和的阴影 */
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1); /* 使用更平滑的缓动函数 */
  position: relative; /* 为高级动效添加相对定位 */
  overflow: hidden; /* 确保内容不会溢出 */
  will-change: transform; /* 优化性能 */
  transform: translateZ(0); /* 启用硬件加速 */
}

/* 卡片悬停动效 */
.anchor-grid-item:hover {
  transform: translateY(-8px) scale(1.02); /* 上浮并轻微放大 */
  box-shadow: 0 12px 30px rgba(255, 198, 51, 0.4); /* 增强阴影 */
  border-color: #FFA500; /* 边框颜色变化 */
}

/* 光泽扫过效果 */
.anchor-grid-item::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -60%;
  width: 20px;
  height: 200%;
  background: linear-gradient(
    to right,
    rgba(255, 255, 255, 0) 0%,
    rgba(255, 255, 255, 0.4) 50%,
    rgba(255, 255, 255, 0) 100%
  );
  transform: rotate(30deg);
  transition: all 0.6s ease;
  z-index: 1;
}

.anchor-grid-item:hover::before {
  left: 120%;
  transition: all 0.8s ease;
}

.anchor-grid-item.live-grid-item {
  border: 2px solid #f9729a; /* 直播状态特殊边框 */
  background: #FFF8E1; /* 浅黄色背景 */
}

.grid-header {
  background: linear-gradient(135deg, #FF8C00, #FFA500); /* 橙色渐变背景 */
  color: white; /* 白色文字 */
  padding: 15px; /* 内边距 */
  border-radius: 15px; /* 圆角 */
  margin-bottom: 15px; /* 间距 */
  display: flex; /* 使用flex布局 */
  align-items: center; /* 垂直居中 */
  justify-content: space-between; /* 两端对齐 */
  flex-direction: column; /* 改为垂直布局 */
  text-align: center; /* 文字居中 */
}

.grid-rank {
  font-weight: bold;
  color: white; /* 白色文字 */
  font-size: 1.4em; /* 适中的字号 */
  min-width: 50px; /* 确保编号区域有足够的宽度 */
  text-align: center; /* 居中对齐 */
  padding: 5px 10px; /* 内边距 */
  border-radius: 20px; /* 圆角 */
  margin-bottom: 10px; /* 与其他元素增加间距 */
}

.grid-name {
  font-weight: bold;
  color: white; /* 白色文字 */
  font-size: 1.2em; /* 适中的字号 */
  text-align: center; /* 居中对齐 */
  padding: 5px 10px; /* 内边距 */
  border-radius: 10px; /* 圆角 */
  margin: 5px 0; /* 与其他元素增加间距 */
}

.grid-union {
  color: white; /* 白色文字 */
  font-weight: 600; /* 加粗 */
  font-size: 1.1em; /* 适中的字号 */
  text-align: center; /* 居中对齐 */
  padding: 5px 10px; /* 内边距 */
  border-radius: 10px; /* 圆角 */
}

.grid-fields {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 自适应网格 */
  gap: 8px;
  margin-bottom: 10px;
}

.field-box {
  background: rgba(255, 248, 225, 0.7); /* 淡黄色半透明背景 */
  border: 1px solid #FFC633;
  border-radius: 10px; /* 增加圆角 */
  padding: 12px; /* 增加内边距 */
  min-width: 120px;
  display: flex; /* 使用flex布局 */
  flex-direction: column; /* 改为垂直布局 */
  align-items: flex-start; /* 左对齐内容 */
  text-align: left; /* 左对齐文本 */
  margin-bottom: 6px; /* 添加底部间距 */
  transition: all 0.2s ease; /* 添加过渡效果 */
}

.field-box:hover {
  background: rgba(255, 240, 180, 0.8); /* 悬停时更亮的背景 */
  transform: translateY(-1px); /* 悬停时轻微上移 */
  box-shadow: 0 3px 8px rgba(255, 198, 51, 0.3); /* 悬停时添加阴影 */
}

.field-label {
  font-weight: bold;
  color: #FF8C00; /* 使用更醒目的颜色 */
  font-size: 1.1em; /* 正常大小 */
  word-break: break-word;
  margin-right: 10px; /* 增加与值之间的间距 */
  flex-shrink: 0; /* 防止标签被压缩 */
  transition: all 0.3s ease; /* 添加颜色过渡效果 */
  background-color: rgba(255, 198, 51, 0.15); /* 添加轻微背景色 */
  padding: 4px 8px; /* 添加内边距 */
  border-radius: 8px; /* 添加圆角 */
}

.field-label:hover {
  color: #FF6600; /* 悬停时更深的颜色 */
  background-color: rgba(255, 165, 0, 0.25); /* 悬停时更深的背景色 */
}

.field-value {
  color: #333;
  font-size: 1.1em; /* 正常大小 */
  word-break: break-word;
  text-align: right; /* 值右对齐 */
  margin-left: 10px; /* 增加与标签之间的间距 */
  overflow: hidden; /* 防止溢出 */
  text-overflow: ellipsis; /* 溢出时显示省略号 */
  transition: all 0.3s ease; /* 添加颜色过渡效果 */
}

.field-value:hover {
  color: #f9729a; /* 悬停时使用主题色 */
}

.total-revenue {
  color: #f9729a !important;
  font-weight: bold;
}

.grid-footer {
  text-align: center;
  margin-top: 8px;
}

.status-field {
  display: flex;
  justify-content: center;
  align-items: center;
}

.status-badge {
  display: inline-block;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: bold;
}

.status-badge.live {
  background: #f9729a; /* 实心洋红色背景，与查看详细数据按钮相同 */
  color: white; /* 白色文字，与查看详细数据按钮相同 */
  border: 2px solid #f9729a; /* 洋红色边框，与查看详细数据按钮相同 */
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  padding: 4px 8px; /* 内边距，与查看详细数据按钮相似 */
  cursor: pointer; /* 鼠标指针 */
  font-size: 0.8rem; /* 字体大小，与查看详细数据按钮相同 */
  transition: all 0.3s ease; /* 过渡效果 */
  font-weight: bold; /* 加粗 */
  text-decoration: none; /* 去除下划线 */
  display: inline-block; /* 行内块显示 */
  min-width: 80px; /* 最小宽度确保圆形效果 */
}

.status-badge.offline {
  background: rgba(255, 255, 255, 0.2);
  color: #ccc;
}

.live-link {
  color: white;
  text-decoration: none;
  font-weight: bold;
  display: inline-block;
}

.live-link:hover {
  text-decoration: underline;
}

.view-btn {
  padding: 6px 12px;
  background: #f9729a; /* 实心洋红色背景 */
  color: white; /* 白色文字 */
  border: 2px solid #f9729a; /* 洋红色边框 */
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  cursor: pointer;
  font-size: 0.8rem;
  transition: all 0.3s ease;
  font-weight: bold;
  min-width: 120px; /* 最小宽度确保圆形效果 */
}

.view-btn:hover {
  background: #e0658a; /* 悬停时更深的洋红色 */
  color: white; /* 悬停时文字保持白色 */
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(249, 114, 154, 0.3);
}

/* 宽屏优化：在大屏幕上显示更多列 */
@media (min-width: 1024px) {
  .grid-container {
    display: grid !important;
    grid-template-columns: repeat(auto-fill, minmax(380px, 1fr)); /* 自动填充，最小380px宽的列 */
    gap: 20px; /* 卡片间距 */
    padding: 15px; /* 内边距 */
  }

  .anchor-grid-item {
    margin-bottom: 0; /* 在网格布局中不需要底部边距 */
    height: fit-content; /* 高度自适应内容 */
    transition: transform 0.3s ease, box-shadow 0.3s ease; /* 添加悬停效果 */
  }

  .anchor-grid-item:hover {
    transform: translateY(-5px); /* 悬停时轻微上移 */
    box-shadow: 0 8px 24px rgba(255, 198, 51, 0.4); /* 增强阴影效果 */
  }
}

/* 中等屏幕：显示2列 */
@media (min-width: 769px) and (max-width: 1023px) {
  .grid-container {
    display: grid !important;
    grid-template-columns: repeat(2, 1fr); /* 固定2列 */
    gap: 15px; /* 卡片间距 */
    padding: 10px; /* 内边距 */
  }

  .anchor-grid-item {
    margin-bottom: 0; /* 在网格布局中不需要底部边距 */
    height: fit-content; /* 高度自适应内容 */
    transition: transform 0.3s ease, box-shadow 0.3s ease; /* 添加悬停效果 */
  }

  .anchor-grid-item:hover {
    transform: translateY(-5px); /* 悬停时轻微上移 */
    box-shadow: 0 8px 24px rgba(255, 198, 51, 0.4); /* 增强阴影效果 */
  }
}

/* 小屏幕：显示1列 */
@media (max-width: 768px) {
  .grid-container {
    display: block; /* 单列显示 */
  }
  
  .anchor-grid-item {
    margin-bottom: 15px; /* 调整间距 */
    padding: 10px; /* 调整内边距 */
  }

  .grid-fields {
    gap: 6px; /* 调整间距 */
  }

  .field-box {
    min-width: 110px; /* 调整最小宽度 */
    padding: 6px; /* 调整内边距 */
    flex-direction: column; /* 移动端改为垂直布局 */
    text-align: center; /* 文字居中 */
  }

  .field-label {
    font-size: 0.85em; /* 调整字体大小 */
    margin-bottom: 2px;
    margin-right: 0; /* 移动端移除右边距 */
    text-align: center; /* 文字居中 */
  }

  .field-value {
    font-size: 0.95em; /* 调整字体大小 */
    margin-left: 0; /* 移动端移除左边距 */
    text-align: center; /* 文字居中 */
  }

  .grid-header {
    padding: 6px; /* 调整内边距 */
  }

  .grid-rank {
    font-size: 1em; /* 调整字体大小 */
  }

  .grid-name {
    font-size: 1em; /* 调整字体大小 */
  }
}

/* 触屏设备优化 */
@media (hover: none) and (pointer: coarse) {
  .anchor-grid-item {
    /* 为触屏设备添加点击反馈 */
    tap-highlight-color: transparent;
    -webkit-tap-highlight-color: transparent;
  }

  .anchor-grid-item:active {
    transform: scale(0.98); /* 点击时轻微缩小 */
    box-shadow: 0 4px 16px rgba(255, 198, 51, 0.3); /* 减弱阴影 */
  }

  .field-label:active,
  .field-value:active {
    transform: scale(0.99); /* 点击时轻微缩小 */
  }
}
</style>