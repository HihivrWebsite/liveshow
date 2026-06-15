export function parseYearMonth(ymString) {
  if (ymString.includes('-')) {
    const [year, month] = ymString.split('-').map(Number)
    return { year, month }
  }
  const year = parseInt(ymString.substring(0, 4))
  const month = parseInt(ymString.substring(4, 6))
  return { year, month }
}

export function getMonthRange(startYm, endYm) {
  const start = parseYearMonth(startYm)
  const end = parseYearMonth(endYm)
  const months = []
  let current = start.year * 12 + start.month
  const last = end.year * 12 + end.month
  while (current <= last) {
    const y = Math.floor((current - 1) / 12)
    const m = ((current - 1) % 12) + 1
    months.push(`${y}${String(m).padStart(2, '0')}`)
    current++
  }
  return months
}

export function getCurrentYearMonth() {
  const now = new Date()
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`
}

export const MIN_MONTH = '2025-08'
