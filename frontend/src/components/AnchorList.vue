<template>
  <div class="anchor-list">
    <div class="controls-section">
      <div class="filter-controls">
        <GlassButton
          @click="switchFilter('all')"
          :variant="currentFilter === 'all' ? 'secondary' : 'primary'"
          size="md"
        >
          维阿PSP斗虫榜
        </GlassButton>
        <GlassButton
          @click="switchFilter('vr')"
          :variant="currentFilter === 'vr' ? 'secondary' : 'primary'"
          size="md"
        >
          维阿斗虫榜
        </GlassButton>
        <GlassButton
          @click="switchFilter('psp')"
          :variant="currentFilter === 'psp' ? 'secondary' : 'primary'"
          size="md"
        >
          PSPlive斗虫榜
        </GlassButton>
      </div>

      <div class="action-controls">
        <GlassButton @click="openMonthSelector" variant="secondary">
          切换不同月份
        </GlassButton>
        <GlassButton @click="openMultiMonthModal" variant="secondary">
          多月份共同统计
        </GlassButton>
      </div>

      <MonthSelector
        :visible="showMonthSelector"
        title="切换月份"
        mode="single"
        @confirm="performMonthSwitch"
        @cancel="closeMonthSelector"
      />

      <MonthSelector
        :visible="showMultiMonthModal"
        title="多月份共同统计"
        mode="range"
        @confirm="performMultiMonthQuery"
        @cancel="closeMultiMonthModal"
      />
    </div>

    <div class="info-section">
      <h2 class="page-title">{{ title }}</h2>
      <p class="refresh-time">刷新时间：{{ refreshTime }}</p>
    </div>

    <div class="chart-button-container">
      <GlassButton @click="showRevenueChart" variant="primary" title="营收占比分析">
        查看营收占比
      </GlassButton>
      <GlassButton
        v-if="currentFilter === 'all'"
        @click="showVRPSPComparison"
        variant="primary"
        title="VR与PSP工会数据对比"
      >
        VR PSP对比图
      </GlassButton>
      <GlassButton @click="hideAllCharts" variant="danger">
        关闭图表
      </GlassButton>
      <GlassButton @click="openRegressionAnalysisModal" variant="primary">
        进行回归分析
      </GlassButton>
      <!--
      <button @click="openClusterAnalysisModal" class="action-btn primary">
        进行聚类分析
      </button>
      -->
    </div>

    <!-- 聚类分析模态框 -->
    <GlassDialog :visible="showClusterModal" title="聚类分析" width="600px" @close="closeClusterModal">
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
        </div>
        <template #footer>
          <GlassButton @click="performClusterAnalysis" variant="secondary" :disabled="!canPerformClusterAnalysis">确定</GlassButton>
          <GlassButton @click="closeClusterModal" variant="default">取消</GlassButton>
        </template>
    </GlassDialog>

    <!-- 回归分析模态框 -->
    <GlassDialog :visible="showRegressionModal" title="回归分析" width="600px" @close="closeRegressionModal">
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
        </div>
        <template #footer>
          <GlassButton @click="performRegressionAnalysis" variant="secondary" :disabled="!canPerformAnalysis || regressionLoading">
            <span v-if="regressionLoading">计算中...</span>
            <span v-else>确定</span>
          </GlassButton>
          <GlassButton @click="closeRegressionModal" variant="default">取消</GlassButton>
        </template>
    </GlassDialog>

    <!-- 聚类分析结果图表容器 -->
    <div v-if="clusterAnalysisVisible" class="cluster-chart-container">
      <div class="chart-header">
        <h3>聚类分析结果</h3>
        <GlassButton @click="closeClusterAnalysis" variant="secondary" size="sm">关闭</GlassButton>
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
        <GlassButton @click="closeRegressionAnalysis" variant="secondary" size="sm">关闭</GlassButton>
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

    <!-- 导出截图模态框 -->
    <GlassDialog :visible="showExportModal" title="导出数据截图" width="420px" @close="closeExportModal">
        <div class="modal-form">
          <div class="form-group">
            <label>起始月份:</label>
            <input type="month" v-model="exportStartMonth" min="2025-08" class="month-input">
          </div>
          <div class="form-group">
            <label>结束月份:</label>
            <input type="month" v-model="exportEndMonth" min="2025-08" class="month-input">
          </div>
        </div>
        <template #footer>
          <GlassButton @click="performExport" variant="secondary" :disabled="exportLoading">
            {{ exportLoading ? '导出中...' : '确定导出' }}
          </GlassButton>
          <GlassButton @click="closeExportModal" variant="default">取消</GlassButton>
        </template>
    </GlassDialog>

    <!-- VR PSP 对比图弹窗 -->
    <GlassDialog :visible="vrpspDialogVisible" title="VR vs PSP 工会数据对比" width="700px" @close="closeVRPSPDialog">
      <div class="vrpsp-dialog-chart">
        <div ref="vrpspChartCanvas" style="width:100%;height:400px"></div>
      </div>
      <div class="chart-legend" v-if="vrpspDialogVisible">
        <span>
          <img :src="vrAvatarUrl" class="legend-avatar">
          <span class="legend-dot" style="background-color: #FF6384"></span>
          VirtuaReal {{ vrpspTotal.vr + vrpspTotal.psp > 0 ? Math.round(vrpspTotal.vr / (vrpspTotal.vr + vrpspTotal.psp) * 100) + '%' : '' }}
        </span>
        <span>
          <img :src="pspAvatarUrl" class="legend-avatar">
          <span class="legend-dot" style="background-color: #36A2EB"></span>
          PSPlive {{ vrpspTotal.vr + vrpspTotal.psp > 0 ? Math.round(vrpspTotal.psp / (vrpspTotal.vr + vrpspTotal.psp) * 100) + '%' : '' }}
        </span>
      </div>
    </GlassDialog>

    <div class="chart-info" v-if="chartVisible">
      <h3 style="color: var(--color-accent); margin-top: 0;"><BarChart3 :size="20" class="heading-icon" /> 图表交互说明</h3>
      <p><strong>图表功能：</strong></p>
      <ul style="text-align: left; display: inline-block;">
        <li>点击图例可以隐藏/显示对应的数据显示</li>
        <li>鼠标悬停在饼图块上可以查看详细数值和百分比</li>
        <li>图表支持缩放和拖拽（如果浏览器支持）</li>
        <li>双击图表可以重置缩放</li>
      </ul>
    </div>

    <div :class="['chart-container', { visible: chartVisible }]">
      <div id="chartCanvas" ref="chartCanvas" style="width:100%;height:100%"></div>
    </div>

    <div class="chart-legend" v-if="chartVisible && chartType === 'revenue'">
      <span v-for="(label, i) in chartLabels" :key="i">
        <img :src="getAvatarSync(chartRoomIds[i])" class="legend-avatar">
        <span class="legend-dot" :style="{ backgroundColor: chartColors[i % chartColors.length] }"></span>
        {{ label }} {{ chartData.length > 0 ? Math.round(chartData[i] / chartData.reduce((a, b) => a + b, 0) * 100) + '%' : '' }}
      </span>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>加载中...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p class="error-message">{{ error }}</p>
      <GlassButton @click="fetchData" variant="success">重试</GlassButton>
    </div>

    <div v-else class="data-section">
      <!-- 导航表格 -->
      <NavigationTable 
        :items="anchors" 
        item-type="anchor" 
        v-if="anchors.length > 0"
        @open-battle="openBattleModal"
        @selection-change="onSelectionChange"
        @open-export="openExportModal"
        @open-rank="openRankModal"
      />


      <!-- 恶意斗虫组件 -->
      <AnchorBattle 
        v-if="showBattle"
        :initial-anchors="battleAnchors"
        @close="closeBattleModal"
      />

      <!-- 排名对比组件 -->
      <RankComparison
        v-if="showRank"
        :initial-anchors="rankAnchors"
        @close="closeRankModal"
      />

      <div class="grid-container" ref="gridContainer">
        <BaseCard
          v-for="(anchor, index) in anchors"
          :key="anchor.room_id || index"
          card-type="anchor"
          :rank="index + 1"
          :title="anchor.anchor_name + ' [' + anchor.union + ']'"
          :subtitle="''"
          :is-live="anchor.status === 1"
          :avatar-url="avatarMap[anchor.room_id] || ''"
          :fields="[
            { label: '关注数', value: formatNumber(anchor.attention), type: 'number' },
            { label: '有效天', value: anchor.effective_days },
            { label: '开播时长', value: anchor.live_duration, type: 'duration' },
            { label: '开播状态', value: anchor.status === 1 ? '正在直播' : '未开播', className: anchor.status === 1 ? 'live-status-field' : 'offline-status-field' },
            { label: '总督', value: anchor.guard_3 || 0 },
            { label: '提督', value: anchor.guard_2 || 0 },
            { label: '舰长', value: anchor.guard_1 || 0 },
            { label: '粉丝团', value: formatNumber(anchor.fans_count || 0), type: 'number' },
            { label: '礼物收入', value: formatCurrency(anchor.gift), type: 'currency' },
            { label: '舰长收入', value: formatCurrency(anchor.guard), type: 'currency' },
            { label: 'SC收入', value: formatCurrency(anchor.super_chat), type: 'currency' },
            { label: '总营收', value: formatCurrency(calculateTotalRevenue(anchor)), type: 'currency' },
            { label: '即时同接', value: anchor.current_concurrency !== null ? formatNumber(anchor.current_concurrency) : '未开播', type: anchor.current_concurrency !== null ? 'number' : 'text' }
          ]"
          :action-button="{ text: '查看详细数据', className: 'view-btn' }"
          :action-data="anchor"
          @action-click="viewLiveSessions(anchor.room_id, anchor.union)"
        >
          <template #actions>
            <GlassButton
              @click="viewLiveSessions(anchor.room_id, anchor.union)"
              variant="secondary"
              size="sm"
              cta
            >
              查看详细数据
            </GlassButton>
          </template>
        </BaseCard>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, watch, nextTick, computed, provide } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import * as echarts from 'echarts'
