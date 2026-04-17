use futures_util::StreamExt;
use tauri::{AppHandle, Emitter, EventTarget};

use crate::lcu::{types::LcuSubscriptionType, ws};

/// 监听LCU客户端事件并转发到前端
///
/// 该函数建立与LCU(League Client Update) WebSocket连接，订阅游戏流程阶段变化事件，
/// 并将接收到的事件数据通过应用句柄发送到标记为"background"的目标。
///
/// # 参数
/// * `app` - 应用程序句柄，用于向前端发送事件数据
///
/// # 返回值
/// 该函数返回一个Future，在异步运行时中执行，无实际返回值
pub async fn listen_client(app: AppHandle) {
    // 建立LCU WebSocket客户端连接
    let mut client = ws::LcuWebSocketClient::connect()
        .await
        .expect("LCU连接失败！");

    // 订阅游戏流程阶段变化事件
    client
        .subscribe(LcuSubscriptionType::JsonApiEvent(
            "/lol-gameflow/v1/gameflow-phase".to_string(),
        ))
        .await
        .expect("订阅失败!");

    // 持续监听并处理事件
    while let Some(event) = client.next().await {
        log::info!("Event: {:?}", event);
        app.emit_to(
            EventTarget::labeled("background"),
            "client_status",
            event.data,
        )
        .expect("发送失败！");
    }
}
