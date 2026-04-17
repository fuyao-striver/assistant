use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ProcessInfoError {
    #[error("Riot/League客户端进程没有发现")]
    ProcessNotAvailable,
    #[error("端口号没有找到")]
    PortNotFound,
    #[error("认证令牌没有找到")]
    AuthTokenNotFound,
    #[error("平台ID没有找到")]
    PlatFormIdNotFound,
}

/// Errors for the Websocket connection to the LCU API
#[derive(Error, Debug, Clone)]
pub enum LcuWebSocketError {
    /// The Lcu API can't be reached
    #[error("LCU API 不可达: {0}")]
    LcuNotAvailable(String),

    /// There was an error preparing the authentication credentials for the connection
    #[error("认证错误")]
    AuthError,

    /// There was an error sending a un-/subscribe message to the API
    #[error("发送信息错误")]
    SendError,

    /// The connection was terminated
    #[error("Websocket不能连接: {0}")]
    Disconnected(String),
}
