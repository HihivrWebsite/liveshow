import { createRouter, createWebHistory } from 'vue-router'
import AnchorList from '@/components/AnchorList.vue'
import LiveSessions from '@/components/LiveSessions.vue'
import SuperChatDetail from '@/components/SuperChatDetail.vue'
import ErrorPage from '@/components/ErrorPage.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: AnchorList
  },
  {
    path: '/by-month',
    name: 'ByMonth',
    component: AnchorList
  },
  {
    path: '/live-sessions',
    name: 'LiveSessions',
    component: LiveSessions
  },
  {
    path: '/superchat-detail',
    name: 'SuperChatDetail',
    component: SuperChatDetail
  },
  {
    path: '/error',
    name: 'Error',
    component: ErrorPage,
    props: true
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: ErrorPage,
    props: { errorMessage: '页面未找到' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router