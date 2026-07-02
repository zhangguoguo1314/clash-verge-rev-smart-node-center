use std::time::Duration;

/// 下载速度测试
///
/// 下载指定大小的数据并计算速度 (Mbps)。
pub async fn download_speed(url: &str, size: u64, timeout: Duration) -> Result<f64, String> {
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| e.to_string())?;

    let start = std::time::Instant::now();

    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let mut downloaded: u64 = 0;
    let target_size = size;
    let mut buf = vec![0u8; 8192];

    while downloaded < target_size {
        let remaining = target_size - downloaded;
        let to_read = remaining.min(buf.len() as u64) as usize;
        match response.chunk().await {
            Ok(Some(chunk)) => {
                let copy_len = chunk.len().min(to_read);
                buf[..copy_len].copy_from_slice(&chunk[..copy_len]);
                downloaded += copy_len as u64;
            }
            Ok(None) => break, // 流结束
            Err(e) => return Err(e.to_string()),
        }
    }

    let elapsed = start.elapsed().as_secs_f64();

    if elapsed == 0.0 || downloaded == 0 {
        return Err(String::from("no data downloaded or zero elapsed time"));
    }

    // 转换为 Mbps (Megabits per second)
    let speed_mbps = (downloaded as f64 * 8.0) / (elapsed * 1_000_000.0);
    Ok(speed_mbps)
}
