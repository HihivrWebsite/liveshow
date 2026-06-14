<template>
  <GlassCard
    v-squircle="32"
    :variant="variant"
    :padding="padding"
    :enter-delay="enterDelay"
    class="stat-card"
  >
    <div class="stat-card-inner">
      <div v-if="icon" class="stat-card-icon">
        <component :is="icon" />
      </div>
      <div class="stat-card-content">
        <div class="stat-card-label">{{ label }}</div>
        <div ref="valueRef" class="stat-card-value">
          {{ displayValue }}
        </div>
      </div>
    </div>
    <div v-if="$slots.extra" class="stat-card-extra">
      <slot name="extra" />
    </div>
  </GlassCard>
</template>

<script>
import { ref, watch, onMounted } from 'vue'
import { countUp } from '@/composables/useGSAP'
import GlassCard from './GlassCard.vue'

export default {
  name: 'StatCard',
  components: { GlassCard },
  props: {
    label: {
      type: String,
      required: true
    },
    value: {
      type: Number,
      required: true
    },
    prefix: {
      type: String,
      default: ''
    },
    suffix: {
      type: String,
      default: ''
    },
    icon: {
      type: [Object, Function],
      default: null
    },
    decimals: {
      type: Number,
      default: 0
    },
    variant: {
      type: String,
      default: 'default'
    },
    padding: {
      type: String,
      default: '20px 24px'
    },
    enterDelay: {
      type: Number,
      default: 0
    },
    animateOnChange: {
      type: Boolean,
      default: false
    }
  },
  setup(props) {
    const valueRef = ref(null)
    const displayValue = ref(props.prefix + '0' + props.suffix)

    const runAnimation = () => {
      if (!valueRef.value) return
      countUp(valueRef.value, props.value, {
        decimals: props.decimals,
        prefix: props.prefix,
        suffix: props.suffix,
        duration: 1.2,
        ease: 'power3.out'
      })
    }

    onMounted(() => {
      runAnimation()
    })

    if (props.animateOnChange) {
      watch(() => props.value, () => {
        runAnimation()
      })
    }

    return { valueRef, displayValue }
  }
}
</script>

<style scoped>
.stat-card {
  position: relative;
  overflow: hidden;
}

.stat-card-inner {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-card-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  border-radius: 16px;
  background: linear-gradient(135deg, var(--color-primary), var(--color-accent));
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 1.4rem;
}

.stat-card-content {
  flex: 1;
  min-width: 0;
}

.stat-card-label {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.stat-card-value {
  font-size: 1.6rem;
  font-weight: 700;
  color: var(--color-text-main);
  line-height: 1.2;
}

.stat-card-extra {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--glass-border);
}
</style>
