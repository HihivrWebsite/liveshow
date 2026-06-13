const CACHE_PREFIX = 'avatar_'
const CACHE_VERSION = 'v1'

function getFromStorage(key) {
  try {
    const data = localStorage.getItem(CACHE_PREFIX + key)
    if (!data) return null
    const parsed = JSON.parse(data)
    if (parsed.version !== CACHE_VERSION) return null
    return parsed.base64
  } catch { return null }
}

function saveToStorage(key, base64) {
  try {
    localStorage.setItem(CACHE_PREFIX + key, JSON.stringify({
      version: CACHE_VERSION,
      base64,
      updated: Date.now()
    }))
  } catch (e) {}
}

function base64ToImage(base64) {
  return new Promise((resolve) => {
    const img = new Image()
    img.onload = () => resolve(img)
    img.onerror = () => resolve(null)
    img.src = base64
  })
}

async function fetchAvatar(url) {
  try {
    const img = new Image()
    img.crossOrigin = 'anonymous'
    img.src = url
    await new Promise((resolve) => {
      img.onload = resolve
      img.onerror = resolve
      setTimeout(resolve, 5000)
    })
    if (!img.complete || img.naturalWidth === 0) return null
    const c = document.createElement('canvas')
    c.width = img.naturalWidth
    c.height = img.naturalHeight
    c.getContext('2d').drawImage(img, 0, 0)
    return c.toDataURL('image/jpeg', 0.8)
  } catch { return null }
}

async function refreshAvatar(key, url) {
  const base64 = await fetchAvatar(url)
  if (base64) saveToStorage(key, base64)
}

export async function getAvatar(roomId) {
  const key = String(roomId)
  const cached = getFromStorage(key)
  if (cached) {
    refreshAvatar(key, `/gift/avatar_proxy?room_id=${roomId}`)
    return base64ToImage(cached)
  }
  for (let i = 0; i < 3; i++) {
    const base64 = await fetchAvatar(`/gift/avatar_proxy?room_id=${roomId}`)
    if (base64) {
      saveToStorage(key, base64)
      return base64ToImage(base64)
    }
    if (i < 2) await new Promise(r => setTimeout(r, 1000))
  }
  return null
}

export async function getAvatarByUid(uid) {
  const key = `uid_${uid}`
  const cached = getFromStorage(key)
  if (cached) {
    refreshAvatar(key, `/gift/avatar_proxy?uid=${uid}`)
    return base64ToImage(cached)
  }
  for (let i = 0; i < 3; i++) {
    const base64 = await fetchAvatar(`/gift/avatar_proxy?uid=${uid}`)
    if (base64) {
      saveToStorage(key, base64)
      return base64ToImage(base64)
    }
    if (i < 2) await new Promise(r => setTimeout(r, 1000))
  }
  return null
}

export async function preloadAllAvatars(anchors) {
  const promises = anchors.map(anchor => getAvatar(anchor.room_id))
  return Promise.all(promises)
}

export function scaleAvatar(img, size) {
  if (!img || !img.complete || img.naturalWidth === 0) return null
  const c = document.createElement('canvas')
  c.width = size; c.height = size
  const ctx = c.getContext('2d')
  ctx.beginPath()
  ctx.arc(size / 2, size / 2, size / 2, 0, Math.PI * 2)
  ctx.closePath(); ctx.clip()
  ctx.drawImage(img, 0, 0, size, size)
  return c
}
