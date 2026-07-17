use reqwest::header::AUTHORIZATION;
use reqwest::{Certificate, header};
use std::time::Duration;

/// 构建用于与 Riot 客户端本地 API 通信的 HTTP 请求客户端
///
/// # 参数
/// - `auth_token`: 可选的认证令牌，传入 `Some(token)` 时会自动添加 `Authorization: Basic {token}` 请求头
///
/// # 返回值
/// 返回配置好的 `reqwest::Client` 实例，包含：
/// - Riot 本地 API 的自签名证书（跳过公共 CA 验证）
/// - 可选的 Basic Auth 认证头
/// - 3 秒请求超时
pub fn build_request_client(auth_token: Option<String>) -> reqwest::Client {
    // 加载 Riot 客户端本地 API 使用的自签名证书（PEM 格式）
    let cert = Certificate::from_pem(include_bytes!("riotgames.pem")).expect("读取证书失败");

    // 初始化默认请求头
    let mut headers = header::HeaderMap::new();

    // 如果提供了认证令牌，将其作为 Basic Auth 添加到请求头中
    if let Some(token) = auth_token {
        let auth_token = header::HeaderValue::from_str(format!("Basic {}", token).as_str())
            .expect("转化字符串失败");
        headers.insert(AUTHORIZATION, auth_token);
    }

    // 构建请求客户端
    reqwest::ClientBuilder::new()
        .default_headers(headers)    // 注册默认请求头（包含可能的 Auth）
        .add_root_certificate(cert)  // 添加 Riot 本地 API 的自签名根证书
        .timeout(Duration::from_secs(3)) // 设置 3 秒超时（本地 API 响应应很快）
        .build()
        .expect("构建请求客户端失败")
}
