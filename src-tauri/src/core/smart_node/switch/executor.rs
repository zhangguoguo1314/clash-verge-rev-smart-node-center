use clash_verge_logging::{Type, logging};
use serde_json;

/// 执行节点切换
///
/// 调用 mihomo 内核的 API 切换指定分组的代理节点。
pub async fn execute_switch(
    group_name: &str,
    node_uid: &str,
    node_name: &str,
) -> Result<(), String> {
    logging!(
        info,
        Type::Core,
        "SNC: switching group '{}' to node '{}' (uid: {})",
        group_name,
        node_name,
        node_uid
    );

    // 使用 mihomo 插件切换节点
    let handle = crate::core::handle::Handle::app_handle();
    let mihomo = handle.mihomo().await;

    match mihomo.select_node_for_group(group_name, node_name).await {
        Ok(_) => {
            logging!(
                info,
                Type::Core,
                "SNC: successfully switched group '{}' to node '{}'",
                group_name,
                node_name
            );

            // 发送切换事件通知
            crate::core::smart_node::notification::SncNotification::emit_node_switched(
                &serde_json::json!({
                    "group_name": group_name,
                    "node_uid": node_uid,
                    "node_name": node_name,
                }),
            );

            // 记录切换历史
            record_switch_history(group_name, node_uid, node_name);

            Ok(())
        }
        Err(e) => {
            let err_msg = format!(
                "SNC: failed to switch group '{}' to node '{}': {}",
                group_name, node_name, e
            );
            logging!(error, Type::Core, "{}", err_msg);
            Err(err_msg)
        }
    }
}

/// 记录切换历史到历史数据文件
fn record_switch_history(group_name: &str, node_uid: &str, node_name: &str) {
    use crate::config::smart_node::ISmartNodeConfig;
    use std::io::Write;

    if let Ok(dir) = ISmartNodeConfig::config_dir() {
        let history_dir = dir.join("history");
        let _ = std::fs::create_dir_all(&history_dir);

        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let file_path = history_dir.join(format!("switch_{}.jsonl", date));

        let record = serde_json::json!({
            "type": "switch",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "group_name": group_name,
            "node_uid": node_uid,
            "node_name": node_name,
        });

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
        {
            let _ = writeln!(file, "{}", record);
        }
    }
}
