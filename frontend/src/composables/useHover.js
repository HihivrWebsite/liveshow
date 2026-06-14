import { onMounted, onUnmounted } from 'vue';
import gsap from 'gsap';

export function useHover(elementRef, options = {}) {
  const { duration = 0.25, ease = 'power2.out', y = -1, scale = 1.005 } = options;

  let enterHandler, leaveHandler;

  onMounted(() => {
    const el = elementRef.value;
    if (!el) return;

    enterHandler = () => gsap.to(el, { y, scale, duration, ease });
    leaveHandler = () => gsap.to(el, { y: 0, scale: 1, duration, ease });

    el.addEventListener('mouseenter', enterHandler);
    el.addEventListener('mouseleave', leaveHandler);
  });

  onUnmounted(() => {
    const el = elementRef.value;
    if (!el) return;
    el.removeEventListener('mouseenter', enterHandler);
    el.removeEventListener('mouseleave', leaveHandler);
  });
}
