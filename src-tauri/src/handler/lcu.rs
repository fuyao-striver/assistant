use crate::handler::listener::listen_client;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_listener(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        listen_client(app).await;
    });
}
