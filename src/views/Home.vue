<template>
  <n-flex vertical item-class="space">
    <n-layout>
      <n-layout-header>
        <HomeHeader />
      </n-layout-header>
      <n-layout class="center" has-sider>
        <n-layout-sider>
          <HomeSlider />
        </n-layout-sider>
        <n-layout-content>
          <router-view />
        </n-layout-content>
      </n-layout>
      <n-layout-footer>
        <HomeFooter />
      </n-layout-footer>
    </n-layout>
  </n-flex>
</template>

<script setup lang="ts">
import HomeHeader from '@/components/HomeHeader.vue'
import HomeSlider from '@/components/HomeSider.vue'
import HomeFooter from '@/components/HomeFooter.vue'
import {
  NFlex,
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NLayoutFooter,
} from 'naive-ui'
</script>

<style scoped>
.n-layout-header,
.n-layout-footer {
  position: fixed;
  width: 100%;
  opacity: 0.8;
  z-index: 1;
  left: 0;
  right: 0;
}
.n-layout-header {
  align-items: center;
  padding: 10px;
}
.center {
  height: 100vh;
}
.n-layout-footer {
  height: 80px;
  padding: 0;
  background-color: #ffffff;
  bottom: 0;
}
</style>
<style>
.n-layout-scroll-container {
  display: flex;
  flex-direction: column;
}
</style>
