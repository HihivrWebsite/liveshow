<template>
  <div
    v-if="!isMobile && state.isVisible"
    ref="ringRef"
    :class="['cursor-ring', { 'cursor-ring-hover': state.isHovering, 'cursor-ring-click': state.isClicking }]"
    :style="ringStyle"
  />
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useCursor } from '@/composables/useCursor'

const { state, isMobile } = useCursor()
const ringRef = ref<HTMLElement | null>(null)

const ringStyle = computed(() => {
  const speed = state.value.speed
  const vx = state.value.velocityX
  const vy = state.value.velocityY

  let scaleX = 1
  let scaleY = 1
  let rotation = 0

  if (speed > 5) {
    const angle = Math.atan2(vy, vx)
    const stretch = Math.min(speed / 500, 0.4)
    scaleX = 1 + stretch * Math.abs(Math.cos(angle))
    scaleY = 1 + stretch * Math.abs(Math.sin(angle))
    rotation = (angle * 180) / Math.PI
  }

  return {
    transform: `translate(${state.value.x - 16}px, ${state.value.y - 16}px) scaleX(${scaleX}) scaleY(${scaleY}) rotate(${rotation}deg)`
  }
})
</script>

<style scoped>
.cursor-ring {
  position: fixed;
  top: 0;
  left: 0;
  width: 32px;
  height: 32px;
  border: 2px solid #FFC633;
  border-radius: 50%;
  pointer-events: none;
  z-index: 99998;
  opacity: 1;
  will-change: transform;
  transition: transform 0.15s cubic-bezier(0.25, 0.8, 0.25, 1),
              width 0.3s ease,
              height 0.3s ease,
              margin 0.3s ease,
              border-width 0.3s ease,
              opacity 0.3s ease;
}

.cursor-ring-hover {
  width: 80px;
  height: 80px;
  margin-left: -24px;
  margin-top: -24px;
  border-width: 3px;
  opacity: 0.8;
}

.cursor-ring-click {
  width: 24px;
  height: 24px;
  margin-left: 4px;
  margin-top: 4px;
}
</style>
