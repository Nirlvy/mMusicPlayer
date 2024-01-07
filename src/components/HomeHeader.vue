<template>
  <n-input placeholder="搜索">
    <template #prefix>
      <n-icon :component="Search" size="20" />
    </template>
  </n-input>
</template>

<script setup lang="ts">
import { NInput, NIcon } from 'naive-ui'
import { Search } from '@vicons/carbon'
</script>

<style scoped>
.n-input {
  text-align: left;
  width: 300px;
  background-color: #f2f2f4;
}
</style>
