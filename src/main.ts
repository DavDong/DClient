import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './styles/cyberpunk.css'
import { applyTheme, getStoredThemeId } from './themes'

const app = createApp(App)
app.use(createPinia())
app.mount('#app')

// 启动时应用保存的主题
getStoredThemeId().then(id => applyTheme(id))
