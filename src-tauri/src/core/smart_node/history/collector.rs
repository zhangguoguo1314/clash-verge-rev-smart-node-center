use serde::{Deserialize, Serialize};
use smartstring::alias::String;
use std::path::Path;

/// 测速记录
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeedTestRecord {
    /// 节点 UID
    pub node_uid: String,
    /// 时间戳 (ISO 8601)
    pub timestamp: String,
    /// 延迟 (ms)
    pub latency_ms: Option<u64>,
    /// 下载速度 (Mbps)
    pub download_speed_mbps: Option<f64>,
    /// 评分
    pub score: f64,
    /// 测试类型 (tcp / http / download / full)
    pub test_type: String,
}

/// 记录测速结果到 JSONL 文件
///
/// 按日期分文件，每行一条 JSON 记录。
pub fn record_speed_test(file_path: &Path, record: &SpeedTestRecord) {
    use std::io::Write;

    if let Some(parent) = file_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match serde_json::to_string(record) {
        Ok(line) => {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
            {
                let _ = writeln!(file, "{}", line);
            }
        }
        Err(e) => {
            clash_verge_logging::logging!(
                error,
                clash_verge_logging::Type::Core,
                "SNC: failed to serialize speed test record: {}",
                e
            );
        }
    }
}

/// 获取今日测速记录文件路径
pub fn today_record_path(history_dir: &Path) -> std::path::PathBuf {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    history_dir.join(format!("speed_test_{}.jsonl", date))
}
