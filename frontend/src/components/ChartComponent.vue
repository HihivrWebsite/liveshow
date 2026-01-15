<template>
  <div class="chart-wrapper" :style="{ height: height }">
    <canvas :ref="chartRef" :id="chartId"></canvas>
  </div>
</template>

<script>
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { Chart, registerables } from 'chart.js'

Chart.register(...registerables)

export default {
  name: 'ChartComponent',
  props: {
    type: {
      type: String,
      default: 'bar'
    },
    data: {
      type: Object,
      required: true
    },
    options: {
      type: Object,
      default: () => ({})
    },
    height: {
      type: String,
      default: '400px'
    },
    chartId: {
      type: String,
      default: () => `chart-${Math.random().toString(36).substr(2, 9)}`
    }
  },
  setup(props) {
    let chartInstance = null
    const chartRef = ref(null)

    const initChart = () => {
      if (chartInstance) {
        chartInstance.destroy()
      }

      if (chartRef.value) {
        const ctx = chartRef.value.getContext('2d')
        chartInstance = new Chart(ctx, {
          type: props.type,
          data: props.data,
          options: {
            responsive: true,
            maintainAspectRatio: false,
            ...props.options
          }
        })
      }
    }

    onMounted(() => {
      initChart()
    })

    onUnmounted(() => {
      if (chartInstance) {
        chartInstance.destroy()
        chartInstance = null
      }
    })

    watch(
      () => [props.data, props.options],
      () => {
        initChart()
      },
      { deep: true }
    )

    return {
      chartRef
    }
  }
}
</script>

<style scoped>
.chart-wrapper {
  position: relative;
  width: 100%;
}
</style>