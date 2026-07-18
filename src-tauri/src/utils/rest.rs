use crate::utils::request::build_request_client;
use serde::Serialize;

/// 联盟客户端（LCU）REST API的客户端
pub struct RestClient {
    port: String,
    client: reqwest::Client,
}

impl RestClient {
    /// 创建LCU REST包装器的新实例
    pub fn new(auth_token: &str, port: String) -> Self {
        let client = build_request_client(Some(auth_token));
        Self { client, port }
    }

    /// 向指定端点发出get请求
    pub async fn get(&self, endpoint: &str) -> Result<serde_json::Value, reqwest::Error> {
        self.client
            .get(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }

    /// 向指定端点发出post请求
    pub async fn post<T: Serialize>(
        &self,
        endpoint: &str,
        body: T,
    ) -> Result<serde_json::Value, reqwest::Error> {
        self.client
            .post(format!("https://127.0.0.1:{}{}", self.port, endpoint))
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .or_else(|_| Ok(serde_json::Value::Null))
    }
}
