<template>
  <div class="anchor-list">
    <div class="controls-section">
      <div class="filter-controls">
        <button
          @click="switchFilter('all')"
          :class="['filter-btn', { active: currentFilter === 'all' }]"
        >
          维阿PSP斗虫榜
        </button>
        <button
          @click="switchFilter('vr')"
          :class="['filter-btn', { active: currentFilter === 'vr' }]"
        >
          维阿斗虫榜
        </button>
        <button
          @click="switchFilter('psp')"
          :class="['filter-btn', { active: currentFilter === 'psp' }]"
        >
          PSPlive斗虫榜
        </button>
      </div>

      <div class="action-controls">
        <button @click="openMonthSelector" class="action-btn secondary">
          切换不同月份
        </button>
        <button @click="openMultiMonthModal" class="action-btn secondary">
          多月份共同统计
        </button>
      </div>

      <!-- 月份选择器模态框 -->
      <div v-if="showMonthSelector" class="modal-overlay" @click="closeMonthSelector">
        <div class="modal-content" @click.stop>
          <h3>切换月份</h3>
          <div class="modal-form">
            <div class="form-group">
              <label>选择月份:</label>
              <input type="month" v-model="monthSelection" class="month-input">
            </div>
            <div class="button-group">
              <button @click="performMonthSwitch" class="confirm-btn">确定</button>
              <button @click="closeMonthSelector" class="cancel-btn">取消</button>
            </div>
          </div>
        </div>
      </div>

      <!-- 多月份统计模态框 -->
      <div v-if="showMultiMonthModal" class="modal-overlay" @click="closeMultiMonthModal">
        <div class="modal-content" @click.stop>
          <h3>多月份共同统计</h3>
          <div class="modal-form">
            <div class="form-group">
              <label>起始月份:</label>
              <input type="month" v-model="startMonth" class="month-input">
            </div>
            <div class="form-group">
              <label>结束月份:</label>
              <input type="month" v-model="endMonth" class="month-input">
            </div>
            <div class="button-group">
              <button @click="performMultiMonthQuery" class="confirm-btn">确定</button>
              <button @click="closeMultiMonthModal" class="cancel-btn">取消</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="info-section">
      <h2 class="page-title">{{ title }}</h2>
      <p class="refresh-time">刷新时间：{{ refreshTime }}</p>
    </div>

    <div class="chart-button-container">
      <button @click="showRevenueChart" class="action-btn primary" title="营收占比分析">
        查看营收占比
      </button>
      <button
        v-if="currentFilter === 'all'"
        @click="showVRPSPComparison"
        class="action-btn primary"
        title="VR与PSP工会数据对比"
      >
        VR PSP对比图
      </button>
      <button @click="hideAllCharts" class="action-btn danger">
        关闭图表
      </button>
      <button @click="openRegressionAnalysisModal" class="action-btn primary">
        进行回归分析
      </button>
      <!--
      <button @click="openClusterAnalysisModal" class="action-btn primary">
        进行聚类分析
      </button>
      -->
    </div>

    <!-- 聚类分析模态框 -->
    <div v-if="showClusterModal" class="modal-overlay" @click="closeClusterModal">
      <div class="modal-content" @click.stop style="width: 600px;">
        <h3>聚类分析</h3>
        <div class="cluster-analysis-form">
          <div class="form-group">
            <label>选择聚类变量 (X):</label>
            <div class="checkbox-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="attention"> 关注数
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="effective_days"> 有效天
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="live_duration"> 开播时长
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="guard_3"> 总督
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="guard_2"> 提督
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="guard_1"> 舰长
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="fans_count"> 粉丝团
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="gift"> 礼物收入
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="guard"> 舰长收入
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="super_chat"> SC收入
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="clusteringVariables" value="total_revenue"> 总营收
              </label>
            </div>
          </div>

          <div class="form-group">
            <label>选择聚类数量:</label>
            <select v-model="numClusters" class="variable-select">
              <option value="2">2类</option>
              <option value="3">3类</option>
              <option value="4">4类</option>
              <option value="5">5类</option>
            </select>
          </div>

          <div class="button-group">
            <button @click="performClusterAnalysis" class="confirm-btn" :disabled="!canPerformClusterAnalysis">确定</button>
            <button @click="closeClusterModal" class="cancel-btn">取消</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 回归分析模态框 -->
    <div v-if="showRegressionModal" class="modal-overlay" @click="closeRegressionModal">
      <div class="modal-content" @click.stop style="width: 600px;">
        <h3>回归分析</h3>
        <div class="regression-analysis-form">
          <div class="form-group">
            <label>选择因变量 (Y):</label>
            <select v-model="dependentVariable" class="variable-select">
              <option value="">请选择</option>
              <option value="attention">关注数</option>
              <option value="effective_days">有效天</option>
              <option value="live_duration">开播时长</option>
              <option value="guard_3">总督</option>
              <option value="guard_2">提督</option>
              <option value="guard_1">舰长</option>
              <option value="fans_count">粉丝团</option>
              <option value="gift">礼物收入</option>
              <option value="guard">舰长收入</option>
              <option value="super_chat">SC收入</option>
              <option value="total_revenue">总营收</option>
            </select>
          </div>

          <div class="form-group">
            <label>选择自变量 (X):</label>
            <div class="checkbox-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="attention"> 关注数
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="effective_days"> 有效天
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="live_duration"> 开播时长
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="guard_3"> 总督
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="guard_2"> 提督
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="guard_1"> 舰长
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="fans_count"> 粉丝团
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="gift"> 礼物收入
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="guard"> 舰长收入
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="super_chat"> SC收入
              </label>
              <label class="checkbox-label">
                <input type="checkbox" v-model="independentVariables" value="total_revenue"> 总营收
              </label>
            </div>
          </div>

          <div class="button-group">
            <button @click="performRegressionAnalysis" class="confirm-btn" :disabled="!canPerformAnalysis || regressionLoading">
              <span v-if="regressionLoading">计算中...</span>
              <span v-else>确定</span>
            </button>
            <button @click="closeRegressionModal" class="cancel-btn">取消</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 聚类分析结果图表容器 -->
    <div v-if="clusterAnalysisVisible" class="cluster-chart-container">
      <div class="chart-header">
        <h3>聚类分析结果</h3>
        <button @click="closeClusterAnalysis" class="close-chart-btn">关闭</button>
      </div>
      <div class="analysis-results">
        <div class="statistics-panel">
          <h4>聚类统计信息</h4>
          <div v-if="clusterResults" class="stats-grid">
            <div class="stat-item">
              <strong>聚类数量:</strong> {{ clusterResults.numClusters }}
            </div>
            <div class="stat-item">
              <strong>变量数量:</strong> {{ clusterResults.variables.length }}
            </div>
            <div class="stat-item">
              <strong>数据点数量:</strong> {{ clusterResults.dataPoints }}
            </div>
            <div class="stat-item">
              <strong>轮廓系数:</strong> {{ clusterResults.silhouetteScore ? clusterResults.silhouetteScore.toFixed(4) : 'N/A' }}
            </div>
          </div>
        </div>

        <div class="cluster-summary">
          <h4>聚类简略说明</h4>
          <div v-if="clusterResults" v-html="clusterResults.summary"></div>
        </div>
      </div>

      <!-- 图表导航 -->
      <div class="chart-navigation">
        <button @click="currentClusterChart = '2d'" :class="{'active': currentClusterChart === '2d'}">二维散点图</button>
        <button @click="currentClusterChart = '3d'" :class="{'active': currentClusterChart === '3d'}">三维散点图</button>
      </div>

      <!-- 图表容器 -->
      <div class="chart-container">
        <canvas v-if="currentClusterChart === '2d'" id="cluster2dChart" ref="cluster2dChart"></canvas>
        <canvas v-if="currentClusterChart === '3d'" id="cluster3dChart" ref="cluster3dChart"></canvas>
      </div>
    </div>

    <!-- 回归分析结果图表容器 -->
    <div v-if="regressionAnalysisVisible" class="regression-chart-container">
      <div class="chart-header">
        <h3>回归分析结果</h3>
        <button @click="closeRegressionAnalysis" class="close-chart-btn">关闭</button>
      </div>
      <div class="analysis-results">
        <div class="statistics-panel">
          <h4>统计信息</h4>
          <div v-if="regressionResults" class="stats-grid">
            <div class="stat-item">
              <strong>R² (拟合优度):</strong> {{ regressionResults.rSquared.toFixed(4) }}
            </div>
            <div class="stat-item">
              <strong>调整R²:</strong> {{ regressionResults.adjustedRSquared.toFixed(4) }}
            </div>
            <div class="stat-item">
              <strong>F统计量:</strong> {{ regressionResults.fStatistic.toFixed(4) }}
            </div>
            <div class="stat-item">
              <strong>总体p值:</strong> {{ regressionResults.overallPValue.toExponential(4) }}
            </div>
          </div>
        </div>

        <div class="coefficients-panel">
          <h4>回归系数</h4>
          <table class="coefficients-table">
            <thead>
              <tr>
                <th>变量</th>
                <th>系数</th>
                <th>标准误差</th>
                <th>t统计量</th>
                <th>p值</th>
                <th>显著性</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(coef, index) in regressionResults.coefficients" :key="index">
                <td>{{ coef.variable }}</td>
                <td>{{ coef.value.toFixed(6) }}</td>
                <td>{{ coef.stdError.toFixed(6) }}</td>
                <td>{{ coef.tStat.toFixed(4) }}</td>
                <td>{{ coef.pValue.toExponential(4) }}</td>
                <td>
                  <span :class="getSignificanceClass(coef.pValue)">
                    {{ getSignificanceLabel(coef.pValue) }}
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div class="analysis-summary">
          <h4>分析摘要</h4>
          <div v-if="regressionResults" v-html="regressionResults.summary"></div>
        </div>
      </div>

      <!--
      图表导航
      <div class="chart-navigation">
        <button @click="regressionCurrentChart = 'scatter'" :class="{'active': regressionCurrentChart === 'scatter'}">散点图 + 拟合线</button>
        <button @click="regressionCurrentChart = 'residual'" :class="{'active': regressionCurrentChart === 'residual'}">残差图</button>
        <button @click="regressionCurrentChart = 'prediction'" :class="{'active': regressionCurrentChart === 'prediction'}">预测 vs 实测</button>
        <button @click="regressionCurrentChart = 'coefficients'" :class="{'active': regressionCurrentChart === 'coefficients'}">系数图</button>
      </div>

      图表容器
      <div class="chart-container">
        <canvas v-if="regressionCurrentChart === 'scatter'" id="scatterChart" ref="scatterChart"></canvas>
        <canvas v-if="regressionCurrentChart === 'residual'" id="residualChart" ref="residualChart"></canvas>
        <canvas v-if="regressionCurrentChart === 'prediction'" id="predictionChart" ref="predictionChart"></canvas>
        <canvas v-if="regressionCurrentChart === 'coefficients'" id="coefficientsChart" ref="coefficientsChart"></canvas>
      </div>
      -->
    </div>

    <div class="chart-info" v-if="chartVisible">
      <h3 style="color: #f9729a; margin-top: 0;">📊 图表交互说明</h3>
      <p><strong>图表功能：</strong></p>
      <ul style="text-align: left; display: inline-block;">
        <li>点击图例可以隐藏/显示对应的数据显示</li>
        <li>鼠标悬停在饼图块上可以查看详细数值和百分比</li>
        <li>图表支持缩放和拖拽（如果浏览器支持）</li>
        <li>双击图表可以重置缩放</li>
      </ul>
    </div>

    <div :class="['chart-container', { visible: chartVisible }]">
      <canvas id="chartCanvas" ref="chartCanvas"></canvas>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <button @click="fetchData" class="retry-btn">重试</button>
    </div>

    <div v-else class="data-section">
      <!-- 导航表格 -->
      <NavigationTable :items="anchors" item-type="anchor" v-if="anchors.length > 0" />

      <div class="grid-container">
        <BaseCard
          v-for="(anchor, index) in anchors"
          :key="anchor.room_id || index"
          card-type="anchor"
          :rank="index + 1"
          :title="anchor.anchor_name + ' [' + anchor.union + ']'"
          :subtitle="''"
          :fields="[
            { label: '关注数', value: formatNumber(anchor.attention), type: 'number' },
            { label: '有效天', value: anchor.effective_days },
            { label: '开播时长', value: anchor.live_duration, type: 'duration' },
            { label: '开播状态', value: anchor.status === 1 ? '正在直播' : '未开播' },
            { label: '总督', value: anchor.guard_3 || 0 },
            { label: '提督', value: anchor.guard_2 || 0 },
            { label: '舰长', value: anchor.guard_1 || 0 },
            { label: '粉丝团', value: formatNumber(anchor.fans_count || 0), type: 'number' },
            { label: '礼物收入', value: formatCurrency(anchor.gift), type: 'currency' },
            { label: '舰长收入', value: formatCurrency(anchor.guard), type: 'currency' },
            { label: 'SC收入', value: formatCurrency(anchor.super_chat), type: 'currency' },
            { label: '总营收', value: formatCurrency(calculateTotalRevenue(anchor)), type: 'currency' }
          ]"
          :action-button="{ text: '查看详细数据', className: 'view-btn' }"
          :action-data="anchor"
          @action-click="viewLiveSessions(anchor.room_id, anchor.union)"
        >
          <template #actions>
            <button
              @click="viewLiveSessions(anchor.room_id, anchor.union)"
              class="view-btn"
            >
              查看详细数据
            </button>
          </template>
        </BaseCard>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, watch, nextTick, computed, provide } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Chart, registerables } from 'chart.js'
