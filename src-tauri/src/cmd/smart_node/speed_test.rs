use crate::cmd::StringifyErr;
use crate::config::smart_node::{ISmartNodeConfig, ISmartNodePool};
use crate::core::smart_node::notification::SncNotification;
use crate::core::smart_node::speed_test::{calculate_score, tcp_ping};
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

/// 对单个节点执行测速
#[tauri::command]
pub async fn snc_test_node_speed(uid: String) -> crate::cmd::CmdResult<serde_json::Value> {
    let config = crate::core::smart_node::SncManager::config().await;
    let config_data = config.data_arc();
    let pool = get_pool_draft().await;

    // 查找节点
    let node = pool
        .data_arc()
        .nodes
        .iter()
        .find(|n| n.uid == uid)
        .cloned()
        .ok_or_else(|| format!("node {} not found", uid))?;

    let tcp_timeout = Duration::from_millis(config_data.speed_test.tcp_timeout_ms);
    let http_timeout = Duration::from_millis(config_data.speed_test.http_timeout_ms);

    // TCP ping
    let latency = tcp_ping(&node.address, node.port, tcp_timeout).await;

    // HTTP ping (如果配置了 URL)
    let http_latency = if !config_data.health_check.check_url.is_empty() {
        http_ping(&config_data.health_check.check_url, http_timeout)
            .await
            .ok()
    } else {
        None
    };

    // 计算评分
    let score = calculate_score(
        &config_data.scoring,
        latency.ok(),
        None, // speed 需要单独测速
        None,
        None,
    );

    // 更新节点
    if let Ok(lat) = &latency {
        update_node_status(
            &uid,
            crate::config::smart_node::NodeStatus::Healthy,
            Some(*lat),
            &pool,
        );
    }

    // 保存
    {
        let pool_data = pool.data_arc();
        let _ = pool_data.save_file().await;
    }

    let result = serde_json::json!({
        "uid": uid,
        "latency_ms": latency.ok(),
        "http_latency_ms": http_latency,
        "score": score,
    });

    // 发送事件
    SncNotification::emit_speed_test_completed(&result);

    Ok(result)
}

/// 对所有节点执行批量测速
#[tauri::command]
pub async fn snc_test_all_nodes() -> crate::cmd::CmdResult<serde_json::Value> {
    let config = crate::core::smart_node::SncManager::config().await;
    let config_data = config.data_arc();
    let pool = get_pool_draft().await;

    let nodes = pool.data_arc().nodes.clone();
    let tcp_timeout = Duration::from_millis(config_data.speed_test.tcp_timeout_ms);
    let concurrent_limit = config_data.speed_test.concurrent_limit as usize;

    let mut results = Vec::new();

    // 使用 semaphore 控制并发
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrent_limit));
    let mut handles = Vec::new();

    for node in nodes {
        let permit = semaphore.clone().acquire_owned().await.map_err(|e| e.to_string())?;
        let addr = node.address.clone();
        let port = node.port;
        let uid = node.uid.clone();
        let scoring_config = config_data.scoring.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            let latency = tcp_ping(&addr, port, tcp_timeout).await.ok();

            let score = calculate_score(
                &scoring_config,
                latency,
                None,
                None,
                None,
            );

            (uid, latency, score)
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok((uid, latency, score)) => {
                if let Some(lat) = latency {
                    update_node_status(
                        &uid,
                        crate::config::smart_node::NodeStatus::Healthy,
                        Some(lat),
                        &pool,
                    );
                }
                results.push(serde_json::json!({
                    "uid": uid,
                    "latency_ms": latency,
                    "score": score,
                }));
            }
            Err(e) => {
                logging!(error, Type::Core, "SNC: speed test task failed: {}", e);
            }
        }
    }

    // 保存 pool
    {
        let pool_data = pool.data_arc();
        let _ = pool_data.save_file().await;
    }

    let result = serde_json::json!({
        "results": results,
        "total": results.len(),
    });

    SncNotification::emit_speed_test_completed(&result);

    Ok(result)
}
