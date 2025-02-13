import { derived, writable } from 'svelte/store'

/** Video scaling options. Values are CSS `object-fit` with alternate terminology in parentheses. */
export enum Crop {
  Contain = 'Contain (Letterbox)',
  Cover = 'Cover (Zoom)',
  Fill = 'Fill (Stretch)'
}

/** Tv model versions that determine method of conversion to use */
export enum Model {
  Tv240x135 = 'TinyTV 2 - 216x135',
  Tv64x64 = 'TinyTV Mini - 64x64',
  Tv96x64 = 'TinyTV DIY Kit - 96x64'
}

/** Settings data structure **/
// export interface Settings {
//   volume: number

//   staticEffects: boolean
//   timeStamp: boolean
//   showChannel: boolean
//   alphabeticOrder: boolean
// }

// export enum Playback {
//   Auto = 'Auto',
//   Loop = 'Loop',
//   FauxLive = 'Faux Live'
// }

// export const playback = writable(Playback.FauxLive)

/** Video conversion options. */
export interface Options {
  path: string
  savePath: string
  outputName: string
  scale: string

  frameRate: string
  videoFrameBytes: number

  sampleBitDepth: number
  sampleRate: string
  audioFrameBytes: number

  // [key in Model]: string
}

/** Video duration in seconds. */
export const duration = writable(NaN) // TODO - is this used?

/** Path variable */
export const savePath = writable('None selected')

/** TV variables */
export const model = writable(Model.Tv240x135) // Default selected option

export const width = derived(model, ($model) => {
  switch ($model) {
    case Model.Tv240x135:
      return 216 // changed from 240
    case Model.Tv64x64:
      return 64
    case Model.Tv96x64:
      return 96
  }
})
export const height = derived(model, ($model) => {
  switch ($model) {
    case Model.Tv240x135:
      return 135
    case Model.Tv64x64:
      return 64
    case Model.Tv96x64:
      return 64
  }
})
export const sampleBitDepth = derived(model, ($model) => {
  switch ($model) {
    case Model.Tv240x135:
      return 8
    case Model.Tv64x64:
      return 10 // TODO
    case Model.Tv96x64:
      return 10
  }
})
export const frameRate = derived(model, ($model) => {
  switch ($model) {
    case Model.Tv240x135:
      return 24 // TODO - should this be 30?
    case Model.Tv64x64:
      return 30 // TODO - should be 30
    case Model.Tv96x64:
      return 24
  }
})

// Video
export const videoFrameBytes = derived([width, height], ([$width, $height]) => 2 * $width * $height)

// Audio values for Tv96x64
export const sampleCountPerFrame = 2 * 512
export const audioFrameBytes = 2 * sampleCountPerFrame
export const sampleRate = derived(frameRate, ($frameRate) => {
  return $frameRate * sampleCountPerFrame
})
export const totalFrames = derived([duration, frameRate], ([$duration, $frameRate]) => {
  return $duration * $frameRate
})

// Crop video options
export const crop = writable(Crop.Contain) // Default selected option
// This link might be helpful for future cropping: https://www.linuxuprising.com/2020/01/ffmpeg-how-to-crop-videos-with-examples.html
export const scale = derived([crop, width, height, model], ([$crop, $width, $height, $model]) => {
  switch ($crop) {
    case Crop.Contain:
      switch ($model) {
        case Model.Tv240x135:
          return `scale=${$width}:${$height}:force_original_aspect_ratio=decrease,pad=228:136:(ow-iw)/2:(oh-ih)/2,setsar=1,hqdn3d` // https://stackoverflow.com/questions/46671252/how-to-add-black-borders-to-video
        case Model.Tv64x64:
          return `scale=${$width}:${$height}:force_original_aspect_ratio=decrease,pad=64:64:(ow-iw)/2:(oh-ih)/2,setsar=1`
        case Model.Tv96x64:
          // return `scale=${$width}:${$height}`
          return `scale=${$width}:${$height}:force_original_aspect_ratio=decrease,pad=96:64:(ow-iw)/2:(oh-ih)/2,setsar=1` // This maybe shoule be tested vs the above?
      }
    // eslint-disable-next-line no-fallthrough
    case Crop.Cover:
      switch ($model) {
        case Model.Tv240x135:
          return `scale=${$width}:${$height}:force_original_aspect_ratio=increase,crop=${$width}:${$height},hqdn3d` // Set height dynamically and then crop off extra height to give zoom effect
        case Model.Tv64x64:
          return `scale=${$width}:${$height}:force_original_aspect_ratio=increase,crop=${$width}:${$height}`
        case Model.Tv96x64:
          return `scale=${$width}:${$height}:force_original_aspect_ratio=increase,crop=${$width}:${$height}`
      }
    // eslint-disable-next-line no-fallthrough
    case Crop.Fill:
      switch ($model) {
        case Model.Tv240x135:
          return `scale=${$width}:${$height},hqdn3d`
        case Model.Tv64x64:
          return `scale=${$width}:${$height}`
        case Model.Tv96x64:
          // return `scale=${$width}:${$height}:force_original_aspect_ratio=decrease,pad=${$width}:${$height}:(ow-iw)/2:(oh-ih)/2`
          return `scale=${$width}:${$height}`
      }
  }
})
