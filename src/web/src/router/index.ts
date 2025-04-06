import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      meta: { title: '系统概览' }
    },
    {
      path: '/software',
      name: 'software',
      component: () => import('../views/SoftwareView.vue'),
      meta: { title: '软件安装' }
    },
    {
      path: '/config-files',
      name: 'config-files',
      component: () => import('../views/ConfigFilesView.vue'),
      meta: { title: '配置文件' }
    },
    {
      path: '/system',
      name: 'system',
      component: () => import('../views/SystemView.vue'),
      meta: { title: '系统信息' }
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
  ],
})

export default router
