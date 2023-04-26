import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { writable } from 'svelte/store'

export const inputPath = writable<string | undefined>()
export const inputError = writable<string | undefined>()
// export const imageDataStore = writable<string | undefined>()

export const inputName = writable<string | undefined>()
export const outputName = writable<string | undefined>()

// watch the filePath for filesystem events; see the `watch` tauri command
inputPath.subscribe((path) => {
  if (path === undefined) return

  invoke('watch', { path }).catch(console.error)
  invoke('output_name', { path })
    .then((path) => {
      inputName.set(path as string)
      outputName.set(path as string)
    })
    .catch(console.error)
})

// listen for filesystem modify/remove events and unset filePath
listen('fs-change', () => {
  inputPath.set(undefined)
  inputError.set('ERROR: The active file was modified or removed - please try again.')
}).catch(console.error)
