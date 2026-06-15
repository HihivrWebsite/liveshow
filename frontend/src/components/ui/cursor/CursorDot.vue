<template>
  <div
    v-if="!isMobile && state.isVisible"
    class="cursor-dot"
    :style="dotStyle"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useCursor } from '@/composables/useCursor'

const { state, isMobile } = useCursor()

const dotStyle = computed(() => ({
  transform: `translate(${state.value.x - 4}px, ${state.value.y - 4}px)`,
  opacity: state.value.isVisible ? 1 : 0
}))
</script>

<style scoped>
.cursor-dot {
  position: fixed;
  top: 0;
  left: 0;
  width: 8px;
  height: 8px;
  background: #FFC633;
  border-radius: 50%;
  pointer-events: none;
  z-index: 99999;
  transition: opacity 0.3s ease;
}
</style>
