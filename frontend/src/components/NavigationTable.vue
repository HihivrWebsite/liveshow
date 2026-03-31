<template>
  <div class="navigation-table-container">
    <h3 class="table-title">📋 快速导航</h3>
    
    <!-- 恶意斗虫控制按钮 -->
    <div class="battle-controls" v-if="itemType === 'anchor'">
      <button 
        v-if="selectedAnchors.length > 0" 
        @click.stop="openBattleModal"
        class="battle-btn">
        🎯 恶意斗虫 ({{ selectedAnchors.length }})
      </button>
    </div>
    
    <div class="table-wrapper">
      <table class="navigation-table">
        <thead>
          <tr>
            <th class="rank-col">排名</th>
            <th class="title-col">{{ titleColumn }}</th>
            <th class="battle-col">恶意对比</th>
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
            <td class="battle-cell" @click.stop>
              <input 
                type="checkbox" 
                :id="'battle-' + item.room_id"
                :checked="selectedAnchors.some(a => a.room_id === item.room_id)"
                @change="toggleBattleSelect(item)">
              <label :for="'battle-' + item.room_id">对比</label>
            </td>
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
      default: 'anchor'
    }
  },
  data() {
    return {
      selectedAnchors: []
    }
  },
  methods: {
    toggleBattleSelect(item) {
      const index = this.selectedAnchors.findIndex(a => a.room_id === item.room_id);
      if (index > -1) {
        this.selectedAnchors.splice(index, 1);
      } else {
        if (this.selectedAnchors.length >= 10) {
          alert('最多只能选择 10 个主播进行对比');
          return;
        }
        // 确保传递 union 字段
        this.selectedAnchors.push({
          room_id: item.room_id,
          anchor_name: item.anchor_name,
          union: item.union || 'VirtuaReal'
        });
      }
    },
    openBattleModal() {
      if (this.selectedAnchors.length < 2) {
        alert('请至少选择 2 个主播进行对比');
        return;
      }
      this.$emit('open-battle', this.selectedAnchors);
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
        cardElement.style.transition = 'background-color 0.5s ease';
        cardElement.style.backgroundColor = 'rgba(249, 114, 154, 0.3)';
        setTimeout(() => {
          cardElement.style.backgroundColor = '';
        }, 2000);
      }
    };

    const handleMiddleClick = (event, index) => {
      if (event.button === 1) {
        const targetId = `card-${index + 1}`;
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
        title = item.title || item.anchor_name || `直播会话 ${item.rank || item.id || 'N/A'}`;
      } else {
        title = item.anchor_name || item.name || `项目 ${item.rank || item.id || 'N/A'}`;
      }
      return title + ' -dc 点 hihivr 点 top';
    };

    const getStatus = (item) => {
      if (props.itemType === 'session') {
        return '-';
      } else {
        if (item.status === 1) {
          return '直播中';
        } else {
          return '未开播';
        }
      }
    };

    const isLive = (item) => {
      if (props.itemType === 'session') {
        return false;
      } else {
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

.battle-controls {
  text-align: center;
  margin: 15px 0;
}

.battle-btn {
  background: linear-gradient(45deg, #ff6b6b, #ff4757);
  color: white;
  padding: 12px 24px;
  border: none;
  border-radius: 25px;
  font-size: 1rem;
  font-weight: bold;
  cursor: pointer;
  box-shadow: 0 4px 15px rgba(255, 107, 107, 0.4);
  transition: all 0.3s ease;
}

.battle-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(255, 107, 107, 0.6);
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
  padding: 12px 10px;
  text-align: left;
  border-bottom: 1px solid #FFC633;
  font-size: 1rem;
}

.navigation-table th {
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
  font-weight: bold;
  position: sticky;
  top: 0;
  z-index: 10;
  font-size: 1.05rem;
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
  width: 100px;
}

.title-col, .title-cell {
  min-width: 250px;
}

.battle-col, .battle-cell {
  text-align: center;
  width: 120px;
}

.battle-cell input[type="checkbox"] {
  width: 18px;
  height: 18px;
  margin-right: 6px;
  cursor: pointer;
}

.battle-cell label {
  cursor: pointer;
  color: #f9729a;
  font-weight: bold;
  font-size: 0.95rem;
  padding: 4px 8px;
}

.status-col, .status-cell {
  text-align: center;
  width: 100px;
}

.status-cell.live-status {
  background-color: #f9729a;
  color: white;
  font-weight: bold;
  border-radius: 30px;
  padding: 5px 10px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 90px;
  height: 28px;
  box-sizing: border-box;
  font-size: 0.9rem;
}

.revenue-col, .revenue-cell {
  text-align: right;
  width: 150px;
  font-size: 1rem;
}

.action-col, .action-cell {
  text-align: center;
  width: 100px;
}

.jump-btn {
  background: #f9729a;
  color: white;
  border: none;
  border-radius: 18px;
  padding: 6px 16px;
  cursor: pointer;
  font-size: 0.9rem;
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
