import { createRouter, createWebHistory } from 'vue-router'

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/songs',
      name: 'home',
      component: () => import('@/views/Home.vue'),
      children: [
        {
          path: '/songs',
          name: 'songs',
          component: () => import('@/views/Songs.vue'),
        },
        {
          path: '/albums',
          name: 'albums',
          component: () => import('@/views/Albums.vue'),
        },
        {
          path: '/artists',
          name: 'artists',
          component: () => import('@/views/Artists.vue'),
        },
        {
          path: '/folders',
          name: 'folders',
          component: () => import('@/views/Folders.vue'),
        },
        {
          path: '/playlists',
          name: 'playlists',
          component: () => import('@/views/Playlists.vue'),
        },
        {
          path: '/settings',
          name: 'settings',
          component: () => import('@/views/Settings.vue'),
        },
      ],
    },
    {
      path: '/:catchAll(.*)',
      component: () => import('@/views/Home.vue'),
    },
  ],
})
