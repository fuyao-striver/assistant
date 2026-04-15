use base64::{Engine, engine::general_purpose};
use sysinfo::System;

use crate::error::ProcessInfoError;

const TARGET_PROCESS: &str = "LeagueClientUx.exe";

/// 认证响应结构体
///
/// 包含认证成功后返回的令牌、端口和区域信息
pub struct AuthResponse {
    /// 认证令牌字符串
    pub token: String,
    /// 服务端口字符串
    pub port: String,
    /// 服务区域字符串
    pub region: String,
}

/// 获取认证信息
///
/// 该函数通过查找系统中运行的目标进程，从其命令行参数中提取认证相关的配置信息，
/// 包括端口号、认证令牌和平台ID，并将其封装成AuthResponse结构体返回。
///
/// # Returns
/// * `Ok(AuthResponse)` - 成功时返回包含认证信息的响应对象
/// * `Err(ProcessInfoError)` - 失败时返回相应的错误信息，可能的原因包括：
///   - 目标进程不存在
///   - 端口号参数未找到
///   - 认证令牌参数未找到
///   - 平台ID参数未找到
pub fn get_auth_info() -> Result<AuthResponse, ProcessInfoError> {
    // 创建系统信息实例并刷新所有进程信息
    let mut sys = System::new_all();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    // 查找目标进程并获取其命令行参数
    let args = sys
        .processes()
        .values()
        .find(|p| p.name() == TARGET_PROCESS)
        .map(|p| p.cmd())
        .ok_or(ProcessInfoError::ProcessNotAvailable)?;

    // 从命令行参数中解析端口号
    let port = args
        .iter()
        .find(|arg| arg.to_string_lossy().starts_with("--app-port="))
        .map(|arg| {
            arg.to_string_lossy()
                .strip_prefix("--app-port=")
                .unwrap()
                .to_string()
        })
        .ok_or(ProcessInfoError::PortNotFound)?;

    // 从命令行参数中解析认证令牌
    let auth_token = args
        .iter()
        .find(|arg| arg.to_string_lossy().starts_with("--remoting-auth-token="))
        .map(|arg| {
            arg.to_string_lossy()
                .strip_prefix("--remoting-auth-token=")
                .unwrap()
                .to_string()
        })
        .ok_or(ProcessInfoError::AuthTokenNotFound)?;

    // 从命令行参数中解析RSO平台ID
    let rso_platform_id = args
        .iter()
        .find(|arg| arg.to_string_lossy().starts_with("--rso_platform_id="))
        .map(|arg| {
            arg.to_string_lossy()
                .strip_prefix("--rso_platform_id=")
                .unwrap()
                .to_string()
        })
        .ok_or(ProcessInfoError::PlatFormIdNotFound)?;

    // 构造认证响应对象，将认证令牌用riot前缀编码
    Ok(AuthResponse {
        token: general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
        port,
        region: rso_platform_id,
    })
}
