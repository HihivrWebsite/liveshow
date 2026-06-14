<template>
  <div
    ref="cardRef"
    v-squircle="32"
    :class="[
      'glass-card',
      `glass-card--${variant}`,
      { 'glass-card--clickable': clickable }
    ]"
    :style="{ padding }"
    @click="clickable && $emit('click', $event)"
  >
    <slot />
  </div>
</template>

<script>
import { ref } from 'vue'
import { useCardEnter } from '@/composables/useCardEnter'
import { useHover } from '@/composables/useHover'

export default {
  name: 'GlassCard',
  props: {
    variant: {
      type: String,
      default: 'default',
      validator: (v) => ['default', 'strong', 'subtle'].includes(v)
    },
    padding: {
      type: String,
      default: '24px'
    },
    clickable: {
      type: Boolean,
      default: false
    },
    enterDelay: {
      type: Number,
      default: 0
    },
    disableAnimation: {
      type: Boolean,
      default: false
    }
  },
  emits: ['click'],
  setup(props) {
    const cardRef = ref(null)

    if (!props.disableAnimation) {
      useCardEnter(cardRef, { delay: props.enterDelay })
      useHover(cardRef)
    }

    return { cardRef }
  }
}
</script>

<style scoped>
.glass-card {
  overflow: hidden;
  box-shadow: var(--shadow-default);
  transition: box-shadow var(--duration-normal) var(--ease-out);
  will-change: transform, opacity;
}

.glass-card--default {
  background: var(--glass-bg);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
  border: 1px solid var(--glass-border);
}

.glass-card--strong {
  background: rgba(255, 255, 255, 0.72);
  backdrop-filter: blur(32px);
  -webkit-backdrop-filter: blur(32px);
  border: 1px solid rgba(255, 255, 255, 0.5);
}

.glass-card--subtle {
  background: rgba(255, 255, 255, 0.35);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.glass-card--clickable {
  cursor: pointer;
}

.glass-card--clickable:hover {
  box-shadow: var(--shadow-hover);
}
</style>
