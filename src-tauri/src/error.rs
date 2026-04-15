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
