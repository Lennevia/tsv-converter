//! Tauri commands.

use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
#[cfg(debug_assertions)]
use std::time::Instant;
// use serde_json::json;
// use base64::encode_config;
// use base64::{Engine, engine::general_purpose, alphabet};
// use base64::{Engine as _, engine::general_purpose};
// use base64::{display::Base64Display, engine::general_purpose::STANDARD};
use base64::{engine::general_purpose, Engine as _};
// use web_sys::window;
// use tauri::window::Window;
// use tauri::event::emit;
use tauri::Window;
// use main::Payload;
use lazy_static::lazy_static;
use std::sync::Mutex;
// extern crate lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::prelude::*;
use image::{ColorType, ImageBuffer, ImageFormat};
use image::{Rgb, Rgba};
use image::buffer::ConvertBuffer;
use image::DynamicImage;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub picture_data: String,
}

// #[derive(Clone, serde::Serialize)]
lazy_static! {
    pub static ref PAYLOAD: Mutex<Payload> = Mutex::new(Payload {
        picture_data: String::new(),
    });
}

// use tide::new;

// use byte_array::ByteArray;
// use libmath::round;
use notify::{Config, EventKind, RecursiveMode, Watcher};
use tauri::async_runtime;
use time::OffsetDateTime;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// File metadata.
#[derive(serde::Serialize)]
pub struct Metadata {
    name: Option<String>,
    mimes: Vec<String>,
    len: Option<u64>,
    #[serde(with = "time::serde::timestamp::option")]
    created: Option<OffsetDateTime>,
    #[serde(with = "time::serde::timestamp::option")]
    modified: Option<OffsetDateTime>,
}

/// Video conversion options.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options<'a> {
    path: &'a str,
    save_path: &'a str, // add mut to tell compiler it's mutable
    output_name: &'a str,
    scale: &'a str,

    // Video
    frame_rate: &'a str,
    video_frame_bytes: usize,

    // Audio
    sample_bit_depth: u8,
    sample_rate: &'a str,
    audio_frame_bytes: usize,
}

/// Get file metadata from a path.
#[tauri::command]
pub fn metadata(path: &Path) -> Metadata {
    let name = path.file_name().map(|s| s.to_string_lossy().to_string());
    let mimes = mime_guess::from_path(path)
        .iter_raw()
        .map(String::from)
        .collect();
    let mut len = None;
    let mut created = None;
    let mut modified = None;

    if let Ok(meta) = path.metadata() {
        len = Some(meta.len());
        created = meta.created().ok().map(OffsetDateTime::from);
        modified = meta.modified().ok().map(OffsetDateTime::from);
    }

    Metadata {
        name,
        mimes,
        len,
        created,
        modified,
    }
}

/// Watches a file path for modify/remove events, and forwards the event to the frontend.
// #[tauri::command]
// pub async fn watch(path: PathBuf, window: tauri::Window) {
//     let (tx, mut rx) = async_runtime::channel(1); // Channel for modified/removed file
//     if !path.is_file() {
//         return;
//     }
//     let mut watcher =
//         notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
//             match res {
//                 Ok(event) => match event.kind {
//                     EventKind::Modify(_) | EventKind::Remove(_) => {
//                         tx.blocking_send(event.kind).unwrap();
//                     }
//                     _ => (),
//                 },
//                 Err(err) => eprintln!("Path watch error: {err:?}"),
//             };
//         })
//         .unwrap();
//     // watcher.configure(Config::PreciseEvents(true)).unwrap();
//     watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

//     if let Some(event_kind) = rx.recv().await {
//         window.emit("fs-change", event_kind).unwrap();
//         watcher.unwatch(&path).unwrap();
//     }
// }

/// Calculate a possible output file stem from a file path.
#[tauri::command]
pub fn output_name(path: &Path) -> String {
    match limit_file_stem(path) {
        Some(stem) => stem,
        None => "out".to_string(),
    }
}

