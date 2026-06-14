let avatarStore = {}
let avatarPromise = null

export function fetchAllAvatars() {
  if (avatarPromise) return avatarPromise
  avatarPromise = fetch('/gift/avatars/batch')
    .then(resp => resp.json())
    .then(data => { avatarStore = data.avatars || {} })
    .catch(() => {})
  return avatarPromise
}

export function getAvatarSync(roomId) {
  return avatarStore[String(roomId)] || ''
}

export async function getAvatar(roomId) {
  await fetchAllAvatars()
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
  await fetchAllAvatars()
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
