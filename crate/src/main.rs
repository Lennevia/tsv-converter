#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]
// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

mod commands;
#[cfg(target_os = "macos")]
mod macos;

use tauri::Manager;
use crate::commands::PAYLOAD;







fn main() {
    tauri::Builder::default()
    .setup(|app| {
        // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
        // the default value is `main`. note that it must be unique
        let main_window = app.get_window("main").unwrap();

        // Grab the picture payload from the screenshot function and emit it back to the svelte frontend
        // let payload = commands::get_screenshot(main_window);
        // let picture_data = payload.picture_data;
        let mut payload = PAYLOAD.lock().unwrap();
        let picture_data_ref = &payload.picture_data;

        main_window.emit_all("screenshotEvent", picture_data_ref).unwrap();
  
        // listen to the `event-name` (emitted on the `main` window)
        // let id = main_window.listen("screenshotEvent", |event| {
        //   println!("got window screenshot with payload {:?}", event.payload());
        // });
        // // unlisten to the event using the `id` returned on the `listen` function
        // // an `once` API is also exposed on the `Window` struct
        // main_window.unlisten(id);
  
        // emit the `event-name` event to the `main` window
        // main_window.emit_all("screenshotEvent", commands::Payload { picture_data: "some data".to_string() }).unwrap();
        Ok(())
      })
    //   .invoke_handler(tauri::generate_handler![init_process])
        .invoke_handler(tauri::generate_handler![
            commands::metadata,
            // commands::watch,
            commands::output_name,
            // commands::convert,
            commands::convert_avi,
            commands::convert_mini_avi,
            commands::convert_diy_avi,
            // commands::video_uri,
            commands::screenshot,
            // commands::get_screenshot,
        ])

        .run(tauri::generate_context!())
        .expect("Error running the application");
}
