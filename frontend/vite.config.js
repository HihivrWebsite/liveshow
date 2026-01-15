import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src')
    }
  },
  server: {
    port: 3000,
    proxy: {
      '/gift': {
        target: 'http://localhost:2992',
        changeOrigin: true,
        rewrite: (path) => path
      },
      '/api': {
        target: 'http://localhost:2992',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, '')
      }
    }
  },
  build: {
    outDir: '../rust_backend/dist', // 构建到Rust后端目录
    emptyOutDir: true
  }
})