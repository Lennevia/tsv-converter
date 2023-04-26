#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
#[cfg(target_os = "macos")]
mod macos;

fn main() {
    tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![
            commands::metadata,
            commands::output_name,
            commands::convert_avi,
            commands::convert_mini_avi,
            commands::convert_diy_avi,
            // commands::screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("Error running the application");
}
