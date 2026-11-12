import test from 'node:test'
import assert from 'node:assert/strict'

import {
  GUILD_FILTER_OPTIONS,
  VUP_FIXED_NAME_SUFFIX,
  getAnchorDisplayName,
  requireGuildFilterConfig
} from '../src/utils/guildConfig.js'

test('梦魇Tsuki 按稳定 room_id 追加固定后缀，不依赖动态名称', () => {
  assert.equal(
    getAnchorDisplayName(
      { room_id: 851181, union: 'VUP', anchor_name: '梦魇Tsuki' },
      'vup'
    ),
    `梦魇Tsuki${VUP_FIXED_NAME_SUFFIX}`
  )
  assert.equal(
    getAnchorDisplayName(
      { room_id: 851181, union: 'VUP', anchor_name: '完全变化的名字' },
      'vup'
    ),
    `完全变化的名字${VUP_FIXED_NAME_SUFFIX}`
  )
})

test('字符串 room_id 同样命中稳定身份，且已有后缀不会重复', () => {
  assert.equal(
    getAnchorDisplayName(
      { room_id: '851181', union: 'VUP', anchor_name: '梦魇Tsuki' },
      null
    ),
    `梦魇Tsuki${VUP_FIXED_NAME_SUFFIX}`
  )
  assert.equal(
    getAnchorDisplayName(
      {
        room_id: 851181,
        union: 'VUP',
        anchor_name: `梦魇Tsuki${VUP_FIXED_NAME_SUFFIX}`
      },
      'vup'
    ),
    `梦魇Tsuki${VUP_FIXED_NAME_SUFFIX}`
  )
})

test('其他 VUP 及非 VUP 主播不会误加后缀', () => {
  assert.equal(
    getAnchorDisplayName(
      { room_id: 123456, union: 'VUP', anchor_name: '梦魇Tsuki' },
      'vup'
    ),
    '梦魇Tsuki'
  )
  assert.equal(
    getAnchorDisplayName(
      { room_id: 851181, union: 'VirtuaReal', anchor_name: '梦魇Tsuki' },
      'vr'
    ),
    '梦魇Tsuki'
  )
})

test('everything 筛选置顶且语义为三源全部', () => {
  assert.equal(GUILD_FILTER_OPTIONS[0].filter, 'everything')
  const config = requireGuildFilterConfig('everything')
  assert.equal(config.label, '全部')
  assert.equal(config.title, '咿喔io粉丝站维阿PSP斗虫榜')
  assert.deepEqual([...config.unions], ['VirtuaReal', 'PSPlive', 'VUP'])
})

test('everything 与 all 一样不触发 VUP 后缀', () => {
  assert.equal(
    getAnchorDisplayName(
      { room_id: 851181, union: 'VUP', anchor_name: '梦魇Tsuki' },
      'everything'
    ),
    '梦魇Tsuki'
  )
})
