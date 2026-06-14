<template>
  <Teleport to="body">
    <Transition
      @before-enter="onBeforeEnter"
      @enter="onEnter"
      @leave="onLeave"
      @after-leave="onAfterLeave"
    >
      <div
        v-if="visible"
        class="dialog-overlay"
        @click.self="onOverlayClick"
      >
        <div
          ref="dialogRef"
          v-squircle="40"
          class="dialog-panel glass"
          :style="panelStyle"
        >
          <div class="dialog-header" v-if="title || $slots.header">
            <slot name="header">
              <h3 class="dialog-title">{{ title }}</h3>
            </slot>
            <button class="dialog-close" @click="close" aria-label="关闭">
              <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                <path d="M15 5L5 15M5 5l10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
            </button>
          </div>
          <div class="dialog-body">
            <slot />
          </div>
          <div class="dialog-footer" v-if="$slots.footer">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { ref, computed, watch, nextTick } from 'vue'
import gsap from 'gsap'

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  title: {
    type: String,
    default: ''
  },
  width: {
    type: String,
    default: '420px'
  },
  closeOnOverlay: {
    type: Boolean,
    default: true
  }
})

const emit = defineEmits(['update:visible', 'close', 'opened'])

const dialogRef = ref(null)

const panelStyle = computed(() => ({
  maxWidth: props.width
}))

function close() {
  emit('update:visible', false)
  emit('close')
}

function onOverlayClick() {
  if (props.closeOnOverlay) {
    close()
  }
}

function onBeforeEnter(el) {
  const panel = el.querySelector('.dialog-panel')
  if (panel) {
    gsap.set(panel, { opacity: 0, y: 60, scale: 0.95 })
  }
  gsap.set(el, { opacity: 0 })
}

function onEnter(el, done) {
  const panel = el.querySelector('.dialog-panel')
  const tl = gsap.timeline({
    onComplete: () => {
      done()
      nextTick(() => emit('opened'))
    }
  })
  tl.to(el, { opacity: 1, duration: 0.25, ease: 'power2.out' })
  if (panel) {
    tl.to(panel, { opacity: 1, y: 0, scale: 1, duration: 0.4, ease: 'back.out(1.2)' }, '-=0.15')
  }
}

function onLeave(el, done) {
  const panel = el.querySelector('.dialog-panel')
  const tl = gsap.timeline({ onComplete: done })
  if (panel) {
    tl.to(panel, { opacity: 0, y: 40, scale: 0.95, duration: 0.3, ease: 'power2.in' })
  }
  tl.to(el, { opacity: 0, duration: 0.2, ease: 'power2.in' }, '-=0.15')
}

function onAfterLeave() {
  // cleanup if needed
}
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(246, 177, 0, 0.15);
  padding: 20px;
}

.dialog-panel {
  width: 100%;
  overflow-x: hidden;
  overflow-y: auto;
  box-shadow: var(--shadow-dialog);
  padding: 28px;
  position: relative;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.dialog-title {
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--color-text-main);
  margin: 0;
}

.dialog-close {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: none;
  background: rgba(93, 75, 36, 0.08);
  color: var(--color-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--duration-fast) var(--ease-out);
  flex-shrink: 0;
}

.dialog-close:hover {
  background: rgba(93, 75, 36, 0.15);
  color: var(--color-text-main);
  transform: rotate(90deg);
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  color: var(--color-text-main);
}

.dialog-footer {
  margin-top: 20px;
  flex-shrink: 0;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
</style>
