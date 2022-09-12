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
    savePath,
    scale,
    videoFrameBytes,
    type Options
  } from '$stores/options'

  const valid = true
  let loading = false

  // Send all the data needed for conversion when the "Convert" button is pressed
  const convert = async (): Promise<void> => {
    loading = true

    if ($inputPath === undefined || $outputName === undefined) return

    const options: Options = {
      path: $inputPath,
      savePath: $savePath,
      outputName: $outputName,
      scale: $scale,

      frameRate: $frameRate.toString(),
      videoFrameBytes: $videoFrameBytes,

      sampleBitDepth: $sampleBitDepth,
      sampleRate: sampleRate.toString(),
      audioFrameBytes

      // [key in Model]: $model
    }

    // Trigger save dialog before a video conversion
    $savePath = await save({
      defaultPath: `${$outputName}`,
      filters: [
        {
          name: `.${$model === Model.Tv96x64 ? 'tsv' : 'avi'}`,
          extensions: [`${$model === Model.Tv96x64 ? 'tsv' : 'avi'}`]
        }
      ]
    })

    if ($model === Model.Tv96x64) await invoke('convert', { options })
    if ($model === Model.Tv240x135) await invoke('convert_avi', { options })
    // if ($model === Model.Tv64x64) await invoke('convert_mini', { options }) // TODO

    loading = false
  }

  // Progress bar ideas:
  // https://devdojo.com/tnylea/creating-a-progress-bar-with-tailwind
  // https://github.com/tauri-apps/tauri/discussions/4069
  // let progress = 0
  // let intervalSpeed = 10
  // let incrementSpeed = 1
  // document.addEventListener('DOMContentLoaded', function () {
  //   let bar = document.getElementById('bar')
  //   progressInterval = setInterval(function () {
  //     progress += incrementSpeed
  //     bar.style.width = progress + '%'
  //     if (progress >= 100) {
  //       clearInterval(progressInterval)
  //     }
  //   }, intervalSpeed)
  // })
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

  <div>Input Path: {$inputPath}</div>
  <div>Saving to: {$savePath}</div>

  <!-- convert button -->
  <button disabled={!valid || loading} class="button button-primary"> Convert</button>
</form>

<br />

<!-- loading bar -->
<div
  disabled={loading}
  aria-label="loading"
  class="w-full rounded-full bg-gray-300 dark:bg-gray-700 {loading ? 'width: 100%' : 'hidden'}"
>
  <div
    class="text-s rounded-full bg-tc-orange p-0.5 text-center font-medium leading-none text-gray-900 {loading
      ? ''
      : 'hidden'}"
    style="width: 0%"
    id="bar"
  >
    0%
  </div>
</div>
