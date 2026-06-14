import { onMounted } from 'vue';
import gsap from 'gsap';

export function usePageEnter(elementRef, options = {}) {
  const { duration = 0.8, ease = 'power4.out', delay = 0, y = 40 } = options;

  onMounted(() => {
    const el = elementRef.value;
    if (!el) return;
    gsap.set(el, { opacity: 0, y });
    gsap.to(el, { opacity: 1, y: 0, duration, ease, delay });
  });
}
