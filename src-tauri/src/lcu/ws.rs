use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{SinkExt, Stream, StreamExt};
use tauri::http::HeaderValue;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    Connector, MaybeTlsStream, WebSocketStream,
    tungstenite::{self, Message, client::IntoClientRequest},
};

use crate::{
    error::LcuWebSocketError,
    lcu::types::{LcuEvent, LcuSubscriptionType},
    utils::process::{self, AuthResponse},
};

#[derive(Debug)]
pub struct LcuWebSocketClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl LcuWebSocketClient {
    /// 连接到Lcu WebSocket服务
    ///
    /// 该函数会从进程信息中获取认证信息，建立TLS连接并进行身份验证
    ///
    /// # 参数
    /// 无
    ///
    /// # 返回值
    /// * `Result<Self, LcuWebSocketError>` - 成功时返回LcuWebSocket实例，失败时返回相应的错误
    ///
    /// # 错误
    /// 返回[LcuWebsocketError]如果API不可达或连接过程中出现其他错误
    pub async fn connect() -> Result<Self, LcuWebSocketError> {
        // 获取认证信息，包括token和端口
        let AuthResponse {
            token: auth_token,
            port,
            ..
        } = process::get_auth_info()
            .map_err(|e| LcuWebSocketError::LcuNotAvailable(e.to_string()))?;

        // 加载Riot Games证书并创建TLS连接器
        let cert = native_tls::Certificate::from_pem(include_bytes!("../utils/riotgames.pem"))
            .expect("证书加载错误！");
        let tls = native_tls::TlsConnector::builder()
            .add_root_certificate(cert)
            .build()
            .expect("tls构建错误!");
        let connector = Connector::NativeTls(tls);

        // 构建WebSocket连接URL并设置认证头
        let mut url = format!("wss://127.0.0.1:{port}")
            .into_client_request()
            .map_err(|_| LcuWebSocketError::AuthError)?;
        url.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(format!("Basic {auth_token}").as_str())
                .map_err(|_| LcuWebSocketError::AuthError)?,
        );

        // 建立WebSocket连接
        let (ws_stream, _response) =
            tokio_tungstenite::connect_async_tls_with_config(url, None, false, Some(connector))
                .await
                .map_err(|e| LcuWebSocketError::Disconnected(e.to_string()))?;

        Ok(Self(ws_stream))
    }

    /// <https://www.mingweisamuel.com/lcu-schema/tool/#/>
    /// 订阅LCU WebSocket事件
    ///
    /// # 参数
    /// * `subscription` - 要订阅的LCU事件类型
    ///
    /// # 返回值
    /// * `Ok(())` - 订阅成功
    /// * `Err(LcuWebSocketError)` - 订阅失败，可能的原因包括连接关闭或发送错误
    pub async fn subscribe(
        &mut self,
        subscription: LcuSubscriptionType,
    ) -> Result<(), LcuWebSocketError> {
        // 发送订阅消息到WebSocket连接，并处理可能的连接错误
        self.0
            .send(Message::text(format!("[5, \"{subscription}\"]")))
            .await
            .map_err(|e| match e {
                tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => {
                    LcuWebSocketError::Disconnected(e.to_string())
                }
                _ => LcuWebSocketError::SendError,
            })
    }
}

impl Stream for LcuWebSocketClient {
    type Item = LcuEvent;

    /// 异步轮询获取下一个LCU事件
    ///
    /// 该函数从WebSocket连接中轮询消息，解析JSON格式的文本消息为LCU事件，
    /// 并过滤掉非文本消息、关闭消息和错误消息
    ///
    /// # 参数
    /// * `self`: Pin到可变引用的Self类型，用于异步状态机实现
    /// * `cx`: 上下文引用，用于异步任务的唤醒机制
    ///
    /// # 返回值
    /// * `Poll<Option<Self::Item>>`: 返回准备好的LCU事件或Pending状态
    ///   - `Poll::Ready(Some(event))`: 成功解析并返回一个LCU事件
    ///   - `Poll::Ready(None)`: 连接关闭或出现错误，流结束
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(Message::Text(text)))) => {
                // 解析JSON文本消息为LCU事件对象
                match serde_json::from_str::<LcuEvent>(&text) {
                    Ok(event) => Poll::Ready(Some(event)),
                    Err(e) => {
                        // JSON解析失败，记录错误并继续等待下一个消息
                        log::warn!("Failed to parse LCU event JSON: {}", e);
                        // 重新调用poll_next以获取下一条消息
                        self.as_mut().poll_next(cx)
                    }
                }
            }
            // 处理关闭消息、错误消息或连接终止的情况
            Poll::Ready(Some(Ok(Message::Close(_))) | Some(Err(_)) | None) => Poll::Ready(None),
            // 其他类型的消息（如二进制消息）则跳过并尝试下一条
            Poll::Ready(Some(Ok(_))) => self.as_mut().poll_next(cx),
        }
    }
}
