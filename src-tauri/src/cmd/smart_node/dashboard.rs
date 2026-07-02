use crate::cmd::StringifyErr;
use crate::config::smart_node::ISmartNodeConfig;
use crate::core::smart_node::SncManager;
use clash_verge_logging::{Type, logging};

/// 获取 Dashboard 所需的汇总数据
#[tauri::command]
pub async fn snc_get_dashboard_data() -> crate::cmd::CmdResult<serde_json::Value> {
    let config = SncManager::config().await;
    let config_data = config.data_arc();

    let pool = crate::config::smart_node::ISmartNodePool::load_file().await;
    let stats = pool.get_stats();

    // 获取 SNC 运行状态
    let state = SncManager::global().get_state();
    let state_str = match state {
        crate::core::smart_node::manager::SncState::Disabled => "disabled",
        crate::core::smart_node::manager::SncState::Starting => "starting",
        crate::core::smart_node::manager::SncState::Running => "running",
        crate::core::smart_node::manager::SncState::Stopping => "stopping",
    };

    // 获取分组数量
    let groups = crate::config::smart_node::ISmartNodeGroups::load_file().await;
    let group_count = groups.groups.len();

    // 计算平均延迟和速度
    let healthy_nodes: Vec<_> = pool.nodes.iter().filter(|n| n.status == crate::config::smart_node::NodeStatus::Healthy).collect();
    let avg_latency = if healthy_nodes.is_empty() {
        None
    } else {
        let sum: u64 = healthy_nodes.iter().filter_map(|n| n.latency_ms).sum();
        let count = healthy_nodes.iter().filter(|n| n.latency_ms.is_some()).count();
        if count > 0 {
            Some(sum as f64 / count as f64)
        } else {
            None
        }
    };

    let avg_speed = if healthy_nodes.is_empty() {
        None
    } else {
        let sum: f64 = healthy_nodes.iter().filter_map(|n| n.download_speed_mbps).sum();
        let count = healthy_nodes.iter().filter(|n| n.download_speed_mbps.is_some()).count();
        if count > 0 {
            Some(sum / count as f64)
        } else {
            None
        }
    };

    Ok(serde_json::json!({
        "state": state_str,
        "enabled": config_data.enabled.unwrap_or(false),
        "stats": {
            "total_nodes": stats.total_count,
            "healthy_nodes": stats.healthy_count,
            "unhealthy_nodes": stats.unhealthy_count,
            "unknown_nodes": stats.unknown_count,
            "groups_count": group_count,
        },
        "performance": {
            "avg_latency_ms": avg_latency,
            "avg_speed_mbps": avg_speed,
        },
        "config": {
            "dashboard_refresh_interval": config_data.dashboard.refresh_interval_seconds,
            "show_media_detect": config_data.dashboard.show_media_detect,
        },
    }))
}
