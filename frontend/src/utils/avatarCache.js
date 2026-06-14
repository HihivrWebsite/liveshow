import DEFAULT_AVATARS from './defaultAvatars'

let avatarStore = { ...DEFAULT_AVATARS }

// 服务器获取功能已禁用，仅使用嵌入的默认头像
// export function fetchAllAvatars() {
//   fetch('/gift/avatars/batch')
//     .then(resp => resp.json())
//     .then(data => {
//       if (data.avatars) Object.assign(avatarStore, data.avatars)
//     })
//     .catch(() => {})
// }

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
