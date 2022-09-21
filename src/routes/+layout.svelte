<script lang="ts">
  import '@fontsource/inter/400.css'
  import '@fontsource/inter/500.css'
  import '@fontsource/inter/600.css'
  import { appWindow } from '@tauri-apps/api/window'
  import { onMount } from 'svelte'

  import '../app.css'

  onMount(async () => {
    await appWindow.show()
  })
</script>

<section class="h-full p-2">
  <slot />
</section>
