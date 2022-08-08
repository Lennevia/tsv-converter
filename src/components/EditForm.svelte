<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  import { save } from '@tauri-apps/api/dialog'

  import { inputName, inputPath, outputName } from '$stores/file'
  import {
    audioFrameBytes,
    Crop,
    crop,
    frameRate,
    Model,
    model,
    sampleBitDepth,
    sampleRate,
    scale,
    videoFrameBytes,
    type Options
  } from '$stores/options'
  import Loading from '~icons/tabler/loader-2'

  const valid = true
  let loading = false

  const convert = async (): Promise<void> => {
    loading = true

    if ($inputPath === undefined || $outputName === undefined) return

    const options: Options = {
      path: $inputPath,
      outputName: $outputName,
      scale: $scale,

      frameRate: $frameRate.toString(),
      videoFrameBytes: $videoFrameBytes,

      sampleBitDepth: $sampleBitDepth,
      sampleRate: sampleRate.toString(),
      audioFrameBytes

      // [key in Model]: $model
    }

    await invoke($model === Model.Tv96x64 ? 'convert' : 'convert_avi', { options })
    loading = false

    // Trigger save dialog after a video conversion
    $inputPath = await save({
      defaultPath: `${$outputName}`,
      filters: [
        {
          name: `.${$model === Model.Tv96x64 ? 'tsv' : 'avi'}`,
          extensions: ['stronghold']
        }
      ]
    })
  }
</script>

<form on:submit|preventDefault={convert} class="flex flex-col items-start space-y-2">
  <!-- TV model selection -->
  <fieldset class="form-fieldset flex flex-col items-start">
    <legend class="form-legend">TV Option</legend>
    {#each Object.values(Model) as opt}
      <label
        class="flex items-center rounded-md px-2 py-0.5 hover:bg-zinc-200 dark:hover:bg-zinc-700"
      >
        <input
          type="radio"
          name="model"
          checked={$model === opt}
          on:change={() => {
            $model = opt
          }}
          class="mr-2"
        />
        <span>{opt}</span>
      </label>
    {/each}
  </fieldset>

  <!-- crop radio group -->
  <fieldset class="form-fieldset flex flex-col items-start">
    <legend class="form-legend">Crop</legend>

    {#each Object.values(Crop) as opt}
      <label
        class="flex items-center rounded-md px-2 py-0.5 hover:bg-zinc-200 dark:hover:bg-zinc-700"
      >
        <input
          type="radio"
          name="crop"
          checked={$crop === opt}
          on:change={() => {
            $crop = opt
          }}
          class="mr-2"
        />
        <span>{opt}</span>
      </label>
    {/each}
  </fieldset>

  <!-- TODO: background color for letterbox crop
    <fieldset>
      <legend>Background Color</legend>

      <input type="text" value="#000000" maxlength="7" />
    </fieldset>
  -->

  <!-- output file name -->
  <fieldset class="form-fieldset">
    <legend class="form-legend">Output Name</legend>

    <div class="flex items-center space-x-2">
      <input
        name="output-name"
        required
        autocorrect="off"
        autocomplete="off"
        spellcheck="false"
        minlength={1}
        maxlength={46}
        pattern="[\w\.- ]+"
        bind:value={$outputName}
        class="grow"
      />
      <button
        type="button"
        disabled={$outputName === $inputName}
        on:click={() => {
          $outputName = $inputName
        }}
        class="button"
      >
        Reset
      </button>
    </div>
  </fieldset>

  <!-- convert button -->
  <button disabled={!valid || loading} class="button button-primary">
    <Loading
      aria-label="loading"
      class="absolute top-[calc(50%-.75rem)] left-[calc(50%-.75rem)] h-6 w-6 {loading
        ? 'animate-spin'
        : 'hidden'}"
    />
    Convert</button
  >
</form>
