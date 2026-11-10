// api/index.js
import axios from 'axios'
import {
  GuildConfigError,
  requireGuildFilterConfig,
  requireUnion
} from '../utils/guildConfig.js'

// 创建axios实例
const apiClient = axios.create({
  baseURL: '/', // 直接调用根路径，后端会处理路由
  timeout: 30000, // 30 秒超时，应对慢响应 API
  headers: {
    'Content-Type': 'application/json'
  }
})

export class ApiError extends Error {
  constructor(message, { status = 0, code = 'API_ERROR', cause = null } = {}) {
    super(message)
    this.name = 'ApiError'
    this.status = status
    this.code = code
    this.cause = cause
  }
}

const STATUS_MESSAGES = Object.freeze({
  400: '请求参数无效，请检查筛选、月份或工会来源',
  404: '请求的数据不存在，请检查房间、月份或工会来源',
  429: '请求过于频繁，请稍后重试',
  503: '服务暂时不可用，请稍后重试'
})

function backendMessage(data) {
  if (!data) return ''
  if (typeof data === 'string') return data.trim()
  for (const key of ['message', 'error', 'detail']) {
    const value = data[key]
    if (typeof value === 'string' && value.trim()) return value.trim()
  }
  return ''
}

export function getApiErrorMessage(error, fallback = '请求失败，请稍后重试') {
  if (error instanceof GuildConfigError || error instanceof ApiError) {
    return error.message
  }
  const status = error?.response?.status
  return backendMessage(error?.response?.data) ||
    (status && STATUS_MESSAGES[status]) ||
    error?.message ||
    fallback
}

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
    const status = error?.response?.status
    const message = backendMessage(error?.response?.data) ||
      (status && STATUS_MESSAGES[status]) ||
      (error?.request ? '网络连接失败，请检查网络连接' : error?.message || '请求失败，请稍后重试')
    const normalized = new ApiError(message, {
      status: status || 0,
      code: status ? `HTTP_${status}` : 'NETWORK_ERROR',
      cause: error
    })
    console.error('API Error:', normalized)
    return Promise.reject(normalized)
  }
)

export default apiClient

// API方法
export const anchorAPI = {
  // 获取主播列表
  getAnchors: (filter = 'all', month = null) => {
    requireGuildFilterConfig(filter)
    const params = new URLSearchParams()
    if (filter) params.append('filter', filter)
    if (month) params.append('month', month)

    const queryString = params.toString()
    return apiClient.get(`/gift${queryString ? '?' + queryString : ''}`)
  },

  // 获取按月份主播数据
  getAnchorsByMonth: (month, filter = 'all') => {
    requireGuildFilterConfig(filter)
    const params = new URLSearchParams()
    params.append('month', month)
    params.append('filter', filter)

    return apiClient.get(`/gift/by_month?${params.toString()}`)
  },

  // 获取直播会话详情
  getLiveSessions: (roomId, union, month) => {
    requireUnion(union)
    const params = new URLSearchParams()
    params.append('room_id', roomId)
    params.append('union', union)
    if (month) params.append('month', month)

    return apiClient.get(`/gift/live_sessions?${params.toString()}`)
  },

  // 获取SC历史数据
  getSuperChatHistory: (roomId, union) => {
    requireUnion(union)
    const params = new URLSearchParams()
    params.append('room_id', roomId)
    params.append('union', union)

    console.log('调用SC历史数据API:', `/gift/sc?${params.toString()}`) // 添加调试日志
    return apiClient.get(`/gift/sc?${params.toString()}`)
  },

  // 获取缓存统计信息
  getCacheStats: () => {
    return apiClient.get('/cache/stats')
  },

  // Invalidate server memory, disk and HTTP caches before requesting fresh data.
  clearCache: async (roomId, month, union) => {
    if (union !== undefined && union !== null && union !== '') {
      requireUnion(union)
    }
    const response = await apiClient.post('/cache/clear', null, {
      params: { room_id: roomId, month, union }
    })
    if (!response.ok) throw new Error(response.message || '清除缓存失败')
    return response
  },

  clearLiveSessionsForRange: async (anchors, months) => {
    const seen = new Set()
    for (const anchor of anchors) {
      const union = anchor.union === undefined || anchor.union === null || anchor.union === ''
        ? undefined
        : requireUnion(anchor.union)
      for (const month of months) {
        const key = `${union}:${anchor.room_id}:${month}`
        if (seen.has(key)) continue
        seen.add(key)
        await anchorAPI.clearCache(anchor.room_id, month, union)
      }
    }
  },

  // 获取包含粉丝计算的直播会话（完整模式）
  getLiveSessionsWithFans: (roomId, union, month) => {
    requireUnion(union)
    const params = new URLSearchParams()
    params.append('room_id', roomId)
    params.append('union', union)
    if (month) params.append('month', month)
    return apiClient.get(`/gift/live_sessions_with_fans?${params.toString()}`)
  },

  // 获取粉丝数快照 - 支持缓存机制
  getAttention: (roomId, month, union) => {
    requireUnion(union)
    const params = new URLSearchParams()
    params.append('room_id', roomId)
    if (month) params.append('month', month)
    params.append('union', union)  // 添加union参数以支持缓存键生成

    return apiClient.get(`/gift/attention?${params.toString()}`)
  },

  // 粉丝重合度查询 - 分析两个主播粉丝重叠情况
  getFanOverlap: (roomIdA, roomIdB) => {
    const params = new URLSearchParams()
    params.append('room_id_a', roomIdA)
    params.append('room_id_b', roomIdB)

    return apiClient.get(`/gift/fans_overlap?${params.toString()}`)
  },

  // 粉丝重合度批量查询 - 支持3-5个主播，返回所有两两配对
  getFanOverlapMulti: (roomIds) => {
    const params = new URLSearchParams()
    params.append('room_ids', roomIds.join(','))

    return apiClient.get(`/gift/fans_overlap?${params.toString()}`)
  },

}
