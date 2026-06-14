<template>
  <GlassCard class="navigation-table-container" padding="15px">
    <h3 class="table-title"><Table2 :size="20" /> 快速导航</h3>
    <p class="export-hint-big"><Camera :size="18" /> 点击导出截图一键截图</p>
    
    <div class="battle-controls" v-if="itemType === 'anchor'">
      <span class="battle-hint" :class="{ 'battle-hint-alert': battleHintVisible }">
        {{ battleHintVisible ? '请先勾选至少2个主播！' : '选择多个主播后点击进行数据对比' }}
      </span>
      <GlassButton 
        variant="danger"
        size="md"
        @click.stop="openBattleModal">
        <Crosshair :size="16" /> 恶意斗虫 ({{ selectedAnchors.length }})
      </GlassButton>
      <GlassButton 
        variant="secondary"
        size="md"
        cta
        @click.stop="$emit('open-export')">
        <Camera :size="16" /> 导出截图 ({{ selectedAnchors.length }})
      </GlassButton>
      <GlassButton 
        variant="info"
        size="md"
        @click.stop="openRankComparison">
        <BarChart3 :size="16" /> 排名对比 ({{ selectedAnchors.length }})
      </GlassButton>
    </div>
    
    <GlassTable
      :columns="tableColumns"
      :data="items"
      :row-key="rowKeyFn"
      hoverable
      row-divider
    >
      <template #cell-rank="{ row, value, index }">
        {{ index + 1 }}
      </template>
      <template #cell-title="{ row }">
        <img v-if="row.room_id && avatars[row.room_id]" :src="avatars[row.room_id]" class="nav-avatar" @error="$event.target.style.display='none'" />
        {{ getItemTitle(row) }}
      </template>
      <template #cell-select="{ row }" >
        <span @click.stop>
          <input 
            type="checkbox" 
            :id="'battle-' + row.room_id"
            :checked="selectedAnchors.some(a => a.room_id === row.room_id)"
            @change="toggleBattleSelect(row)">
          <label :for="'battle-' + row.room_id" class="select-label">多选</label>
        </span>
      </template>
      <template #cell-status="{ row }">
        <AnimatedTag v-if="isLive(row)" variant="live" text="直播中" />
        <AnimatedTag v-else variant="offline" text="未开播" />
      </template>
      <template #cell-revenue="{ row }">
        {{ formatCurrency(calculateTotalRevenue(row)) }}
      </template>
      <template #cell-action="{ row }">
        <GlassButton
          variant="secondary"
          size="sm"
          @click.stop="scrollToCard(items.indexOf(row))"
          @mousedown.stop="handleMiddleClick($event, items.indexOf(row))"
        >
          <ArrowDown :size="14" /> 跳转
        </GlassButton>
      </template>
    </GlassTable>
  </GlassCard>
</template>

<script>
import { formatCurrency } from '@/utils/dataProcessor'
import { anchorAPI } from '@/api'
import { getAvatarSync } from '@/utils/avatarCache'
import GlassCard from '@/components/ui/GlassCard.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import GlassTable from '@/components/ui/GlassTable.vue'
import AnimatedTag from '@/components/ui/AnimatedTag.vue'
import { Table2, Camera, Crosshair, BarChart3, ArrowDown } from 'lucide-vue-next'

