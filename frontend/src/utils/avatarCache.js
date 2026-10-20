import DEFAULT_AVATARS from './defaultAvatars.js'

let avatarStore = { ...DEFAULT_AVATARS }
let refreshPromise = null

// 默认头像始终作为离线回退；在线批量刷新只覆盖同键的最新 WebP data URL。
export function fetchAllAvatars() {
  if (refreshPromise) return refreshPromise
  refreshPromise = fetch('/gift/avatars/batch')
    .then(resp => {
      if (!resp.ok) throw new Error(`avatar batch HTTP ${resp.status}`)
      return resp.json()
    })
    .then(data => {
      if (data && data.avatars && typeof data.avatars === 'object') {
        Object.assign(avatarStore, data.avatars)
      }
      return avatarStore
    })
    .catch(() => avatarStore)
    .finally(() => {
      refreshPromise = null
    })
  return refreshPromise
}

export function getAvatarSync(roomId) {
  return avatarStore[String(roomId)] || ''
}

export async function getAvatar(roomId) {
  const base64 = avatarStore[String(roomId)]
  if (!base64) return null
  return new Promise(resolve => {
    const img = new Image()
    img.onload = () => resolve(img)
    img.onerror = () => resolve(null)
    img.src = base64
  })
}

export async function getAvatarByUid(uid) {
  const base64 = avatarStore[`uid_${uid}`]
  if (!base64) return null
  return new Promise(resolve => {
    const img = new Image()
    img.onload = () => resolve(img)
    img.onerror = () => resolve(null)
    img.src = base64
  })
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
