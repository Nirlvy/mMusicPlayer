<template>
  <n-space vertical>
    <n-slider :value="timeNow" :max="timeMax" />
  </n-space>
</template>

<script setup lang="ts">
import { NSpace, NSlider } from 'naive-ui'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
const timeNow = ref(0)
const timeMax = ref(0)

const mpdApi = async () => {
  invoke('connect', { address: 'localhost', port: 6600 }).then((res) =>
    console.log(res),
  )
}

const WebSocketAPI = () => {
  const webSocket = new WebSocket('wss://127.0.0.1:9002')
  webSocket.onmessage = function (event) {
    let received_msg = JSON.parse(event.data)
    console.log(received_msg)
  }
}

mpdApi()
WebSocketAPI()
</script>

<style scoped>
.n-space {
  padding: 0;
  background: transparent;
}
</style>
