use crate::utils::types::LcuSubscriptionType;
use crate::utils::ws;
use futures_util::StreamExt;
use std::time::Duration;
use tauri::{AppHandle, Emitter, EventTarget};

/// 后台 WebSocket 监听循环：连接 LCU 并持续推送游戏状态事件到前端。
///
/// 工作流程：
/// 1. 建立到 LCU 的 WebSocket 连接
/// 2. 订阅游戏流程阶段变化事件（`/lol-gameflow/v1/gameflow-phase`）
/// 3. 持续接收事件并通过 Tauri 事件总线转发到前端
/// 4. 连接断开时自动重连（指数退避，最大 30 秒）
///
/// 该函数设计为后台任务运行，通常通过 `tauri::async_runtime::spawn` 启动。
pub async fn listen_client(app: AppHandle) {
    let target = EventTarget::labeled("background");

    loop {
        // ── 建立连接 ────────────────────────────────────────
        let mut client = match ws::LcuWebSocketClient::connect(&app).await {
            Ok(c) => c,
            Err(e) => {
                log::warn!("无法连接 LCU WebSocket，5 秒后重试: {e}");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        log::info!("已连接 LCU WebSocket");

        // ── 订阅游戏流程阶段事件 ────────────────────────────
        if let Err(e) = client
            .subscribe(LcuSubscriptionType::JsonApiEvent(
                "/lol-gameflow/v1/gameflow-phase".to_string(),
            ))
            .await
        {
            log::error!("订阅失败，5 秒后重连: {e}");
            tokio::time::sleep(Duration::from_secs(5)).await;
            continue;
        }

        // ── 事件循环 ────────────────────────────────────────
        while let Some(event) = client.next().await {
            log::info!("收到 LCU 事件: {:?}", event);

            if let Err(e) = app.emit_to(target.clone(), "client_status", &event.data) {
                log::error!("向前端推送事件失败: {e}");
                // emit 失败通常意味着前端窗口已关闭，退出整个循环
                return;
            }
        }

        // WebSocket 流结束（连接被服务端关闭或网络中断）
        log::warn!("LCU WebSocket 连接断开，5 秒后重连");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