/// Limit a file stem to 46 bytes of ASCII (Output file name with `.tsv` extension must be a C
/// `char[]` < 50 bytes).
fn limit_file_stem(path: &Path) -> Option<String> {
    let stem = path
        .file_stem()?
        .to_str()?
        .chars()
        .filter(|c| match c {
            // FIXME: is this all the supported characters?
            c if c.is_ascii_alphanumeric() => true,
            '_' | '-' | '.' | ' ' => true,
            _ => false,
        })
        .take(46)
        .collect::<String>();

    if stem.is_empty() {
        None
    } else {
        Some(stem)
    }
}

/// Find a sidecar executable's path.
fn sidecar_path(name: &str) -> PathBuf {
    let path = tauri::utils::platform::current_exe()
        .unwrap()
        .with_file_name(name);

    if cfg!(windows) {
        path.with_extension("exe")
    } else {
        path
    }
}

/*
/// Convert to Tiny Screen Video .tsv filetype.
#[tauri::command]
pub fn convert(options: Options<'_>) {
    // let path = Path::new(&options.path);
    let output_path = Path::new(&options.save_path)
        .with_file_name(&options.output_name)
        .with_extension("tsv");

    let output_file = OpenOptions::new()
        .create(true)
        .truncate(true) // necessary?
        .write(true) // could this be append instead
        .open(&output_path)
        .unwrap();
    let mut writer = BufWriter::with_capacity(
        options.video_frame_bytes.max(options.audio_frame_bytes),
        output_file,
    );
    let ffmpeg_path = sidecar_path("ffmpeg");
    #[cfg(debug_assertions)]
    let timer = Instant::now();

    let mut video_cmd = Command::new(&ffmpeg_path);
    #[rustfmt::skip]
    video_cmd.args([
        "-i", options.path,
        "-f", "image2pipe",
        "-r", options.frame_rate,
        "-vf", options.scale,
        "-vcodec", "rawvideo",
        "-pix_fmt", "bgr565be",
        "-f", "rawvideo",
        "-",
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::null());

    // windows creation flag CREATE_NO_WINDOW: stops the process from creating a CMD window
    // https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    #[cfg(windows)]
    video_cmd.creation_flags(0x08000000);

    let mut video_child = video_cmd.spawn().unwrap();
    let mut video_stdout = video_child.stdout.take().unwrap();
    let mut video_frame = vec![0; options.video_frame_bytes];

    let mut audio_cmd = Command::new(&ffmpeg_path);
    #[rustfmt::skip]
    audio_cmd.args([
        "-i", options.path,
        "-f", "s16le",
        "-acodec", "pcm_s16le",
        "-ar", options.sample_rate,
        "-ac", "1",
        "-",
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::null());

    #[cfg(windows)]
    audio_cmd.creation_flags(0x08000000);

    let mut audio_child = audio_cmd.spawn().unwrap();
    let mut audio_stdout = audio_child.stdout.take().unwrap();
    let mut audio_frame = vec![0; options.audio_frame_bytes]; // 0 - 2048
                                                              // let mut audio_data = ByteArray(audio_frame);

    // audio_frame.resize(options.audio_frame_bytes, 0);
    // let result = audio_stdout.read_to_end(&mut audio_frame);
    // println!("Stdout read: {}", result);

    println!("end of audio def");

    let mut counter = 0;

    while video_stdout.read_exact(&mut video_frame).is_ok() {
        println!("enter while loop");
        writer.write_all(&video_frame).unwrap();
        println!("video has been unwrapped");

        let result = audio_stdout.read_exact(&mut audio_frame);
        println!("Result = {:?}", result);
        // println!("audio frame data is_ok");
        println!("{:?}", audio_frame);

        if audio_stdout.read_exact(&mut audio_frame).is_ok() {
            // OH NO
            println!("audio frame data is_ok");
            println!("{:?}", audio_frame);

            // for j in 0..options.audio_frame_bytes + 1 {
            //     audio_stdout.read_to_end(&mut audio_frame);
            //     print!("After read{} ", audio_frame[j]);
            // }
            // println!();

            if audio_frame.len() == options.audio_frame_bytes {
                // how can I change this?
                for i in 0..options.audio_frame_bytes / 2 {
                    // audio_data = ByteArray::new();
                    // audio_data.write(&(audio_frame as u32));

                    let temp_sample = ((u32::from(audio_frame[(i * 2) + 1]) << 8)
                        | u32::from(audio_frame[i * 2]))
                        + 0x8000;
                    let sample = (temp_sample >> (16 - 10)) & (0x0000FFFF >> (16 - 10));

                    audio_frame[i * 2] = (sample & 0xFF) as u8;
                    audio_frame[(i * 2) + 1] = (sample >> 8) as u8;

                    // for j in 0..options.audio_frame_bytes / 2 {
                    //     print!("{} ", audio_frame[j]);
                    // }
                    // println!();

                    // audio_stdout = audio_child.stdout.take().unwrap();
                    // audio_frame = vec![0; options.audio_frame_bytes];

                    // println!("audio samples have been converted {} times", counter);
                    // counter += 1;
                }
                println!("audio samples have been converted {} times", counter);
                counter += 1;
            } else {
                println!("fill with 0");
                // let mut empty_samples = vec![];
                // for _i in 0..options.audio_frame_bytes / 2 {
                //     empty_samples.push(0x00);
                //     empty_samples.push(0x00);
                // }
                // audio_frame.extend(empty_samples);

                audio_frame.fill(0x00);
            }

            println!("after sample conversion");
            // for j in 0..2048 {
            //     print!("{} ", audio_frame[j]);
            // }
            // println!();
        }

        writer.write_all(&audio_frame).unwrap();
    }

    // for j in 0..options.audio_frame_bytes / 2 {
    //     print!("Initial: {} ", audio_frame[j]);
    // }
    // println!();

    // while video_stdout.read_exact(&mut video_frame).is_ok() {
    //     writer.write_all(&video_frame).unwrap();

    //     if audio_stdout.read_exact(&mut audio_frame).is_ok() {
    //         for i in 0..options.audio_frame_bytes / 2 {
    //             let sample = ((0x8000
    //                 + (u32::from(audio_frame[i * 2 + 1]) << 8 | u32::from(audio_frame[i * 2])))
    //                 >> (16 - u32::from(options.sample_bit_depth)))
    //                 & (0xFFFF >> (16 - u32::from(options.sample_bit_depth)));

    //             audio_frame[i * 2] = (sample & 0xFF) as u8;
    //             audio_frame[i * 2 + 1] = (sample >> 8) as u8;
    //         }
    //     } else {
    //         audio_frame.fill(0);
    //     }

    //     writer.write_all(&audio_frame).unwrap();
    // }

    // let mut audio_data;

    //-----------------------------------------------------
    // println!("end of audio def");

    // let mut counter = 0;

    // while video_stdout.read_exact(&mut video_frame).is_ok() {
    //     println!("enter while loop");
    //     writer.write_all(&video_frame).unwrap();
    //     println!("video has been unwrapped");

    //     let result = audio_stdout.read_to_end(&mut audio_frame);
    //     println!("Result = {:?}", result);

    //     if audio_stdout.read_to_end(&mut audio_frame).is_ok() {
    //         // OH NO
    //         println!("audio frame data is_ok");

    //         for j in 0..options.audio_frame_bytes {
    //             print!("After read{} ", audio_frame[j]);
    //         }
    //         println!();

    //         if audio_frame.len() == options.audio_frame_bytes {
    //             for i in 0..options.audio_frame_bytes / 2 {
    //                 // audio_data = ByteArray::new();
    //                 // audio_data.write(&(audio_frame as u32));

    //                 let temp_sample = ((u32::from(audio_frame[(i * 2) + 1]) << 8)
    //                     | u32::from(audio_frame[i * 2]))
    //                     + 0x8000;
    //                 let sample = (temp_sample >> (16 - 10)) & (0x0000FFFF >> (16 - 10));

    //                 audio_frame[i * 2] = (sample & 0xFF) as u8;
    //                 audio_frame[(i * 2) + 1] = (sample >> 8) as u8;

    //                 // for j in 0..options.audio_frame_bytes / 2 {
    //                 //     print!("{} ", audio_frame[j]);
    //                 // }
    //                 // println!();

    //                 // audio_stdout = audio_child.stdout.take().unwrap();
    //                 // audio_frame = vec![0; options.audio_frame_bytes];

    //                 // println!("audio samples have been converted {} times", counter);
    //                 // counter += 1;
    //             }
    //             println!("audio samples have been converted {} times", counter);
    //             counter += 1;
    //         } else {
    //             println!("fill with 0");
    //             let mut empty_samples = vec![];
    //             for _i in 0..options.audio_frame_bytes / 2 {
    //                 empty_samples.push(0x00);
    //                 empty_samples.push(0x00);
    //             }
    //             audio_frame.extend(empty_samples);

    //             println!("fill with 0");

    //             // audio_frame.fill(0x00);
    //         }

    //         println!("after sample conversion");
    //         // for j in 0..2048 {
    //         //     print!("{} ", audio_frame[j]);
    //         // }
    //         // println!();
    //     }

    //     writer.write_all(&audio_frame).unwrap();
    // }
    // --------------------------------------------------------

    video_child.wait().unwrap();
    audio_child.wait().unwrap();

    #[cfg(debug_assertions)]
    {
        let elapsed = timer.elapsed();
        dbg!(elapsed);
    }

    writer.flush().unwrap();
}
*/

