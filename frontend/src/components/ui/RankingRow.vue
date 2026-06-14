<template>
  <div ref="rowRef" v-squircle="16" class="ranking-row">
    <div class="ranking-row__rank" :class="rankClass">
      {{ displayRank }}
    </div>
    <div class="ranking-row__name">{{ name }}</div>
    <div class="ranking-row__value">
      <span class="ranking-row__number">{{ formattedValue }}</span>
      <span
        v-if="change !== 0 && change !== undefined"
        :class="['ranking-row__change', change > 0 ? 'ranking-row__change--up' : 'ranking-row__change--down']"
      >
        {{ change > 0 ? '▲' : '▼' }} {{ Math.abs(change) }}
      </span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useHover } from '@/composables/useHover'
import gsap from 'gsap'

const props = defineProps({
  rank: {
    type: Number,
    required: true
  },
  name: {
    type: String,
    required: true
  },
  value: {
    type: [Number, String],
    default: 0
  },
  change: {
    type: Number,
    default: 0
  }
})

const rowRef = ref(null)
const animatedValue = ref(0)

useHover(rowRef, { y: -2, scale: 1.01 })

const displayRank = computed(() => {
  if (props.rank <= 3) return ['🥇', '🥈', '🥉'][props.rank - 1]
  return `#${props.rank}`
})

const rankClass = computed(() => {
  if (props.rank === 1) return 'ranking-row__rank--gold'
  if (props.rank === 2) return 'ranking-row__rank--silver'
  if (props.rank === 3) return 'ranking-row__rank--bronze'
  return ''
})

const formattedValue = computed(() => {
  if (typeof props.value === 'string') return props.value
  return animatedValue.value.toLocaleString()
})

onMounted(() => {
  if (typeof props.value === 'number') {
    gsap.to(animatedValue, { value: props.value, duration: 1, ease: 'power3.out' })
  }
})

watch(() => props.value, (newVal) => {
  if (typeof newVal === 'number') {
    gsap.to(animatedValue, { value: newVal, duration: 0.6, ease: 'power2.out' })
  }
})
</script>

<style scoped>
.ranking-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 16px;
  background: rgba(255, 248, 225, 0.5);
  border: 1px solid rgba(255, 198, 51, 0.2);
  transition: background 0.2s ease;
  will-change: transform;
}

.ranking-row:hover {
  background: rgba(255, 248, 225, 0.8);
}

.ranking-row__rank {
  min-width: 40px;
  text-align: center;
  font-weight: 700;
  font-size: 1rem;
  color: var(--color-text-secondary, #8E7B50);
}

.ranking-row__rank--gold   { color: #F6B100; font-size: 1.2rem; }
.ranking-row__rank--silver { color: #A0A0A0; font-size: 1.2rem; }
.ranking-row__rank--bronze { color: #CD7F32; font-size: 1.2rem; }

.ranking-row__name {
  flex: 1;
  font-weight: 600;
  color: var(--color-text-main, #5D4B24);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ranking-row__value {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.ranking-row__number {
  font-weight: 700;
  color: var(--color-text-main, #5D4B24);
}

.ranking-row__change {
  font-size: 0.75rem;
  font-weight: 600;
}

.ranking-row__change--up   { color: #27AE60; }
.ranking-row__change--down { color: #E74C3C; }
</style>
