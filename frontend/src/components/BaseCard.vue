<template>
  <div :class="['base-card', cardType ? `${cardType}-card` : '']" :id="'card-' + (rank || Math.random())">
    <div class="card-header" v-if="showHeader">
      <div class="header-main">
        <div v-if="rank !== undefined" class="card-rank">#{{ rank }}</div>
        <div v-if="title" class="card-title">{{ title }}</div>
      </div>
      <div v-if="subtitle" class="card-subtitle">{{ subtitle }}</div>
      <div class="card-subtitle-secondary"><strong><em>https</em></strong>斜杠<strong><em>dc</em></strong>点<strong><em>hihivr</em></strong>点<strong><em>top</em></strong></div>
    </div>

    <div class="card-body" @click.stop>
      <div
        v-for="(field, index) in fields"
        :key="index"
        :class="['field-box', field.className || '']"
      >
        <div class="field-label">{{ field.label }}</div>
        <div
          :class="['field-value', field.valueClassName || '']"
          v-if="!field.isLink"
          v-html="processValue(field.value, field.type)"
        ></div>
        <a
          v-else
          :href="field.href"
          :title="field.title || field.label"
          :class="['field-value', 'link-field', field.valueClassName || '']"
          target="_blank"
          v-html="processValue(field.value, field.type)"
          @click.stop
        >
        </a>
      </div>
    </div>

    <div class="card-footer" v-if="$slots.actions || showActions" @click.stop>
      <slot name="actions">
        <button
          v-if="actionButton"
          @click.stop="$emit('action-click', actionData)"
          :class="['action-btn', actionButton.className || '']"
        >
          {{ actionButton.text }}
        </button>
      </slot>
    </div>
  </div>
</template>

<script>
import { formatCurrency, formatNumber } from '@/utils/dataProcessor'

