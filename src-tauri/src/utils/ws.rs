use crate::utils::error::LcuWebsocketError;
use crate::utils::process_info::LeagueClientUx;
use crate::utils::request::{RIOT_CERT, build_tls_config};
use crate::utils::types::{LcuEvent, LcuSubscriptionType};
use futures_util::{SinkExt, Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tauri::{AppHandle, Manager};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::http::header::AUTHORIZATION;
use tokio_tungstenite::{
    Connector, MaybeTlsStream, WebSocketStream, connect_async_tls_with_config, tungstenite,
};

/// 联盟客户端（LCU）WebSocket API的客户端
#[derive(Debug)]
pub struct LcuWebSocketClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl LcuWebSocketClient {
    /// 尝试建立到 LCU WebSocket API 的连接。
    ///
    /// 通过 Tauri 应用状态获取 League 客户端的连接信息（端口与认证令牌），
    /// 使用 [`rustls`] 配置 TLS（仅信任 Riot 自签证书），
    /// 然后通过 WebSocket 安全连接到 `wss://127.0.0.1:{port}`。
    ///
    /// # 参数
    /// - `app`: Tauri 应用句柄，用于从全局状态中获取 [`LeagueClientUx`] 连接信息
    ///
    /// # Errors
    /// - [`LcuWebsocketError::AuthError`] — 请求 URL 或认证头构建失败
    /// - [`LcuWebsocketError::Disconnected`] — WebSocket 连接建立失败（客户端未运行或端口不可达）
    pub async fn connect(app: &AppHandle) -> Result<Self, LcuWebsocketError> {
        // 从 Tauri 状态中获取 League 客户端的端口和认证令牌
        let league_info = app.state::<LeagueClientUx>();

        // 复用缓存的 Riot 自签证书，构建 rustls TLS 配置
        let tls_config = build_tls_config(&*RIOT_CERT);
        let connector = Connector::Rustls(Arc::new(tls_config));

        // 构建 WebSocket 请求 URL
        let mut url = format!("wss://127.0.0.1:{}", league_info.port)
            .into_client_request()
            .map_err(|_| LcuWebsocketError::AuthError)?;

        // 添加 Basic Auth 认证头，令牌来源于 League 客户端进程信息
        url.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", league_info.token))
                .map_err(|_| LcuWebsocketError::AuthError)?,
        );

        // 建立 WebSocket 安全连接，禁用 DNS 名称匹配（自签证书场景）
        let (ws_stream, _response) =
            connect_async_tls_with_config(url, None, false, Some(connector))
                .await
                .map_err(|e| LcuWebsocketError::Disconnected(e.to_string()))?;

        Ok(Self(ws_stream))
    }

    /// 向 LCU WebSocket 订阅指定事件频道。
    ///
    /// 向服务器发送订阅帧 `[5, "<channel>"]`，
    /// 其中 `<channel>` 由 [`LcuSubscriptionType::Display`] 生成。
    ///
    /// # 参数
    /// - `subscription`: 要订阅的事件类型，参见
    ///   [LCU Schema 文档](https://www.mingweisamuel.com/lcu-schema/tool/#/)
    ///
    /// # Errors
    /// - [`LcuWebsocketError::Disconnected`] — 连接已关闭或网络中断
    /// - [`LcuWebsocketError::SendError`] — 发送失败（其他错误）
    pub async fn subscribe(
        &mut self,
        subscription: LcuSubscriptionType,
    ) -> Result<(), LcuWebsocketError> {
        // 使用 serde_json 安全构造 JSON，避免手动拼接带来的注入风险
        // LCU WebSocket 订阅协议：[opcode, "channel_name"]
        let payload = serde_json::json!([5, subscription.to_string()]);

        self.0
            .send(Message::text(payload.to_string()))
            .await
            .map_err(|e| match e {
                tungstenite::Error::ConnectionClosed
                | tungstenite::Error::AlreadyClosed
                | tungstenite::Error::Io(_) => {
                    // 连接级错误：连接已关闭、重复关闭、底层 IO 异常（断网/重置）
                    LcuWebsocketError::Disconnected(e.to_string())
                }
                _ => LcuWebsocketError::SendError,
            })
    }
}

impl Stream for LcuWebSocketClient {
    type Item = LcuEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            return match self.0.poll_next_unpin(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Some(Ok(Message::Text(text)))) => {
                    let Ok(event) = serde_json::from_str::<LcuEvent>(&text) else {
                        continue;
                    };
                    Poll::Ready(Some(event))
                }
                Poll::Ready(Some(Ok(Message::Close(_))) | Some(Err(_)) | None) => Poll::Ready(None),
                _ => continue,
            };
        }
    }
}
