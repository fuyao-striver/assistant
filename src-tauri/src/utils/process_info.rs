use crate::utils::error::ProcessInfoError;
use base64::Engine;
use base64::engine::general_purpose;
use std::ffi::OsString;
use sysinfo::System;

/// 英雄联盟程序进程名称
const TARGET_PROCESS: &str = "LeagueClientUx.exe";

/// 英雄联盟客户端进程信息
///
/// 通过解析客户端启动参数获取连接所需的认证凭据与网络配置，
/// 用于与本地 LCU（League Client Update）API 进行通信。
pub struct LeagueClientUx {
    /// LCU API 的认证令牌（Remoting Auth Token）
    ///
    /// 由客户端启动时随机生成，作为请求头 `Authorization` 的凭据。
    pub token: String,

    /// LCU API 监听的本地端口号（Remoting Auth Port）
    ///
    /// 客户端每次启动时随机分配，结合 `token` 即可发起 `https://127.0.0.1:{port}` 请求。
    pub port: String,

    /// 当前登录账号所属的服务器区域标识
    ///
    /// 例如 `"KR"`, `"NA1"`, `"EUW1"` 等，用于确定 API 的区域路由与数据上下文。
    pub region: String,
}

/// 从系统进程中获取当前运行的英雄联盟客户端连接信息
///
/// # 执行流程
///
/// 1. 通过 `sysinfo` 遍历系统所有进程，按名称匹配 `TARGET_PROCESS`（Riot/League Client）；
/// 2. 从匹配进程的命令行参数中依次提取：
///    - `--app-port` → LCU API 监听端口
///    - `--remoting-auth-token` → API 认证令牌原始值
///    - `--rso_platform_id` → 服务器区域标识
/// 3. 将认证令牌按 `riot:{token}` 格式进行 Base64 编码，作为 HTTP Basic Auth 的 `Authorization` 头值。
///
/// # 返回值
///
/// - `Ok(LeagueClientUx)` — 成功解析到全部必要信息，可直接用于构建 LCU API 请求。
/// - `Err(ProcessInfoError)` — 任一环节失败时返回对应错误，详见 [`ProcessInfoError`]。
///
/// # 注意事项
///
/// - 客户端必须已在运行，否则会返回 [`ProcessInfoError::ProcessNotAvailable`]。
/// - 客户端的启动参数由 Riot Client 在拉起进程时动态注入，用户无法手动指定。
/// - Base64 编码使用标准字母表（`STANDARD`），对应 HTTP Basic Auth 的要求。
pub fn get_league_client_info() -> Result<LeagueClientUx, ProcessInfoError> {
    // 获取系统全量进程快照（包含名称与命令行参数）
    let sys = System::new_all();

    // 按进程名定位 League Client，并提取其命令行参数列表
    let args = sys
        .processes()
        .values()
        .find(|p| p.name() == TARGET_PROCESS)
        .map(|p| p.cmd())
        .ok_or(ProcessInfoError::ProcessNotAvailable)?;

    // 从命令行参数中解析 --app-port
    let port = extract_arg(args, "--app-port=").ok_or(ProcessInfoError::PortNotFound)?;

    // 从命令行参数中解析 --remoting-auth-token
    let auth_token =
        extract_arg(args, "--remoting-auth-token=").ok_or(ProcessInfoError::AuthTokenNotFound)?;

    // 从命令行参数中解析 --rso_platform_id
    let region =
        extract_arg(args, "--rso_platform_id=").ok_or(ProcessInfoError::PlatformIdNotFound)?;

    // 将认证信息编码为 Basic Auth 格式：Base64("riot:{raw_token}")
    Ok(LeagueClientUx {
        token: general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        port,
        region,
    })
}

/// 从命令行参数列表中提取指定前缀的值
///
/// 在 `args` 中查找以 `prefix` 开头的参数，剥离前缀后返回其值。
fn extract_arg(args: &[OsString], prefix: &str) -> Option<String> {
    args.iter()
        .find(|arg| arg.to_string_lossy().starts_with(prefix))
        .map(|arg| {
            arg.to_string_lossy()
                .strip_prefix(prefix)
                .unwrap_or_else(|| panic!("无法剥离{}前缀", prefix))
                .to_string()
        })
}
