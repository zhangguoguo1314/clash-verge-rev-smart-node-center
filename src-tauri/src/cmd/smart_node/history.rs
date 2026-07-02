use crate::cmd::StringifyErr;
use crate::config::smart_node::ISmartNodeConfig;
use crate::core::smart_node::history::{cleanup_old_files, today_record_path};
use crate::core::smart_node::notification::SncNotification;
use clash_verge_logging::{Type, logging};
use std::path::PathBuf;

/// 获取今日测速历史记录
#[tauri::command]
pub async fn snc_get_today_history() -> crate::cmd::CmdResult<serde_json::Value> {
    let dir = ISmartNodeConfig::config_dir().map_err(|e| e.to_string())?;
    let history_dir = dir.join("history");
    let file_path = today_record_path(&history_dir);

    if !file_path.exists() {
        return Ok(serde_json::json!({
            "records": [],
            "total": 0,
        }));
    }

    let content = tokio::fs::read_to_string(&file_path)
        .await
        .map_err(|e| e.to_string())?;

    let mut records = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(record) = serde_json::from_str::<serde_json::Value>(line) {
            records.push(record);
        }
    }

    // 按时间倒序
    records.reverse();

    Ok(serde_json::json!({
        "records": records,
        "total": records.len(),
        "date": chrono::Local::now().format("%Y-%m-%d").to_string(),
    }))
}

/// 手动清理历史数据
#[tauri::command]
pub async fn snc_cleanup_history(retention_days: Option<u32>) -> crate::cmd::CmdResult<serde_json::Value> {
    let dir = ISmartNodeConfig::config_dir().map_err(|e| e.to_string())?;
    let history_dir = dir.join("history");

    let days = retention_days.unwrap_or(30);

    let deleted = cleanup_old_files(&history_dir, days).map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "deleted_count": deleted,
        "retention_days": days,
    }))
}
