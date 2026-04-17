use crate::utils::request::build_request_client;

/// League-Client(LCU) REST API客户端
pub struct RESTClient {
    port: String,
    client: reqwest::Client,
}

impl RESTClient {
    /// 创建一个新的实例
    ///
    /// # 参数
    ///
    /// * `auth_token` - 认证令牌，用于构建请求客户端
    /// * `port` - 端口号字符串
    ///
    /// # 返回值
    ///
    /// 返回 `anyhow::Result<Self>`，成功时包含新创建的实例
    pub fn new(auth_token: String, port: String) -> anyhow::Result<Self> {
        let client = build_request_client(Some(auth_token));
        Ok(Self { port, client })
    }

    /// 执行HTTP GET请求并返回JSON响应
    ///
    /// # 参数
    /// * `endpoint` - 要请求的API端点路径（不包括主机名和端口）
    ///
    /// # 返回值
    /// * `Ok(serde_json::Value)` - 请求成功时返回解析后的JSON值
    /// * `Err(anyhow::Error)` - HTTP请求失败时返回错误（JSON解析失败不会返回错误）
    ///
    /// # 注意
    /// - 如果JSON解析失败，函数会返回 `serde_json::Value::Null` 而不是错误
    pub async fn get(&self, endpoint: &str) -> Result<serde_json::Value, serde_json::Value> {
        let url = format!("https://127.0.0.1:{}{}", self.port, endpoint);
        log::debug!("[HTTP GET] URL: {}", url);

        // 记录请求开始时间用于性能监控
        let start = std::time::Instant::now();

        let response = self.client.get(&url).send().await.map_err(|e| {
            log::error!("响应错误: {}", e);
            serde_json::Value::Null
        })?;

        let duration = start.elapsed();
        log::debug!(
            "[HTTP GET] 响应时间: {:?}, 状态: {}",
            duration,
            response.status()
        );

        let result = response
            .error_for_status()
            .map_err(|e| {
                log::error!("状态错误: {}", e);
                serde_json::Value::Null
            })?
            .json()
            .await;

        // 处理JSON解析结果，解析失败时返回Null而不是错误
        match result {
            Ok(json_value) => Ok(json_value),
            Err(e) => {
                log::error!("[HTTP GET] JSON解析错误: {:?}", e);
                Ok(serde_json::Value::Null)
            }
        }
    }
}
