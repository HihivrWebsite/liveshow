<template>
  <span v-squircle="999" :class="['animated-tag', `animated-tag--${variant}`]">
    <span class="animated-tag__dot" />
    {{ text }}
  </span>
</template>

<script setup>
defineProps({
  variant: {
    type: String,
    default: 'info',
    validator: (v) => ['live', 'offline', 'info'].includes(v)
  },
  text: {
    type: String,
    default: ''
  }
})
</script>

<style scoped>
.animated-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 14px;
  font-size: 0.8rem;
  font-weight: 600;
  white-space: nowrap;
  transition: transform 0.2s ease;
}

.animated-tag:hover {
  transform: scale(1.05);
}

.animated-tag__dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.animated-tag--live {
  background: rgba(255, 107, 157, 0.15);
  color: #FF6B9D;
}
.animated-tag--live .animated-tag__dot {
  background: #FF6B9D;
  animation: pulse-dot 1.4s ease-in-out infinite;
}

.animated-tag--offline {
  background: rgba(142, 123, 80, 0.12);
  color: #8E7B50;
}
.animated-tag--offline .animated-tag__dot {
  background: #8E7B50;
}

.animated-tag--info {
  background: rgba(0, 188, 212, 0.12);
  color: #00BCD4;
}
.animated-tag--info .animated-tag__dot {
  background: #00BCD4;
}

@keyframes pulse-dot {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.4; transform: scale(0.7); }
}
</style>
