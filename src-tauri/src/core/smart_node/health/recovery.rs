use std::time::Duration;

use crate::config::smart_node::SmartNode;

/// 恢复检测
///
/// 检查之前不健康的节点是否已恢复可用。
pub async fn check_recovery(
    node: &SmartNode,
    method: &str,
    url: Option<&str>,
    timeout: Duration,
) -> bool {
    let result = super::checker::check_node_health(
        &node.address,
        node.port,
        method,
        url,
        timeout,
    )
    .await;

    result.healthy
}
