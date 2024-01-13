<template>
  <n-flex vertical>
    <n-slider
      :value="timeNow"
      :max="timeMax"
      :format-tooltip="tooltip"
      :on-dragstart="dragstart"
      :on-dragend="dragend"
    />
    <n-flex justify="space-around">
      <n-flex justify="center">
        <n-avatar
          :size="50"
          src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
        />
        <n-flex vertical>
          <p class="title">{{ song?.title ? song.title : song?.file }}</p>
          <p class="artists">{{ song?.artist ? song.artist : '' }}</p>
        </n-flex>
      </n-flex>
      <n-flex justify="space-around" style="width: 15%">
        <n-button text @click="setState('Prev')">
          <template #icon>
            <n-icon size="20"><play-skip-back /></n-icon>
          </template>
        </n-button>
        <n-button text @click="setState('Toggle')">
          <template #icon>
            <n-icon v-if="status?.state != 'play'" size="40"><play /></n-icon>
            <n-icon v-else size="40"><pause /></n-icon>
          </template>
        </n-button>
        <n-button text @click="setState('Next')">
          <template #icon>
            <n-icon size="20"><play-skip-forward /></n-icon>
          </template>
        </n-button>
      </n-flex>
      <n-flex justify="center">
        <n-button text>
          <template #icon>
            <n-icon size="20"><repeat /></n-icon>
          </template>
        </n-button>
        <n-button text>
          <template #icon>
            <n-icon size="20"><shuffle /></n-icon>
          </template>
        </n-button>
      </n-flex>
    </n-flex>
  </n-flex>
</template>

<script setup lang="ts">
import { NFlex, NSlider, NAvatar, useMessage, NButton, NIcon } from 'naive-ui'
import {
  Play,
  Pause,
  PlaySkipBack,
  PlaySkipForward,
  Shuffle,
  Repeat,
} from '@vicons/ionicons5'
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import WebSocket, { Message, MessageKind } from '@tauri-apps/plugin-websocket'

const status = ref<MpdStatus>()
const song = ref<Song>()
const ws = ref<null | WebSocket>(null)

const timeNow = ref(0)
const timeMax = ref(0)
const drag = ref(false)

const message = useMessage()

onMounted(async () => {
  invoke('connect', { address: 'localhost', port: 6600 }).then((res) => {
    if (res === 'mpd connect success') {
      websocket()
      getCurrentSong()
    } else {
      message.error(res as string)
    }
  })
})

onUnmounted(() => {
  ws.value?.disconnect()
  ws.value = null
})

const websocket = async () => {
  ws.value = await WebSocket.connect('ws://127.0.0.1:9002')
  ws.value.addListener(getStatus)
}

const getStatus = (res: Message) => {
  const message = res as MessageKind<'Text', string>
  status.value = JSON.parse(message.data)
  if (status.value?.time) {
    if (timeNow.value > status.value.time[0].secs) getCurrentSong()
    if (!drag.value) timeNow.value = status.value.time[0].secs
    timeMax.value = status.value.time[1].secs
  }
}

const getCurrentSong = async () => {
  await invoke('get_currentsong').then((res) => {
    song.value = JSON.parse(res as string)
  })
}

const tooltip = () => {
  return (
    Math.floor(timeNow.value / 60) +
    ':' +
    (timeNow.value % 60).toString().padStart(2, '0')
  )
}

const setState = (state: setState) => {
  invoke('set_playstate', { state: state }).then((res) => {
    const r = res as string
    if (r.length != 0) message.error(res as string)
  })
}

const dragstart = () => {
  drag.value = true
}

const dragend = () => {}
</script>

<style scoped>
.title {
  font-size: 18px;
  height: 18px;
  font-weight: bold;
}
.artists {
  font-size: 14px;
  height: 14px;
  text-align: left;
}
</style>
