import { defineStore } from 'pinia'

export const useStatusStore = defineStore('status', {
  state: () => ({
    state: 0,
    random: 1,
    repeat: 0,
    single: 0,
    consume: 0,
  }),
  persist: true,
})
