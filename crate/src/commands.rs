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
use base64::{display::Base64Display, engine::general_purpose::STANDARD};
// use web_sys::window;
// use tauri::window::Window;
// use tauri::event::emit;
use tauri::Window;
// use main::Payload;
use std::sync::Mutex;
use lazy_static::lazy_static;
// extern crate lazy_static;


// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload{
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


/// Take a screen capture of the video to display in the app for UI/UX
/// ffmpeg -ss 01:23:45 -i input -frames:v 1 -q:v 2 output.jpg
/// from: https://stackoverflow.com/questions/27568254/how-to-extract-1-screenshot-for-a-video-with-ffmpeg-at-a-given-time
#[tauri::command]
pub fn screenshot(options: Options<'_>) {
    let ffmpeg_path = sidecar_path("ffmpeg");
    let preview_time = "10"; // example preview time
    let output_width = 192; // example output width
    let output_height = 128; // example output height


let vidcommand = vec![
    "-ss", preview_time,
    "-i", options.path,
    // "-ss", "1",
    "-f", "image2pipe",
    "-vf", options.scale,
    "-pix_fmt", "rgb24",
    "-vcodec", "rawvideo",
    "-"
];


#[cfg(windows)]
let mut info_pipe = Command::new(&ffmpeg_path)
    .args(&vidcommand)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .startup_info(startup_info)
    .spawn()
    .expect("Failed to execute command");
#[cfg(not(windows))]
let mut info_pipe = Command::new(&ffmpeg_path)
    .args(&vidcommand)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Failed to execute command");

    // windows creation flag CREATE_NO_WINDOW: stops the process from creating a CMD window
    // https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    #[cfg(windows)]
    cmd.creation_flags(0x08000000);

    let mut vid_frame = vec![0u8; (output_width*2 * output_height*2)*3];

    info_pipe.stdout.as_mut().unwrap().read(&mut vid_frame).unwrap();


    let mut xdata = format!("P6 {} {} 255 ", output_width*2, output_height*2).into_bytes();
    xdata.extend_from_slice(&vid_frame);


    info_pipe.kill().expect("Failed to kill ffmpeg process");
    info_pipe.wait_with_output().expect("Failed to wait for process");
    
    // let screenshot_data = Base64Display::new(&xdata, &STANDARD);
    // let encoded_data = format!("data:image/png;base64,{}", screenshot_data);

    // // let my_struct = MyStruct { my_field: 42 };

    // let main_window = app.get_window("main").unwrap();

    // let payload = Payload { picture_data: encoded_data };
    // main_window.emit_all("screenshotEvent", payload).unwrap();




    let screenshot_data = Base64Display::new(&xdata, &STANDARD);
    // let picture_data = format!("data:image/png;base64,{}", screenshot_data);

    // acquire a lock on the mutex
    let mut payload = PAYLOAD.lock().unwrap();

// modify the picture_data field
// payload.picture_data = "some picture data".to_string();
    payload.picture_data = format!("data:image/png;base64,{}", screenshot_data);
    // Payload { picture_data }

    // let payload: Payload;

    // payload.picture_data = encoded_data;

    // emit a custom event to the front-end with the encoded image data as the payload
    // window.emit("screenshot", Payload::picture_data).expect("Failed to emit screenshot event");
}

// #[tauri::command]
// pub fn get_screenshot( window: Window) -> String {
//     let mut payload = PAYLOAD.lock().unwrap();

//     payload.picture_data
// }