export default {
  name: 'NavigationTable',
  components: { GlassCard, GlassButton, GlassTable, AnimatedTag, Table2, Camera, Crosshair, BarChart3, ArrowDown },
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
      selectedAnchors: [],
      battleHintVisible: false,
      avatars: {}
    }
  },
  computed: {
    isAllSelected() {
      return this.items.length > 0 && this.selectedAnchors.length === this.items.length;
    },
    tableColumns() {
      if (this.itemType === 'session') {
        return [
          { key: 'rank', title: '排名', width: '100px', align: 'center' },
          { key: 'title', title: this.titleColumn, width: '250px' },
          { key: 'revenue', title: '总营收', width: '150px', align: 'right' },
          { key: 'action', title: '操作', width: '100px', align: 'center' }
        ]
      }
      return [
        { key: 'rank', title: '排名', width: '100px', align: 'center' },
        { key: 'title', title: this.titleColumn, width: '250px' },
        { key: 'select', title: '全选', width: '120px', align: 'center' },
        { key: 'status', title: '开播状态', width: '100px', align: 'center' },
        { key: 'revenue', title: '总营收', width: '150px', align: 'right' },
        { key: 'action', title: '操作', width: '100px', align: 'center' }
      ]
    }
  },
  created() {
    if (this.itemType === 'anchor') {
      this.items.forEach(item => {
        if (item.room_id) {
          this.avatars[item.room_id] = getAvatarSync(item.room_id)
        }
      })
    }
  },
  methods: {
    rowKeyFn(item) {
      return item.room_id || item.id
    },
    toggleBattleSelect(item) {
      const index = this.selectedAnchors.findIndex(a => a.room_id === item.room_id);
      if (index > -1) {
        this.selectedAnchors.splice(index, 1);
      } else {
        this.selectedAnchors.push({
          room_id: item.room_id,
          anchor_name: item.anchor_name,
          union: item.union || 'VirtuaReal'
        });
      }
      this.$emit('selection-change', [...this.selectedAnchors]);
    },
    toggleSelectAll() {
      if (this.isAllSelected) {
        this.selectedAnchors = [];
      } else {
        this.selectedAnchors = this.items.map(item => ({
          room_id: item.room_id,
          anchor_name: item.anchor_name,
          union: item.union || 'VirtuaReal'
        }));
      }
      this.$emit('selection-change', [...this.selectedAnchors]);
    },
    openBattleModal() {
      if (this.selectedAnchors.length === 0) {
        this.battleHintVisible = true;
        setTimeout(() => { this.battleHintVisible = false; }, 2000);
        return;
      }
      if (this.selectedAnchors.length < 2) {
        this.battleHintVisible = true;
        setTimeout(() => { this.battleHintVisible = false; }, 2000);
        return;
      }
      this.$emit('open-battle', this.selectedAnchors);
    },
    openRankComparison() {
      if (this.selectedAnchors.length < 2) {
        this.battleHintVisible = true;
        setTimeout(() => { this.battleHintVisible = false; }, 2000);
        return;
      }
      this.$emit('open-rank', this.selectedAnchors);
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
        cardElement.style.backgroundColor = 'rgba(246, 177, 0, 0.2)';
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
  margin: 20px 0 50px 0;
  position: relative;
  overflow: visible;
  padding-bottom: 10px;
}

.table-title {
  color: var(--color-primary);
  text-align: center;
  margin: 0 0 15px 0;
  font-size: 1.2em;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.export-hint-big {
  color: var(--color-accent);
  font-size: 1.3rem;
  font-weight: bold;
  text-align: center;
  margin: 5px 0 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

.battle-controls {
  text-align: center;
  margin: 15px 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  flex-wrap: wrap;
}

.battle-hint-alert {
  color: #E74C3C !important;
  animation: hint-shake 0.5s ease-in-out;
}

@keyframes hint-shake {
  0%, 100% { transform: translateX(0); }
  20% { transform: translateX(-5px); }
  40% { transform: translateX(5px); }
  60% { transform: translateX(-3px); }
  80% { transform: translateX(3px); }
}

.battle-hint {
  color: var(--color-primary);
  font-size: 1rem;
  font-weight: bold;
  margin-right: 10px;
}

.nav-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  object-fit: cover;
  vertical-align: middle;
  margin-right: 6px;
  border: 1px solid var(--color-primary);
}

.select-label {
  cursor: pointer;
  color: var(--color-accent);
  font-weight: bold;
  font-size: 0.95rem;
  padding: 4px 8px;
}

@media (max-width: 768px) {
  .battle-controls {
    flex-direction: column;
  }
}
</style>
