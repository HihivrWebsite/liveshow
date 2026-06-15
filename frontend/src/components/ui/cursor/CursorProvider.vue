<template>
  <div class="cursor-provider" v-if="!isMobile">
    <CursorDot />
    <CursorRing />
    <CursorRipple />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import gsap from 'gsap'
import { useCursor } from '@/composables/useCursor'
import CursorDot from './CursorDot.vue'
import CursorRing from './CursorRing.vue'
import CursorRipple from './CursorRipple.vue'

const { state, isMobile } = useCursor()

function setupHoverElements() {
  const hoverSelectors = 'button, a, [data-cursor-hover]'

  document.addEventListener('mouseover', (e) => {
    const target = e.target as HTMLElement
    if (target.closest(hoverSelectors)) {
      state.value.isHovering = true
    }
  })

  document.addEventListener('mouseout', (e) => {
    const target = e.target as HTMLElement
    if (target.closest(hoverSelectors)) {
      state.value.isHovering = false
    }
  })
}

function setupMagneticElements() {
  const magneticElements = document.querySelectorAll('[data-magnetic]')

  magneticElements.forEach(el => {
    const htmlEl = el as HTMLElement

    htmlEl.addEventListener('mousemove', (e: MouseEvent) => {
      const rect = htmlEl.getBoundingClientRect()
      const x = (e.clientX - rect.left - rect.width / 2) * 0.15
      const y = (e.clientY - rect.top - rect.height / 2) * 0.15

      gsap.to(htmlEl, {
        x,
        y,
        duration: 0.3,
        ease: 'power4.out'
      })
    })

    htmlEl.addEventListener('mouseleave', () => {
      gsap.to(htmlEl, {
        x: 0,
        y: 0,
        duration: 0.5,
        ease: 'elastic.out(1, 0.3)'
      })
    })
  })
}

onMounted(() => {
  if (!isMobile.value) {
    document.body.style.cursor = 'none'
    setupHoverElements()
    setupMagneticElements()
  }
})
</script>

<style>
.cursor-provider ~ * {
  cursor: none !important;
}

.cursor-provider ~ * a,
.cursor-provider ~ * button,
.cursor-provider ~ * [data-cursor-hover] {
  cursor: none !important;
}
</style>
