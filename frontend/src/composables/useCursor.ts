import { ref, onMounted, onUnmounted } from 'vue'
import gsap from 'gsap'

interface CursorState {
  x: number
  y: number
  prevX: number
  prevY: number
  velocityX: number
  velocityY: number
  speed: number
  isHovering: boolean
  isClicking: boolean
  isVisible: boolean
}

const state = ref<CursorState>({
  x: 0, y: 0,
  prevX: 0, prevY: 0,
  velocityX: 0, velocityY: 0,
  speed: 0,
  isHovering: false,
  isClicking: false,
  isVisible: false
})

let rafId: number | null = null
let lastTime = 0

function updateVelocity(currentX: number, currentY: number, deltaTime: number) {
  if (deltaTime > 0) {
    state.value.velocityX = (currentX - state.value.prevX) / deltaTime
    state.value.velocityY = (currentY - state.value.prevY) / deltaTime
    state.value.speed = Math.sqrt(
      state.value.velocityX ** 2 + state.value.velocityY ** 2
    )
  }
  state.value.prevX = currentX
  state.value.prevY = currentY
}

export function useCursor() {
  const isMobile = ref(false)

  function handleMouseMove(e: MouseEvent) {
    state.value.x = e.clientX
    state.value.y = e.clientY
    state.value.isVisible = true

    const now = performance.now()
    const deltaTime = (now - lastTime) / 1000
    lastTime = now
    updateVelocity(e.clientX, e.clientY, deltaTime)
  }

  function handleMouseDown() {
    state.value.isClicking = true
  }

  function handleMouseUp() {
    state.value.isClicking = false
  }

  function handleMouseLeave() {
    state.value.isVisible = false
  }

  function handleMouseEnter() {
    state.value.isVisible = true
  }

  function checkMobile() {
    isMobile.value = window.innerWidth < 768 || 'ontouchstart' in window
  }

  onMounted(() => {
    checkMobile()
    if (!isMobile.value) {
      document.addEventListener('mousemove', handleMouseMove)
      document.addEventListener('mousedown', handleMouseDown)
      document.addEventListener('mouseup', handleMouseUp)
      document.addEventListener('mouseleave', handleMouseLeave)
      document.addEventListener('mouseenter', handleMouseEnter)
      window.addEventListener('resize', checkMobile)
    }
  })

  onUnmounted(() => {
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mousedown', handleMouseDown)
    document.removeEventListener('mouseup', handleMouseUp)
    document.removeEventListener('mouseleave', handleMouseLeave)
    document.removeEventListener('mouseenter', handleMouseEnter)
    window.removeEventListener('resize', checkMobile)
  })

  return {
    state,
    isMobile
  }
}
