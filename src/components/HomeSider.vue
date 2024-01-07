<template>
  <n-menu :options="menuOptions" :indent="20" />
</template>

<script setup lang="ts">
import { h, Component } from 'vue'
import { NMenu, NIcon } from 'naive-ui'
import type { MenuOption } from 'naive-ui'
import { RouterLink } from 'vue-router'
import {
  MusicalNotes,
  Albums,
  People,
  FolderOpen,
  List,
  Settings,
} from '@vicons/ionicons5'

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}
const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'songs',
          },
        },
        { default: () => '歌曲' },
      ),
    key: 'songs',
    icon: renderIcon(MusicalNotes),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'albums',
          },
        },
        { default: () => '专辑' },
      ),
    key: 'albums',
    icon: renderIcon(Albums),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'artists',
          },
        },
        { default: () => '艺术家' },
      ),
    key: 'artists',
    icon: renderIcon(People),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'folders',
          },
        },
        { default: () => '文件夹' },
      ),
    key: 'folders',
    icon: renderIcon(FolderOpen),
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'playlists',
          },
        },
        { default: () => '播放列表' },
      ),
    key: 'playlists',
    icon: renderIcon(List),
  },
  {
    type: 'divider',
  },

  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'settings',
          },
        },
        { default: () => '设置' },
      ),
    key: 'settings',
    icon: renderIcon(Settings),
  },
]
</script>

<style scoped></style>
<style>
.n-layout-sider-scroll-container {
  display: flex;
  align-items: center;
  text-align: left;
}
</style>
