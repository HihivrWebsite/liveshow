import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import apiClient, { anchorAPI } from '../src/api/index.js'
import { getMonthRange } from '../src/utils/monthUtils.js'
import DEFAULT_AVATARS from '../src/utils/defaultAvatars.js'

const require = createRequire(import.meta.url)
const { parse } = require('@vue/compiler-sfc')

test('online avatar batch overlays the offline JPEG defaults with WebP data URLs', async () => {
  const { fetchAllAvatars, getAvatarSync } = await import('../src/utils/avatarCache.js')
  const key = Object.keys(DEFAULT_AVATARS)[0]
  const fallback = getAvatarSync(key)
  assert.match(fallback, /^data:image\/jpeg;base64,/)

  const previousFetch = globalThis.fetch
  let calls = 0
  globalThis.fetch = async url => {
    calls++
    assert.equal(url, '/gift/avatars/batch')
    return {
      ok: true,
      async json() {
        return { avatars: { [key]: 'data:image/webp;base64,UklGRg==' }, count: 1 }
      }
    }
  }
  try {
    const first = fetchAllAvatars()
    const second = fetchAllAvatars()
    assert.equal(first, second)
    await Promise.all([first, second])
  } finally {
    globalThis.fetch = previousFetch
  }
  assert.equal(calls, 1)
  assert.equal(getAvatarSync(key), 'data:image/webp;base64,UklGRg==')
})

test('range invalidation is serial, deduplicated and includes source identity', async () => {
  const requests = []
  let active = 0
  apiClient.defaults.adapter = async config => {
    assert.equal(++active, 1)
    requests.push({ url: config.url, method: config.method, params: config.params })
    await new Promise(resolve => setTimeout(resolve, 1))
    active--
    return { data: { ok: true }, status: 200, statusText: 'OK', headers: {}, config }
  }
  await anchorAPI.clearLiveSessionsForRange([
    { room_id: 1, union: 'VirtuaReal' }, { room_id: 1, union: 'VirtuaReal' },
    { room_id: 1, union: 'PSPlive' }
  ], ['202609', '202610'])
  assert.equal(requests.length, 4)
  assert.ok(requests.every(r => r.method === 'post' && r.url === '/cache/clear'))
  assert.deepEqual(requests[0].params, { room_id: 1, month: '202609', union: 'VirtuaReal' })
  assert.deepEqual(requests[3].params, { room_id: 1, month: '202610', union: 'PSPlive' })
})

test('a rejected invalidation stops the range and surfaces its error', async () => {
  let calls = 0
  apiClient.defaults.adapter = async config => {
    calls++
    return { data: { ok: false, message: 'invalid month' }, status: 200, statusText: 'OK', headers: {}, config }
  }
  await assert.rejects(anchorAPI.clearLiveSessionsForRange([{ room_id: 1 }], ['bad', '202610']), /invalid month/)
  assert.equal(calls, 1)
})

function component(name, api) {
  const file = new URL(`../src/components/${name}.vue`, import.meta.url)
  const { descriptor } = parse(readFileSync(file, 'utf8'))
  const script = descriptor.script.content.replace(/^import .*$/gm, '').replace('export default', 'return')
  return new Function('anchorAPI', 'Chart', 'registerables', 'getMonthRange', 'getAvatar', 'scaleAvatar', 'HeaderSection', 'FooterSection', script)(
    api, { register() {} }, [], getMonthRange, () => null, () => null, {}, {}
  )
}

for (const name of ['AnchorBattle', 'RankComparison']) {
  test(`${name} refresh invalidates the server before fetching and clears all local ranges`, async () => {
    const events = []
    const api = { async clearLiveSessionsForRange(anchors, months) {
      events.push('clear')
      assert.equal(anchors[0].room_id, 1)
      assert.deepEqual(months, ['202609', '202610'])
    } }
    const context = {
      selectedAnchors: [{ room_id: 1 }], startDate: '2026-09', endDate: '2026-10',
      dataCache: new Map([['current', {}], ['other-range', {}]]),
      sessionsData: [Array(10).fill({})],
      getCacheKey: () => 'current',
      async fetchBattleData() { events.push('fetch') },
      async renderBattleChart() { events.push('render') }
    }
    const previousAlert = globalThis.alert
    globalThis.alert = () => {}
    try { await component(name, api).methods.refreshData.call(context) }
    finally { globalThis.alert = previousAlert }
    assert.deepEqual(events.slice(0, 2), ['clear', 'fetch'])
    assert.equal(context.dataCache.size, 0)
    assert.equal(context.isRefreshing, false)
  })

  test(`${name} does not refetch or discard local data when server invalidation fails`, async () => {
    let fetched = false
    const context = {
      selectedAnchors: [{ room_id: 1 }], startDate: '2026-09', endDate: '2026-10',
      dataCache: new Map([['current', {}]]),
      async fetchBattleData() { fetched = true }
    }
    const api = { async clearLiveSessionsForRange() { throw new Error('disk unavailable') } }
    const previousAlert = globalThis.alert
    globalThis.alert = () => {}
    try { await component(name, api).methods.refreshData.call(context) }
    finally { globalThis.alert = previousAlert }
    assert.equal(fetched, false)
    assert.equal(context.dataCache.size, 1)
    assert.equal(context.isRefreshing, false)
  })
}
