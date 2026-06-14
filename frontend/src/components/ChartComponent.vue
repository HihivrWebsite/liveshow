<template>
  <GlassCard variant="strong" padding="20px">
    <div class="chart-wrapper" :style="{ height: height }">
      <div ref="chartRef" :id="chartId" style="width: 100%; height: 100%;"></div>
    </div>
  </GlassCard>
</template>

<script>
import { ref, onMounted, onUnmounted, watch, shallowRef } from 'vue'
import * as echarts from 'echarts'
import '@/utils/echartsTheme.js'
import GlassCard from '@/components/ui/GlassCard.vue'

export default {
  name: 'ChartComponent',
  components: { GlassCard },
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
    const chartInstance = shallowRef(null)
    const chartRef = ref(null)

    const convertType = (chartJsType) => {
      const typeMap = {
        bar: 'bar',
        line: 'line',
        pie: 'pie',
        doughnut: 'pie',
        radar: 'radar',
        polarArea: 'radar',
        scatter: 'scatter',
        bubble: 'scatter'
      }
      return typeMap[chartJsType] || 'bar'
    }

    const convertData = (chartJsData, chartType) => {
      const labels = chartJsData.labels || []
      const datasets = chartJsData.datasets || []

      if (chartType === 'pie' || chartType === 'doughnut') {
        const ds = datasets[0] || {}
        return {
          series: [{
            type: 'pie',
            data: labels.map((label, i) => ({
              name: label,
              value: ds.data ? ds.data[i] : 0,
              itemStyle: ds.backgroundColor
                ? { color: Array.isArray(ds.backgroundColor) ? ds.backgroundColor[i] : ds.backgroundColor }
                : undefined
            })),
            radius: chartType === 'doughnut' ? ['40%', '70%'] : '70%'
          }]
        }
      }

      const echartsType = convertType(chartType)

      return {
        xAxis: {
          type: 'category',
          data: labels
        },
        yAxis: {
          type: 'value'
        },
        series: datasets.map((ds, index) => ({
          name: ds.label,
          type: echartsType,
          data: ds.data,
          itemStyle: ds.backgroundColor ? {
            color: Array.isArray(ds.backgroundColor) ? ds.backgroundColor[0] : ds.backgroundColor
          } : undefined,
          lineStyle: ds.borderColor ? {
            color: Array.isArray(ds.borderColor) ? ds.borderColor[0] : ds.borderColor,
            width: ds.borderWidth || 2
          } : undefined,
          smooth: ds.tension ? ds.tension > 0 : false,
          barWidth: chartType === 'bar' ? '60%' : undefined
        }))
      }
    }

    const buildOption = () => {
      const echartsType = convertType(props.type)
      const converted = convertData(props.data, props.type)

      const baseOption = {
        ...converted,
        tooltip: {
          trigger: echartsType === 'pie' ? 'item' : 'axis'
        },
        legend: {
          show: true
        },
        grid: {
          left: '3%',
          right: '4%',
          bottom: '3%',
          containLabel: true
        }
      }

      return { ...baseOption, ...props.options }
    }

    const initChart = () => {
      if (chartInstance.value) {
        chartInstance.value.dispose()
      }

      if (chartRef.value) {
        const instance = echarts.init(chartRef.value, 'liveshow')
        instance.setOption(buildOption())
        chartInstance.value = instance
      }
    }

    onMounted(() => {
      initChart()
      window.addEventListener('resize', handleResize)
    })

    onUnmounted(() => {
      window.removeEventListener('resize', handleResize)
      if (chartInstance.value) {
        chartInstance.value.dispose()
        chartInstance.value = null
      }
    })

    const handleResize = () => {
      if (chartInstance.value) {
        chartInstance.value.resize()
      }
    }

    watch(
      () => [props.data, props.options],
      () => {
        if (chartInstance.value) {
          chartInstance.value.setOption(buildOption(), true)
        } else {
          initChart()
        }
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
