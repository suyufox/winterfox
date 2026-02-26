import { createApp } from 'vue'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';

import App from './App.vue'
import router from './router'

// 导入全局SCSS样式
// import '@/assets/styles/global.scss'

const winterfox = createApp(App)

winterfox.use(createPinia())
winterfox.use(router)
winterfox.use(PrimeVue, {
  theme: {
    preset: Aura
  }
});


winterfox.mount('#winterfox')
