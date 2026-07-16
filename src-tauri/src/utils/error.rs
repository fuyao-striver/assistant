use thiserror::Error;

/// 获取英雄联盟客户端进程信息时可能发生的错误
///
/// 在通过系统进程列表定位 Riot/League 客户端并解析其启动参数的过程中，
/// 任何一个必要信息缺失都会导致对应的错误变体被返回。
#[derive(Error, Debug)]
pub enum ProcessInfoError {
    /// 未找到 Riot Client 或 League Client 的运行进程
    ///
    /// 在当前系统的进程列表中未检测到目标进程，通常意味着客户端尚未启动，
    /// 或进程名称与预期不匹配。
    #[error("找不到Riot/League客户端流程")]
    ProcessNotAvailable,

    /// 无法从进程启动参数中解析出 LCU API 监听端口
    ///
    /// 对应的命令行参数 `--app-port` 缺失或格式异常，
    /// 导致无法确定 `https://127.0.0.1:{port}` 中的目标端口。
    #[error("无法从进程参数分析API端口")]
    PortNotFound,

    /// 无法从进程启动参数中解析出 API 认证令牌
    ///
    /// 对应的命令行参数 `--remoting-auth-token` 缺失，
    /// 后续所有 LCU API 请求将无法通过 `Authorization` 头的校验。
    #[error("无法从进程参数分析API身份验证令牌")]
    AuthTokenNotFound,

    /// 无法从进程启动参数中解析出 RSO 平台标识
    ///
    /// 对应的命令行参数 `--rso_platform_id`（如 `"KR"`, `"NA1"` 等）缺失，
    /// 无法确定当前客户端所连接的服务器区域，影响 API 路由与数据上下文的构建。
    #[error("无法从进程参数分析RSO平台ID")]
    PlatformIdNotFound,
}
