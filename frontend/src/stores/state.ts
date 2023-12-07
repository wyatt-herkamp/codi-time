import { defineStore } from 'pinia'
import { ref, type Ref } from 'vue'
import http from '@/http'
import { type State } from '@/types'

export const stateStore = defineStore(
  'stateStore',
  () => {
    const configuration: Ref<State | undefined> = ref(undefined)
    async function load(): Promise<State | undefined> {
      return await http
        .get<State>('/api/state')
        .then((response) => {
          console.log(`Configuration From Backend: ${JSON.stringify(response.data)}`)
          configuration.value = response.data
          return response.data
        })
        .catch(() => {
          configuration.value = undefined
          return undefined
        })
    }

    return { configuration, load }
  },
  {
    persist: true
  }
)
