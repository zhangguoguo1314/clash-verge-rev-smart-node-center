use std::time::Duration;

/// TCP 延迟测试
///
/// 通过建立 TCP 连接来测量延迟，返回毫秒数。
pub async fn tcp_ping(addr: &str, port: u16, timeout: Duration) -> Result<u64, String> {
    let start = std::time::Instant::now();

    let addr_str = format!("{}:{}", addr, port);
    let connect_result = tokio::time::timeout(timeout, async {
        tokio::net::TcpStream::connect(&addr_str).await
    })
    .await
    .map_err(|_| String::from("tcp ping timeout"))?
    .map_err(|e| e.to_string())?;

    let elapsed = start.elapsed();
    let _ = connect_result; // 连接成功即关闭

    Ok(elapsed.as_millis() as u64)
}
