<template>
  <div v-squircle="999" class="live-counter">
    <span ref="dotRef" class="live-dot"></span>
    <span class="live-label">{{ label }}</span>
    <span ref="numberRef" class="live-number">{{ displayValue }}</span>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import gsap from 'gsap'

const props = defineProps({
  count: {
    type: Number,
    default: 0
  },
  label: {
    type: String,
    default: '正在直播'
  }
})

const dotRef = ref(null)
const numberRef = ref(null)
const displayValue = ref('0')
let breatheTween = null
let countTween = null

function animateCount(from, to) {
  if (countTween) countTween.kill()
  const obj = { value: from }
  countTween = gsap.to(obj, {
    value: to,
    duration: 0.8,
    ease: 'power2.out',
    onUpdate() {
      displayValue.value = Math.round(obj.value).toString()
    }
  })
}

onMounted(() => {
  animateCount(0, props.count)

  if (dotRef.value) {
    breatheTween = gsap.to(dotRef.value, {
      scale: 1.4,
      opacity: 0.5,
      duration: 1.2,
      ease: 'sine.inOut',
      repeat: -1,
      yoyo: true
    })
  }
})

watch(() => props.count, (newVal, oldVal) => {
  animateCount(oldVal ?? 0, newVal)
})

onBeforeUnmount(() => {
  if (breatheTween) breatheTween.kill()
  if (countTween) countTween.kill()
})
</script>

<style scoped>
.live-counter {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: rgba(255, 107, 157, 0.1);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
}

.live-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--color-accent);
  flex-shrink: 0;
}

.live-label {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.live-number {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--color-text-main);
  min-width: 1.5em;
  text-align: right;
}
</style>