/// Convert to .AVI file type fixed to the resolution of the 240x135 TV.
#[tauri::command]
pub fn convert_avi(options: Options<'_>) {
    // let path = Path::new(&options.path);
    let output_path = Path::new(&options.save_path)
        .with_file_name(&options.output_name)
        .with_extension("avi");

    let _ = fs::remove_file(&output_path);

    let ffmpeg_path = sidecar_path("ffmpeg");
    #[cfg(debug_assertions)]
    let timer = Instant::now();

    let mut cmd = Command::new(&ffmpeg_path);
    #[rustfmt::skip]
    cmd.args([
        // "-hide_banner", "-loglevel quiet", // makes ffmpeg not print verbose output to terminal
        "-i", options.path,
        "-r", options.frame_rate,
        "-vf", options.scale,
        "-b:v", "1500k",
        "-maxrate", "1500k",
        "-bufsize", "64k",
        "-c:v", "mjpeg",
        "-acodec", "pcm_u8",
        "-ar", "10000",
        "-ac", "1",
        options.save_path,
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::piped());

    // windows creation flag CREATE_NO_WINDOW: stops the process from creating a CMD window
    // https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    #[cfg(windows)]
    cmd.creation_flags(0x08000000);

    let mut child = cmd.spawn().unwrap();
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();
    child.wait().unwrap();

    #[cfg(debug_assertions)]
    {
        dbg!(stderr);
        let elapsed = timer.elapsed();
        dbg!(elapsed);
    }
}

/// Convert to .AVI file type fixed to the resolution of the 64x64 TV mini
#[tauri::command]
pub fn convert_mini_avi(options: Options<'_>) {
    // let path = Path::new(&options.path);
    let output_path = Path::new(&options.save_path)
        .with_file_name(&options.output_name)
        .with_extension("avi");
    let _ = fs::remove_file(&output_path);
    let ffmpeg_path = sidecar_path("ffmpeg");
    #[cfg(debug_assertions)]
    let timer = Instant::now();

    let mut cmd = Command::new(&ffmpeg_path);
    #[rustfmt::skip]
    cmd.args([
        // "-hide_banner", "-loglevel quiet", // makes ffmpeg not print verbose output to terminal
        "-i", options.path,
        "-r", options.frame_rate,
        "-vf", options.scale,
        "-b:v", "300k",  
        "-maxrate", "300k",
        "-bufsize", "64k",
        "-c:v", "mjpeg",
        "-acodec", "pcm_u8",
        "-ar", "10000",
        "-ac", "1",
        options.save_path,
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::piped());

    // windows creation flag CREATE_NO_WINDOW: stops the process from creating a CMD window
    // https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    #[cfg(windows)]
    cmd.creation_flags(0x08000000);

    let mut child = cmd.spawn().unwrap();
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();
    child.wait().unwrap();

    #[cfg(debug_assertions)]
    {
        dbg!(stderr);
        let elapsed = timer.elapsed();
        dbg!(elapsed);
    }
}

/// Convert to .AVI file type fixed to the resolution of the 96x64 DIY TV
#[tauri::command]
pub fn convert_diy_avi(options: Options<'_>) {
    // let path = Path::new(&options.path);
    let output_path = Path::new(&options.save_path)
        .with_file_name(&options.output_name)
        .with_extension("avi");
    let _ = fs::remove_file(&output_path);
    let ffmpeg_path = sidecar_path("ffmpeg");
    #[cfg(debug_assertions)]
    let timer = Instant::now();

    let mut cmd = Command::new(&ffmpeg_path);
    #[rustfmt::skip]
    cmd.args([
        "-i", options.path,
        "-r", options.frame_rate,
        "-vf", options.scale,
        "-b:v", "300k",  
        "-maxrate", "300k",
        "-bufsize", "64k",
        "-c:v", "mjpeg",
        "-acodec", "pcm_u8",
        "-ar", "10000",
        "-ac", "1",
        options.save_path,
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::piped());

    // windows creation flag CREATE_NO_WINDOW: stops the process from creating a CMD window
    // https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    #[cfg(windows)]
    cmd.creation_flags(0x08000000);

    let mut child = cmd.spawn().unwrap();
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();
    child.wait().unwrap();

    #[cfg(debug_assertions)]
    {
        dbg!(stderr);
        let elapsed = timer.elapsed();
        dbg!(elapsed);
    }
}

#[derive(Serialize)]
pub struct ScreenshotResponse {
    pub data: String,
}

/// Take a screen capture of the video to display in the app for UI/UX
/// ffmpeg -ss 01:23:45 -i input -frames:v 1 -q:v 2 output.jpg
/// from: https://stackoverflow.com/questions/27568254/how-to-extract-1-screenshot-for-a-video-with-ffmpeg-at-a-given-time
///
/*
#[tauri::command]
pub fn screenshot(options: Options<'_>) -> ScreenshotResponse {
    let ffmpeg_path = sidecar_path("ffmpeg");
    // let preview_time = "00:00:00"; // example preview time
    let output_width = 192; // example output width
    let output_height = 128; // example output height

    let vidcommand = vec![
        "-ss",
        "00:00:00",
        "-i",
        options.path,
        // "-ss",
        // "1",
        "-f",
        "image2pipe",
        "-vf",
        "scale=192:128",
        "-pix_fmt",
        "rgb24",
        "-vcodec",
        "rawvideo",
        "-",
    ];

    // let mut info_pipe = Command::new(&ffmpeg_path)
    //     .args(&vidcommand)
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to execute command");

    // windows creation flag CREATE_NO_WINDOW: stops the process from creating a CMD window
    // https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    // #[cfg(windows)]
    // info_pipe.creation_flags(0x08000000);

    // Create a buffer for storing the raw video frame data read from ffmpeg:
    // vid_frame is a vector initialized with length (output_width * 2 * output_height * 2) * 3 and filled with zeros
    // The output_width * 2 * output_height * 2 expression calculates the number of pixels in the frame, and * 3 accounts
    // for the three color channels (red, green, blue) that are encoded in each pixel.
    /*let mut vid_frame = vec![0u8; (output_width * 2 * output_height * 2) * 3];

        // From the ffmpeg terminal, collect the image data into the vid_frame vector
        info_pipe
            .stdout
            .as_mut()
            .unwrap()
            .read_exact(&mut vid_frame)
            .unwrap();

        let mut xdata = format!("P6 {} {} 255 ", output_width * 2, output_height * 2).into_bytes();
        xdata.extend_from_slice(&vid_frame);

        info_pipe.kill().expect("Failed to kill ffmpeg process");
        info_pipe
            .wait_with_output()
            .expect("Failed to wait for process");
    */

    let mut info_pipe = Command::new(&ffmpeg_path)
        .args(&vidcommand)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to execute command");

    let mut vid_frame = vec![0u8; (output_width * output_height * 3) as usize];

    // Read data from the pipe into the vid_frame vector
    let mut stdout = info_pipe.stdout.take().unwrap();
    stdout.read_exact(&mut vid_frame).unwrap();

    // let screenshot_data = encode_config(&xdata, STANDARD);
    // let screenshot_data = Base64Display::new(&xdata, &STANDARD);

    let screenshot_data = general_purpose::STANDARD.encode(&vid_frame);

    // println!("data:image/png;base64,{}", screenshot_data);

    info_pipe.kill().unwrap();
    info_pipe.wait_with_output().unwrap();

    println!("data:image/png;base64,{}", screenshot_data);

    ScreenshotResponse {
        data: format!("data:image/png;base64,{}", screenshot_data),
    }
}

// let response = ScreenshotResponse {
//     data: format!("data:image/png;base64,{}", screenshot_data),
// };

// serde_json::to_string(&response).unwrap()

// #[derive(serde::Serialize)]
// serialize(format!("data:image/png;base64,{}", screenshot_data))

// acquire a lock on the mutex
// let mut payload = PAYLOAD.lock().unwrap();

// modify the picture_data field
// payload.picture_data = format!("data:image/png;base64,{}", screenshot_data);
// }

// #[tauri::command]
// pub fn get_screenshot( window: Window) -> String {
//     let mut payload = PAYLOAD.lock().unwrap();

//     payload.picture_data
// }
*/

#[tauri::command]
pub fn screenshot(options: Options<'_>) -> ScreenshotResponse {
    let ffmpeg_path = sidecar_path("ffmpeg");

// THIS WORKS:
// ffmpeg -ss 00:00:00 -i hj.mp4 -s 192x128 -pix_fmt rgb24 -vcodec rawvideo -f rawvideo output.rgb24


    let mut ffmpeg_cmd = Command::new(&ffmpeg_path)
        .args(&[
            "-ss",
            "00:00:00",
            "-i",
            options.path,
            "-s", "192x128",
            "-pix_fmt",
            "rgb24",
            "-vcodec",
            "rawvideo",
            "-f",
            "rawvideo",
            "-",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    println!("PATH: {}", options.path);

    let mut rgb_data: Vec<u8> = Vec::new();
    ffmpeg_cmd
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut rgb_data)
        .unwrap();


    // Convert RGB24 data to PNG format

let width = 192;
let height = 128;

let img_buf = ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, rgb_data).unwrap();
let mut rgba_img = DynamicImage::ImageRgb8(img_buf).into_rgba8();

// Set the alpha channel to 255 (fully opaque)
for pixel in rgba_img.chunks_exact_mut(4) {
    pixel[3] = 255;
}

let mut png_data = Vec::new();
let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
encoder
    .encode(
        &rgba_img,
        width as u32,
        height as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();


let screenshot_data = base64::encode(&png_data);

    ScreenshotResponse {
        data: screenshot_data,
    }
}

    // Build the ffmpeg command to extract a single frame of video data
    // let vidcommand = vec![
    //     "-ss", // Seek to a specific timestamp in the video
    //     "00:00:01",
    //     "-i", // Specify the input file
    //     options.path,
    //     "-f", // Force the output format to image2pipe
    //     "image2pipe",
    //     "-vf", // Apply a video filter to scale the output to a specific size
    //     "scale=192:128",
    //     "-pix_fmt", // Specify the output pixel format
    //     "rgb24",
    //     "-vcodec", // Specify the output codec
    //     "rawvideo",
    //     "-", // Output the raw data to stdout
    // ];



    /*
    // Start the ffmpeg process and capture its stdout stream
    let mut info_pipe = Command::new(&ffmpeg_path)
        .args(&vidcommand)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to execute command");

    // Create a buffer for storing the raw video frame data read from ffmpeg
    let mut vid_frame = vec![0u8; (output_width * 2 * output_height * 2) * 3];

    // Read the raw video frame data from ffmpeg into the buffer
    info_pipe
        .stdout
        .as_mut()
        .unwrap()
        .read_exact(&mut vid_frame)
        .unwrap();

    // Build the PPM format header for the raw video frame data
    let mut xdata = format!("P6 {} {} 255 ", output_width * 2, output_height * 2).into_bytes();

    // Append the raw video frame data to the PPM format header
    xdata.extend_from_slice(&vid_frame);
    */