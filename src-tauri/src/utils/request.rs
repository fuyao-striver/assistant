use std::io::BufReader;
use std::sync::LazyLock;
use std::time::Duration;

use reqwest::header;
use reqwest::header::AUTHORIZATION;
use rustls::RootCertStore;
use rustls::pki_types::CertificateDer;

/// Riot 自签证书（PEM → DER），首次访问时解析并缓存
pub static RIOT_CERT: LazyLock<CertificateDer<'static>> = LazyLock::new(|| {
    let mut reader = BufReader::new(include_bytes!("riotgames.pem").as_slice());
    rustls_pemfile::certs(&mut reader)
        .next()
        .expect("证书文件为空")
        .expect("解析第一个证书失败")
});

/// 构建用于与 Riot 客户端本地 API 通信的 HTTP 请求客户端。
///
/// 底层 TLS 使用 [`rustls`]，仅信任 Riot 自签证书（不加载系统证书链）。
///
/// # 参数
/// - `auth_token`: 可选的认证令牌，传入 `Some(token)` 时会自动添加
///   `Authorization: Basic {token}` 请求头。
///
/// # 返回值
/// 返回配置好的 [`reqwest::Client`] 实例，包含：
/// - 通过 [`rustls::ClientConfig`] 注入 Riot 自签证书
/// - 可选的 Basic Auth 认证头
/// - 3 秒请求超时
pub fn build_request_client(auth_token: Option<&str>) -> reqwest::Client {
    let mut headers = header::HeaderMap::new();

    if let Some(token) = auth_token {
        let val = header::HeaderValue::from_str(&format!("Basic {token}")).expect("转化字符串失败");
        headers.insert(AUTHORIZATION, val);
    }

    let tls_config = build_tls_config(&*RIOT_CERT);

    reqwest::Client::builder()
        .default_headers(headers)
        .use_preconfigured_tls(tls_config) // 注入 rustls ClientConfig
        .timeout(Duration::from_secs(3))
        .build()
        .expect("构建请求客户端失败")
}

/// 基于 Riot 自签证书构建 [`rustls::ClientConfig`]。
///
/// 使用空的 [`RootCertStore`] 并仅添加 Riot 证书，
/// 不信任任何公共 CA，避免中间人攻击的风险。
pub fn build_tls_config(cert: &CertificateDer<'static>) -> rustls::ClientConfig {
    let mut root_store = RootCertStore::empty();
    root_store.add(cert.clone()).expect("添加根证书失败");

    rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}
