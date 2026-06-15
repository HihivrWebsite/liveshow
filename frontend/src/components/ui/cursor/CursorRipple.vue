<template>
  <div
    v-for="ripple in ripples"
    :key="ripple.id"
    class="cursor-ripple"
    :style="{
      left: ripple.x + 'px',
      top: ripple.y + 'px'
    }"
  />
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface Ripple {
  id: number
  x: number
  y: number
}

const ripples = ref<Ripple[]>([])
let rippleId = 0

function handleClick(e: MouseEvent) {
  const ripple: Ripple = {
    id: rippleId++,
    x: e.clientX,
    y: e.clientY
  }
  ripples.value.push(ripple)

  setTimeout(() => {
    ripples.value = ripples.value.filter(r => r.id !== ripple.id)
  }, 500)
}

onMounted(() => {
  document.addEventListener('click', handleClick)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClick)
})
</script>

<style scoped>
.cursor-ripple {
  position: fixed;
  width: 40px;
  height: 40px;
  margin-left: -20px;
  margin-top: -20px;
  border: 2px solid #FFC633;
  border-radius: 50%;
  pointer-events: none;
  z-index: 99997;
  animation: ripple-expand 0.5s ease-out forwards;
}

@keyframes ripple-expand {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  100% {
    transform: scale(2.5);
    opacity: 0;
  }
}
</style>
