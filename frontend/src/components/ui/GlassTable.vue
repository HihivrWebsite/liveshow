<template>
  <div ref="tableRef" v-squircle="24" class="glass-table-wrap">
    <div class="glass-table-scroll">
      <table class="glass-table">
        <thead>
          <tr>
            <th
              v-for="col in columns"
              :key="col.key"
              :style="{ width: col.width, textAlign: col.align || 'left' }"
              @click="col.sortable && handleSort(col.key)"
            >
              <span class="glass-table-th-content">
                {{ col.title }}
                <span v-if="col.sortable && sortKey === col.key" class="glass-table-sort-icon">
                  {{ sortOrder === 'asc' ? '↑' : '↓' }}
                </span>
              </span>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(row, index) in sortedData"
            :key="row[rowKey] || index"
            :class="['glass-table-row', { 'glass-table-row--hoverable': hoverable, 'glass-table-row--divider': rowDivider }]"
          >
            <td
              v-for="col in columns"
              :key="col.key"
              :style="{ textAlign: col.align || 'left' }"
            >
              <slot :name="`cell-${col.key}`" :row="row" :value="row[col.key]" :index="index">
                {{ row[col.key] }}
              </slot>
            </td>
          </tr>
          <tr v-if="!data.length">
            <td :colspan="columns.length" class="glass-table-empty">
              <slot name="empty">暂无数据</slot>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="$slots.footer" class="glass-table-footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<script>
import { ref, computed } from 'vue'
import { useCardEnter } from '@/composables/useCardEnter'

export default {
  name: 'GlassTable',
  props: {
    columns: {
      type: Array,
      required: true
    },
    data: {
      type: Array,
      default: () => []
    },
    rowKey: {
      type: String,
      default: 'id'
    },
    hoverable: {
      type: Boolean,
      default: true
    },
    rowDivider: {
      type: Boolean,
      default: false
    },
    defaultSort: {
      type: String,
      default: ''
    },
    defaultSortOrder: {
      type: String,
      default: 'asc'
    }
  },
  emits: ['sort-change'],
  setup(props, { emit }) {
    const tableRef = ref(null)
    const sortKey = ref(props.defaultSort)
    const sortOrder = ref(props.defaultSortOrder)

    useCardEnter(tableRef)

    const sortedData = computed(() => {
      if (!sortKey.value) return props.data
      return [...props.data].sort((a, b) => {
        const va = a[sortKey.value]
        const vb = b[sortKey.value]
        if (va == null) return 1
        if (vb == null) return -1
        const cmp = typeof va === 'number' ? va - vb : String(va).localeCompare(String(vb))
        return sortOrder.value === 'asc' ? cmp : -cmp
      })
    })

    const handleSort = (key) => {
      if (sortKey.value === key) {
        sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
      } else {
        sortKey.value = key
        sortOrder.value = 'asc'
      }
      emit('sort-change', { key: sortKey.value, order: sortOrder.value })
    }

    return { tableRef, sortKey, sortOrder, sortedData, handleSort }
  }
}
</script>

<style scoped>
.glass-table-wrap {
  background: var(--glass-bg);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
  border: 1px solid var(--glass-border);
  overflow: hidden;
  box-shadow: var(--shadow-default);
}

.glass-table-scroll {
  overflow-x: auto;
}

.glass-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.95rem;
  color: var(--color-text-main);
}

.glass-table thead {
  position: sticky;
  top: 0;
  z-index: 2;
}

.glass-table th {
  background: rgba(246, 177, 0, 0.12);
  padding: 0 16px;
  height: 48px;
  font-weight: 600;
  color: var(--color-text-main);
  border-bottom: 1px solid var(--glass-border);
  white-space: nowrap;
  user-select: none;
}

.glass-table th[sortable],
.glass-table th .glass-table-sort-icon {
  cursor: pointer;
}

.glass-table-th-content {
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.glass-table-sort-icon {
  font-size: 0.8em;
  color: var(--color-primary);
}

.glass-table td {
  padding: 0 16px;
  height: 48px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 300px;
}

.glass-table-row--hoverable {
  transition: background var(--duration-fast) var(--ease-out);
}

.glass-table-row--hoverable:hover {
  background: rgba(246, 177, 0, 0.06);
}

.glass-table-row:last-child td {
  border-bottom: none;
}

.glass-table-row--divider td {
  border-bottom: 2px solid rgba(246, 177, 0, 0.4);
}

.glass-table-row--divider:last-child td {
  border-bottom: none;
}

.glass-table-empty {
  text-align: center;
  color: var(--color-text-secondary);
  padding: 32px 16px !important;
  height: auto !important;
}

.glass-table-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--glass-border);
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 8px;
}
</style>
