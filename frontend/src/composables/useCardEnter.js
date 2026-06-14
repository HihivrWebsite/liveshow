import { onMounted } from 'vue';
import gsap from 'gsap';

export function useCardEnter(elementRef, options = {}) {
  const { duration = 0.6, ease = 'expo.out', delay = 0, y = 30, scale = 0.95 } = options;

  onMounted(() => {
    const el = elementRef.value;
    if (!el) return;
    gsap.set(el, { opacity: 0, y, scale });
    gsap.to(el, { opacity: 1, y: 0, scale: 1, duration, ease, delay });
  });
}

export function useStaggerCards(containerRef, selector = ':scope > *', options = {}) {
  const { duration = 0.6, ease = 'expo.out', stagger = 0.1, y = 30, scale = 0.95 } = options;

  onMounted(() => {
    const container = containerRef.value;
    if (!container) return;
    const children = container.querySelectorAll(selector);
    if (!children.length) return;
    gsap.set(children, { opacity: 0, y, scale });
    gsap.to(children, { opacity: 1, y: 0, scale: 1, duration, ease, stagger });
  });
}
