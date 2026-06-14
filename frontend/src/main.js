import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { vSquircle } from '@/directives/vSquircle'
import './assets/styles/main.css'

const app = createApp(App)
app.directive('squircle', vSquircle)
app.use(router)
app.mount('#app')
