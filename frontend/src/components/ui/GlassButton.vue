<template>
  <button
    ref="btnRef"
    v-squircle="999"
    :class="[
      'glass-btn',
      `glass-btn--${variant}`,
      `glass-btn--${size}`,
      { 'glass-btn--disabled': disabled, 'glass-btn--cta': cta }
    ]"
    :disabled="disabled"
    @click="$emit('click', $event)"
  >
    <slot />
  </button>
</template>

<script setup>
import { ref } from 'vue'
import { useHover } from '@/composables/useHover'
import { useCTAShine } from '@/composables/useCTAShine'

const props = defineProps({
  variant: {
    type: String,
    default: 'primary',
    validator: (v) => ['primary', 'secondary', 'danger', 'success', 'default', 'info', 'debug'].includes(v)
  },
  cta: {
    type: Boolean,
    default: false
  },
  disabled: {
    type: Boolean,
    default: false
  },
  size: {
    type: String,
    default: 'md',
    validator: (v) => ['sm', 'md', 'lg'].includes(v)
  }
})

defineEmits(['click'])

const btnRef = ref(null)

useHover(btnRef, { y: -3, scale: 1.03 })

if (props.cta) {
  useCTAShine(btnRef, { repeatDelay: 4 })
}
</script>

<style scoped>
.glass-btn {
  position: relative;
  overflow: hidden;
  border: none;
  font-weight: 600;
  cursor: pointer;
  color: #fff;
  background: var(--btn-color);
  box-shadow: var(--shadow-default, 0 8px 24px rgba(0, 0, 0, 0.04));
  transition: box-shadow 0.25s ease, opacity 0.25s ease;
  white-space: nowrap;
  will-change: transform;
}

.glass-btn:hover {
  box-shadow: var(--shadow-hover, 0 12px 32px rgba(0, 0, 0, 0.08));
}

.glass-btn--primary   { --btn-color: #F6B100; }
.glass-btn--secondary { --btn-color: #FF6B9D; }
.glass-btn--danger    { --btn-color: #E74C3C; }
.glass-btn--success   { --btn-color: #27AE60; }
.glass-btn--default   { --btn-color: #8E7B50; }
.glass-btn--info      { --btn-color: #00BCD4; }
.glass-btn--debug     { --btn-color: #9C27B0; }

.glass-btn--sm {
  padding: 6px 16px;
  font-size: 0.8rem;
}
.glass-btn--md {
  padding: 10px 28px;
  font-size: 0.95rem;
}
.glass-btn--lg {
  padding: 14px 40px;
  font-size: 1.1rem;
}

.glass-btn--disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}
</style>
