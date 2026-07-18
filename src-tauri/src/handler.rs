use crate::utils::process_info::get_league_client_info;
use crate::utils::rest::RestClient;
use serde_json::Value;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

pub mod lcu;
pub mod tracker;

#[tauri::command]
pub async fn invoke_lcu(
    method: &str,
    uri: &str,
    body: &str,
    app: AppHandle,
) -> Result<Value, Value> {
    let client = app.state::<RestClient>();
    match method {
        "get" => match client.get(uri).await {
            Ok(result) => Ok(result),
            Err(_) => Err(Value::Null),
        },
        "post" => {
            match serde_json::from_str::<Value>(body) {
                Ok(result) => Ok(client.post(uri, result).await.expect("post请求失败")),
                Err(_) => Ok(client.post(uri,Value::Null).await.expect("post请求失败")),
            }
        }
        _ => Ok(Value::Null),
    }
}

#[tauri::command]
pub fn listen_for_client_start(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let start_time = Instant::now();
        let time_out = Duration::from_secs(180);
        loop {
            // 获取客户端信息
            if let Ok(value) = get_league_client_info() {
                let rest_client = RestClient::new(&value.token, value.port.clone());
                app.manage(rest_client);
                app.manage(value);
                app.emit_to("background", "client_status", "ClientStarted")
                    .expect("sent background error");
                break;
            }
            // 超过指定的超时时间则退出
            if start_time.elapsed() > time_out {
                log::error!("客户端启动超时，未能获取信息。");
                break;
            }
            // 每隔一段时间重新检查
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });
}
