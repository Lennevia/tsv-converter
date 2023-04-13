<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  import { open, type DialogFilter, type OpenDialogOptions } from '@tauri-apps/api/dialog'
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte'
 

  import { inputError, inputPath } from '$stores/file'
  import {
    audioFrameBytes,
    // Crop,
    // crop,
    frameRate,
    // Model,
    // model,
    sampleBitDepth,
    sampleRate,
    savePath,
    scale,
    videoFrameBytes,
    type Options
  } from '$stores/options'
  // import { type Options, scale } from '$stores/options'
  import Loading from '~icons/tabler/loader-2'


  onMount(() => {
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur()
      console.log(imageData)
    console.dir(imageData)
    }
  //   window.tauri.listen("screenshot", (data) => {
  //   screenshotData = data;
  // });
  })

  const videoFilter: DialogFilter = {
    name: 'Videos',
    extensions: ['mp4', 'mov', 'mpg', 'mpeg', 'avi', 'gif']
  }

  const openDialogOptions: OpenDialogOptions = {
    filters: [videoFilter]
  }

  let className = ''
  export { className as class }
  let loading = false
  let imageData: string | null = null;

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

    if ($inputPath === undefined ) return

    // keep loading icon when transitioning views
    if (
      (originalPath !== undefined && $inputPath !== undefined) ||
      (originalPath === undefined && $inputPath === undefined)
    ) {
      loading = false
    }

    const dummyoutput = 'string';

    const options: Options = {
      path: $inputPath,
      savePath: $savePath,
      outputName: dummyoutput,
      scale: $scale,

      frameRate: $frameRate.toString(),
      videoFrameBytes: $videoFrameBytes,

      sampleBitDepth: $sampleBitDepth,
      sampleRate: sampleRate.toString(),
      audioFrameBytes

      // [key in Model]: $model
    }

    await invoke('screenshot', { options })




    // listen for the custom "screenshot" event from the Rust backend
    listen('screenshotEvent', (event) => {
      imageData = event.payload as string;
    });

    // console.log(imageData)
    // console.dir(imageData)

    // console.log(imageData)
    // console.dir(imageData)


    // const src = `data:image/png;base64,${imageData}`;

  }


</script>




<!-- svelte-ignore a11y-media-has-caption -->
<!-- <video id="video_source" controls autoplay>
  <source type="video/mp4" />
</video> -->


<!-- <img src="output.jpg" alt="Italian Trulli"> -->

<!-- <img src={screenshotData} alt="alternate" class:invisible={loading}/> -->
<!-- <img id="screenshot" alt="Screenshot" /> -->
<!-- <img src={imageData} alt="Screenshot"> -->
<img src={`data:image/png;base64,${imageData}`} alt="Screenshot" />



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
