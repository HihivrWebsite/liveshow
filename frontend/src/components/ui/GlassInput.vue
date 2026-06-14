<template>
  <div v-squircle="24" :class="['glass-input-wrap', { 'glass-input-wrap--error': error, 'glass-input-wrap--disabled': disabled }]">
    <input
      ref="inputRef"
      class="glass-input"
      :type="type"
      :placeholder="placeholder"
      :disabled="disabled"
      :value="modelValue"
      @input="$emit('update:modelValue', $event.target.value)"
    />
    <span v-if="error" class="glass-input-error">{{ error }}</span>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useHover } from '@/composables/useHover'

defineProps({
  modelValue: {
    type: [String, Number],
    default: ''
  },
  type: {
    type: String,
    default: 'text'
  },
  placeholder: {
    type: String,
    default: ''
  },
  disabled: {
    type: Boolean,
    default: false
  },
  error: {
    type: String,
    default: ''
  }
})

defineEmits(['update:modelValue'])

const inputRef = ref(null)
useHover(inputRef, { y: -1, scale: 1.01 })
</script>

<style scoped>
.glass-input-wrap {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.glass-input {
  border: 2px solid rgba(142, 123, 80, 0.25);
  padding: 10px 20px;
  font-size: 0.95rem;
  color: var(--color-text-main, #5D4B24);
  background: rgba(255, 248, 225, 0.6);
  backdrop-filter: blur(var(--glass-blur));
  outline: none;
  transition: border-color 0.25s ease, box-shadow 0.25s ease;
  box-shadow: var(--shadow-default, 0 8px 24px rgba(0, 0, 0, 0.04));
  will-change: transform;
}

.glass-input::placeholder {
  color: var(--color-text-secondary, #8E7B50);
  opacity: 0.7;
}

.glass-input:focus {
  border-color: var(--color-primary, #F6B100);
  box-shadow: 0 0 0 3px rgba(246, 177, 0, 0.2), var(--shadow-hover, 0 12px 32px rgba(0, 0, 0, 0.08));
}

.glass-input-wrap--error .glass-input {
  border-color: #E74C3C;
}

.glass-input-wrap--error .glass-input:focus {
  box-shadow: 0 0 0 3px rgba(231, 76, 60, 0.2);
}

.glass-input-error {
  color: #E74C3C;
  font-size: 0.8rem;
  padding-left: 16px;
}

.glass-input-wrap--disabled .glass-input {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
