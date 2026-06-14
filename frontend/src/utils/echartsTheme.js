import * as echarts from 'echarts'

const theme = {
  color: ['#F6B100', '#FF6B9D', '#00BCD4', '#27AE60', '#E74C3C', '#9C27B0'],
  backgroundColor: 'transparent',
  textStyle: {
    color: '#5D4B24',
    fontFamily: "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif"
  },
  title: {
    textStyle: {
      color: '#5D4B24',
      fontWeight: 'bold'
    },
    subtextStyle: {
      color: '#8E7B50'
    }
  },
  legend: {
    textStyle: {
      color: '#8E7B50'
    }
  },
  tooltip: {
    backgroundColor: 'rgba(255, 255, 255, 0.9)',
    borderColor: 'rgba(246, 177, 0, 0.2)',
    textStyle: {
      color: '#5D4B24'
    }
  },
  categoryAxis: {
    axisLine: {
      lineStyle: {
        color: 'rgba(246, 177, 0, 0.2)'
      }
    },
    axisTick: {
      lineStyle: {
        color: 'rgba(246, 177, 0, 0.2)'
      }
    },
    axisLabel: {
      color: '#8E7B50'
    },
    splitLine: {
      lineStyle: {
        color: 'rgba(246, 177, 0, 0.08)'
      }
    }
  },
  valueAxis: {
    axisLine: {
      lineStyle: {
        color: 'rgba(246, 177, 0, 0.2)'
      }
    },
    axisTick: {
      lineStyle: {
        color: 'rgba(246, 177, 0, 0.2)'
      }
    },
    axisLabel: {
      color: '#8E7B50'
    },
    splitLine: {
      lineStyle: {
        color: 'rgba(246, 177, 0, 0.08)'
      }
    }
  },
  line: {
    smooth: true,
    symbolSize: 8,
    lineStyle: {
      width: 2
    }
  },
  bar: {
    barWidth: '60%',
    itemStyle: {
      borderRadius: [4, 4, 0, 0]
    }
  },
  pie: {
    itemStyle: {
      borderColor: '#fff',
      borderWidth: 2
    }
  },
  scatter: {
    symbolSize: 10
  }
}

echarts.registerTheme('liveshow', theme)

export default theme
