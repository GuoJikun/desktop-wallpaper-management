import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

import Home from '@/views/home.vue'

const routes: Readonly<RouteRecordRaw[]> = [
    {
        path: '/',
        component: Home,
    },
    {
        path: '/store',
        component: () => import('@/views/store.vue'),
    },
    {
        path: '/installed',
        component: () => import('@/views/installed.vue'),
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
})

export default router
