import {createRouter, createWebHistory} from 'vue-router'
import klepView from '@/views/klepView.vue'
import batchView from "@/views/batchView.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'Single Klep',
            component: klepView,
        },
        {
            path: '/batch',
            name: 'Klep W/ Chooms',
            component: batchView,
        }
    ]
})

export default router
