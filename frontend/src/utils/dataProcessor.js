/**
 * 数据处理工具函数
 */

/**
 * 格式化货币数值
 * @param {number|string} value - 数值
 * @returns {string} 格式化后的货币字符串
 */
export const formatCurrency = (value) => {
  const num = parseFloat(value || 0)
  return isNaN(num) ? '0.00' : num.toFixed(2)
}

/**
 * 格式化数字
 * @param {number|string} value - 数值
 * @returns {string} 格式化后的数字字符串
 */
export const formatNumber = (value) => {
  const num = parseInt(value || 0)
  return isNaN(num) ? '0' : new Intl.NumberFormat().format(num)
}

/**
 * 计算百分比
 * @param {number} value - 当前值
 * @param {number} total - 总值
 * @returns {string} 百分比字符串
 */
export const calculatePercentage = (value, total) => {
  if (!total || total <= 0) return '0.0'
  const percentage = (parseFloat(value || 0) / total) * 100
  return percentage.toFixed(1)
}

/**
 * 计算持续时间
 * @param {string} startTime - 开始时间
 * @param {string} endTime - 结束时间
 * @returns {string} 持续时间字符串
 */
export const calculateDuration = (startTime, endTime) => {
  if (!startTime || !endTime) return 'N/A'

  try {
    const start = new Date(startTime.replace(' ', 'T'))
    const end = new Date(endTime.replace(' ', 'T'))

    const diffMs = end.getTime() - start.getTime()
    const diffMins = Math.round(diffMs / 60000)

    const hours = Math.floor(diffMins / 60)
    const minutes = diffMins % 60

    return `${diffMins}分钟 (${hours}小时${minutes}分钟)`
  } catch (e) {
    console.error('计算持续时间时出错:', e)
    return 'N/A'
  }
}

/**
 * 计算总收入
 * @param {Object} item - 包含收入数据的对象
 * @returns {number} 总收入
 */
export const calculateTotalRevenue = (item) => {
  const gift = parseFloat(item.gift) || 0
  const guard = parseFloat(item.guard) || 0
  const superChat = parseFloat(item.super_chat) || 0
  return gift + guard + superChat
}

/**
 * 防抖函数
 * @param {Function} func - 要防抖的函数
 * @param {number} delay - 延迟时间（毫秒）
 * @returns {Function} 防抖后的函数
 */
export const debounce = (func, delay) => {
  let timeoutId
  return (...args) => {
    clearTimeout(timeoutId)
    timeoutId = setTimeout(() => func.apply(this, args), delay)
  }
}

/**
 * 节流函数
 * @param {Function} func - 要节流的函数
 * @param {number} limit - 限制时间（毫秒）
 * @returns {Function} 节流后的函数
 */
export const throttle = (func, limit) => {
  let inThrottle
  return function() {
    const args = arguments
    const context = this
    if (!inThrottle) {
      func.apply(context, args)
      inThrottle = true
      setTimeout(() => inThrottle = false, limit)
    }
  }
}

/**
 * 日期格式化
 * @param {Date|string} date - 日期对象或字符串
 * @param {string} format - 格式字符串
 * @returns {string} 格式化后的日期字符串
 */
export const formatDate = (date, format = 'YYYY-MM-DD HH:mm:ss') => {
  const d = new Date(date)
  
  const year = d.getFullYear()
  const month = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  const hours = String(d.getHours()).padStart(2, '0')
  const minutes = String(d.getMinutes()).padStart(2, '0')
  const seconds = String(d.getSeconds()).padStart(2, '0')

  return format
    .replace('YYYY', year)
    .replace('MM', month)
    .replace('DD', day)
    .replace('HH', hours)
    .replace('mm', minutes)
    .replace('ss', seconds)
}