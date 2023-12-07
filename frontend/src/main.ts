import 'normalize.css/normalize.css'
import './assets/styles/main.scss'
import 'vue-final-modal/style.css'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import Notifications from '@kyvg/vue3-notification'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createVfm } from 'vue-final-modal'
import { createMetaManager } from 'vue-meta'

import App from './App.vue'
import router from './router'

const app = createApp(App)
const vfm = createVfm()
app.use(vfm)
app.use(router)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(createMetaManager())
app.use(pinia)

app.use(router)
app.use(Notifications)

app.mount('#app')
