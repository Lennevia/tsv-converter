// import { Command } from '@tauri-apps/api/shell'

// /**
//  * Read file metadata from `path` with ffprobe.
//  * @returns `true` if ffprobe exited successfully.
//  */
// export const ffprobe = async (path: string): Promise<boolean> => {
//   const { code } = await Command.sidecar('bin/ffprobe', path).execute()
//   return code === 0
// }

// /** Convert file size in bytes to a human readable size. */
// export const fileSize = (size: number): string => {
//   const unitsSi = ['B', 'KB', 'MB', 'GB', 'TB']
//   const unitsIec = ['B', 'KiB', 'MiB', 'GiB', 'TiB']

//   let i = 0
//   let j = 0
//   let sizeSi = size
//   let sizeIec = size

//   for (; sizeSi >= 1000; i++) {
//     sizeSi /= 1000
//   }
//   for (; sizeIec >= 1024; j++) {
//     sizeIec /= 1024
//   }

//   return (
//     `${sizeSi.toFixed(1)} ${unitsSi[i] as string}` +
//     ` (${sizeIec.toFixed(1)} ${unitsIec[j] as string})`
//   )
// }

// /** Convert seconds to `HH:MM:SS`. */
// export const secondsToHHMMSS = (seconds: number): string => {
//   const h = Math.floor(seconds / 3600)
//   const m = Math.floor((seconds - h * 3600) / 60)
//   const s = Math.round(seconds - h * 3600 - m * 60)

//   return (
//     `${h < 10 ? h.toString().padStart(2, '0') : h}` +
//     `:${m.toString().padStart(2, '0')}` +
//     `:${s.toString().padStart(2, '0')}`
//   )
// }
