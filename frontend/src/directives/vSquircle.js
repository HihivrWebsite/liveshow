import { getSvgPath } from 'figma-squircle'

export const vSquircle = {
  mounted(el, binding) {
    const radius = binding.value || 32

    const updateClipPath = () => {
      const { width, height } = el.getBoundingClientRect()
      if (width === 0 || height === 0) return
      const svgPath = getSvgPath({
        width,
        height,
        cornerRadius: radius,
        cornerSmoothing: 0.6
      })
      el.style.clipPath = `path('${svgPath}')`
    }

    updateClipPath()

    const observer = new ResizeObserver(updateClipPath)
    observer.observe(el)
    el._squircleCleanup = () => observer.disconnect()
  },
  unmounted(el) {
    el._squircleCleanup?.()
  }
}
