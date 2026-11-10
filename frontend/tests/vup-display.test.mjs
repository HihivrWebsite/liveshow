import test from 'node:test'
import assert from 'node:assert/strict'

import {
  VUP_FIXED_NAME_SUFFIX,
  getAnchorDisplayName
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
