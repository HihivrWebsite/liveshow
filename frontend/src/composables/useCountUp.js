import { onMounted, ref } from 'vue';
import gsap from 'gsap';

export function useCountUp(targetValue, options = {}) {
  const { duration = 1.2, ease = 'power3.out', delay = 0, decimals = 0, prefix = '', suffix = '' } = options;

  const displayValue = ref(prefix + '0'.padStart(decimals > 0 ? decimals + 2 : 1, '0') + suffix);

  onMounted(() => {
    const obj = { value: 0 };
    gsap.to(obj, {
      value: targetValue,
      duration,
      ease,
      delay,
      onUpdate() {
        displayValue.value = prefix + obj.value.toFixed(decimals) + suffix;
      },
    });
  });

  return { displayValue };
}
