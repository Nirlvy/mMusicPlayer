import { createApp } from 'vue'
import '@/assets/main.css'
import App from './App.vue'

import { router } from './router/index.ts'

import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

const pinia = createPinia()
const app = createApp(App)

app.use(pinia.use(piniaPluginPersistedstate)).use(router).mount('#app')