import '@/utils/echartsTheme.js'
import html2canvas from 'html2canvas'
import { anchorAPI } from '@/api'
import BaseCard from '@/components/BaseCard.vue'
import NavigationTable from '@/components/NavigationTable.vue'
import AnchorBattle from '@/components/AnchorBattle.vue'
import RankComparison from '@/components/RankComparison.vue'
import MonthSelector from '@/components/MonthSelector.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import GlassDialog from '@/components/ui/GlassDialog.vue'
import { BarChart3 } from 'lucide-vue-next'
import { getMonthRange } from '@/utils/monthUtils'
import { provideGlobalCardState } from '@/composables/useGlobalCardState'
import { getAvatar, getAvatarByUid, scaleAvatar, getAvatarSync } from '@/utils/avatarCache'
import { staggerEnter } from '@/composables/useGSAP'
import gsap from 'gsap'

export default {
  name: 'AnchorList',
  components: {
    BaseCard,
    NavigationTable,
    AnchorBattle,
    RankComparison,
    MonthSelector,
    GlassButton,
    GlassDialog,
    BarChart3
  },
  setup() {
    const router = useRouter()
    const route = useRoute()
    const chartVisible = ref(false)
    const chartType = ref('')
    const chartLabels = ref([])
    const chartRoomIds = ref([])
    const chartData = ref([])
    const vrpspTotal = ref({ vr: 0, psp: 0 })
    const chartColors = [
      '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF',
      '#FF9F40', '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4',
      '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE'
    ]
    const anchors = ref([])
    const title = ref('维阿PSP斗虫榜')
    const refreshTime = ref(new Date().toLocaleString())
    const currentFilter = ref('all')
    const loading = ref(true)
    const error = ref(null)
    const avatarMap = ref({})
    let currentChart = null
    const chartCanvas = ref(null)
    const vrAvatarUrl = ref('')
    const pspAvatarUrl = ref('')
    const gridContainer = ref(null)
    const vrpspDialogVisible = ref(false)
    const vrpspChartCanvas = ref(null)
    let vrpspChart = null



    // 恶意斗虫相关
    const showBattle = ref(false)
    const battleAnchors = ref([])

    const openBattleModal = (selectedAnchors) => {
      battleAnchors.value = selectedAnchors
      showBattle.value = true
    }

    const closeBattleModal = () => {
      showBattle.value = false
      battleAnchors.value = []
    }

    // 排名对比相关
    const showRank = ref(false)
    const rankAnchors = ref([])

    const openRankModal = (selectedAnchors) => {
      rankAnchors.value = selectedAnchors
      showRank.value = true
    }

    const closeRankModal = () => {
      showRank.value = false
      rankAnchors.value = []
    }

    // 导出截图相关
    const selectedAnchorsForExport = ref([])
    const selectedAnchorsCount = ref(0)
    const showExportModal = ref(false)
    const exportStartMonth = ref('')
    const exportEndMonth = ref('')
    const exportLoading = ref(false)

    const onSelectionChange = (selected) => {
      selectedAnchorsForExport.value = selected
      selectedAnchorsCount.value = selected.length
    }

    const openExportModal = () => {
      if (selectedAnchorsForExport.value.length === 0) {
        alert('请先在导航表格中勾选至少一个主播')
        return
      }
      showExportModal.value = true
    }

    const closeExportModal = () => {
      showExportModal.value = false
    }

    const getExportFields = (anchor) => {
      return [
        { label: '关注数', value: formatNumber(anchor.attention) },
        { label: '有效天', value: anchor.effective_days },
        { label: '开播时长', value: anchor.live_duration },
        { label: '总督', value: anchor.guard_3 || 0 },
        { label: '提督', value: anchor.guard_2 || 0 },
        { label: '舰长', value: anchor.guard_1 || 0 },
        { label: '粉丝团', value: formatNumber(anchor.fans_count || 0) },
        { label: '礼物收入', value: formatCurrency(anchor.gift) },
        { label: '舰长收入', value: formatCurrency(anchor.guard) },
        { label: 'SC收入', value: formatCurrency(anchor.super_chat) },
        { label: '总营收', value: formatCurrency(calculateTotalRevenue(anchor)) },
        { label: '即时同接', value: anchor.current_concurrency !== null ? formatNumber(anchor.current_concurrency) : '未开播' }
      ]
    }

    const imageToBase64 = (url) => new Promise((resolve) => {
      const img = new Image()
      if (url.startsWith('http')) {
        img.crossOrigin = 'anonymous'
      }
      img.onload = () => {
        const canvas = document.createElement('canvas')
        canvas.width = img.naturalWidth
        canvas.height = img.naturalHeight
        const ctx = canvas.getContext('2d')
        ctx.drawImage(img, 0, 0)
        resolve(canvas.toDataURL('image/png'))
      }
      img.onerror = () => resolve('')
      img.src = url
    })

    const performExport = async () => {
      if (!exportStartMonth.value || !exportEndMonth.value) {
        alert('请选择起始和结束月份')
        return
      }

      exportLoading.value = true

      try {
        const logo1 = await imageToBase64('/logo1.png')
        const logo2 = await imageToBase64('/logo2.png')

        const months = getMonthRange(exportStartMonth.value, exportEndMonth.value)
        let combinedAnchors = {}

        for (const month of months) {
          try {
            const response = await anchorAPI.getAnchorsByMonth(month, currentFilter.value)
            const anchorsForMonth = response.anchors || response.data || []
            anchorsForMonth.forEach(anchor => {
              const key = anchor.room_id || anchor.anchor_name
              if (!combinedAnchors[key]) {
                combinedAnchors[key] = { ...anchor }
                combinedAnchors[key].attention = parseFloat(anchor.attention) || 0
                combinedAnchors[key].effective_days = parseInt(anchor.effective_days) || 0
                combinedAnchors[key].guard_1 = parseInt(anchor.guard_1) || 0
                combinedAnchors[key].guard_2 = parseInt(anchor.guard_2) || 0
                combinedAnchors[key].guard_3 = parseInt(anchor.guard_3) || 0
                combinedAnchors[key].fans_count = parseInt(anchor.fans_count) || 0
                combinedAnchors[key].gift = parseFloat(anchor.gift) || 0
                combinedAnchors[key].guard = parseFloat(anchor.guard) || 0
                combinedAnchors[key].super_chat = parseFloat(anchor.super_chat) || 0
              } else {
                combinedAnchors[key].attention = parseFloat(anchor.attention) || 0
                combinedAnchors[key].effective_days += parseInt(anchor.effective_days) || 0
                combinedAnchors[key].guard_1 += parseInt(anchor.guard_1) || 0
                combinedAnchors[key].guard_2 += parseInt(anchor.guard_2) || 0
                combinedAnchors[key].guard_3 += parseInt(anchor.guard_3) || 0
                combinedAnchors[key].fans_count = parseInt(anchor.fans_count) || 0
                combinedAnchors[key].gift += parseFloat(anchor.gift) || 0
                combinedAnchors[key].guard += parseFloat(anchor.guard) || 0
                combinedAnchors[key].super_chat += parseFloat(anchor.super_chat) || 0
              }
            })
          } catch (err) {
            console.error(`获取${month}月份数据失败:`, err)
          }
        }

        const selectedRoomIds = new Set(selectedAnchorsForExport.value.map(a => a.room_id))

        // 先对全部主播排名（总排名）
        const allAnchorsSorted = Object.values(combinedAnchors).sort((a, b) => {
          const revA = parseFloat(a.gift || 0) + parseFloat(a.guard || 0) + parseFloat(a.super_chat || 0)
          const revB = parseFloat(b.gift || 0) + parseFloat(b.guard || 0) + parseFloat(b.super_chat || 0)
          return revB - revA
        })
        const overallRanks = {}
        allAnchorsSorted.forEach((anchor, index) => {
          overallRanks[anchor.room_id] = index + 1
        })

        // 筛选选中的主播
        const exportData = allAnchorsSorted.filter(a => selectedRoomIds.has(a.room_id))

        if (exportData.length === 0) {
          alert('选中的主播在指定月份范围内没有数据')
          exportLoading.value = false
          return
        }

        const avatarMapExport = {}
        for (const anchor of exportData) {
          const img = await getAvatar(anchor.room_id)
          if (img) {
            const c = document.createElement('canvas')
            c.width = img.naturalWidth
            c.height = img.naturalHeight
            c.getContext('2d').drawImage(img, 0, 0)
            avatarMapExport[anchor.room_id] = c.toDataURL('image/png')
          } else {
            avatarMapExport[anchor.room_id] = ''
          }
        }

        const fields = getExportFields(exportData[0])
        const sm = exportStartMonth.value
        const em = exportEndMonth.value
        const startLabel = `${sm.substring(0, 4)}年${sm.substring(5, 7)}月`
        const endLabel = `${em.substring(0, 4)}年${em.substring(5, 7)}月`

        const headerCells = fields.map(f => `<th style="padding:10px 12px;text-align:right;white-space:nowrap;font-size:0.85rem;color:#5D4B24;">${f.label}</th>`).join('')

        let tableRows = ''
        exportData.forEach(anchor => {
          const avatarBase64 = avatarMapExport[anchor.room_id] || ''
          const avatarImg = avatarBase64
            ? `<div style="width:40px;height:40px;border-radius:50%;overflow:hidden;display:inline-block;flex-shrink:0;"><img src="${avatarBase64}" style="width:40px;height:40px;object-fit:cover;display:block;" /></div>`
            : ''
          const fieldsHtml = getExportFields(anchor).map(f => `<td style="padding:8px 12px;border-bottom:1px solid #F6B100;text-align:right;white-space:nowrap;font-size:0.85rem;color:#5D4B24;">${f.value}</td>`).join('')
          tableRows += `<tr>
            <td style="padding:8px 12px;border-bottom:1px solid #F6B100;text-align:center;font-weight:bold;color:#F6B100;">${overallRanks[anchor.room_id]}</td>
            <td style="padding:8px 12px;border-bottom:1px solid #F6B100;text-align:center;">${avatarImg}</td>
            <td style="padding:8px 12px;border-bottom:1px solid #F6B100;font-weight:bold;white-space:nowrap;color:#5D4B24;">${anchor.anchor_name}</td>
            ${fieldsHtml}
          </tr>`
        })

        const siteUrl = 'https:斜杠hihivr点top'
        const htmlContent = `
          <div style="font-family:'Segoe UI','Microsoft YaHei',sans-serif;background:#F7F1DF;padding:20px;width:1200px;">
            <header style="background:#F7F1DF;padding:20px 0;">
              <div style="margin:0 auto;padding:0 20px;">
                <div style="display:flex;align-items:center;justify-content:center;gap:15px;margin-bottom:15px;">
                  ${logo1 ? `<img src="${logo1}" style="height:100px;" />` : ''}
                  ${logo2 ? `<img src="${logo2}" style="height:100px;" />` : ''}
                  <h1 style="color:#F6B100;font-size:2rem;font-weight:bold;text-shadow:2px 2px 4px rgba(93,75,36,0.3);margin:0;">维阿PSP斗虫榜_${siteUrl}</h1>
                </div>
                <div style="text-align:center;">
                  <p style="color:#FF6B9D;font-size:1.2rem;margin-bottom:10px;line-height:1.4;font-weight:bold;text-align:center;">特别感谢某热心小礼猫-千秋紫莹提供的斗虫数据API，感谢其对本项目提供了巨大的帮助</p>
                </div>
              </div>
            </header>
            <div style="background:rgba(246,177,0,0.1);border:1px solid #F6B100;border-radius:32px;padding:10px 15px;margin-bottom:12px;color:#5D4B24;font-size:0.9rem;box-shadow:0 2px 8px rgba(93,75,36,0.1);">
              <div style="font-weight:bold;margin-bottom:4px;">使用导出功能制作</div>
              <div>使用方法：在主页勾选主播 → 点击导出截图 → 选择时间范围 → 确定导出</div>
            </div>
            <div style="color:#FF6B9D;font-weight:bold;font-size:1.1rem;margin-bottom:10px;">${startLabel}-${endLabel}数据</div>
            <table style="width:100%;border-collapse:collapse;background:#F7F1DF;border-radius:32px;overflow:hidden;box-shadow:0 4px 16px rgba(93,75,36,0.15);">
              <thead>
                <tr style="background:linear-gradient(45deg,#F6B100,#FF6B9D);color:#5D4B24;">
                  <th style="padding:10px 12px;text-align:center;width:50px;">排名</th>
                  <th style="padding:10px 12px;text-align:center;width:50px;">头像</th>
                  <th style="padding:10px 12px;text-align:left;white-space:nowrap;">主播名</th>
                  ${headerCells}
                </tr>
              </thead>
              <tbody>${tableRows}</tbody>
            </table>
          </div>
        `

        const container = document.createElement('div')
        container.style.position = 'fixed'
        container.style.left = '0'
        container.style.top = '0'
        container.style.opacity = '0'
        container.style.pointerEvents = 'none'
        container.style.zIndex = '-1'
        container.innerHTML = htmlContent
        document.body.appendChild(container)

        try {
          await new Promise(r => requestAnimationFrame(r))

          const canvas = await html2canvas(container.firstElementChild, {
            useCORS: false,
            allowTaint: false,
            scale: 2,
            backgroundColor: '#F7F1DF',
            logging: false
          })

          const link = document.createElement('a')
          link.download = `斗虫榜数据_${startLabel}-${endLabel}.png`
          link.href = canvas.toDataURL('image/png')
          link.click()

          closeExportModal()
        } catch (err) {
          console.error('导出截图失败:', err)
          alert('导出截图失败: ' + err.message)
        } finally {
          if (container.parentNode) {
            document.body.removeChild(container)
          }
        }
      } catch (err) {
        console.error('导出截图失败:', err)
        alert('导出截图失败: ' + err.message)
      } finally {
        exportLoading.value = false
      }
    }
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

        anchors.value.forEach(anchor => {
          if (anchor.room_id) {
            avatarMap.value[anchor.room_id] = getAvatarSync(anchor.room_id)
          }
        })
      } catch (err) {
        console.error('获取主播数据失败:', err)
        error.value = '获取数据失败，请稍后重试'
      } finally {
        loading.value = false
        nextTick(() => {
          if (gridContainer.value) {
            const cards = gridContainer.value.querySelectorAll(':scope > *')
            if (cards.length) staggerEnter(cards)
          }
        })
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

    const openMonthSelector = () => {
      showMonthSelector.value = true
    }

    const closeMonthSelector = () => {
      showMonthSelector.value = false
    }

    const performMonthSwitch = ({ selectedMonth } = {}) => {
      if (!selectedMonth) {
        alert('请选择月份')
        return
      }

      router.push({
        path: '/by-month',
        query: { month: selectedMonth, filter: currentFilter.value }
      })
      closeMonthSelector()
    }

    const showRevenueChart = async () => {
      chartType.value = 'revenue'
      const data = []
      const labels = []
      const roomIds = []

      anchors.value.forEach(anchor => {
        const revenue = parseFloat(anchor.total_revenue || anchor.gift + anchor.guard + anchor.super_chat || 0)
        if (!isNaN(revenue) && revenue > 0) {
          data.push(revenue)
          labels.push(anchor.anchor_name)
          roomIds.push(anchor.room_id)
        }
      })

      if (data.length === 0) {
        alert('没有可用的数据来生成图表')
        return
      }

      await Promise.all(roomIds.map(id => getAvatar(id)))

      chartVisible.value = true
      chartLabels.value = labels
      chartRoomIds.value = roomIds
      chartData.value = data
      await nextTick()
      if (currentChart) {
        currentChart.dispose()
        currentChart = null
      }

      const fallbackColors = [
        '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF',
        '#FF9F40', '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4',
        '#FFEAA7', '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE'
      ]

      const AVATAR_SIZE = 60
      const seriesData = []
      for (let i = 0; i < labels.length; i++) {
        const img = await getAvatar(roomIds[i])
        const scaled = scaleAvatar(img, AVATAR_SIZE)
        let color = fallbackColors[i % fallbackColors.length]
        if (scaled) {
          color = { image: scaled, repeat: 'repeat' }
        }
        seriesData.push({ name: labels[i], value: data[i], itemStyle: { color } })
      }

      currentChart = echarts.init(chartCanvas.value, 'liveshow')
      currentChart.setOption({
        title: {
          text: '主播营收占比',
          left: 'center',
          textStyle: { fontSize: 16 }
        },
        tooltip: {
          trigger: 'item',
          formatter: function(params) {
            const total = data.reduce((sum, d) => sum + d, 0)
            const percentage = Math.round((params.value / total) * 100)
            return `${params.name}: ${params.value.toFixed(2)} (${percentage}%)`
          }
        },
        series: [{
          name: '总营收占比',
          type: 'pie',
          radius: '82%',
          center: ['50%', '55%'],
          data: seriesData,
          label: { show: false },
          emphasis: {
            itemStyle: {
              shadowBlur: 10,
              shadowOffsetX: 0,
              shadowColor: 'rgba(0, 0, 0, 0.5)'
            }
          }
        }]
      })

      window.addEventListener('resize', () => {
        if (currentChart) currentChart.resize()
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

      hideAllCharts()
      chartType.value = 'vrpsp'

      vrpspTotal.value = { vr: vrTotal, psp: pspTotal }

      const [vrImg, pspImg] = await Promise.all([
        getAvatarByUid('413748120'),
        getAvatarByUid('454673997')
      ])

      if (vrImg) {
        const c = document.createElement('canvas')
        c.width = vrImg.naturalWidth; c.height = vrImg.naturalHeight
        c.getContext('2d').drawImage(vrImg, 0, 0)
        vrAvatarUrl.value = c.toDataURL('image/jpeg', 0.8)
      }
      if (pspImg) {
        const c = document.createElement('canvas')
        c.width = pspImg.naturalWidth; c.height = pspImg.naturalHeight
        c.getContext('2d').drawImage(pspImg, 0, 0)
        pspAvatarUrl.value = c.toDataURL('image/jpeg', 0.8)
      }

      const AVATAR_SIZE = 60
      const vrScaled = scaleAvatar(vrImg, AVATAR_SIZE)
      const pspScaled = scaleAvatar(pspImg, AVATAR_SIZE)

      vrpspDialogVisible.value = true
      await nextTick()
      if (vrpspChart) {
        vrpspChart.dispose()
        vrpspChart = null
      }

      vrpspChart = echarts.init(vrpspChartCanvas.value, 'liveshow')
      vrpspChart.setOption({
        title: {
          text: 'VR vs PSP 总营收对比',
          left: 'center',
          textStyle: { fontSize: 16 }
        },
        tooltip: {
          trigger: 'item',
          formatter: function(params) {
            const total = vrTotal + pspTotal
            const percentage = Math.round((params.value / total) * 100)
            return `${params.name}: ${params.value.toFixed(2)} (${percentage}%)`
          }
        },
        series: [{
          name: '工会总营收对比',
          type: 'pie',
          radius: '82%',
          center: ['50%', '55%'],
          data: [
            { name: 'VirtuaReal', value: vrTotal, itemStyle: { color: vrScaled ? { image: vrScaled, repeat: 'repeat' } : '#FF6384' } },
            { name: 'PSPlive', value: pspTotal, itemStyle: { color: pspScaled ? { image: pspScaled, repeat: 'repeat' } : '#36A2EB' } }
          ],
          label: { show: false },
          emphasis: {
            itemStyle: {
              shadowBlur: 10,
              shadowOffsetX: 0,
              shadowColor: 'rgba(0, 0, 0, 0.5)'
            }
          }
        }]
      })

      window.addEventListener('resize', () => {
        if (vrpspChart) vrpspChart.resize()
      })
    }

    const closeVRPSPDialog = () => {
      vrpspDialogVisible.value = false
      if (vrpspChart) {
        vrpspChart.dispose()
        vrpspChart = null
      }
    }

    const hideAllCharts = () => {
      chartVisible.value = false
      if (currentChart) {
        currentChart.dispose()
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
        regressionChartInstance.dispose()
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
        let minY = Math.min(...residuals)
        let maxY = Math.max(...residuals)

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
          console.error('预测vs实测图容器不存在')
          return
        }

        if (predictionChartInstance) {
          predictionChartInstance.dispose()
        }

        const actualValues = validAnchors.map(anchor => parseFloat(anchor[dependentVariable.value]))

        if (!actualValues || actualValues.length === 0 || !predictedValues || predictedValues.length === 0) {
          console.error('数据无效或为空')
          return
        }

        const minVal = Math.min(...actualValues, ...predictedValues)
        const maxVal = Math.max(...actualValues, ...predictedValues)

        const chart = echarts.init(predictionChart.value, 'liveshow')
        predictionChartInstance = chart

        chart.setOption({
          title: {
            text: '预测 vs 实测: 预测值 vs 实测值',
            left: 'center',
            textStyle: { fontSize: 16 }
          },
          tooltip: {
            trigger: 'item',
            formatter: function(params) {
              if (params.seriesIndex === 0) {
                return `实测: ${params.data[0].toFixed(2)}<br/>预测: ${params.data[1].toFixed(2)}`
              }
              return ''
            }
          },
          legend: {
            data: ['预测 vs 实测', 'y=x 参考线'],
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
            name: `实测${formatVariableName(dependentVariable.value)}`,
            splitLine: { lineStyle: { type: 'dashed' } }
          },
          yAxis: {
            type: 'value',
            name: `预测${formatVariableName(dependentVariable.value)}`,
            splitLine: { lineStyle: { type: 'dashed' } }
          },
          series: [
            {
              name: '预测 vs 实测',
              type: 'scatter',
              data: actualValues.map((actual, idx) => [actual, predictedValues[idx]]),
              symbolSize: 8,
              itemStyle: { color: 'rgba(75, 192, 192, 0.6)' }
            },
            {
              name: 'y=x 参考线',
              type: 'line',
              data: [[minVal, minVal], [maxVal, maxVal]],
              lineStyle: {
                color: 'rgba(255, 99, 132, 1)',
                width: 2,
                type: 'dashed'
              },
              symbol: 'none',
              smooth: false
            }
          ]
        })

        window.addEventListener('resize', () => {
          if (predictionChartInstance) predictionChartInstance.resize()
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

    const performMultiMonthQuery = async ({ startMonth: sMonth, endMonth: eMonth } = {}) => {
      if (!sMonth || !eMonth) {
        alert('请选择起始和结束月份')
        return
      }

      try {
        loading.value = true
        error.value = null

        const months = getMonthRange(sMonth, eMonth)

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
                // 累加数据（但关注数和粉丝团数使用最后一个月的数据，不累加）
                combinedAnchors[key].attention = parseFloat(anchor.attention) || 0  // 使用最后一个月的数据
                combinedAnchors[key].effective_days += parseInt(anchor.effective_days) || 0
                combinedAnchors[key].guard_1 += parseInt(anchor.guard_1) || 0
                combinedAnchors[key].guard_2 += parseInt(anchor.guard_2) || 0
                combinedAnchors[key].guard_3 += parseInt(anchor.guard_3) || 0
                combinedAnchors[key].fans_count = parseInt(anchor.fans_count) || 0  // 使用最后一个月的数据
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
        const startYear = sMonth.substring(0, 4)
        const startMon = sMonth.substring(4, 6)
        const endYear = eMonth.substring(0, 4)
        const endMon = eMonth.substring(4, 6)
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

        // 检查是否有scrollTo参数，如果有则跳转到指定元素
        if (newQuery.scrollTo) {
          nextTick(() => {
            const targetElement = document.getElementById(newQuery.scrollTo)
            if (targetElement) {
              targetElement.scrollIntoView({ behavior: 'smooth', block: 'start' })

              gsap.set(targetElement, { backgroundColor: 'rgba(249, 114, 154, 0.3)' })
              gsap.to(targetElement, { backgroundColor: 'transparent', duration: 0.5, delay: 1.5 })
            }
          })
        }
      },
      { immediate: true }
    )

    onMounted(() => {
      fetchData()
    })

    return {
      chartVisible,
      chartType,
      chartLabels,
      chartRoomIds,
      chartColors,
      anchors,
      title,
      refreshTime,
      currentFilter,
      loading,
      error,
      chartCanvas,
      chartData,
      vrpspTotal,
      vrAvatarUrl,
      pspAvatarUrl,
      vrpspDialogVisible,
      vrpspChartCanvas,
      closeVRPSPDialog,
      avatarMap,
      getAvatarSync,
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
      openMultiMonthModal,
      closeMultiMonthModal,
      performMultiMonthQuery,
      // 月份选择器相关
      showMonthSelector,
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
      // 恶意斗虫相关
      showBattle,
      battleAnchors,
      openBattleModal,
      closeBattleModal,
      // 排名对比相关
      showRank,
      rankAnchors,
      openRankModal,
      closeRankModal,
      // 导出截图相关
      selectedAnchorsCount,
      showExportModal,
      exportStartMonth,
      exportEndMonth,
      exportLoading,
      onSelectionChange,
      openExportModal,
      closeExportModal,
      performExport,
      globalCardState,
      fetchData,
      gridContainer
    }
  }
}
</script>

<style scoped>
.heading-icon { vertical-align: middle; margin-right: 4px }
.anchor-list {
  background: var(--color-card);
  border-radius: var(--radius-card);
  padding: 20px;
  margin: 20px 0;
  border: 1px solid var(--color-primary);
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
  border: 2px solid var(--color-primary);
  border-radius: var(--radius-button);
  cursor: pointer;
  font-size: 0.9rem;
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: var(--color-text-main);
  font-weight: bold;
}

.filter-btn.active {
  background: linear-gradient(45deg, var(--color-accent), var(--color-accent));
  border-color: var(--color-accent);
  color: white;
  box-shadow: 0 4px 12px rgba(249, 114, 154, 0.3);
}

.chart-button-container {
  display: flex;
  gap: 20px;
  justify-content: center;
  flex-wrap: wrap;
  margin: 30px 0 10px 0;
}


.action-controls {
  display: flex;
  gap: 20px;
  justify-content: center;
  flex-wrap: wrap;
  margin: 30px 0;
}

.action-btn {
  padding: 8px 16px;
  border: none;
  border-radius: var(--radius-button);
  cursor: pointer;
  font-size: 0.85rem;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 120px;
}

.action-btn.primary {
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: var(--color-text-main);
  font-weight: bold;
}

.action-btn.secondary {
  background: linear-gradient(45deg, var(--color-accent), var(--color-accent));
  color: white;
}

.action-btn.danger {
  background: linear-gradient(45deg, #E74C3C, #c82333);
  color: white;
}

.info-section {
  text-align: center;
  margin-bottom: 20px;
}

.page-title {
  color: var(--color-primary);
  font-size: 1.5rem;
  margin-bottom: 5px;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
}

.refresh-time {
  color: var(--color-accent);
  font-size: 0.9rem;
}

.chart-info {
  background: rgba(255, 107, 157, 0.1);
  border: 2px solid var(--color-accent);
  border-radius: var(--radius-card);
  padding: 15px;
  margin: 20px 0;
  text-align: center;
}

.chart-container {
  display: none;
  text-align: center;
  margin: 20px 0;
  height: 750px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: var(--radius-card);
  padding: 20px;
}

.chart-container.visible {
  display: block;
}

#chartCanvas {
  width: 100% !important;
  height: 100% !important;
}

.vrpsp-dialog-chart {
  width: 100%;
  min-height: 400px;
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
  border: 4px solid rgba(246, 177, 0, 0.3);
  border-top: 4px solid var(--color-primary);
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
  background: linear-gradient(45deg, #27AE60, #27AE60);
  color: white;
  border: none;
  border-radius: var(--radius-button);
  cursor: pointer;
  font-size: 0.9rem;
}


.number-cell {
  text-align: right;
  font-family: 'Courier New', monospace;
  color: #fff;
}

.total-revenue {
  color: var(--color-accent) !important; /* 高亮重要数值 */
  font-weight: bold;
  font-size: 1.1em; /* 稍微增大重要数值的字号 */
}

.duration-cell {
  color: var(--color-accent);
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
  background: var(--color-accent);
  color: white;
  border: 2px solid var(--color-accent);
  border-radius: var(--radius-button);
  padding: 4px 8px;
  cursor: pointer;
  font-size: 0.8rem;
  font-weight: bold;
  text-decoration: none;
  display: inline-block;
  min-width: 80px;
}

.status-badge.offline {
  background: rgba(255, 255, 255, 0.2);
  color: rgba(142, 123, 80, 0.3);
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
  background: var(--color-accent);
  color: white;
  border: 2px solid var(--color-accent);
  border-radius: var(--radius-button);
  cursor: pointer;
  font-size: 0.8rem;
  font-weight: bold;
  min-width: 120px;
}


/* 移动端网格容器 */
.grid-container {
  display: none; /* 默认隐藏网格布局 */
}

/* 网格布局样式 */
.anchor-grid-item {
  background: linear-gradient(135deg, var(--color-card), var(--color-card));
  border: 1px solid var(--color-primary);
  border-radius: 20px;
  padding: 15px;
  margin-bottom: 15px;
  box-shadow: 0 6px 16px rgba(246, 177, 0, 0.2);
  position: relative;
  overflow: hidden;
  will-change: transform;
  transform: translateZ(0);
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
  z-index: 1;
}

.anchor-grid-item.live-grid-item {
  border: 2px solid var(--color-accent);
  background: var(--color-card);
}

.grid-header {
  background: linear-gradient(135deg, var(--color-primary), var(--color-primary));
  color: white;
  padding: 12px;
  border-radius: 12px;
  margin-bottom: 12px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: 0 4px 12px rgba(246, 177, 0, 0.3);
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
  background: rgba(255, 248, 225, 0.7);
  border: 1px solid var(--color-primary);
  border-radius: 10px;
  padding: 12px;
  min-width: 120px;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  text-align: left;
  margin-bottom: 6px;
}

.field-label {
  font-weight: bold;
  color: var(--color-primary);
  font-size: 1.1em;
  word-break: break-word;
  margin-right: 10px;
  flex-shrink: 0;
  background-color: rgba(246, 177, 0, 0.15);
  padding: 4px 8px;
  border-radius: 8px;
}

.field-value {
  color: var(--color-text-main);
  font-size: 1.1em;
  word-break: break-word;
  text-align: right;
  margin-left: 10px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.total-revenue {
  color: var(--color-accent) !important;
  font-weight: bold;
}

.grid-footer {
  text-align: center;
  margin-top: 8px;
}

/* 旧的卡片布局样式（保留用于可能的回退） */
.anchor-card {
  background: linear-gradient(135deg, var(--color-card), var(--color-card));
  border: 1px solid var(--color-primary);
  border-radius: 20px;
  padding: 12px;
  margin-bottom: 12px;
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.1);
}

.anchor-card.live-card {
  border: 2px solid var(--color-accent);
  background: var(--color-card);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(45deg, var(--color-primary), var(--color-primary));
  color: white;
  padding: 10px;
  border-radius: 10px;
  margin-bottom: 8px;
}

.card-rank {
  font-weight: bold;
  color: var(--color-primary);
  font-size: 1.3em;
}

.card-name {
  font-weight: bold;
  color: var(--color-text-main);
  flex-grow: 1;
  text-align: center;
  font-size: 1.2em;
}

.card-union {
  color: var(--color-accent);
  font-weight: 600;
  font-size: 1.1em;
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 6px; /* 压缩间距 */
}

.card-row {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
  border-bottom: 1px solid #f0f0f0;
}

.field-label {
  font-weight: bold;
  color: var(--color-primary);
  min-width: 100px;
  font-size: 1.1em;
  margin-right: 10px;
  background-color: rgba(246, 177, 0, 0.15);
  padding: 4px 8px;
  border-radius: 8px;
  flex-shrink: 0;
}

.field-value {
  text-align: right;
  color: var(--color-text-main);
  flex-grow: 1;
  font-size: 1.1em;
  word-break: break-word;
  font-weight: 500;
}

.total-revenue {
  color: var(--color-accent) !important;
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
  background: var(--color-card);
  border-radius: var(--radius-card);
  padding: 20px;
  margin: 20px 0;
  border: 1px solid var(--color-primary);
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
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-weight: bold;
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
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  flex: 1;
  min-width: 120px;
}

.chart-navigation button.active {
  background: color-mix(in srgb, var(--color-accent) 80%, black);
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
  background: var(--color-card);
  border-radius: var(--radius-card);
  padding: 30px;
  text-align: center;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--color-accent);
  border-top: 4px solid transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 15px;
}

.variable-select {
  padding: 10px;
  border: 2px solid var(--color-accent);
  border-radius: 10px;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.8);
  width: 100%;
}

.variable-select:focus {
  outline: none;
  border-color: color-mix(in srgb, var(--color-accent) 80%, black);
  box-shadow: 0 0 10px rgba(249, 114, 154, 0.3);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--color-accent);
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
  border: 2px solid var(--color-accent);
  border-radius: 10px;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.8);
  width: 100%;
}

.variable-select:focus {
  outline: none;
  border-color: color-mix(in srgb, var(--color-accent) 80%, black);
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
  background: var(--color-card);
  border-radius: var(--radius-card);
  padding: 20px;
  margin: 20px 0;
  border: 1px solid var(--color-primary);
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
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: var(--radius-button);
  cursor: pointer;
  font-weight: bold;
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
  background: rgba(255, 107, 157, 0.2);
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
  background: var(--color-card);
  border-radius: var(--radius-card);
  padding: 25px;
  width: 400px;
  max-width: 90%;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(var(--glass-blur));
  -webkit-backdrop-filter: blur(var(--glass-blur));
}

.modal-content h3 {
  margin-top: 0;
  margin-bottom: 20px;
  color: var(--color-accent);
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
  color: var(--color-text-main);
}

.month-input {
  padding: 10px;
  border: 2px solid var(--color-accent);
  border-radius: 10px;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.8);
}

.month-input:focus {
  outline: none;
  border-color: color-mix(in srgb, var(--color-accent) 80%, black);
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
  border-radius: var(--radius-button);
  font-size: 16px;
  font-weight: bold;
  cursor: pointer;
}

.confirm-btn {
  background: linear-gradient(45deg, var(--color-accent), var(--color-accent));
  color: white;
}

.cancel-btn {
  background: var(--color-text-secondary);
  color: white;
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
  background: var(--color-accent);
  color: white;
  border: none;
  border-radius: var(--radius-button);
  cursor: pointer;
  font-size: 14px;
  flex: 1;
  min-width: 120px;
}

.chart-navigation button.active {
  background: color-mix(in srgb, var(--color-accent) 80%, black);
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
  }
}

/* 中等屏幕：显示2列 */
@media (min-width: 769px) and (max-width: 1023px) {
  .grid-container {
    display: grid !important;
    grid-template-columns: repeat(2, 1fr);
    gap: 15px;
    padding: 10px;
  }

  .anchor-grid-item {
    margin-bottom: 0;
    height: fit-content;
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

.chart-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  justify-content: center;
  margin: 10px 0;
  padding: 10px;
}

.chart-legend > span {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
  color: var(--color-text-main);
}

.legend-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  object-fit: cover;
  vertical-align: middle;
  margin-right: 4px;
  border: 1px solid var(--color-primary);
}

.legend-dot {
  display: inline-block;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  vertical-align: middle;
  margin-right: 4px;
}
</style>