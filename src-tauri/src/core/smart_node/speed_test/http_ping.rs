use std::time::Duration;

/// HTTP/HTTPS 延迟测试
///
/// 发送 HTTP GET 请求测量延迟，返回毫秒数。
pub async fn http_ping(url: &str, timeout: Duration) -> Result<u64, String> {
    let start = std::time::Instant::now();

    let client = reqwest::Client::builder()
        .timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    let result = client
        .head(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let _ = result.error_for_status_ref().map_err(|e| e.to_string())?;

    let elapsed = start.elapsed();
    Ok(elapsed.as_millis() as u64)
}
