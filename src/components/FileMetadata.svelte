<!-- <script lang="ts">
  import { invoke } from '@tauri-apps/api'

  import { fileSize, secondsToHHMMSS } from '$lib/fileUtils'

  type Metadata = Readonly<{
    name?: string
    mimes: string[]
    len?: number
    created?: number
    modified?: number
  }>

  export let path: string
  export let duration: number
  export let videoWidth: number
  export let videoHeight: number

  let metadata: Metadata | undefined

  $: (async () => {
    metadata = await invoke('metadata', { path })
  })()
</script> -->

<!-- TODO: use a table -->
<!-- {#if metadata}
  <div class="rounded-md border border-zinc-400 dark:border-zinc-600">
    <div
      class="max-w-full overflow-x-auto rounded-t-md bg-zinc-300 px-2 text-center dark:bg-zinc-600"
    >
      {#if metadata.name !== undefined}
        <code class="break-all line-clamp-1">{metadata.name}</code>
      {:else}
        <code>Video</code>
      {/if}
    </div>

    <div class="grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 p-2">
      {#if metadata.mimes.length > 0}
        <span class="metadata-child">Type</span>
        <code class="metadata-child overflow-x-auto rounded-md bg-zinc-300 px-2 dark:bg-zinc-600"
          >{metadata.mimes.join(', ')}</code
        >
      {/if}

      {#if duration}
        <span class="metadata-child">Duration</span>
        <span class="metadata-child">{secondsToHHMMSS(duration)}</span>
      {/if}

      {#if videoWidth && videoHeight}
        <span class="metadata-child">Dimensions</span>
        <span class="metadata-child">{`${videoWidth}x${videoHeight}`}</span>
      {/if}

      {#if metadata.len !== undefined}
        <span class="metadata-child">Size</span>
        <span class="metadata-child">{fileSize(metadata.len)}</span>
      {/if}

      {#if metadata.modified !== undefined}
        <span class="metadata-child">Modified</span>
        <span class="metadata-child">{new Date(metadata.modified * 1000).toLocaleString()}</span>
      {/if}

      {#if metadata.created !== undefined}
        <span class="metadata-child">Created</span>
        <span class="metadata-child">{new Date(metadata.created * 1000).toLocaleString()}</span>
      {/if}
    </div>
  </div>
{/if} -->
