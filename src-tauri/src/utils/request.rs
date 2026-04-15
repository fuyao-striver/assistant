use std::time::Duration;

use reqwest::{Certificate, header};

/// 构建HTTP请求客户端
///
/// 创建一个配置好的reqwest客户端，用于发送HTTP请求。
/// 支持可选的身份验证令牌和自定义证书配置。
///
/// # 参数
///
/// * `auth_token` - 可选的身份验证令牌字符串。如果提供，将作为Basic认证头添加到请求中
///
/// # 返回值
///
/// 返回配置好的reqwest::Client实例
pub fn build_request_client(auth_token: Option<String>) -> reqwest::Client {
    // 加载自定义根证书用于SSL验证
    let cert = Certificate::from_pem(include_bytes!("riotgames.pem")).expect("证书加载错误！");

    // 初始化请求头映射
    let mut headers = header::HeaderMap::new();

    // 如果提供了认证令牌，则添加Authorization头部
    if let Some(token) = auth_token {
        let auth_header = header::HeaderValue::from_str(format!("Basic {}", token).as_str())
            .expect("创建请求头失败！");
        headers.insert("Authorization", auth_header);
    }

    // 构建并配置HTTP客户端
    reqwest::ClientBuilder::new()
        .add_root_certificate(cert)
        .default_headers(headers)
        .timeout(Duration::from_secs(3))
        .build()
        .expect("构建客户端失败！")
}
