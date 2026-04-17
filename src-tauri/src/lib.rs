mod error;
mod handler;
mod rest;
mod utils;

use handler::{
    close_lol_client, get_client_path, init_keyboard, launch_lol, listen_for_client_start,
};

#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            launch_lol,
            close_lol_client,
            listen_for_client_start,
            init_keyboard,
            get_client_path
        ])
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
