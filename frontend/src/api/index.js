// api/index.js
import axios from 'axios'

// 创建axios实例
const apiClient = axios.create({
  baseURL: '/', // 直接调用根路径，后端会处理路由
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// 请求拦截器
apiClient.interceptors.request.use(
  config => {
    // 在发送请求之前做些什么
    return config
  },
  error => {
    // 对请求错误做些什么
    console.error('Request Error:', error)
    return Promise.reject(error)
  }
)

// 响应拦截器
apiClient.interceptors.response.use(
  response => {
    // 对响应数据做点什么
    return response.data
  },
  error => {
    // 对响应错误做点什么
    console.error('API Error:', error)
    return Promise.reject(error)
  }
)

export default apiClient

// API方法
export const anchorAPI = {
  // 获取主播列表
  getAnchors: (filter = 'all', month = null) => {
    const params = new URLSearchParams()
    if (filter) params.append('filter', filter)
    if (month) params.append('month', month)

    const queryString = params.toString()
    return apiClient.get(`/gift${queryString ? '?' + queryString : ''}`)
  },

  // 获取按月份主播数据
  getAnchorsByMonth: (month, filter = 'all') => {
    const params = new URLSearchParams()
    params.append('month', month)
    params.append('filter', filter)

    return apiClient.get(`/gift/by_month?${params.toString()}`)
  },

  // 获取直播会话详情
  getLiveSessions: (roomId, union, month) => {
    const params = new URLSearchParams()
    params.append('room_id', roomId)
    params.append('union', union)
    if (month) params.append('month', month)

    return apiClient.get(`/gift/live_sessions?${params.toString()}`)
  },

  // 获取SC历史数据
  getSuperChatHistory: (roomId, union = 'VirtuaReal') => {
    const params = new URLSearchParams()
    params.append('room_id', roomId)
    params.append('union', union)

    console.log('调用SC历史数据API:', `/gift/sc?${params.toString()}`) // 添加调试日志
    return apiClient.get(`/gift/sc?${params.toString()}`)
  }
}