use crate::cmd::StringifyErr;
use crate::config::smart_node::{ISmartNodeConfig, ISmartNodePool};
use crate::core::smart_node::health::check_node_health;
use crate::core::smart_node::notification::SncNotification;
use crate::feat::smart_node::pool::update_node_status;
use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging};
use std::time::Duration;
use tokio::sync::OnceCell as TokioOnceCell;

/// 获取全局 Pool Draft
async fn get_pool_draft() -> Draft<ISmartNodePool> {
    static POOL_DRAFT: TokioOnceCell<Draft<ISmartNodePool>> = TokioOnceCell::const_new();
    POOL_DRAFT
        .get_or_init(|| async {
            let pool = ISmartNodePool::load_file().await;
            Draft::new(pool)
        })
        .await
        .clone()
}

/// 检查单个节点的健康状态
#[tauri::command]
pub async fn snc_check_node_health(uid: String) -> crate::cmd::CmdResult<serde_json::Value> {
    let config = crate::core::smart_node::SncManager::config().await;
    let config_data = config.data_arc();
    let pool = get_pool_draft().await;

    let node = pool
        .data_arc()
        .nodes
        .iter()
        .find(|n| n.uid == uid)
        .cloned()
        .ok_or_else(|| format!("node {} not found", uid))?;

    let timeout = Duration::from_millis(config_data.health_check.timeout_ms);
    let check_url = if config_data.health_check.check_url.is_empty() {
        None
    } else {
        Some(config_data.health_check.check_url.as_str())
    };

    let result = check_node_health(
        &node.address,
        node.port,
        &config_data.health_check.check_method,
        check_url,
        timeout,
    )
    .await;

    // 更新节点状态
    let status = if result.healthy {
        crate::config::smart_node::NodeStatus::Healthy
    } else {
        crate::config::smart_node::NodeStatus::Unhealthy
    };
    update_node_status(&uid, status, result.latency_ms, &pool);

    // 保存
    {
        let pool_data = pool.data_arc();
        let _ = pool_data.save_file().await;
    }

    let json_result = serde_json::json!({
        "uid": uid,
        "healthy": result.healthy,
        "latency_ms": result.latency_ms,
        "error": result.error,
    });

    SncNotification::emit_health_check_completed(&json_result);

    Ok(json_result)
}

/// 对所有节点执行健康检查
#[tauri::command]
pub async fn snc_check_all_health() -> crate::cmd::CmdResult<serde_json::Value> {
    let config = crate::core::smart_node::SncManager::config().await;
    let config_data = config.data_arc();
    let pool = get_pool_draft().await;

    let nodes = pool.data_arc().nodes.clone();
    let timeout = Duration::from_millis(config_data.health_check.timeout_ms);

    let mut results = Vec::new();

    for node in &nodes {
        let check_url = if config_data.health_check.check_url.is_empty() {
            None
        } else {
            Some(config_data.health_check.check_url.as_str())
        };

        let result = check_node_health(
            &node.address,
            node.port,
            &config_data.health_check.check_method,
            check_url,
            timeout,
        )
        .await;

        let status = if result.healthy {
            crate::config::smart_node::NodeStatus::Healthy
        } else {
            crate::config::smart_node::NodeStatus::Unhealthy
        };
        update_node_status(&node.uid, status, result.latency_ms, &pool);

        results.push(serde_json::json!({
            "uid": node.uid,
            "healthy": result.healthy,
            "latency_ms": result.latency_ms,
            "error": result.error,
        }));
    }

    // 保存
    {
        let pool_data = pool.data_arc();
        let _ = pool_data.save_file().await;
    }

    let json_result = serde_json::json!({
        "results": results,
        "total": results.len(),
    });

    SncNotification::emit_health_check_completed(&json_result);

    Ok(json_result)
}
