import { onMounted, onUnmounted } from 'vue';
import gsap from 'gsap';

export function useCTAShine(elementRef, options = {}) {
  const { duration = 1.5, ease = 'power2.inOut', delay = 0, autoPlay = true, repeat = -1, repeatDelay = 3 } = options;

  let timeline = null;

  function createShine(el) {
    el.style.position = 'relative';
    el.style.overflow = 'hidden';

    const shine = document.createElement('span');
    shine.style.cssText = `
      position: absolute;
      top: 0;
      left: -100%;
      width: 100%;
      height: 100%;
      background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
      pointer-events: none;
    `;
    el.appendChild(shine);

    timeline = gsap.timeline({ delay, repeat, repeatDelay });
    timeline.fromTo(
      shine,
      { left: '-100%' },
      { left: '100%', duration, ease }
    );

    if (!autoPlay) timeline.pause();
    return timeline;
  }

  onMounted(() => {
    const el = elementRef.value;
    if (!el) return;
    createShine(el);
  });

  onUnmounted(() => {
    if (timeline) {
      timeline.kill();
      timeline = null;
    }
  });

  function play() { timeline?.play(); }
  function pause() { timeline?.pause(); }
  function restart() { timeline?.restart(); }

  return { play, pause, restart };
}