import { anchorAPI } from '@/api'
import BaseCard from '@/components/BaseCard.vue'
import NavigationTable from '@/components/NavigationTable.vue'
import { provideGlobalCardState } from '@/composables/useGlobalCardState'

Chart.register(...registerables)

export default {
  name: 'AnchorList',
  components: {
    BaseCard,
    NavigationTable
  },
  setup() {
    const router = useRouter()
    const route = useRoute()
    const chartVisible = ref(false)
    const anchors = ref([])
    const title = ref('维阿PSP斗虫榜')
    const refreshTime = ref(new Date().toLocaleString())
    const currentFilter = ref('all')
    const loading = ref(true)
    const error = ref(null)
    let currentChart = null
    const chartCanvas = ref(null)

    // 创建并提供全局卡片状态
    const globalCardState = provideGlobalCardState()
    provide('globalCardState', globalCardState)

    // 从路由参数获取初始值
    const filterFromRoute = route.query.filter || 'all'
    const monthFromRoute = route.query.month || null
    currentFilter.value = filterFromRoute

    // 更新标题
    if (monthFromRoute) {
      const year = monthFromRoute.substring(0, 4)
      const month = parseInt(monthFromRoute.substring(4, 6)).toString().padStart(2, '0')
      title.value = `维阿PSP斗虫榜_${year}年${month}月记录数据（点击"正在直播"跳转到对应直播间）`
    } else {
      title.value = filterFromRoute === 'vr' ? '维阿斗虫榜' :
                   filterFromRoute === 'psp' ? 'PSPlive斗虫榜' : '维阿PSP斗虫榜'
    }

    // 获取数据
    const fetchData = async () => {
      try {
        loading.value = true
        error.value = null
        let response;
        const currentMonth = route.query.month || null;
        if (currentMonth) {
          response = await anchorAPI.getAnchorsByMonth(currentMonth, currentFilter.value);
        } else {
          response = await anchorAPI.getAnchors(currentFilter.value, currentMonth);
        }
        anchors.value = response.anchors || response.data || []
        refreshTime.value = response.refresh_time || new Date().toLocaleString()
      } catch (err) {
        console.error('获取主播数据失败:', err)
        error.value = '获取数据失败，请稍后重试'
      } finally {
        loading.value = false
      }
    }

    const viewLiveSessions = (roomId, union) => {
      const currentMonth = route.query.month || new Date().toISOString().slice(0, 7).replace('-', '');
      router.push(`/live-sessions?room_id=${roomId}&union=${union}&month=${currentMonth}`)
    }

    const switchFilter = (filterType) => {
      router.push({
        path: '/',
        query: { ...route.query, filter: filterType }
      })
    }

    // 控制所有卡片展开/收起的方法 - 使用全局状态展开所有卡片
    const toggleAllCards = () => {
      // 使用全局卡片状态管理器来切换所有卡片的展开状态
      globalCardState.toggleAllCards();
    }

    // 月份选择器相关
    const showMonthSelector = ref(false)
    const monthSelection = ref('')

    const openMonthSelector = () => {
      // 设置默认值为当前年月
      const now = new Date()
      const currentYear = now.getFullYear()
      const currentMonth = String(now.getMonth() + 1).padStart(2, '0')
      monthSelection.value = `${currentYear}-${currentMonth}`
      showMonthSelector.value = true
    }

    const closeMonthSelector = () => {
      showMonthSelector.value = false
    }

    const performMonthSwitch = () => {
      if (!monthSelection.value) {
        alert('请选择月份')
        return
      }

      // 验证月份格式
      const selectedDate = new Date(monthSelection.value)
      if (isNaN(selectedDate.getTime())) {
        alert('无效的月份')
        return
      }

      const selectedMonth = monthSelection.value.replace('-', '')

      router.push({
        path: '/by-month',
        query: { month: selectedMonth, filter: currentFilter.value }
      })
      closeMonthSelector()
    }

    const showRevenueChart = async () => {
      const data = []
      const labels = []

      anchors.value.forEach(anchor => {
        const revenue = parseFloat(anchor.total_revenue || anchor.gift + anchor.guard + anchor.super_chat || 0)
        if (!isNaN(revenue) && revenue > 0) {
          data.push(revenue)
          labels.push(anchor.anchor_name)
        }
      })

      if (data.length === 0) {
        alert('没有可用的数据来生成图表')
        return
      }

      chartVisible.value = true

      await nextTick()

      if (currentChart) {
        currentChart.destroy()
      }

      const ctx = chartCanvas.value.getContext('2d')
      currentChart = new Chart(ctx, {
        type: 'pie',
        data: {
          labels: labels,
          datasets: [{
            label: '总营收占比',
            data: data,
            backgroundColor: [
              '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF',
              '#FF9F40', '#FF6384', '#C9CBCF', '#4BC0C0', '#FF6384',
              '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', '#FF9F40'
            ],
            borderWidth: 2,
            borderColor: '#fff'
          }]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            title: {
              display: true,
              text: '主播营收占比',
              font: {
                size: 16
              }
            },
            legend: {
              position: 'right',
              labels: {
                padding: 8,  // 减少间距
                usePointStyle: true,
                pointStyle: 'circle',
                borderRadius: 6,  // 增大图例图标
                fontSize: 12
              }
            },
            tooltip: {
              callbacks: {
                label: function(context) {
                  const value = context.parsed
                  const total = context.dataset.data.reduce((a, b) => a + b, 0)
                  const percentage = Math.round((value / total) * 100)
                  return `${context.label}: ${value.toFixed(2)} (${percentage}%)`
                }
              }
            }
          }
        }
      })
    }

    const showVRPSPComparison = async () => {
      if (currentFilter.value !== 'all') {
        alert('VR PSP对比图仅在"维阿PSP斗虫榜"模式下可用')
        return
      }

      let vrTotal = 0
      let pspTotal = 0

      anchors.value.forEach(anchor => {
        const revenue = parseFloat(anchor.total_revenue || anchor.gift + anchor.guard + anchor.super_chat || 0) || 0

        if (anchor.union === 'VirtuaReal') {
          vrTotal += revenue
        } else if (anchor.union === 'PSPlive') {
          pspTotal += revenue
        }
      })

      if (vrTotal === 0 && pspTotal === 0) {
        alert('没有可用的数据来生成对比图')
        return
      }

      chartVisible.value = true

      await nextTick()

      if (currentChart) {
        currentChart.destroy()
      }

      const ctx = chartCanvas.value.getContext('2d')
      currentChart = new Chart(ctx, {
        type: 'pie',
        data: {
          labels: ['VirtuaReal', 'PSPlive'],
          datasets: [{
            label: '工会总营收对比',
            data: [vrTotal, pspTotal],
            backgroundColor: ['#FF6384', '#36A2EB'],
            borderWidth: 2,
            borderColor: '#fff'
          }]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            title: {
              display: true,
              text: 'VR vs PSP 总营收对比',
              font: {
                size: 16
              }
            },
            legend: {
              position: 'right',
              labels: {
                padding: 8,  // 减少间距
                usePointStyle: true,
                pointStyle: 'circle',
                borderRadius: 6,  // 增大图例图标
                fontSize: 12
              }
            },
            tooltip: {
              callbacks: {
                label: function(context) {
                  const value = context.parsed
                  const total = context.dataset.data.reduce((a, b) => a + b, 0)
                  const percentage = Math.round((value / total) * 100)
                  return `${context.label}: ${value.toFixed(2)} (${percentage}%)`
                }
              }
            }
          }
        }
      })
    }

    const hideAllCharts = () => {
      chartVisible.value = false
      if (currentChart) {
        currentChart.destroy()
        currentChart = null
      }
    }

    const calculateTotalRevenue = (anchor) => {
      return parseFloat(anchor.gift || 0) + parseFloat(anchor.guard || 0) + parseFloat(anchor.super_chat || 0)
    }

    const formatCurrency = (value) => {
      return parseFloat(value || 0).toFixed(2)
    }

    const formatNumber = (value) => {
      return new Intl.NumberFormat().format(value || 0)
    }

    const formatLiveDuration = (durationStr) => {
      // 解析 API 返回的 HH:MM:SS 格式
      if (!durationStr) return '0小时0分钟 (0分钟)'

      // 尝试解析 HH:MM:SS 格式
      const timeParts = durationStr.split(':')
      if (timeParts.length >= 2) {
        const hours = parseInt(timeParts[0]) || 0
        const minutes = parseInt(timeParts[1]) || 0

        // 计算总分钟数
        const totalMinutes = hours * 60 + minutes

        let result = `${hours}小时${minutes}分钟 (${totalMinutes}分钟)`
        // 在括号前插入换行标记，类似LiveSessions.vue中的处理方式
        return result.replace(/\s\(/, '<br>(')
      }

      // 如果不是 HH:MM:SS 格式，返回原值
      return durationStr
    }

    const formatDurationWithBreak = (durationStr) => {
      // 格式化时长并在括号前添加换行
      const formatted = formatLiveDuration(durationStr);
      // 在括号前添加换行
      const parts = formatted.split(' (');
      if (parts.length > 1) {
        return `${parts[0]}<br>(${parts.slice(1).join('(')}`;
      }
      return formatted;
    }

    // 多月份统计相关
    const showMultiMonthModal = ref(false)
    const startMonth = ref('')
    const endMonth = ref('')

    // 回归分析相关
    const showRegressionModal = ref(false)
    const dependentVariable = ref('')
    const independentVariables = ref([])
    const regressionAnalysisVisible = ref(false)
    const regressionResults = ref(null)
    let regressionChartInstance = null
    const regressionAnalysisChart = ref(null)
    const regressionLoading = ref(false)
    const regressionErrorMessage = ref('')
    const regressionCurrentChart = ref('scatter') // 默认显示散点图
    const scatterChart = ref(null)
    const residualChart = ref(null)
    const predictionChart = ref(null)
    const coefficientsChart = ref(null)
    let scatterChartInstance = null
    let residualChartInstance = null
    let predictionChartInstance = null
    let coefficientsChartInstance = null

    // 聚类分析相关
    const showClusterModal = ref(false)
    const clusteringVariables = ref([])
    const numClusters = ref('3')
    const clusterAnalysisVisible = ref(false)
    const clusterResults = ref(null)
    const clusterLoading = ref(false)
    const clusterError = ref(null)
    const currentClusterChart = ref('2d') // 默认显示2D图
    const cluster2dChart = ref(null)
    const cluster3dChart = ref(null)
    let cluster2dChartInstance = null
    let cluster3dChartInstance = null

    const openMultiMonthModal = () => {
      // 设置默认值为当前年月
      const now = new Date()
      const currentYear = now.getFullYear()
      const currentMonth = String(now.getMonth() + 1).padStart(2, '0')
      startMonth.value = `${currentYear}-${currentMonth}`
      endMonth.value = `${currentYear}-${currentMonth}`
      showMultiMonthModal.value = true
    }

    const closeMultiMonthModal = () => {
      showMultiMonthModal.value = false
    }

    // 回归分析相关方法
    const openRegressionAnalysisModal = () => {
      showRegressionModal.value = true
    }

    const closeRegressionModal = () => {
      showRegressionModal.value = false
    }

    const closeRegressionAnalysis = () => {
      regressionAnalysisVisible.value = false
      if (regressionChartInstance) {
        regressionChartInstance.destroy()
        regressionChartInstance = null
      }
    }

    const canPerformAnalysis = computed(() => {
      return dependentVariable.value && independentVariables.value.length > 0
    })

    // 执行回归分析
    const performRegressionAnalysis = async () => {
      if (!canPerformAnalysis.value) {
        alert('请至少选择一个因变量和一个自变量')
        return
      }

      // 显示计算中提示
      regressionLoading.value = true
      regressionErrorMessage.value = ''

      try {
        // 准备数据
        const validAnchors = anchors.value.filter(anchor => {
          // 检查因变量和自变量是否都有有效值
          const hasDependent = anchor[dependentVariable.value] !== undefined &&
                              anchor[dependentVariable.value] !== null &&
                              !isNaN(parseFloat(anchor[dependentVariable.value]))

          const hasIndependents = independentVariables.value.every(varName =>
            anchor[varName] !== undefined &&
            anchor[varName] !== null &&
            !isNaN(parseFloat(anchor[varName]))
          )

          return hasDependent && hasIndependents
        })

        if (validAnchors.length < independentVariables.value.length + 1) {
          throw new Error(`数据不足，至少需要${independentVariables.value.length + 1}个有效数据点，但只有${validAnchors.length}个`)
        }

        // 提取数据
        const yData = validAnchors.map(anchor => parseFloat(anchor[dependentVariable.value]))
        const xData = validAnchors.map(anchor =>
          independentVariables.value.map(varName => parseFloat(anchor[varName]))
        )

        // 执行多元线性回归分析
        const regressionResult = performLinearRegression(yData, xData)

        // 检查结果是否有效
        if (!regressionResult || !regressionResult.coefficients || !regressionResult.predictedValues) {
          throw new Error('回归分析未能生成有效结果')
        }

        // 计算统计量
        const stats = calculateRegressionStats(yData, regressionResult.predictedValues, xData)

        // 生成分析摘要
        const summary = generateAnalysisSummary(dependentVariable.value, independentVariables.value, stats, regressionResult)

        // 保存结果
        regressionResults.value = {
          coefficients: regressionResult.coefficients.map((coef, idx) => ({
            variable: idx === 0 ? '截距' : independentVariables.value[idx - 1],
            value: coef.value,
            stdError: coef.stdError,
            tStat: coef.tStat,
            pValue: coef.pValue
          })),
          rSquared: stats.rSquared,
          adjustedRSquared: stats.adjustedRSquared,
          fStatistic: stats.fStatistic,
          overallPValue: stats.overallPValue,
          predictedValues: regressionResult.predictedValues,
          summary: summary
        }

        // 显示结果
        regressionAnalysisVisible.value = true
        showRegressionModal.value = false

        // 绘制当前选择的图表
        await nextTick()
        drawCurrentChart(validAnchors, regressionResult.predictedValues)
      } catch (error) {
        console.error('回归分析失败:', error)
        regressionErrorMessage.value = `回归分析失败: ${error.message}`
        alert(`回归分析失败: ${error.message}`)
      } finally {
        // 隐藏计算中提示
        regressionLoading.value = false
      }
    }

    // 多元线性回归实现
    const performLinearRegression = (y, x) => {
      try {
        const n = y.length
        const k = x[0].length // 自变量数量

        // 构造设计矩阵 X (添加截距项)
        const X = Array.from({ length: n }, (_, i) => [1, ...x[i]])

        // 计算 X'X
        const XtX = multiplyMatrix(transpose(X), X)

        // 计算 (X'X)^(-1)
        const XtXInv = inverseMatrix(XtX)

        // 计算回归系数 β̂ = (X'X)^(-1)X'y
        const XtY = multiplyMatrix(transpose(X), y.map(val => [val]))
        const coefficientsMatrix = multiplyMatrix(XtXInv, XtY)

        // 提取系数
        const coefficients = coefficientsMatrix.map(row => row[0])

        // 计算预测值
        const predicted = X.map(row =>
          row.reduce((sum, val, idx) => sum + val * coefficients[idx], 0)
        )

        // 计算标准误差和t统计量
        const residuals = y.map((actual, idx) => actual - predicted[idx])
        const mse = residuals.reduce((sum, res) => sum + res * res, 0) / (n - k - 1)

        const varCoefficients = XtXInv.map(row => row.map(val => val * mse))
        const stdErrors = Array.from({ length: coefficients.length }, (_, i) =>
          Math.sqrt(Math.abs(varCoefficients[i][i]))
        )

        const tStats = coefficients.map((coef, idx) =>
          stdErrors[idx] !== 0 ? coef / stdErrors[idx] : 0
        )

        // 计算p值 (使用正态分布近似)
        const pValues = tStats.map(t => {
          try {
            return 2 * (1 - cumulativeNormal(Math.abs(t)))
          } catch (e) {
            console.error('计算p值时出错:', e)
            return 1 // 返回默认值
          }
        })

        return {
          coefficients: coefficients.map((value, idx) => ({
            value,
            stdError: stdErrors[idx] || 0,
            tStat: tStats[idx] || 0,
            pValue: pValues[idx] || 1
          })),
          predictedValues: predicted
        }
      } catch (error) {
        console.error('回归分析计算出错:', error)
        // 返回默认值
        return {
          coefficients: Array(k + 1).fill({ value: 0, stdError: 0, tStat: 0, pValue: 1 }),
          predictedValues: y.map(() => 0)
        }
      }
    }

    // 矩阵乘法
    const multiplyMatrix = (a, b) => {
      const rowsA = a.length
      const colsA = a[0].length
      const rowsB = b.length
      const colsB = b[0].length

      if (colsA !== rowsB) {
        throw new Error('矩阵维度不匹配')
      }

      const result = Array.from({ length: rowsA }, () => Array(colsB).fill(0))

      for (let i = 0; i < rowsA; i++) {
        for (let j = 0; j < colsB; j++) {
          for (let k = 0; k < colsA; k++) {
            result[i][j] += a[i][k] * b[k][j]
          }
        }
      }

      return result
    }

    // 矩阵转置
    const transpose = (matrix) => {
      const rows = matrix.length
      const cols = matrix[0].length
      const result = Array.from({ length: cols }, () => Array(rows).fill(0))

      for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
          result[j][i] = matrix[i][j]
        }
      }

      return result
    }

    // 矩阵求逆 (使用高斯-约旦消元法)
    const inverseMatrix = (matrix) => {
      const n = matrix.length
      // 创建增广矩阵 [A|I]
      const augmented = matrix.map((row, i) =>
        [...row, ...Array(n).fill(0).map((_, j) => i === j ? 1 : 0)]
      )

      // 高斯-约旦消元
      for (let i = 0; i < n; i++) {
        // 寻找主元素
        let maxRow = i
        for (let j = i + 1; j < n; j++) {
          if (Math.abs(augmented[j][i]) > Math.abs(augmented[maxRow][i])) {
            maxRow = j
          }
        }

        // 交换行
        [augmented[i], augmented[maxRow]] = [augmented[maxRow], augmented[i]]

        // 检查奇异矩阵
        if (Math.abs(augmented[i][i]) < 1e-10) {
          throw new Error('矩阵不可逆')
        }

        // 缩放主行
        const pivot = augmented[i][i]
        for (let j = 0; j < 2 * n; j++) {
          augmented[i][j] /= pivot
        }

        // 消元其他行
        for (let j = 0; j < n; j++) {
          if (j !== i) {
            const factor = augmented[j][i]
            for (let k = 0; k < 2 * n; k++) {
              augmented[j][k] -= factor * augmented[i][k]
            }
          }
        }
      }

      // 提取逆矩阵
      return augmented.map(row => row.slice(n))
    }

    // 计算回归统计量
    const calculateRegressionStats = (yActual, yPredicted, xData) => {
      const n = yActual.length
      const k = xData[0].length // 自变量数量

      // 总平方和
      const yMean = yActual.reduce((sum, val) => sum + val, 0) / n
      const tss = yActual.reduce((sum, val) => sum + Math.pow(val - yMean, 2), 0)

      // 残差平方和
      const rss = yActual.reduce((sum, val, idx) =>
        sum + Math.pow(val - yPredicted[idx], 2), 0)

      // 回归平方和
      const ess = tss - rss

      // R²
      const rSquared = 1 - (rss / tss)

      // 调整R²
      const adjustedRSquared = 1 - ((rss / (n - k - 1)) / (tss / (n - 1)))

      // F统计量
      const fStatistic = (ess / k) / (rss / (n - k - 1))

      // 整体p值 (F分布近似)
      const overallPValue = 1 - cumulativeFDistribution(fStatistic, k, n - k - 1)

      return {
        rSquared,
        adjustedRSquared,
        fStatistic,
        overallPValue
      }
    }

    // 标准正态分布累积函数 (近似)
    const cumulativeNormal = (z) => {
      // 使用近似公式
      const t = 1 / (1 + 0.2316419 * Math.abs(z))
      const d = 0.3989423 * Math.exp((-z * z) / 2)
      let prob = d * t * (0.3193815 + t * (-0.3565638 + t * (1.781478 + t * (-1.821256 + t * 1.330274))))

      if (z > 0) {
        prob = 1 - prob
      }

      return prob
    }

    // F分布累积函数 (简化近似)
    const cumulativeFDistribution = (f, df1, df2) => {
      // 使用Beta函数近似
      if (f <= 0) return 0

      const x = (df1 * f) / (df1 * f + df2)
      return incompleteBeta(x, df1 / 2, df2 / 2)
    }

    // 不完全Beta函数 (近似)
    const incompleteBeta = (x, a, b) => {
      if (x <= 0) return 0
      if (x >= 1) return 1

      // 使用连续分数展开
      const lbeta = logGamma(a) + logGamma(b) - logGamma(a + b)
      const fac = Math.exp(a * Math.log(x) + b * Math.log(1 - x) - lbeta) / a

      let c = 1
      let d = 1 - (a + b) * x / (a + 1)
      if (Math.abs(d) < 1e-30) d = 1e-30
      d = 1 / d
      let h = d

      for (let m = 1; m <= 200; m++) {
        const m2 = 2 * m
        let aa = m * (b - m) * x / ((a + m2 - 2) * (a + m2 - 1))
        d = 1 + aa * d
        if (Math.abs(d) < 1e-30) d = 1e-30
        c = 1 + aa / c
        if (Math.abs(c) < 1e-30) c = 1e-30
        d = 1 / d
        h *= d * c
        aa = -(a + m - 1) * (a + b + m - 1) * x / ((a + m2 - 1) * (a + m2))
        d = 1 + aa * d
        if (Math.abs(d) < 1e-30) d = 1e-30
        c = 1 + aa / c
        if (Math.abs(c) < 1e-30) c = 1e-30
        d = 1 / d
        const del = d * c
        h *= del
        if (Math.abs(del - 1) < 1e-10) break
      }

      return h * fac
    }

    // Gamma函数对数 (近似)
    const logGamma = (xx) => {
      const x = xx - 1.0
      let y = x
      let tmp = x + 5.5
      tmp = (x + 0.5) * Math.log(tmp) - tmp
      let ser = 1.000000000190015
      const cof = [
        76.18009172947146, -86.50532032941677, 24.01409824083091,
        -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5
      ]

      for (let j = 0; j < 6; j++) {
        y += 1
        ser += cof[j] / y
      }

      return tmp + Math.log(2.5066282746310005 * ser / x)
    }

    // 生成分析摘要
    const generateAnalysisSummary = (depVar, indepVars, stats, regressionResult) => {
      let summary = `<p><strong>回归模型:</strong> ${formatVariableName(depVar)} = `

      regressionResult.coefficients.forEach((coef, idx) => {
        const sign = idx === 0 ? '' : (coef.value >= 0 ? ' + ' : ' - ')
        const varName = idx === 0 ? '' : formatVariableName(indepVars[idx - 1])
        const absValue = Math.abs(coef.value)
        summary += `${sign}${absValue.toFixed(4)} × ${varName}`
      })

      summary += `</p>`
      summary += `<p><strong>模型拟合度:</strong> R² = ${stats.rSquared.toFixed(4)} (调整R² = ${stats.adjustedRSquared.toFixed(4)})</p>`
      summary += `<p><strong>模型显著性:</strong> F(${indepVars.length}, ${anchors.value.length - indepVars.length - 1}) = ${stats.fStatistic.toFixed(4)}, p = ${stats.overallPValue.toExponential(4)}</p>`

      if (stats.overallPValue < 0.05) {
        summary += `<p><strong>结论:</strong> 模型整体显著，自变量组合对因变量有显著解释力。</p>`
      } else {
        summary += `<p><strong>结论:</strong> 模型整体不显著，自变量组合对因变量的解释力有限。</p>`
      }

      // 分析各个系数
      summary += `<p><strong>各变量影响分析:</strong></p><ul>`
      regressionResult.coefficients.forEach((coef, idx) => {
        if (idx === 0) return // 跳过截距

        const varName = formatVariableName(indepVars[idx - 1])
        const pValue = coef.pValue
        const direction = coef.value > 0 ? '正向' : '负向'

        summary += `<li>${varName}: 系数 = ${coef.value.toFixed(4)}, t = ${coef.tStat.toFixed(4)}, p = ${pValue.toExponential(4)}`
        if (pValue < 0.001) {
          summary += ` (<span style="color: red;">极显著</span>) - 对${formatVariableName(depVar)}有极显著的${direction}影响`
        } else if (pValue < 0.01) {
          summary += ` (<span style="color: orange;">高度显著</span>) - 对${formatVariableName(depVar)}有高度显著的${direction}影响`
        } else if (pValue < 0.05) {
          summary += ` (<span style="color: blue;">显著</span>) - 对${formatVariableName(depVar)}有显著的${direction}影响`
        } else {
          summary += ` (不显著) - 对${formatVariableName(depVar)}的影响不显著`
        }
        summary += `</li>`
      })
      summary += `</ul>`

      return summary
    }

    // 格式化变量名
    const formatVariableName = (varName) => {
      const names = {
        'attention': '关注数',
        'effective_days': '有效天',
        'live_duration': '开播时长',
        'guard_3': '总督',
        'guard_2': '提督',
        'guard_1': '舰长',
        'fans_count': '粉丝团',
        'gift': '礼物收入',
        'guard': '舰长收入',
        'super_chat': 'SC收入',
        'total_revenue': '总营收'
      }
      return names[varName] || varName
    }

    // 绘制散点图 + 拟合线（使用ECharts）
    const drawScatterChart = async (validAnchors, predictedValues) => {
      try {
        if (!scatterChart.value) {
          console.error('散点图画布不存在')
          return
        }

        const actualValues = validAnchors.map(anchor => parseFloat(anchor[dependentVariable.value]))

        // 检查数据有效性
        if (!actualValues || actualValues.length === 0 || !predictedValues || predictedValues.length === 0) {
          console.error('数据无效或为空')
          return
        }

        // 销毁现有图表实例
        if (scatterChartInstance) {
          scatterChartInstance.dispose()
        }

        // 准备散点数据
        const scatterData = actualValues.map((actual, idx) => [idx, actual])
        const lineData = predictedValues.map((predicted, idx) => [idx, predicted])

        // 初始化ECharts实例
        const chart = echarts.init(scatterChart.value)
        scatterChartInstance = chart

        const option = {
          title: {
            text: `散点图 + 拟合线: ${formatVariableName(dependentVariable.value)} vs 样本`,
            left: 'center',
            textStyle: {
              fontSize: 16
            }
          },
          tooltip: {
            trigger: 'axis',
            formatter: function(params) {
              let result = params[0].axisValue + '<br/>'
              params.forEach(param => {
                result += param.seriesName + ': ' + param.data[1].toFixed(2) + '<br/>'
              })
              return result
            }
          },
          legend: {
            data: [`实际${formatVariableName(dependentVariable.value)}`, `拟合${formatVariableName(dependentVariable.value)}`],
            top: '10%'
          },
          grid: {
            left: '10%',
            right: '10%',
            bottom: '15%',
            top: '20%'
          },
          xAxis: {
            type: 'value',
            name: '样本',
            splitLine: {
              lineStyle: {
                type: 'dashed'
              }
            }
          },
          yAxis: {
            type: 'value',
            name: formatVariableName(dependentVariable.value)
          },
          series: [
            {
              name: `实际${formatVariableName(dependentVariable.value)}`,
              type: 'scatter',
              data: scatterData,
              symbolSize: 8,
              itemStyle: {
                color: 'rgba(54, 162, 235, 0.6)'
              }
            },
            {
              name: `拟合${formatVariableName(dependentVariable.value)}`,
              type: 'line',
              data: lineData,
              itemStyle: {
                color: 'rgba(255, 99, 132, 1)'
              },
              lineStyle: {
                width: 2
              },
              smooth: true
            }
          ]
        }

        chart.setOption(option)

        // 监听窗口大小变化
        window.addEventListener('resize', () => {
          if (scatterChartInstance) {
            scatterChartInstance.resize()
          }
        })

      } catch (error) {
        console.error('绘制散点图失败:', error)
      }
    }

    // 绘制残差图（使用原生Canvas）
    const drawResidualChart = async (validAnchors, predictedValues) => {
      try {
        if (!residualChart.value) {
          console.error('残差图画布不存在')
          return
        }

        const canvas = residualChart.value
        const ctx = canvas.getContext('2d')

        // 设置画布尺寸
        canvas.width = canvas.offsetWidth
        canvas.height = canvas.offsetHeight

        // 清空画布
        ctx.clearRect(0, 0, canvas.width, canvas.height)

        const actualValues = validAnchors.map(anchor => parseFloat(anchor[dependentVariable.value]))

        // 检查数据有效性
        if (!actualValues || actualValues.length === 0 || !predictedValues || predictedValues.length === 0) {
          console.error('数据无效或为空')
          return
        }

        // 计算残差
        const residuals = actualValues.map((actual, idx) => actual - predictedValues[idx])

        // 计算数据范围
        const maxX = Math.max(residuals.length - 1, 1)
        const minY = Math.min(...residuals)
        const maxY = Math.max(...residuals)

        // 如果所有残差都相同，稍微扩展范围
        if (minY === maxY) {
          const offset = Math.abs(minY) > 0 ? Math.abs(minY) * 0.1 : 1
          minY -= offset
          maxY += offset
        }

        // 边距
        const margin = 50
        const chartWidth = canvas.width - 2 * margin
        const chartHeight = canvas.height - 2 * margin

        // 绘制坐标轴
        ctx.beginPath()
        ctx.moveTo(margin, canvas.height - margin)
        ctx.lineTo(canvas.width - margin, canvas.height - margin) // X轴
        ctx.lineTo(canvas.width - margin, margin) // Y轴
        ctx.strokeStyle = '#333'
        ctx.stroke()

        // 绘制标题
        ctx.fillStyle = '#333'
        ctx.font = '16px Arial'
        ctx.textAlign = 'center'
        ctx.fillText('残差图: 残差 vs 样本', canvas.width / 2, 30)

        // 绘制残差散点 (橙色)
        ctx.fillStyle = 'rgba(255, 159, 64, 0.6)'
        for (let i = 0; i < residuals.length; i++) {
          const x = margin + (i / maxX) * chartWidth
          const y = canvas.height - margin - ((residuals[i] - minY) / (maxY - minY)) * chartHeight

          // 绘制圆点
          ctx.beginPath()
          ctx.arc(x, y, 4, 0, Math.PI * 2)
          ctx.fill()
        }

        // 绘制y=0参考线
        const zeroY = canvas.height - margin - ((0 - minY) / (maxY - minY)) * chartHeight
        ctx.beginPath()
        ctx.strokeStyle = 'rgba(255, 99, 132, 1)' // 红色参考线
        ctx.lineWidth = 1
        ctx.setLineDash([5, 5]) // 虚线
        ctx.moveTo(margin, zeroY)
        ctx.lineTo(canvas.width - margin, zeroY)
        ctx.stroke()
        ctx.setLineDash([]) // 重置线型

        // 绘制图例
        ctx.fillStyle = 'rgba(255, 159, 64, 0.6)'
        ctx.fillRect(margin, 40, 15, 15)
        ctx.fillStyle = '#333'
        ctx.font = '12px Arial'
        ctx.textAlign = 'left'
        ctx.fillText('残差', margin + 20, 52)

        ctx.strokeStyle = 'rgba(255, 99, 132, 1)'
        ctx.lineWidth = 1
        ctx.setLineDash([5, 5])
        ctx.beginPath()
        ctx.moveTo(margin + 7, 70)
        ctx.lineTo(margin + 22, 70)
        ctx.stroke()
        ctx.setLineDash([])
        ctx.fillStyle = '#333'
        ctx.fillText('y=0参考线', margin + 25, 72)

      } catch (error) {
        console.error('绘制残差图失败:', error)
      }
    }

    // 绘制预测 vs 实测图
    const drawPredictionChart = async (validAnchors, predictedValues) => {
      try {
        if (!predictionChart.value) {
          console.error('预测vs实测图画布不存在')
          return
        }

        // 销毁现有图表实例
        if (predictionChartInstance) {
          predictionChartInstance.destroy()
        }

        const actualValues = validAnchors.map(anchor => parseFloat(anchor[dependentVariable.value]))

        // 检查数据有效性
        if (!actualValues || actualValues.length === 0 || !predictedValues || predictedValues.length === 0) {
          console.error('数据无效或为空')
          return
        }

        // 准备预测vs实测数据
        const predictionData = actualValues.map((actual, idx) => ({
          x: actual,
          y: predictedValues[idx]
        }))

        // 准备y=x参考线数据
        const minVal = Math.min(...actualValues, ...predictedValues)
        const maxVal = Math.max(...actualValues, ...predictedValues)

        const referenceLine = [
          { x: minVal, y: minVal },
          { x: maxVal, y: maxVal }
        ]

        const ctx = predictionChart.value.getContext('2d')
        predictionChartInstance = new Chart(ctx, {
          type: 'scatter',
          data: {
            datasets: [
              {
                label: '预测 vs 实测',
                data: predictionData,
                backgroundColor: 'rgba(75, 192, 192, 0.6)',
                borderColor: 'rgba(75, 192, 192, 1)',
                pointRadius: 5,
                showLine: false
              },
              {
                label: 'y=x 参考线',
                data: referenceLine,
                borderColor: 'rgba(255, 99, 132, 1)',
                borderWidth: 2,
                showLine: true,
                pointRadius: 0,
                fill: false,
                borderDash: [5, 5]
              }
            ]
          },
          options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
              title: {
                display: true,
                text: `预测 vs 实测: 预测值 vs 实测值`,
                font: {
                  size: 16
                }
              },
              legend: {
                position: 'top',
              }
            },
            scales: {
              x: {
                title: {
                  display: true,
                  text: `实测${formatVariableName(dependentVariable.value)}`
                }
              },
              y: {
                title: {
                  display: true,
                  text: `预测${formatVariableName(dependentVariable.value)}`
                }
              }
            }
          }
        })
      } catch (error) {
        console.error('绘制预测vs实测图失败:', error)
      }
    }

    // 绘制系数图（使用ECharts）
    const drawCoefficientsChart = async () => {
      try {
        if (!coefficientsChart.value) {
          console.error('系数图画布不存在')
          return
        }

        // 检查回归结果是否存在
        if (!regressionResults.value || !regressionResults.value.coefficients) {
          console.error('回归结果不存在')
          return
        }

        // 排除截距项，只显示自变量的系数
        const coefficientData = regressionResults.value.coefficients.filter(coef => coef.variable !== '截距')
        if (coefficientData.length === 0) {
          console.error('没有有效的系数数据')
          return
        }

        // 销毁现有图表实例
        if (coefficientsChartInstance) {
          coefficientsChartInstance.dispose()
        }

        // 准备系数数据
        const labels = coefficientData.map(coef => coef.variable)
        const values = coefficientData.map(coef => coef.value)

        // 初始化ECharts实例
        const chart = echarts.init(coefficientsChart.value)
        coefficientsChartInstance = chart

        const option = {
          title: {
            text: '回归系数图',
            left: 'center',
            textStyle: {
              fontSize: 16
            }
          },
          tooltip: {
            trigger: 'axis',
            axisPointer: {
              type: 'shadow'
            },
            formatter: function(params) {
              return params[0].name + '<br/>' +
                     params[0].seriesName + ': ' + params[0].value.toFixed(4)
            }
          },
          grid: {
            left: '10%',
            right: '10%',
            bottom: '20%',
            top: '15%'
          },
          xAxis: {
            type: 'category',
            data: labels,
            name: '变量',
            axisLabel: {
              interval: 0,
              rotate: 45
            }
          },
          yAxis: {
            type: 'value',
            name: '系数值',
            splitLine: {
              lineStyle: {
                type: 'dashed'
              }
            }
          },
          series: [
            {
              name: '回归系数',
              type: 'bar',
              data: values,
              itemStyle: {
                color: function(params) {
                  // 根据值的正负设置不同颜色
                  return params.value >= 0 ? 'rgba(153, 102, 255, 0.6)' : 'rgba(255, 159, 64, 0.6)'
                }
              },
              emphasis: {
                itemStyle: {
                  color: function(params) {
                    // 高亮时使用更鲜艳的颜色
                    return params.value >= 0 ? 'rgba(153, 102, 255, 1)' : 'rgba(255, 159, 64, 1)'
                  }
                }
              }
            }
          ]
        }

        chart.setOption(option)

        // 监听窗口大小变化
        window.addEventListener('resize', () => {
          if (coefficientsChartInstance) {
            coefficientsChartInstance.resize()
          }
        })

      } catch (error) {
        console.error('绘制系数图失败:', error)
      }
    }

    // 根据当前选择绘制相应图表
    const drawCurrentChart = async (validAnchors, predictedValues) => {
      // 由于现在使用原生Canvas，不需要销毁Chart.js实例
      // 直接调用相应的绘制函数即可

      switch (regressionCurrentChart.value) {
        case 'scatter':
          await drawScatterChart(validAnchors, predictedValues)
          break
        case 'residual':
          await drawResidualChart(validAnchors, predictedValues)
          break
        case 'prediction':
          await drawPredictionChart(validAnchors, predictedValues)
          break
        case 'coefficients':
          await drawCoefficientsChart()
          break
        default:
          await drawScatterChart(validAnchors, predictedValues)
      }
    }

    // 监听图表切换
    watch(regressionCurrentChart, async () => {
      if (regressionAnalysisVisible.value && regressionResults.value) {
        // 获取当前有效的锚点数据
        const validAnchors = anchors.value.filter(anchor => {
          const hasDependent = anchor[dependentVariable.value] !== undefined &&
                              anchor[dependentVariable.value] !== null &&
                              !isNaN(parseFloat(anchor[dependentVariable.value]))

          const hasIndependents = independentVariables.value.every(varName =>
            anchor[varName] !== undefined &&
            anchor[varName] !== null &&
            !isNaN(parseFloat(anchor[varName]))
          )

          return hasDependent && hasIndependents
        })

        // 重新绘制当前选择的图表
        if (regressionResults.value.predictedValues) {
          // 等待DOM更新完成后再绘制图表
          await nextTick()
          // 添加短暂延迟确保canvas元素已准备好
          setTimeout(async () => {
            await drawCurrentChart(validAnchors, regressionResults.value.predictedValues)
          }, 100)
        }
      }
    })

    // 设置图表引用的辅助函数
    const setChartRef = (el, chartType) => {
      if (el) {
        switch (chartType) {
          case 'scatter':
            scatterChart.value = el
            break
          case 'residual':
            residualChart.value = el
            break
          case 'prediction':
            predictionChart.value = el
            break
          case 'coefficients':
            coefficientsChart.value = el
            break
        }
      }
    }

    // 获取显著性类别
    const getSignificanceClass = (pValue) => {
      if (pValue < 0.001) return 'highly-significant'
      if (pValue < 0.01) return 'significant'
      if (pValue < 0.05) return 'moderately-significant'
      return 'not-significant'
    }

    // 打开聚类分析模态框
    const openClusterAnalysisModal = () => {
      showClusterModal.value = true
    }

    // 关闭聚类分析模态框
    const closeClusterModal = () => {
      showClusterModal.value = false
    }

    // 检查是否可以执行聚类分析
    const canPerformClusterAnalysis = computed(() => {
      return clusteringVariables.value.length >= 2 // 至少需要2个变量进行聚类
    })

    // 执行聚类分析
    const performClusterAnalysis = async () => {
      if (!canPerformClusterAnalysis.value) {
        alert('至少需要选择2个变量进行聚类分析')
        return
      }

      try {
        clusterLoading.value = true
        error.value = null

        // 准备数据
        const validAnchors = anchors.value.filter(anchor => {
          return clusteringVariables.value.every(varName =>
            anchor[varName] !== undefined &&
            anchor[varName] !== null &&
            !isNaN(parseFloat(anchor[varName]))
          )
        })

        if (validAnchors.length < parseInt(numClusters.value)) {
          alert(`数据点数量(${validAnchors.length})少于聚类数量(${numClusters.value})，无法进行聚类分析`)
          return
        }

        // 提取数据
        const data = validAnchors.map(anchor =>
          clusteringVariables.value.map(varName => parseFloat(anchor[varName]))
        )

        // 执行K-means聚类
        const clusters = performKMeansClustering(data, parseInt(numClusters.value))

        // 计算轮廓系数
        const silhouetteScore = calculateSilhouetteScore(data, clusters)

        // 生成聚类结果
        clusterResults.value = {
          numClusters: parseInt(numClusters.value),
          variables: clusteringVariables.value,
          dataPoints: validAnchors.length,
          clusters: clusters,
          silhouetteScore: silhouetteScore,
          summary: generateClusterSummary(validAnchors, clusters, clusteringVariables.value, parseInt(numClusters.value), silhouetteScore)
        }

        // 显示结果
        clusterAnalysisVisible.value = true
        showClusterModal.value = false

        // 绘制图表
        await nextTick()
        drawCluster2DChart(validAnchors, clusters)
      } catch (err) {
        console.error('聚类分析失败:', err)
        clusterError.value = `聚类分析失败: ${err.message || '未知错误'}`
      } finally {
        clusterLoading.value = false
      }
    }

    // K-means聚类算法实现
    const performKMeansClustering = (data, k) => {
      if (data.length < k) {
        throw new Error(`数据点数量(${data.length})少于聚类数量(${k})`)
      }

      // 初始化中心点
      const centroids = initializeCentroids(data, k)
      const assignments = new Array(data.length).fill(0)
      let changed = true
      let iterations = 0
      const maxIterations = 100

      while (changed && iterations < maxIterations) {
        changed = false
        iterations++

        // 分配每个点到最近的中心
        for (let i = 0; i < data.length; i++) {
          let minDist = Infinity
          let cluster = 0

          for (let j = 0; j < k; j++) {
            const dist = euclideanDistance(data[i], centroids[j])
            if (dist < minDist) {
              minDist = dist
              cluster = j
            }
          }

          if (assignments[i] !== cluster) {
            assignments[i] = cluster
            changed = true
          }
        }

        // 更新中心点
        for (let j = 0; j < k; j++) {
          const clusterPoints = data.filter((_, idx) => assignments[idx] === j)
          if (clusterPoints.length > 0) {
            centroids[j] = calculateCentroid(clusterPoints)
          }
        }
      }

      return assignments
    }

    // 计算欧几里得距离
    const euclideanDistance = (point1, point2) => {
      let sum = 0
      for (let i = 0; i < point1.length; i++) {
        sum += Math.pow(point1[i] - point2[i], 2)
      }
      return Math.sqrt(sum)
    }

    // 初始化中心点
    const initializeCentroids = (data, k) => {
      const centroids = []
      // 使用随机初始化
      for (let i = 0; i < k; i++) {
        const randomIndex = Math.floor(Math.random() * data.length)
        centroids.push([...data[randomIndex]])
      }
      return centroids
    }

    // 计算中心点
    const calculateCentroid = (points) => {
      const centroid = []
      const dimensions = points[0].length

      for (let dim = 0; dim < dimensions; dim++) {
        let sum = 0
        for (let i = 0; i < points.length; i++) {
          sum += points[i][dim]
        }
        centroid[dim] = sum / points.length
      }

      return centroid
    }

    // 计算轮廓系数
    const calculateSilhouetteScore = (data, clusters) => {
      const n = data.length
      if (n <= 1) return 0

      const clusterCount = Math.max(...clusters) + 1
      if (clusterCount <= 1) return 0

      let totalScore = 0

      for (let i = 0; i < n; i++) {
        const clusterI = clusters[i]

        // 计算a(i) - 点i到同簇其他点的平均距离
        let sumWithin = 0
        let countWithin = 0
        for (let j = 0; j < n; j++) {
          if (i !== j && clusters[j] === clusterI) {
            sumWithin += euclideanDistance(data[i], data[j])
            countWithin++
          }
        }
        const aI = countWithin > 0 ? sumWithin / countWithin : 0

        // 计算b(i) - 点i到其他簇的最小平均距离
        let bI = Infinity
        for (let c = 0; c < clusterCount; c++) {
          if (c !== clusterI) {
            let sumBetween = 0
            let countBetween = 0
            for (let j = 0; j < n; j++) {
              if (clusters[j] === c) {
                sumBetween += euclideanDistance(data[i], data[j])
                countBetween++
              }
            }
            if (countBetween > 0) {
              const avgDist = sumBetween / countBetween
              if (avgDist < bI) {
                bI = avgDist
              }
            }
          }
        }

        if (bI === Infinity) bI = 0 // 如果没有其他簇，则bI设为0

        // 计算轮廓系数
        const sI = (bI - aI) / Math.max(aI, bI)
        totalScore += sI
      }

      return totalScore / n
    }

    // 生成聚类摘要
    const generateClusterSummary = (validAnchors, clusters, variables, numClusters, silhouetteScore) => {
      let summary = `<p><strong>聚类分析结果:</strong></p>`
      summary += `<p>使用 ${variables.join(', ')} 变量进行 ${numClusters} 类聚类分析</p>`
      summary += `<p>轮廓系数: ${silhouetteScore.toFixed(4)} (值越接近1表示聚类效果越好)</p>`

      // 统计每个聚类的数量
      const clusterCounts = new Array(numClusters).fill(0)
      clusters.forEach(cluster => {
        clusterCounts[cluster]++
      })

      summary += `<p><strong>各聚类数据点数量:</strong></p><ul>`
      for (let i = 0; i < numClusters; i++) {
        summary += `<li>聚类 ${i + 1}: ${clusterCounts[i]} 个数据点</li>`
      }
      summary += `</ul>`

      // 简要分析
      if (silhouetteScore > 0.5) {
        summary += `<p><strong>评价:</strong> <span style="color: green;">聚类效果良好</span> - 轮廓系数较高，表明聚类间区分明显</p>`
      } else if (silhouetteScore > 0.3) {
        summary += `<p><strong>评价:</strong> <span style="color: orange;">聚类效果一般</span> - 轮廓系数中等，聚类间有一定重叠</p>`
      } else {
        summary += `<p><strong>评价:</strong> <span style="color: red;">聚类效果较差</span> - 轮廓系数较低，可能需要调整聚类数量或变量</p>`
      }

      return summary
    }

    // 绘制2D聚类图
    const drawCluster2DChart = async (validAnchors, clusters) => {
      if (!cluster2dChart.value) {
        console.error('2D聚类图画布不存在')
        return
      }

      // 销毁现有图表实例
      if (cluster2dChartInstance) {
        cluster2dChartInstance.destroy()
      }

      const ctx = cluster2dChart.value.getContext('2d')
      const canvas = cluster2dChart.value

      // 设置画布尺寸
      canvas.width = canvas.offsetWidth
      canvas.height = canvas.offsetHeight

      // 清空画布
      ctx.clearRect(0, 0, canvas.width, canvas.height)

      // 如果变量少于2个，无法绘制2D图
      if (clusteringVariables.value.length < 2) {
        ctx.fillStyle = '#333'
        ctx.font = '16px Arial'
        ctx.textAlign = 'center'
        ctx.fillText('需要至少2个变量才能绘制2D散点图', canvas.width / 2, canvas.height / 2)
        return
      }

      // 使用前两个变量作为X和Y轴
      const xVar = clusteringVariables.value[0]
      const yVar = clusteringVariables.value[1]

      // 提取X和Y数据
      const xData = validAnchors.map(anchor => parseFloat(anchor[xVar]))
      const yData = validAnchors.map(anchor => parseFloat(anchor[yVar]))

      // 计算数据范围
      const minX = Math.min(...xData)
      const maxX = Math.max(...xData)
      const minY = Math.min(...yData)
      const maxY = Math.max(...yData)

      // 边距
      const margin = 50
      const chartWidth = canvas.width - 2 * margin
      const chartHeight = canvas.height - 2 * margin

      // 绘制坐标轴
      ctx.beginPath()
      ctx.moveTo(margin, canvas.height - margin)
      ctx.lineTo(canvas.width - margin, canvas.height - margin) // X轴
      ctx.lineTo(canvas.width - margin, margin) // Y轴
      ctx.strokeStyle = '#333'
      ctx.stroke()

      // 绘制标题
      ctx.fillStyle = '#333'
      ctx.font = '16px Arial'
      ctx.textAlign = 'center'
      ctx.fillText(`聚类分析: ${formatVariableName(xVar)} vs ${formatVariableName(yVar)}`, canvas.width / 2, 30)

      // 定义聚类颜色
      const clusterColors = [
        'rgba(255, 99, 132, 0.6)', // 红色
        'rgba(54, 162, 235, 0.6)', // 蓝色
        'rgba(75, 192, 192, 0.6)', // 绿色
        'rgba(255, 206, 86, 0.6)', // 黄色
        'rgba(153, 102, 255, 0.6)'  // 紫色
      ]

      // 绘制数据点
      for (let i = 0; i < validAnchors.length; i++) {
        const x = margin + ((xData[i] - minX) / (maxX - minX)) * chartWidth
        const y = canvas.height - margin - ((yData[i] - minY) / (maxY - minY)) * chartHeight
        const cluster = clusters[i]

        // 设置点的颜色
        ctx.fillStyle = clusterColors[cluster % clusterColors.length]
        ctx.strokeStyle = clusterColors[cluster % clusterColors.length].replace('0.6', '1')
        ctx.lineWidth = 1

        // 绘制圆形点
        ctx.beginPath()
        ctx.arc(x, y, 6, 0, Math.PI * 2)
        ctx.fill()
        ctx.stroke()
      }

      // 绘制图例
      for (let i = 0; i < parseInt(numClusters.value); i++) {
        const x = margin + 10
        const y = margin + 20 + i * 25

        // 绘制颜色标记
        ctx.fillStyle = clusterColors[i % clusterColors.length]
        ctx.strokeStyle = clusterColors[i % clusterColors.length].replace('0.6', '1')
        ctx.lineWidth = 1
        ctx.beginPath()
        ctx.arc(x, y, 6, 0, Math.PI * 2)
        ctx.fill()
        ctx.stroke()

        // 绘制标签
        ctx.fillStyle = '#333'
        ctx.font = '12px Arial'
        ctx.textAlign = 'left'
        ctx.fillText(`聚类 ${i + 1}`, x + 10, y + 4)
      }

      // 添加轴标签
      ctx.fillStyle = '#333'
      ctx.font = '12px Arial'
      ctx.textAlign = 'center'
      ctx.fillText(formatVariableName(xVar), canvas.width / 2, canvas.height - 10)

      ctx.save()
      ctx.translate(15, canvas.height / 2)
      ctx.rotate(-Math.PI / 2)
      ctx.textAlign = 'center'
      ctx.fillText(formatVariableName(yVar), 0, 0)
      ctx.restore()
    }

    // 绘制3D聚类图（使用2D投影方式模拟）
    const drawCluster3DChart = async (validAnchors, clusters) => {
      if (!cluster3dChart.value) {
        console.error('3D聚类图画布不存在')
        return
      }

      // 销毁现有图表实例
      if (cluster3dChartInstance) {
        cluster3dChartInstance.destroy()
      }

      const ctx = cluster3dChart.value.getContext('2d')
      const canvas = cluster3dChart.value

      // 设置画布尺寸
      canvas.width = canvas.offsetWidth
      canvas.height = canvas.offsetHeight

      // 清空画布
      ctx.clearRect(0, 0, canvas.width, canvas.height)

      // 如果变量少于3个，无法绘制3D图
      if (clusteringVariables.value.length < 3) {
        ctx.fillStyle = '#333'
        ctx.font = '16px Arial'
        ctx.textAlign = 'center'
        ctx.fillText('需要至少3个变量才能绘制3D散点图', canvas.width / 2, canvas.height / 2)
        return
      }

      // 使用前三个变量作为X、Y、Z轴
      const xVar = clusteringVariables.value[0]
      const yVar = clusteringVariables.value[1]
      const zVar = clusteringVariables.value[2]

      // 提取X、Y、Z数据
      const xData = validAnchors.map(anchor => parseFloat(anchor[xVar]))
      const yData = validAnchors.map(anchor => parseFloat(anchor[yVar]))
      const zData = validAnchors.map(anchor => parseFloat(anchor[zVar]))

      // 计算数据范围
      const minX = Math.min(...xData)
      const maxX = Math.max(...xData)
      const minY = Math.min(...yData)
      const maxY = Math.max(...yData)
      const minZ = Math.min(...zData)
      const maxZ = Math.max(...zData)

      // 边距
      const margin = 50
      const chartWidth = canvas.width - 2 * margin
      const chartHeight = canvas.height - 2 * margin

      // 绘制坐标轴
      ctx.beginPath()
      ctx.moveTo(margin, canvas.height - margin)
      ctx.lineTo(canvas.width - margin, canvas.height - margin) // X轴
      ctx.lineTo(canvas.width - margin, margin) // Y轴
      ctx.strokeStyle = '#333'
      ctx.stroke()

      // 绘制标题
      ctx.fillStyle = '#333'
      ctx.font = '16px Arial'
      ctx.textAlign = 'center'
      ctx.fillText(`3D聚类分析: ${formatVariableName(xVar)} vs ${formatVariableName(yVar)} vs ${formatVariableName(zVar)}`, canvas.width / 2, 30)

      // 定义聚类颜色
      const clusterColors = [
        'rgba(255, 99, 132, 0.6)', // 红色
        'rgba(54, 162, 235, 0.6)', // 蓝色
        'rgba(75, 192, 192, 0.6)', // 绿色
        'rgba(255, 206, 86, 0.6)', // 黄色
        'rgba(153, 102, 255, 0.6)'  // 紫色
      ]

      // 绘制数据点，使用Z值影响点的大小来模拟深度
      for (let i = 0; i < validAnchors.length; i++) {
        const x = margin + ((xData[i] - minX) / (maxX - minX)) * chartWidth
        const y = canvas.height - margin - ((yData[i] - minY) / (maxY - minY)) * chartHeight
        // 使用Z值来影响点的大小，模拟深度效果
        const normalizedZ = (zData[i] - minZ) / (maxZ - minZ)
        const pointSize = 4 + normalizedZ * 6 // 点大小在4-10之间变化
        const cluster = clusters[i]

        // 设置点的颜色
        ctx.fillStyle = clusterColors[cluster % clusterColors.length]
        ctx.strokeStyle = clusterColors[cluster % clusterColors.length].replace('0.6', '1')
        ctx.lineWidth = 1

        // 绘制圆形点，大小根据Z值变化
        ctx.beginPath()
        ctx.arc(x, y, pointSize, 0, Math.PI * 2)
        ctx.fill()
        ctx.stroke()
      }

      // 绘制图例
      for (let i = 0; i < parseInt(numClusters.value); i++) {
        const x = margin + 10
        const y = margin + 20 + i * 25

        // 绘制颜色标记
        ctx.fillStyle = clusterColors[i % clusterColors.length]
        ctx.strokeStyle = clusterColors[i % clusterColors.length].replace('0.6', '1')
        ctx.lineWidth = 1
        ctx.beginPath()
        ctx.arc(x, y, 6, 0, Math.PI * 2)
        ctx.fill()
        ctx.stroke()

        // 绘制标签
        ctx.fillStyle = '#333'
        ctx.font = '12px Arial'
        ctx.textAlign = 'left'
        ctx.fillText(`聚类 ${i + 1}`, x + 10, y + 4)
      }

      // 添加轴标签
      ctx.fillStyle = '#333'
      ctx.font = '12px Arial'
      ctx.textAlign = 'center'
      ctx.fillText(formatVariableName(xVar), canvas.width / 2, canvas.height - 10)

      ctx.save()
      ctx.translate(15, canvas.height / 2)
      ctx.rotate(-Math.PI / 2)
      ctx.textAlign = 'center'
      ctx.fillText(formatVariableName(yVar), 0, 0)
      ctx.restore()
    }

    // 关闭聚类分析
    const closeClusterAnalysis = () => {
      clusterAnalysisVisible.value = false
      if (cluster2dChartInstance) {
        cluster2dChartInstance.destroy()
        cluster2dChartInstance = null
      }
      if (cluster3dChartInstance) {
        cluster3dChartInstance.destroy()
        cluster3dChartInstance = null
      }
    }

    // 获取显著性标签
    const getSignificanceLabel = (pValue) => {
      if (pValue < 0.001) return '极显著 ***'
      if (pValue < 0.01) return '高度显著 **'
      if (pValue < 0.05) return '显著 *'
      return '不显著'
    }

    const performMultiMonthQuery = async () => {
      if (!startMonth.value || !endMonth.value) {
        alert('请选择起始和结束月份')
        return
      }

      // 验证月份格式
      const start = new Date(startMonth.value)
      const end = new Date(endMonth.value)

      if (start > end) {
        alert('起始月份不能晚于结束月份')
        return
      }

      try {
        loading.value = true
        error.value = null

        // 将月份格式转换为 YYYYMM 格式
        const startMonthFormatted = startMonth.value.replace('-', '')
        const endMonthFormatted = endMonth.value.replace('-', '')

        // 获取起始和结束月份之间的所有月份
        const months = []
        const startDate = new Date(startMonth.value)
        const endDate = new Date(endMonth.value)

        // 设置为月初，避免日期问题
        startDate.setDate(1)
        endDate.setDate(1)

        const current = new Date(startDate)
        while (current <= endDate) {
          const year = current.getFullYear()
          const month = String(current.getMonth() + 1).padStart(2, '0')
          months.push(`${year}${month}`)
          current.setMonth(current.getMonth() + 1)
        }

        // 获取所有月份的数据并合并
        let combinedAnchors = {}
        for (const month of months) {
          try {
            const response = await anchorAPI.getAnchorsByMonth(month, currentFilter.value)
            const anchorsForMonth = response.anchors || response.data || []

            anchorsForMonth.forEach(anchor => {
              const key = anchor.room_id || anchor.anchor_name
              if (!combinedAnchors[key]) {
                // 初始化新主播的数据
                combinedAnchors[key] = { ...anchor }
                // 将数值字段初始化为数字
                combinedAnchors[key].attention = parseFloat(anchor.attention) || 0
                combinedAnchors[key].effective_days = parseInt(anchor.effective_days) || 0
                combinedAnchors[key].guard_1 = parseInt(anchor.guard_1) || 0
                combinedAnchors[key].guard_2 = parseInt(anchor.guard_2) || 0
                combinedAnchors[key].guard_3 = parseInt(anchor.guard_3) || 0
                combinedAnchors[key].fans_count = parseInt(anchor.fans_count) || 0
                combinedAnchors[key].gift = parseFloat(anchor.gift) || 0
                combinedAnchors[key].guard = parseFloat(anchor.guard) || 0
                combinedAnchors[key].super_chat = parseFloat(anchor.super_chat) || 0
                combinedAnchors[key].total_revenue = parseFloat(anchor.total_revenue) || 0
              } else {
                // 累加数据
                combinedAnchors[key].attention += parseFloat(anchor.attention) || 0
                combinedAnchors[key].effective_days += parseInt(anchor.effective_days) || 0
                combinedAnchors[key].guard_1 += parseInt(anchor.guard_1) || 0
                combinedAnchors[key].guard_2 += parseInt(anchor.guard_2) || 0
                combinedAnchors[key].guard_3 += parseInt(anchor.guard_3) || 0
                combinedAnchors[key].fans_count += parseInt(anchor.fans_count) || 0
                combinedAnchors[key].gift += parseFloat(anchor.gift) || 0
                combinedAnchors[key].guard += parseFloat(anchor.guard) || 0
                combinedAnchors[key].super_chat += parseFloat(anchor.super_chat) || 0
                combinedAnchors[key].total_revenue += parseFloat(anchor.total_revenue) || 0
              }
            })
          } catch (err) {
            console.error(`获取${month}月份数据失败:`, err)
            // 继续处理下一个月份
          }
        }

        // 转换为数组并更新显示
        let combinedAnchorsArray = Object.values(combinedAnchors)

        // 按总营收降序排序
        combinedAnchorsArray.sort((a, b) => {
          const totalRevenueA = parseFloat(a.total_revenue) || 0
          const totalRevenueB = parseFloat(b.total_revenue) || 0
          return totalRevenueB - totalRevenueA
        })

        // 重新分配排名
        combinedAnchorsArray.forEach((anchor, index) => {
          anchor.rank = index + 1
        })

        anchors.value = combinedAnchorsArray

        // 更新标题
        const startYear = startMonth.value.substring(0, 4)
        const startMon = startMonth.value.substring(5, 7)
        const endYear = endMonth.value.substring(0, 4)
        const endMon = endMonth.value.substring(5, 7)
        title.value = `维阿PSP斗虫榜_${startYear}年${startMon}月-${endYear}年${endMon}月累计数据`

        refreshTime.value = new Date().toLocaleString()
      } catch (err) {
        console.error('多月份统计查询失败:', err)
        error.value = '多月份统计查询失败，请稍后重试'
      } finally {
        loading.value = false
        closeMultiMonthModal()
      }
    }

    // 监听路由变化
    watch(
      () => route.query,
      (newQuery) => {
        // 页面变化时关闭所有图表
        hideAllCharts()

        currentFilter.value = newQuery.filter || 'all'
        const newMonth = newQuery.month || null
        if (newMonth) {
          const year = newMonth.substring(0, 4)
          const monthNum = parseInt(newMonth.substring(4, 6)).toString().padStart(2, '0')
          title.value = `维阿PSP斗虫榜_${year}年${monthNum}月记录数据（点击"正在直播"跳转到对应直播间）`
        } else {
          title.value = currentFilter.value === 'vr' ? '维阿斗虫榜' :
                       currentFilter.value === 'psp' ? 'PSPlive斗虫榜' : '维阿PSP斗虫榜'
        }
        fetchData()
      },
      { immediate: true }
    )

    onMounted(() => {
      fetchData()
    })

    return {
      chartVisible,
      anchors,
      title,
      refreshTime,
      currentFilter,
      loading,
      error,
      chartCanvas,
      viewLiveSessions,
      switchFilter,
      openMonthSelector,
      showRevenueChart,
      showVRPSPComparison,
      hideAllCharts,
      calculateTotalRevenue,
      formatCurrency,
      formatNumber,
      formatLiveDuration,
      formatDurationWithBreak,
      // 多月份统计相关
      showMultiMonthModal,
      startMonth,
      endMonth,
      openMultiMonthModal,
      closeMultiMonthModal,
      performMultiMonthQuery,
      // 月份选择器相关
      showMonthSelector,
      monthSelection,
      closeMonthSelector,
      performMonthSwitch,
      // 回归分析相关
      showRegressionModal,
      dependentVariable,
      independentVariables,
      regressionAnalysisVisible,
      regressionResults,
      regressionAnalysisChart,
      regressionLoading,
      regressionErrorMessage,
      regressionCurrentChart,
      scatterChart,
      residualChart,
      predictionChart,
      coefficientsChart,
      openRegressionAnalysisModal,
      closeRegressionModal,
      closeRegressionAnalysis,
      canPerformAnalysis,
      performRegressionAnalysis,
      getSignificanceClass,
      getSignificanceLabel,
      // 聚类分析相关
      showClusterModal,
      clusteringVariables,
      numClusters,
      clusterAnalysisVisible,
      clusterResults,
      clusterLoading,
      clusterError,
      cluster2dChart,
      cluster3dChart,
      currentClusterChart,
      openClusterAnalysisModal,
      closeClusterModal,
      canPerformClusterAnalysis,
      performClusterAnalysis,
      closeClusterAnalysis,
      drawCluster2DChart,
      drawCluster3DChart,
      globalCardState
    }
  }
}
</script>

