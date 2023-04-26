<script lang="ts">
  // import { invoke } from '@tauri-apps/api'
  import { open, type DialogFilter, type OpenDialogOptions } from '@tauri-apps/api/dialog'
  // import { listen, emit } from '@tauri-apps/api/event'
  // import { onMount } from 'svelte'


  // import { inputError, inputPath, outputName, imageDataStore } from '$stores/file'
  // import { inputError, inputPath, outputName } from '$stores/file'
  // import {
  //   audioFrameBytes,
  //   // Crop,
  //   // crop,
  //   frameRate,
  //   // Model,
  //   // model,
  //   sampleBitDepth,
  //   sampleRate,
  //   savePath,
  //   scale,
  //   videoFrameBytes,
  //   type Options
  // } from '$stores/options'
  import { inputError, inputPath } from '$stores/file'
  import Loading from '~icons/tabler/loader-2'

  // onMount(() => {
  //   if (document.activeElement instanceof HTMLElement) {
  //     document.activeElement.blur()
  //   }

  // })

  const videoFilter: DialogFilter = {
    name: 'Videos',
    extensions: ['mp4']
  }

  const openDialogOptions: OpenDialogOptions = {
    filters: [videoFilter]
  }

  let className = ''
  export { className as class }
  let loading = false


  const openFileDialog = async (): Promise<void> => {
    loading = true
    const originalPath = $inputPath
    const path = await open(openDialogOptions)

    if (path === null) {
      loading = false
      return
    }
    if (Array.isArray(path)) {
      $inputError = 'ERROR: Please select only one file.'
      loading = false
      return
    }

    // if (await ffprobe(path)) { //  If the url cannot be opened or recognized as a multimedia file, a positive exit code is returned.
    //   $inputError = undefined
    $inputPath = path

    





    // } else {
    //   $inputPath = undefined
    //   $inputError = "Couldn't read the file's metadata"
    // }

    if ($inputPath === undefined) return

    // keep loading icon when transitioning views
    if (
      (originalPath !== undefined && $inputPath !== undefined) ||
      (originalPath === undefined && $inputPath === undefined)
    ) {
      loading = false
    }



    // const options: Options = {
//   path: $inputPath,
//   savePath: $savePath,
//   outputName: $outputName ?? 'default-output-name',
//   scale: $scale,

//   frameRate: $frameRate.toString(),
//   videoFrameBytes: $videoFrameBytes,

//   sampleBitDepth: $sampleBitDepth,
//   sampleRate: sampleRate.toString(),
//   audioFrameBytes

//   // [key in Model]: $model
// }


  // // Call the screenshot function from rust in commands.rs
  // const response = await invoke<{ data: string }>('screenshot', { options })
  // const imageData = response.data;

  // // Set the imageDataStore with the newly fetched data so that it can be used in page.svelte
  // imageDataStore.set(imageData);


  }


</script>




<button
  type="button"
  disabled={loading}
  on:click={openFileDialog}
  class="button-primary button relative {className}"
>
  <Loading
    aria-label="loading"
    class="absolute top-[calc(50%-.75rem)] left-[calc(50%-.75rem)] h-6 w-6 {loading
      ? 'animate-spin'
      : 'hidden'}"
  />

  <div />
  <span class:invisible={loading}>Select a video</span>
</button>

