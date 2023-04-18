<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/tauri'
  import { fade } from 'svelte/transition'

  import EditForm from '$components/EditForm.svelte'
  import FileInput from '$components/FileInput.svelte'
  import { inputError, inputPath } from '$stores/file'
  import { Crop, crop } from '$stores/options'
  // import { type Options, scale } from '$stores/options'
  // import Loading from '~icons/tabler/loader-2'

  const videoObjectFit = {
    [Crop.Contain]: 'object-contain',
    [Crop.Cover]: 'object-cover',
    [Crop.Fill]: 'object-fill'
  }

  // let duration: number
  let videoWidth: number
  let videoHeight: number
</script>

{#if $inputPath === undefined}
  <div
    in:fade={{ delay: 300, duration: 300 }}
    out:fade={{ duration: 300 }}
    class="flex h-full flex-col items-center justify-center"
  >
    <FileInput />

    <br />
    {#if $inputError !== undefined}
      <p class="whitespace-pre">
        {$inputError}
      </p>
    {/if}
  </div>
{:else}
  <div
    in:fade={{ delay: 300, duration: 300 }}
    out:fade={{ duration: 300 }}
    class="flex h-full space-x-2"
  >
    <!-- left group -->
    <div class="w-1/2 space-y-2">
      <div class="flex aspect-[3/2] items-center justify-center rounded-md bg-black">
        <!-- svelte-ignore a11y-media-has-caption -->
        <video
          src={convertFileSrc($inputPath)}
          controls
          loop
          preload="auto"
          bind:videoHeight
          bind:videoWidth
          class="h-full w-full rounded-md {videoObjectFit[$crop]}"
        />
      </div>
      <!-- 
      <canvas bind:this={canvas} style="display:none; border: 1px solid white;"></canvas>
      <img src="{imageSrc}" alt="Snapshot" style="border: 1px solid pink;"> -->
      <!-- <img src="/Users/rena/Documents/TinyTV-Tested-Video-Thumbnail.jpg" alt="imagejjk" style="border: 1px solid blue;">  -->
      <!-- <img src="https://cdn.shopify.com/s/files/1/1125/2198/products/greythumby1065x1065-min_1800x1800.jpg?v=1674158925" alt="test" style="border: 1px solid orange;">  -->
      <div style="overflow-wrap: break-word"><b>Input Path:</b> {$inputPath}</div>
    </div>

    <!-- right group -->
    <div class="w-1/2 space-y-2">
      <FileInput />
      <EditForm />
    </div>
  </div>
{/if}
