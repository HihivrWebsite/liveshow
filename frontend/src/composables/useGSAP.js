import gsap from 'gsap';

export function pageEnter(element, options = {}) {
  if (!element) return null;
  const { duration = 0.8, ease = 'power4.out', delay = 0, y = 40, opacity = 0 } = options;
  gsap.set(element, { opacity: 0, y });
  return gsap.to(element, { opacity: 1, y: 0, duration, ease, delay });
}

export function cardEnter(element, options = {}) {
  if (!element) return null;
  const { duration = 0.6, ease = 'expo.out', delay = 0, y = 30, opacity = 0, scale = 0.95 } = options;
  gsap.set(element, { opacity: 0, y, scale });
  return gsap.to(element, { opacity: 1, y: 0, scale: 1, duration, ease, delay });
}

export function hoverIn(element, options = {}) {
  if (!element) return null;
  const { duration = 0.25, ease = 'power2.out', y = -4, scale = 1.02 } = options;
  return gsap.to(element, { y, scale, duration, ease });
}

export function hoverOut(element, options = {}) {
  if (!element) return null;
  const { duration = 0.25, ease = 'power2.out', y = 0, scale = 1 } = options;
  return gsap.to(element, { y, scale, duration, ease });
}

export function countUp(element, targetValue, options = {}) {
  if (!element) return null;
  const { duration = 1.2, ease = 'power3.out', delay = 0, decimals = 0, prefix = '', suffix = '' } = options;
  const obj = { value: 0 };
  return gsap.to(obj, {
    value: targetValue,
    duration,
    ease,
    delay,
    onUpdate() {
      element.textContent = prefix + obj.value.toFixed(decimals) + suffix;
    },
  });
}

export function staggerEnter(elements, options = {}) {
  if (!elements || (elements.length !== undefined && elements.length === 0)) return null;
  const { duration = 0.6, ease = 'expo.out', stagger = 0.1, y = 30, opacity = 0, scale = 0.95 } = options;
  gsap.set(elements, { opacity: 0, y, scale });
  return gsap.to(elements, { opacity: 1, y: 0, scale: 1, duration, ease, stagger });
}
