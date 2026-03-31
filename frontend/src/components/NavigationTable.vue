<template>
  <div class="navigation-table-container">
    <h3 class="table-title">📋 快速导航</h3>
    <div class="table-wrapper">
      <table class="navigation-table">
        <thead>
          <tr>
            <th class="rank-col">排名</th>
            <th class="title-col">{{ titleColumn }}</th>
            <th class="status-col">开播状态</th>
            <th class="revenue-col">总营收</th>
            <th class="action-col">操作</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(item, index) in items"
            :key="item.room_id || item.id || index"
            @click="scrollToCard(index)"
            class="nav-row"
          >
            <td class="rank-cell">{{ index + 1 }}</td>
            <td class="title-cell">{{ getItemTitle(item) }}</td>
            <td class="status-cell" :class="{ 'live-status': isLive(item) }">{{ getStatus(item) }}</td>
            <td class="revenue-cell">{{ formatCurrency(calculateTotalRevenue(item)) }}</td>
            <td class="action-cell">
              <button
                class="jump-btn"
                @click.stop="scrollToCard(index)"
                @mousedown.stop="handleMiddleClick($event, index)"
              >跳转</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
import { formatCurrency } from '@/utils/dataProcessor'

export default {
  name: 'NavigationTable',
  props: {
    items: {
      type: Array,
      required: true,
      default: () => []
    },
    itemType: {
      type: String,
      default: 'anchor' // 'anchor' for anchors, 'session' for live sessions
    }
  },
  setup(props) {
    const scrollToCard = (index) => {
      const cardElement = document.getElementById(`card-${index + 1}`);
      if (cardElement) {
        cardElement.scrollIntoView({
          behavior: 'smooth',
          block: 'start'
        });

        // 添加临时高亮效果
        cardElement.style.transition = 'background-color 0.5s ease';
        cardElement.style.backgroundColor = 'rgba(249, 114, 154, 0.3)';
        setTimeout(() => {
          cardElement.style.backgroundColor = '';
        }, 2000);
      }
    };

    const handleMiddleClick = (event, index) => {
      if (event.button === 1) { // 中键点击 (鼠标滚轮)
        // 获取目标卡片元素的ID
        const targetId = `card-${index + 1}`;

        // 在新标签页中打开当前页面，但需要通过查询参数传递目标ID
        const newUrl = `${window.location.origin}${window.location.pathname}?scrollTo=${targetId}`;
        window.open(newUrl, '_blank');
      }
    };

    const calculateTotalRevenue = (item) => {
      const gift = parseFloat(item.gift) || 0;
      const guard = parseFloat(item.guard) || 0;
      const superChat = parseFloat(item.super_chat) || 0;
      return gift + guard + superChat;
    };

    const getItemTitle = (item) => {
      let title;
      if (props.itemType === 'session') {
        // 直播会话数据
        title = item.title || item.anchor_name || `直播会话 ${item.rank || item.id || 'N/A'}`;
      } else {
        // 主播数据
        title = item.anchor_name || item.name || `项目 ${item.rank || item.id || 'N/A'}`;
      }
      return title + ' -dc 点 hihivr 点 top';
    };

    const getStatus = (item) => {
      if (props.itemType === 'session') {
        // 直播会话数据，不显示状态
        return '-';
      } else {
        // 主播数据，显示开播状态
        if (item.status === 1) {
          return '直播中';
        } else {
          return '未开播';
        }
      }
    };

    const isLive = (item) => {
      if (props.itemType === 'session') {
        // 直播会话数据，不适用
        return false;
      } else {
        // 主播数据，判断是否直播
        return item.status === 1;
      }
    };

    const titleColumn = props.itemType === 'session' ? '直播标题-dc 点 hihivr 点 top' : '主播名称-dc 点 hihivr 点 top';

    return {
      scrollToCard,
      calculateTotalRevenue,
      formatCurrency,
      getItemTitle,
      getStatus,
      handleMiddleClick,
      isLive,
      titleColumn
    };
  }
};
</script>

<style scoped>
.navigation-table-container {
  background: #FFF8E1;
  border: 1px solid #FFC633;
  border-radius: 20px;
  padding: 15px;
  margin: 20px 0;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  position: relative;
  overflow: hidden;
}

.table-title {
  color: #FFC633;
  text-align: center;
  margin: 0 0 15px 0;
  font-size: 1.2em;
}

.table-wrapper {
  overflow-x: auto;
}

.navigation-table {
  width: 100%;
  border-collapse: collapse;
  background: #FFF8E1;
  border-radius: 15px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 5;
}

.navigation-table th,
.navigation-table td {
  padding: 10px 8px;
  text-align: left;
  border-bottom: 1px solid #FFC633;
}

.navigation-table th {
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
  font-weight: bold;
  position: sticky;
  top: 0;
  z-index: 10;
}

.nav-row {
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.nav-row:hover {
  background: #FFE5B4;
}

.rank-col, .rank-cell {
  text-align: center;
  width: 80px;
}

.title-col, .title-cell {
  min-width: 200px;
}

.status-col, .status-cell {
  text-align: center;
  width: 80px;
}

.status-cell.live-status {
  background-color: #f9729a; /* 洋红色背景表示直播中 */
  color: white;
  font-weight: bold;
  border-radius: 30px; /* 胶囊形圆角 */
  padding: 3px 8px; /* 调整内边距以匹配按钮样式 */
  display: inline-block; /* 行内块显示 */
  min-width: 80px; /* 最小宽度 */
  text-align: center; /* 居中对齐 */
  line-height: 1.4; /* 调整行高以实现垂直居中 */
  box-sizing: border-box; /* 确保padding不会超出边界 */
  vertical-align: middle; /* 垂直居中对齐 */
  align-self: center; /* 自身居中对齐 */
  height: calc(1.4em + 6px); /* 设置高度以匹配按钮样式 */
  display: flex; /* 使用flex布局 */
  align-items: center; /* 垂直居中内容 */
  justify-content: center; /* 水平居中内容 */
}

.revenue-col, .revenue-cell {
  text-align: right;
  width: 120px;
}

.action-col, .action-cell {
  text-align: center;
  width: 80px;
}

.jump-btn {
  background: #f9729a;
  color: white;
  border: none;
  border-radius: 15px;
  padding: 4px 12px;
  cursor: pointer;
  font-size: 0.8em;
  transition: background 0.3s ease;
}

.jump-btn:hover {
  background: #e0658a;
}

@media (max-width: 768px) {
  .navigation-table th,
  .navigation-table td {
    padding: 8px 5px;
    font-size: 0.85em;
  }
  
  .title-col, .title-cell {
    min-width: 150px;
  }
  
  .revenue-col, .revenue-cell {
    width: 100px;
  }
}

@media (max-width: 480px) {
  .navigation-table {
    font-size: 0.8em;
  }
  
  .navigation-table th,
  .navigation-table td {
    padding: 6px 4px;
  }
  
  .title-col, .title-cell {
    min-width: 120px;
  }
  
  .revenue-col, .revenue-cell {
    width: 80px;
  }
}
</style>