import { parseYearMonth } from './monthUtils.js'

export class GuildConfigError extends Error {
  constructor(message) {
    super(message)
    this.name = 'GuildConfigError'
    this.status = 400
  }
}

export const UNION_LABELS = Object.freeze({
  VirtuaReal: '维阿',
  PSPlive: 'PSPlive',
  VUP: 'VUP'
})

export const GUILD_FILTERS = Object.freeze({
  all: Object.freeze({
    filter: 'all',
    label: '维阿PSP斗虫榜',
    title: '咿喔io粉丝站维阿PSP斗虫榜',
    unions: Object.freeze(['VirtuaReal', 'PSPlive']),
    uid: null
  }),
  vr: Object.freeze({
    filter: 'vr',
    label: '维阿斗虫榜',
    title: '咿喔io粉丝站维阿斗虫榜',
    unions: Object.freeze(['VirtuaReal']),
    uid: '413748120'
  }),
  psp: Object.freeze({
    filter: 'psp',
    label: 'PSPlive斗虫榜',
    title: '咿喔io粉丝站PSPlive斗虫榜',
    unions: Object.freeze(['PSPlive']),
    uid: '454673997'
  }),
  vup: Object.freeze({
    filter: 'vup',
    label: 'VUP斗虫榜',
    title: '咿喔io粉丝站VUP斗虫榜',
    unions: Object.freeze(['VUP']),
    uid: null
  })
})

export const GUILD_FILTER_OPTIONS = Object.freeze([
  GUILD_FILTERS.all,
  GUILD_FILTERS.vr,
  GUILD_FILTERS.psp,
  GUILD_FILTERS.vup
])

// “全部”仅由 VR 与 PSP 组成，VUP 只能通过独立筛选进入。
export const GUILD_COMPARISON_CONFIG = Object.freeze([
  Object.freeze({
    key: 'vr',
    union: 'VirtuaReal',
    label: 'VirtuaReal',
    shortLabel: 'VR',
    uid: '413748120'
  }),
  Object.freeze({
    key: 'psp',
    union: 'PSPlive',
    label: 'PSPlive',
    shortLabel: 'PSP',
    uid: '454673997'
  })
])

export const SUPPORTED_UNIONS = Object.freeze(
  Object.keys(UNION_LABELS)
)

export const VUP_FIXED_NAME_SUFFIX = '（不是VUP）'
export const VUP_NAME_OVERRIDES = Object.freeze({
  851181: Object.freeze({ suffix: VUP_FIXED_NAME_SUFFIX })
})

export function isSupportedUnion(union) {
  return SUPPORTED_UNIONS.includes(union)
}

export function getGuildFilterConfig(filter = 'all') {
  if (filter === null || filter === undefined || filter === '') {
    return GUILD_FILTERS.all
  }
  return Object.prototype.hasOwnProperty.call(GUILD_FILTERS, filter)
    ? GUILD_FILTERS[filter]
    : null
}

export function requireGuildFilterConfig(filter = 'all') {
  const config = getGuildFilterConfig(filter)
  if (!config) {
    throw new GuildConfigError(`不支持的榜单筛选：${filter}`)
  }
  return config
}

export function requireUnion(union) {
  if (!isSupportedUnion(union)) {
    throw new GuildConfigError(`不支持的工会来源：${union || '未提供'}`)
  }
  return union
}

export function getUnionLabel(union) {
  return UNION_LABELS[union] || union || '未知工会'
}

export function getAnchorDisplayName(anchor, filter = null) {
  if (!anchor) return ''
  const rawName = anchor.anchor_name || anchor.name || ''
  const override = VUP_NAME_OVERRIDES[anchor.room_id]
  const belongsToVupGroup = anchor.union === 'VUP' &&
    (filter === null || filter === undefined || filter === 'vup')

  if (!override || !belongsToVupGroup || rawName.endsWith(override.suffix)) {
    return rawName
  }
  return `${rawName}${override.suffix}`
}

function formatYearMonthParts(yearMonth) {
  const { year, month } = parseYearMonth(yearMonth)
  return `${year}年${String(month).padStart(2, '0')}月`
}

export function getGuildBoardTitle(
  filter = 'all',
  { month = null, range = null, detailSuffix = '', siteSuffix = '' } = {}
) {
  const config = requireGuildFilterConfig(filter)
  let suffix = detailSuffix

  if (range) {
    suffix = `_${formatYearMonthParts(range.startMonth)}-${formatYearMonthParts(range.endMonth)}累计数据${detailSuffix}`
  } else if (month) {
    suffix = `_${formatYearMonthParts(month)}记录数据${detailSuffix}`
  }

  return `${config.title}${suffix}${siteSuffix}`
}
