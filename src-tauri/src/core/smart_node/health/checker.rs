use std::time::Duration;

/// 健康检测原始结果
#[derive(Debug, Clone)]
pub struct RawHealthCheckResult {
    pub healthy: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

/// 健康检测结果 (可序列化)
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HealthCheckResult {
    pub healthy: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

impl From<RawHealthCheckResult> for HealthCheckResult {
    fn from(raw: RawHealthCheckResult) -> Self {
        Self {
            healthy: raw.healthy,
            latency_ms: raw.latency_ms,
            error: raw.error,
        }
    }
}

/// 检测单个节点的健康状态
///
/// 根据 method 选择检测方式:
/// - "tcp": 建立 TCP 连接
/// - "http"/"https": 发送 HTTP 请求
pub async fn check_node_health(
    addr: &str,
    port: u16,
    method: &str,
    url: Option<&str>,
    timeout: Duration,
) -> HealthCheckResult {
    let result = match method.to_lowercase().as_str() {
        "tcp" => check_tcp(addr, port, timeout).await,
        "http" | "https" => {
            if let Some(check_url) = url {
                check_http(check_url, timeout).await
            } else {
                RawHealthCheckResult {
                    healthy: false,
                    latency_ms: None,
                    error: Some(String::from("no URL provided for HTTP health check")),
                }
            }
        }
        _ => RawHealthCheckResult {
            healthy: false,
            latency_ms: None,
            error: Some(format!("unsupported check method: {}", method)),
        },
    };

    result.into()
}

async fn check_tcp(addr: &str, port: u16, timeout: Duration) -> RawHealthCheckResult {
    let start = std::time::Instant::now();
    let addr_str = format!("{}:{}", addr, port);

    match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(&addr_str)).await {
        Ok(Ok(_stream)) => {
            let elapsed = start.elapsed().as_millis() as u64;
            RawHealthCheckResult {
                healthy: true,
                latency_ms: Some(elapsed),
                error: None,
            }
        }
        Ok(Err(e)) => RawHealthCheckResult {
            healthy: false,
            latency_ms: None,
            error: Some(e.to_string()),
        },
        Err(_) => RawHealthCheckResult {
            healthy: false,
            latency_ms: None,
            error: Some(String::from("tcp health check timeout")),
        },
    }
}

async fn check_http(url: &str, timeout: Duration) -> RawHealthCheckResult {
    let start = std::time::Instant::now();

    let client = match reqwest::Client::builder()
        .timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return RawHealthCheckResult {
                healthy: false,
                latency_ms: None,
                error: Some(e.to_string()),
            }
        }
    };

    match client.head(url).send().await {
        Ok(response) => {
            let status = response.status();
            let elapsed = start.elapsed().as_millis() as u64;
            // 2xx/3xx 视为健康
            let healthy = status.is_success() || status.is_redirection();
            RawHealthCheckResult {
                healthy,
                latency_ms: Some(elapsed),
                error: if healthy {
                    None
                } else {
                    Some(format!("HTTP status: {}", status))
                },
            }
        }
        Err(e) => RawHealthCheckResult {
            healthy: false,
            latency_ms: None,
            error: Some(e.to_string()),
        },
    }
}
