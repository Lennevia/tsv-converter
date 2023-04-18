#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
#[cfg(target_os = "macos")]
mod macos;

// use crate::commands::PAYLOAD;
// use tauri::Manager;

fn main() {
    tauri::Builder::default()
        // .setup(|app| {
        //     // let id = app.listen_global("screenshotEvent", |event| {
        //     //     println!("got screenShotEvent");
        //     // });
        //     // // unlisten to the event using the `id` returned on the `listen_global` function
        //     // app.unlisten(id);
        //     // // Grab the picture payload and emit a reference to it back to the svelte frontend
        //     // let mut payload = PAYLOAD.lock().unwrap();
        //     // let picture_data_ref = &payload.picture_data;
        //     // println!("mystring of data: {}", payload.picture_data); // prints empty string at yarn dev initialization and then doesn't emit again
        //     // app.emit_all("screenshotEvent", picture_data_ref).unwrap();
        //     // Ok(())
        // })
        // .invoke_handler(tauri::generate_handler![init_process])
        .invoke_handler(tauri::generate_handler![
            commands::metadata,
            commands::output_name,
            commands::convert_avi,
            commands::convert_mini_avi,
            commands::convert_diy_avi,
            commands::screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("Error running the application");
}