<style scoped>
.anchor-list {
  background: #FFF8E1;
  border-radius: 30px; /* 超椭圆曲线 */
  padding: 20px;
  margin: 20px 0;
  border: 1px solid #FFC633;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.controls-section {
  margin-bottom: 20px;
}

.filter-controls {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
  flex-wrap: wrap;
  justify-content: center;
}

.filter-btn {
  padding: 10px 20px;
  border: 2px solid #FFC633;
  border-radius: 25px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  background: linear-gradient(45deg, #FFC633, #FFA500); /* 添加渐变背景 */
  color: #333;
  font-weight: bold;
}

.filter-btn.active {
  background: linear-gradient(45deg, #f9729a, #f75982);
  border-color: #f9729a;
  color: white;
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.filter-btn:hover:not(.active) {
  background: linear-gradient(45deg, #FFDB58, #FFC633); /* 悬停时的渐变 */
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.chart-button-container {
  display: flex;
  gap: 20px; /* 增加按钮间距 */
  justify-content: center;
  flex-wrap: wrap;
  margin: 30px 0 10px 0; /* 增加上下间距，让按钮离下面更远，但离表格有一定距离 */
}

.action-controls {
  display: flex;
  gap: 20px; /* 增加按钮间距 */
  justify-content: center;
  flex-wrap: wrap;
  margin: 30px 0; /* 增加上下间距，让按钮离下面更远 */
}

.action-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 120px; /* 最小宽度确保圆形效果 */
}

.action-btn.primary {
  background: linear-gradient(45deg, #FFC633, #FFA500);
  color: #333;
  font-weight: bold;
}

.action-btn.secondary {
  background: linear-gradient(45deg, #f9729a, #f75982);
  color: white;
}

.action-btn.danger {
  background: linear-gradient(45deg, #dc3545, #c82333);
  color: white;
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.info-section {
  text-align: center;
  margin-bottom: 20px;
}

.page-title {
  color: #FFC633;
  font-size: 1.5rem;
  margin-bottom: 5px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
}

.refresh-time {
  color: #f9729a;
  font-size: 0.9rem;
}

.chart-info {
  background: #FEEFEF;
  border: 2px solid #f9729a;
  border-radius: 30px; /* 超椭圆曲线 */
  padding: 15px;
  margin: 20px 0;
  text-align: center;
}

.chart-container {
  display: none;
  text-align: center;
  margin: 20px 0;
  height: 500px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 30px; /* 超椭圆曲线 */
  padding: 20px;
}

.chart-container.visible {
  display: block;
}

#chartCanvas {
  width: 100% !important;
  height: 100% !important;
}

.loading-state, .error-state {
  text-align: center;
  padding: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(255, 198, 51, 0.3);
  border-top: 4px solid #FFC633;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 15px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-message {
  color: #ff6b6b;
  font-size: 1.1rem;
  margin-bottom: 15px;
}

.retry-btn {
  padding: 10px 20px;
  background: linear-gradient(45deg, #33CC99, #28a745);
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

.retry-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(40, 167, 69, 0.3);
}


.number-cell {
  text-align: right;
  font-family: 'Courier New', monospace;
  color: #fff;
}

.total-revenue {
  color: #f9729a !important; /* 高亮重要数值 */
  font-weight: bold;
  font-size: 1.1em; /* 稍微增大重要数值的字号 */
}

.duration-cell {
  color: #f9729a;
}

.duration-value {
  text-align: right; /* 保持右对齐 */
  display: block; /* 确保为块级元素 */
  line-height: 1.4; /* 增加行高以改善垂直间距 */
  word-break: break-word; /* 确保内容可以正确换行 */
}

.status-cell {
  text-align: center;
}

.status-field {
  display: flex;
  justify-content: center;
  align-items: center;
}

.status-badge {
  display: inline-block;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: bold;
}

.status-badge.live {
  background: #f9729a; /* 实心洋红色背景，与查看详细数据按钮相同 */
  color: white; /* 白色文字，与查看详细数据按钮相同 */
  border: 2px solid #f9729a; /* 洋红色边框，与查看详细数据按钮相同 */
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  padding: 4px 8px; /* 内边距，与查看详细数据按钮相似 */
  cursor: pointer; /* 鼠标指针 */
  font-size: 0.8rem; /* 字体大小，与查看详细数据按钮相同 */
  transition: all 0.3s ease; /* 过渡效果 */
  font-weight: bold; /* 加粗 */
  text-decoration: none; /* 去除下划线 */
  display: inline-block; /* 行内块显示 */
  min-width: 80px; /* 最小宽度确保圆形效果 */
}

.status-badge.offline {
  background: rgba(255, 255, 255, 0.2);
  color: #ccc;
}

.live-link {
  color: white;
  text-decoration: none;
  font-weight: bold;
  display: inline-block;
}

.live-link:hover {
  text-decoration: underline;
}

.view-btn {
  padding: 6px 12px;
  background: #f9729a; /* 实心洋红色背景 */
  color: white; /* 白色文字 */
  border: 2px solid #f9729a; /* 洋红色边框 */
  border-radius: 30px; /* 更圆润的超椭圆形状 */
  cursor: pointer;
  font-size: 0.8rem;
  transition: all 0.3s ease;
  font-weight: bold;
  min-width: 120px; /* 最小宽度确保圆形效果 */
}

.view-btn:hover {
  background: #e0658a; /* 悬停时更深的洋红色 */
  color: white; /* 悬停时文字保持白色 */
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(249, 114, 154, 0.3);
}


/* 移动端网格容器 */
.grid-container {
  display: none; /* 默认隐藏网格布局 */
}

/* 网格布局样式 */
.anchor-grid-item {
  background: linear-gradient(135deg, #FFF8E1, #FFF5C2); /* 添加轻微渐变背景 */
  border: 1px solid #FFC633;
  border-radius: 20px; /* 增加圆角 */
  padding: 15px; /* 增加内边距 */
  margin-bottom: 15px; /* 增加外边距 */
  box-shadow: 0 6px 16px rgba(255, 198, 51, 0.2); /* 添加更柔和的阴影 */
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1); /* 使用更平滑的缓动函数 */
  position: relative; /* 为高级动效添加相对定位 */
  overflow: hidden; /* 确保内容不会溢出 */
  will-change: transform; /* 优化性能 */
  transform: translateZ(0); /* 启用硬件加速 */
}

/* 卡片悬停动效 */
.anchor-grid-item:hover {
  transform: translateY(-8px) scale(1.02); /* 上浮并轻微放大 */
  box-shadow: 0 12px 30px rgba(255, 198, 51, 0.4); /* 增强阴影 */
  border-color: #FFA500; /* 边框颜色变化 */
}

/* 光泽扫过效果 */
.anchor-grid-item::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -60%;
  width: 20px;
  height: 200%;
  background: linear-gradient(
    to right,
    rgba(255, 255, 255, 0) 0%,
    rgba(255, 255, 255, 0.4) 50%,
    rgba(255, 255, 255, 0) 100%
  );
  transform: rotate(30deg);
  transition: all 0.6s ease;
  z-index: 1;
}

.anchor-grid-item:hover::before {
  left: 120%;
  transition: all 0.8s ease;
}

.anchor-grid-item.live-grid-item {
  border: 2px solid #f9729a; /* 直播状态特殊边框 */
  background: #FFF8E1; /* 浅黄色背景 */
}

.grid-header {
  background: linear-gradient(135deg, #FFC633, #FFA500); /* 深色背景 */
  color: white; /* 白色文字 */
  padding: 12px; /* 增加内边距 */
  border-radius: 12px; /* 增加圆角 */
  margin-bottom: 12px; /* 增加间距 */
  display: flex; /* 使用flex布局 */
  align-items: center; /* 垂直居中 */
  justify-content: space-between; /* 两端对齐 */
  box-shadow: 0 4px 12px rgba(255, 198, 51, 0.3); /* 添加阴影 */
}

.grid-rank {
  font-weight: bold;
  font-size: 1.1em; /* 正常大小 */
}

.grid-name {
  font-weight: bold;
  margin: 5px 0;
  font-size: 1.1em; /* 正常大小 */
}

.grid-union {
  font-size: 1.1em; /* 正常大小 */
}

.grid-fields {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); /* 自适应网格 */
  gap: 8px;
  margin-bottom: 10px;
}

.field-box {
  background: rgba(255, 248, 225, 0.7); /* 淡黄色半透明背景 */
  border: 1px solid #FFC633;
  border-radius: 10px; /* 增加圆角 */
  padding: 12px; /* 增加内边距 */
  min-width: 120px;
  display: flex; /* 使用flex布局 */
  flex-direction: column; /* 改为垂直布局 */
  align-items: flex-start; /* 左对齐内容 */
  text-align: left; /* 左对齐文本 */
  margin-bottom: 6px; /* 添加底部间距 */
  transition: all 0.2s ease; /* 添加过渡效果 */
}

.field-box:hover {
  background: rgba(255, 240, 180, 0.8); /* 悬停时更亮的背景 */
  transform: translateY(-1px); /* 悬停时轻微上移 */
  box-shadow: 0 3px 8px rgba(255, 198, 51, 0.3); /* 悬停时添加阴影 */
}

.field-label {
  font-weight: bold;
  color: #FF8C00; /* 使用更醒目的颜色 */
  font-size: 1.1em; /* 正常大小 */
  word-break: break-word;
  margin-right: 10px; /* 增加与值之间的间距 */
  flex-shrink: 0; /* 防止标签被压缩 */
  transition: all 0.3s ease; /* 添加颜色过渡效果 */
  background-color: rgba(255, 198, 51, 0.15); /* 添加轻微背景色 */
  padding: 4px 8px; /* 添加内边距 */
  border-radius: 8px; /* 添加圆角 */
}

.field-label:hover {
  color: #FF6600; /* 悬停时更深的颜色 */
  background-color: rgba(255, 165, 0, 0.25); /* 悬停时更深的背景色 */
}

.field-value {
  color: #333;
  font-size: 1.1em; /* 正常大小 */
  word-break: break-word;
  text-align: right; /* 值右对齐 */
  margin-left: 10px; /* 增加与标签之间的间距 */
  overflow: hidden; /* 防止溢出 */
  text-overflow: ellipsis; /* 溢出时显示省略号 */
  transition: all 0.3s ease; /* 添加颜色过渡效果 */
}

.field-value:hover {
  color: #f9729a; /* 悬停时使用主题色 */
}

.total-revenue {
  color: #f9729a !important;
  font-weight: bold;
}

.grid-footer {
  text-align: center;
  margin-top: 8px;
}

/* 旧的卡片布局样式（保留用于可能的回退） */
.anchor-card {
  background: linear-gradient(135deg, #FFF8E1, #FFF5C2); /* 添加轻微渐变背景 */
  border: 1px solid #FFC633;
  border-radius: 20px;
  padding: 12px; /* 压缩内边距 */
  margin-bottom: 12px; /* 压缩外边距 */
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.1); /* 减少阴影 */
  transition: all 0.3s ease;
}

.anchor-card.live-card {
  border: 2px solid #f9729a; /* 直播状态特殊边框 */
  background: #FFF8E1; /* 浅黄色背景 */
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(45deg, #FFC633, #FFA500); /* 深色背景 */
  color: white; /* 白色文字 */
  padding: 10px;
  border-radius: 10px;
  margin-bottom: 8px; /* 压缩间距 */
}

.card-rank {
  font-weight: bold;
  color: #FF8C00; /* 更醒目的颜色 */
  font-size: 1.3em; /* 增大字号 */
}

.card-name {
  font-weight: bold;
  color: #333;
  flex-grow: 1;
  text-align: center;
  font-size: 1.2em; /* 增大字号 */
}

.card-union {
  color: #f9729a;
  font-weight: 600; /* 加粗 */
  font-size: 1.1em; /* 增大字号 */
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 6px; /* 压缩间距 */
}

.card-row {
  display: flex;
  justify-content: space-between;
  padding: 4px 0; /* 优化间距 */
  border-bottom: 1px solid #f0f0f0; /* 使用实线分隔 */
}

.field-label {
  font-weight: bold;
  color: #FF8C00; /* 使用更醒目的颜色 */
  min-width: 100px; /* 增加最小宽度 */
  font-size: 1.1em; /* 增大字号 */
  margin-right: 10px; /* 增加标签右侧间距 */
  background-color: rgba(255, 198, 51, 0.15); /* 添加轻微背景色 */
  padding: 4px 8px; /* 添加内边距 */
  border-radius: 8px; /* 添加圆角 */
  flex-shrink: 0; /* 防止标签被压缩 */
}

.field-value {
  text-align: right;
  color: #333;
  flex-grow: 1;
  font-size: 1.1em; /* 增大字号 */
  word-break: break-word; /* 允许长内容换行 */
  font-weight: 500; /* 稍微加粗 */
}

.total-revenue {
  color: #f9729a !important; /* 高亮重要数值 */
  font-weight: bold;
}

.card-footer {
  margin-top: 10px; /* 压缩间距 */
  text-align: center;
}

/* 响应式设计 */
@media (max-width: 1300px) {
  .anchor-table th,
  .anchor-table td {
    padding: 8px 5px;
    font-size: 0.8rem;
  }

  .table-container {
    overflow-x: auto;
  }
}

@media (max-width: 1200px) {
  .anchor-table th,
  .anchor-table td {
    padding: 7px 4px;
    font-size: 0.75rem;
  }
}

@media (max-width: 1024px) {
  .anchor-list {
    padding: 15px 10px;
    margin: 10px 5px;
  }

  .page-title {
    font-size: 1.4rem;
    text-align: center;
  }

  .anchor-table {
    font-size: 0.75rem;
    min-width: auto; /* 移除固定最小宽度，让表格适应屏幕 */
    width: 100%; /* 让表格占满容器宽度 */
  }

  .anchor-table th,
  .anchor-table td {
    padding: 6px 3px;
  }

  .table-container {
    overflow-x: auto;
  }
}

@media (max-width: 600px) {
  .anchor-list {
    padding: 15px 8px;
    margin: 8px 0;
  }

  .filter-controls,
  .action-controls {
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .filter-btn,
  .action-btn {
    width: 100%;
    max-width: 280px;
    margin: 5px 0;
    padding: 10px 15px;
  }

  .page-title {
    font-size: 1.3rem;
    text-align: center;
  }

  /* 移动端使用网格布局 */
  .grid-container {
    display: block; /* 显示网格布局 */
    width: 100%;
  }

  .anchor-grid-item {
    margin-bottom: 15px; /* 调整间距 */
    padding: 10px; /* 调整内边距 */
  }

  .grid-fields {
    gap: 6px; /* 调整间距 */
  }

  .field-box {
    min-width: 110px; /* 调整最小宽度 */
    padding: 6px; /* 调整内边距 */
    flex-direction: column; /* 移动端改为垂直布局 */
    text-align: center; /* 文字居中 */
  }

  .field-label {
    font-size: 0.85em; /* 调整字体大小 */
    margin-bottom: 2px;
    margin-right: 0; /* 移动端移除右边距 */
    text-align: center; /* 文字居中 */
  }

  .field-value {
    font-size: 0.95em; /* 调整字体大小 */
    margin-left: 0; /* 移动端移除左边距 */
    text-align: center; /* 文字居中 */
  }

  .grid-header {
    padding: 6px; /* 调整内边距 */
  }

  .grid-rank {
    font-size: 1em; /* 调整字体大小 */
  }

  .grid-name {
    font-size: 1em; /* 调整字体大小 */
  }
}

@media (max-width: 600px) {
  .anchor-list {
    padding: 12px 6px;
    margin: 6px 0;
  }

  .filter-btn,
  .action-btn {
    max-width: 100%;
    padding: 8px 12px;
    font-size: 0.85rem;
  }

  .page-title {
    font-size: 1.2rem;
  }

  .refresh-time {
    font-size: 0.75rem;
    text-align: center;
  }

  .anchor-table {
    font-size: 0.65rem;
    min-width: auto;
    width: 100%;
  }

  .anchor-table th,
  .anchor-table td {
    padding: 6px 4px;
    min-width: 50px;
  }

  .name-cell {
    min-width: 80px;
    white-space: normal; /* 允许名称换行 */
    font-size: 0.7rem;
  }

  .number-cell, .duration-cell, .total-revenue {
    font-size: 0.75em;
    word-break: break-word;
    text-align: right;
  }

  .view-btn {
    padding: 6px 8px;
    font-size: 0.7rem;
    min-width: 90px;
    width: 100%;
  }
}

@media (max-width: 480px) {
  .anchor-list {
    padding: 10px 4px;
  }

  .filter-btn,
  .action-btn {
    padding: 7px 10px;
    font-size: 0.8rem;
    margin: 4px 0;
  }

  .page-title {
    font-size: 1.1rem;
  }

  .refresh-time {
    font-size: 0.7rem;
  }

  .anchor-table {
    font-size: 0.6rem;
    min-width: auto;
    width: 100%;
  }

  .anchor-table th,
  .anchor-table td {
    padding: 5px 3px;
    min-width: 40px;
  }

  .number-cell,
  .duration-cell {
    text-align: center;
    font-size: 0.8em;
  }

  .action-cell {
    text-align: center;
    min-width: 100px;
  }

  .view-btn {
    padding: 5px 6px;
    font-size: 0.65rem;
    min-width: 80px;
    width: 100%;
  }

  .status-badge.live {
    min-width: 70px;
    padding: 3px 6px;
    font-size: 0.7rem;
  }
}

@media (max-width: 360px) {
  .anchor-list {
    padding: 8px 2px;
  }

  .page-title {
    font-size: 1rem;
  }

  .anchor-table {
    font-size: 0.55rem;
    min-width: 600px; /* 在极小屏幕上保持表格可读性 */
  }

  .anchor-table th,
  .anchor-table td {
    padding: 2.5px 0.5px;
    min-width: 40px;
  }

  .view-btn {
    padding: 4px 6px;
    font-size: 0.65rem;
    min-width: 80px;
  }
}

/* 多月份统计模态框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

/* 聚类分析相关样式 */
.cluster-analysis-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 200px;
  overflow-y: auto;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 5px;
  background: rgba(255, 255, 255, 0.5);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.cluster-chart-container {
  background: #FFF8E1;
  border-radius: 20px;
  padding: 20px;
  margin: 20px 0;
  border: 1px solid #FFC633;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.close-chart-btn {
  padding: 8px 16px;
  background: #f9729a;
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-weight: bold;
  transition: all 0.3s ease;
}

.close-chart-btn:hover {
  background: #e0658a;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.analysis-results {
  margin-bottom: 20px;
}

.statistics-panel, .cluster-summary {
  margin-bottom: 20px;
  padding: 15px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 10px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 10px;
  margin-top: 10px;
}

.stat-item {
  padding: 8px;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 5px;
  font-size: 14px;
}

.chart-navigation {
  display: flex;
  gap: 10px;
  margin: 20px 0;
  flex-wrap: wrap;
}

.chart-navigation button {
  padding: 8px 16px;
  background: #f9729a;
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.3s ease;
  flex: 1;
  min-width: 120px;
}

.chart-navigation button:hover {
  background: #e0658a;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.chart-navigation button.active {
  background: #e0658a;
  font-weight: bold;
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.chart-container {
  height: 400px;
  margin-top: 20px;
}

/* 回归分析相关样式 */
.regression-analysis-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

/* 加载遮罩样式 */
.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 2000;
}

.loading-content {
  background: #FFF8E1;
  border-radius: 20px;
  padding: 30px;
  text-align: center;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f9729a;
  border-top: 4px solid transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 15px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.variable-select {
  padding: 10px;
  border: 2px solid #f9729a;
  border-radius: 10px;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.8);
  transition: all 0.3s ease;
  width: 100%;
}

.variable-select:focus {
  outline: none;
  border-color: #e0658a;
  box-shadow: 0 0 10px rgba(249, 114, 154, 0.3);
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 200px;
  overflow-y: auto;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 5px;
  background: rgba(255, 255, 255, 0.5);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 14px;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.regression-chart-container {
  background: #FFF8E1;
  border-radius: 20px;
  padding: 20px;
  margin: 20px 0;
  border: 1px solid #FFC633;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.close-chart-btn {
  padding: 8px 16px;
  background: #f9729a;
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-weight: bold;
  transition: all 0.3s ease;
}

.close-chart-btn:hover {
  background: #e0658a;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.analysis-results {
  margin-bottom: 20px;
}

.statistics-panel, .coefficients-panel, .analysis-summary {
  margin-bottom: 20px;
  padding: 15px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 10px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 10px;
  margin-top: 10px;
}

.stat-item {
  padding: 8px;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 5px;
  font-size: 14px;
}

.coefficients-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 10px;
}

.coefficients-table th,
.coefficients-table td {
  padding: 8px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

.coefficients-table th {
  background: rgba(249, 114, 154, 0.2);
  font-weight: bold;
}

.highly-significant {
  color: #d32f2f;
  font-weight: bold;
}

.significant {
  color: #f57c00;
  font-weight: bold;
}

.moderately-significant {
  color: #1976d2;
  font-weight: bold;
}

.not-significant {
  color: #666;
  font-weight: normal;
}

.modal-content {
  background: #FFF8E1;
  border-radius: 20px;
  padding: 25px;
  width: 400px;
  max-width: 90%;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.modal-content h3 {
  margin-top: 0;
  margin-bottom: 20px;
  color: #f9729a;
  text-align: center;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.form-group label {
  font-weight: bold;
  color: #333;
}

.month-input {
  padding: 10px;
  border: 2px solid #f9729a;
  border-radius: 10px;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.8);
  transition: all 0.3s ease;
}

.month-input:focus {
  outline: none;
  border-color: #e0658a;
  box-shadow: 0 0 10px rgba(249, 114, 154, 0.3);
}

.button-group {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.confirm-btn, .cancel-btn {
  flex: 1;
  padding: 10px;
  border: none;
  border-radius: 10px;
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.3s ease;
}

.confirm-btn {
  background: linear-gradient(45deg, #f9729a, #f75982);
  color: white;
}

.confirm-btn:hover {
  background: linear-gradient(45deg, #e0658a, #d05572);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.cancel-btn {
  background: #ccc;
  color: white;
}

.cancel-btn:hover {
  background: #bbb;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

/* 图表导航样式 */
.chart-navigation {
  display: flex;
  gap: 10px;
  margin: 20px 0;
  flex-wrap: wrap;
}

.chart-navigation button {
  padding: 8px 16px;
  background: #f9729a;
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.3s ease;
  flex: 1;
  min-width: 120px;
}

.chart-navigation button:hover {
  background: #e0658a;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.chart-navigation button.active {
  background: #e0658a;
  font-weight: bold;
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.chart-container {
  height: 400px;
  margin-top: 20px;
  position: relative;
}

.chart-container canvas {
  width: 100% !important;
  height: 100% !important;
}

/* 宽屏优化：在大屏幕上显示更多列 */
@media (min-width: 1024px) {
  .grid-container {
    display: grid !important;
    grid-template-columns: repeat(auto-fill, minmax(380px, 1fr)); /* 自动填充，最小380px宽的列 */
    gap: 20px; /* 卡片间距 */
    padding: 15px; /* 内边距 */
  }

  .anchor-grid-item {
    margin-bottom: 0; /* 在网格布局中不需要底部边距 */
    height: fit-content; /* 高度自适应内容 */
    transition: transform 0.3s ease, box-shadow 0.3s ease; /* 添加悬停效果 */
  }

  .anchor-grid-item:hover {
    transform: translateY(-5px); /* 悬停时轻微上移 */
    box-shadow: 0 8px 24px rgba(255, 198, 51, 0.4); /* 增强阴影效果 */
  }
}

/* 中等屏幕：显示2列 */
@media (min-width: 769px) and (max-width: 1023px) {
  .grid-container {
    display: grid !important;
    grid-template-columns: repeat(2, 1fr); /* 固定2列 */
    gap: 15px; /* 卡片间距 */
    padding: 10px; /* 内边距 */
  }

  .anchor-grid-item {
    margin-bottom: 0; /* 在网格布局中不需要底部边距 */
    height: fit-content; /* 高度自适应内容 */
    transition: transform 0.3s ease, box-shadow 0.3s ease; /* 添加悬停效果 */
  }

  .anchor-grid-item:hover {
    transform: translateY(-5px); /* 悬停时轻微上移 */
    box-shadow: 0 8px 24px rgba(255, 198, 51, 0.4); /* 增强阴影效果 */
  }
}

/* 小屏幕：显示1列 */
@media (max-width: 768px) {
  .grid-container {
    display: block; /* 单列显示 */
  }
}

/* 触屏设备优化 */
@media (hover: none) and (pointer: coarse) {
  .anchor-grid-item {
    /* 为触屏设备添加点击反馈 */
    tap-highlight-color: transparent;
    -webkit-tap-highlight-color: transparent;
  }

  .anchor-grid-item:active {
    transform: scale(0.98); /* 点击时轻微缩小 */
    box-shadow: 0 4px 16px rgba(255, 198, 51, 0.3); /* 减弱阴影 */
  }

  .field-label:active,
  .field-value:active {
    transform: scale(0.99); /* 点击时轻微缩小 */
  }
}
</style>