export default {
  name: 'BaseCard',
  props: {
    // 通用属性
    cardType: {
      type: String,
      default: ''  // 'anchor', 'session', 'sc' 等
    },
    rank: {
      type: Number,
      default: null
    },
    title: {
      type: String,
      default: ''
    },
    subtitle: {
      type: String,
      default: ''
    },
    showHeader: {
      type: Boolean,
      default: true
    },
    showActions: {
      type: Boolean,
      default: true
    },
    fields: {
      type: Array,
      required: true,
      default: () => []
    },
    actionButton: {
      type: Object,
      default: null
    },
    actionData: {
      type: [Object, String, Number],
      default: null
    }
  },
  emits: ['action-click'],
  setup() {
    const processValue = (value, type) => {
      if (type === 'currency') {
        return formatCurrency(value)
      } else if (type === 'number') {
        return formatNumber(value)
      } else if (type === 'duration') {
        // 格式化时长并在括号前添加换行
        const formatted = formatDuration(value);
        return formatted.replace(/\s\(/, '<br>(');
      } else if (typeof value === 'string' && value.includes('\n')) {
        // 处理换行符
        return value.replace(/\n/g, '<br>');
      }
      return value;
    }

    const formatDuration = (durationStr) => {
      // 解析 API 返回的 HH:MM:SS 格式
      if (!durationStr) return '0小时0分钟 (0分钟)'

      // 尝试解析 HH:MM:SS 格式
      const timeParts = durationStr.split(':')
      if (timeParts.length >= 2) {
        const hours = parseInt(timeParts[0]) || 0
        const minutes = parseInt(timeParts[1]) || 0

        // 计算总分钟数
        const totalMinutes = hours * 60 + minutes

        return `${hours}小时${minutes}分钟 (${totalMinutes}分钟)`
      }

      // 如果不是 HH:MM:SS 格式，返回原值
      return durationStr
    }

    return {
      processValue
    }
  }
}
</script>

<style scoped>
/* 通用卡片样式 */
.base-card {
  background: linear-gradient(135deg, #FFF8E1, #FFF5C2); /* 添加轻微渐变背景 */
  border: 1px solid #FFC633;
  border-radius: 20px;
  padding: 15px;
  margin-bottom: 15px;
  box-shadow: 0 6px 16px rgba(255, 198, 51, 0.2); /* 添加更柔和的阴影 */
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1); /* 使用更平滑的缓动函数 */
  position: relative;
  overflow: hidden;
  will-change: transform;
  transform: translateZ(0); /* 启用硬件加速 */
}

.base-card:hover {
  transform: translateY(-8px) scale(1.02); /* 上浮并轻微放大 */
  box-shadow: 0 12px 30px rgba(255, 198, 51, 0.4); /* 增强阴影 */
  border-color: #FFA500; /* 边框颜色变化 */
}

/* 光泽扫过效果 */
.base-card::before {
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

.base-card:hover::before {
  left: 120%;
  transition: all 0.8s ease;
}

/* 卡片头部样式 */
.card-header {
  background: linear-gradient(135deg, #FF8C00, #FFA500); /* 橙色渐变背景 */
  color: white; /* 白色文字 */
  padding: 15px;
  border-radius: 15px;
  margin-bottom: 15px;
  display: flex;
  flex-direction: column; /* 改为垂直布局 */
  align-items: center; /* 水平居中 */
  cursor: pointer; /* 添加手型光标表示可点击 */
  transition: background 0.3s ease, box-shadow 0.3s ease; /* 添加过渡效果 */
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); /* 添加轻微阴影 */
}

.card-header.expanded {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15); /* 展开时的阴影 */
}

.card-header:hover {
  background: linear-gradient(135deg, #FF9900, #FFB84D); /* 悬停时稍微亮一些 */
}

.header-main {
  display: flex; /* 使用flex布局 */
  align-items: center; /* 垂直居中 */
  justify-content: space-between; /* 两端对齐 */
  width: 100%; /* 占满容器宽度 */
  margin-bottom: 8px; /* 与副标题保持间距 */
}

.card-rank {
  font-weight: bold;
  color: white; /* 白色文字 */
  font-size: 1.4em; /* 适中的字号 */
  min-width: 50px; /* 确保编号区域有足够的宽度 */
  text-align: center; /* 居中对齐 */
  padding: 5px 10px; /* 内边距 */
  border-radius: 20px; /* 圆角 */
}

.card-title {
  font-weight: bold;
  color: white; /* 白色文字 */
  flex-grow: 1;
  text-align: center;
  font-size: 1.2em; /* 适中的字号 */
  margin: 0 15px; /* 左右留空 */
  padding: 5px 10px; /* 内边距 */
  border-radius: 10px; /* 圆角 */
}

.collapse-toggle {
  display: flex;
  align-items: center;
  margin-left: 10px;
}

.toggle-icon {
  font-size: 1.2em;
  font-weight: bold;
  color: white;
  margin-right: 4px;
  transition: transform 0.3s ease;
}

.toggle-text {
  color: white;
  font-size: 0.9em;
  white-space: nowrap;
}

.collapsed .toggle-icon {
  transform: rotate(90deg);
}

.card-subtitle {
  color: white; /* 白色文字 */
  font-weight: 600;
  font-size: 1.1em; /* 适中的字号 */
  text-align: center; /* 居中对齐 */
  padding: 5px 10px; /* 内边距 */
  border-radius: 10px; /* 圆角 */
  flex: 1 1 100%; /* 在小屏幕上独占一行 */
  margin-top: 5px;
}

.card-subtitle-secondary {
  color: white; /* 白色文字 */
  font-weight: 500;
  font-size: 1.0em; /* 适中的字号 */
  text-align: center; /* 居中对齐 */
  width: 100%; /* 占满容器宽度 */
  padding: 8px; /* 内边距 */
  border-radius: 10px; /* 圆角 */
}

/* 卡片主体样式 - 添加过渡效果 */
.card-body {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 自适应网格 */
  gap: 8px;
  margin-bottom: 10px;
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1); /* 使用更平滑的缓动函数 */
  overflow: hidden; /* 隐藏溢出内容 */
  opacity: 1;
}

.card-body.collapsed-body {
  margin: 0;
  padding: 0;
}

/* 字段盒子样式 */
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
  font-size: 1.1em; /* 增大字号 */
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
  font-size: 1.1em; /* 增大字号 */
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

.link-field {
  color: #f9729a;
  text-decoration: underline;
}

.link-field:hover {
  color: #e0658a;
}

/* 特殊字段类型样式 */
.total-revenue {
  color: #f9729a !important;
  font-weight: bold;
}

.currency-cell {
  color: #f9729a !important;
  font-weight: bold;
}

.duration-value {
  text-align: right;
  display: block;
  line-height: 1.4;
  word-break: break-word;
}

/* 卡片底部样式 */
.card-footer {
  text-align: center;
  margin-top: 8px;
}

.action-btn {
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

.action-btn:hover {
  background: #e0658a; /* 悬停时更深的洋红色 */
  color: white; /* 悬停时文字保持白色 */
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(249, 114, 154, 0.3);
}

/* 直播状态样式 */
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

/* 响应式设计 */
@media (min-width: 1025px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 宽屏下更多列 */
    gap: 8px;
  }
}

@media (min-width: 769px) and (max-width: 1024px) {
  .card-body {
    grid-template-columns: repeat(2, 1fr); /* 中等屏幕固定2列 */
    gap: 8px;
  }
}

@media (max-width: 768px) {
  .base-card {
    margin-bottom: 15px;
    padding: 10px;
  }

  .card-header {
    padding: 6px;
    flex-direction: column;
    gap: 5px;
    text-align: center;
  }

  .card-rank {
    font-size: 1em;
  }

  .card-title {
    font-size: 1em;
  }

  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); /* 移动端调整最小宽度 */
    gap: 6px;
  }

  .field-box {
    min-width: 110px;
    padding: 6px;
    flex-direction: column;
    text-align: center;
  }

  .field-label {
    font-size: 0.85em;
    margin-bottom: 2px;
    margin-right: 0;
    text-align: center;
  }

  .field-value {
    font-size: 0.95em;
    margin-left: 0;
    text-align: center;
  }
}

@media (max-width: 480px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); /* 小屏设备调整 */
    gap: 5px;
  }

  .field-box {
    min-width: 90px;
    padding: 5px;
  }

  .field-label {
    font-size: 0.8em;
  }

  .field-value {
    font-size: 0.9em;
  }

  .action-btn {
    padding: 5px 8px;
    font-size: 0.75rem;
    min-width: 100px;
  }
}

@media (max-width: 360px) {
  .card-body {
    grid-template-columns: repeat(auto-fill, minmax(80px, 1fr)); /* 极小屏设备调整 */
    gap: 4px;
  }

  .field-box {
    min-width: 80px;
    padding: 4px;
  }

  .field-label {
    font-size: 0.75em;
  }

  .field-value {
    font-size: 0.85em;
  }
}
</style>