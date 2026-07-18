use crate::handler::tracker::{AppState, launch_lol, start_tracking_loop, sync_tracker_config};
use crate::handler::{invoke_lcu, listen_for_client_start};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

pub mod handler;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            is_enabled: Arc::new(AtomicBool::new(false)),
            is_running: Arc::new(AtomicBool::new(false)),
            dock_side: Arc::new(Mutex::new(String::from("Right"))),
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            sync_tracker_config,
            start_tracking_loop,
            launch_lol,
            listen_for_client_start,
            invoke_lcu
        ])
        .plugin(tauri_plugin_process::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
