import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import APIDocsVue from '@/views/APIDocs.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/api-docs',
      name: 'APIDocs',
      component: APIDocsVue
    }
  ]
})

export default router
