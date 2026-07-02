use crate::config::smart_node::{ISmartNodeGroups, ISmartNodePool, NodeStatus, SmartGroup};
use clash_verge_draft::Draft;

/// 故障转移处理
///
/// 当某节点被标记为不健康时，查找分组中下一个可用节点。
/// 优先使用备用节点 (fallback_node_uids)，最后从 Master Pool 兜底。
pub async fn handle_failover(
    pool: &Draft<ISmartNodePool>,
    groups: &Draft<ISmartNodeGroups>,
    fail_uid: &str,
) -> Option<String> {
    let pool_data = pool.data_arc();
    let groups_data = groups.data_arc();

    // 查找该节点所属的分组
    let group = groups_data.groups.iter().find(|g| {
        g.node_uids.iter().any(|uid| uid == fail_uid)
            || g.fallback_node_uids.iter().any(|uid| uid == fail_uid)
    });

    let group = match group {
        Some(g) => g,
        None => return None,
    };

    // 1. 优先尝试备用节点 (按原始顺序)
    for fallback_uid in &group.fallback_node_uids {
        if *fallback_uid == fail_uid {
            continue;
        }
        if let Some(node) = pool_data.nodes.iter().find(|n| n.uid == *fallback_uid) {
            if node.status == NodeStatus::Healthy {
                return Some(fallback_uid.clone());
            }
        }
    }

    // 2. 从分组普通节点中查找健康的
    for node_uid in &group.node_uids {
        if *node_uid == fail_uid {
            continue;
        }
        if let Some(node) = pool_data.nodes.iter().find(|n| n.uid == *node_uid) {
            if node.status == NodeStatus::Healthy {
                return Some(node_uid.clone());
            }
        }
    }

    // 3. 从 Master Pool 中查找健康的同协议节点 (兜底)
    let fail_node = pool_data.nodes.iter().find(|n| n.uid == fail_uid);
    if let Some(fail_node) = fail_node {
        for node in &pool_data.nodes {
            if node.uid == fail_uid {
                continue;
            }
            if node.protocol == fail_node.protocol
                && node.status == NodeStatus::Healthy
                && !group.node_uids.contains(&node.uid)
                && !group.fallback_node_uids.contains(&node.uid)
            {
                return Some(node.uid.clone());
            }
        }
    }

    None
}
