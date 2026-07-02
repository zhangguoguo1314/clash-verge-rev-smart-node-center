use crate::config::smart_node::{
    ImportResult, ISmartNodePool, NodeStatus, PoolStats, SmartNode,
};
use crate::utils::help;
use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging, logging_error};
use serde::{Deserialize, Serialize};
use smartstring::alias::String;

/// 节点输入参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmartNodeInput {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: u16,
    pub uri: Option<String>,
    pub source: Option<String>,
    pub source_url: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// 节点更新参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmartNodeUpdate {
    pub name: Option<String>,
    pub uri: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// 添加节点到 Master Pool
///
/// - 为每个节点生成 UUID v4 作为 uid
/// - 执行去重检查 (protocol+address+port)
/// - 写入 Draft pool
pub async fn add_nodes(
    nodes: Vec<SmartNodeInput>,
    pool: &Draft<ISmartNodePool>,
) -> ImportResult {
    let mut result = ImportResult::default();

    // 收集已有的去重键
    let existing_keys: std::collections::HashSet<String> = pool
        .data_arc()
        .nodes
        .iter()
        .map(|n| n.dedup_key())
        .collect();

    let mut new_nodes = Vec::new();

    for input in nodes {
        let dedup_key = format!("{}:{}:{}", input.protocol, input.address, input.port);

        if existing_keys.contains(&dedup_key) {
            result.duplicated += 1;
            continue;
        }

        let uid = crate::utils::help::get_uid("snc");
        let mut node = SmartNode::new(
            uid.into(),
            input.name,
            input.protocol,
            input.address,
            input.port,
        );
        node.uri = input.uri;
        node.source = input.source;
        node.source_url = input.source_url;
        node.tags = input.tags;

        existing_keys.insert(dedup_key.clone());
        new_nodes.push(node);
        result.imported += 1;
    }

    // 写入 pool
    if !new_nodes.is_empty() {
        pool.edit_draft(|d| {
            d.nodes.extend(new_nodes);
            d.mark_updated();
        });
        pool.apply();

        let data = pool.data_arc();
        logging_error!(Type::Core, data.save_file().await);
    }

    logging!(
        info,
        Type::Core,
        "SNC: added nodes: imported={}, duplicated={}",
        result.imported,
        result.duplicated
    );

    result
}

/// 从 Master Pool 移除节点
pub async fn remove_node(uid: &str, pool: &Draft<ISmartNodePool>) -> bool {
    let found = {
        let data = pool.data_arc();
        data.nodes.iter().any(|n| n.uid == uid)
    };

    if found {
        pool.edit_draft(|d| {
            d.nodes.retain(|n| n.uid != uid);
            d.mark_updated();
        });
        pool.apply();

        let data = pool.data_arc();
        logging_error!(Type::Core, data.save_file().await);

        logging!(info, Type::Core, "SNC: removed node uid={}", uid);
        true
    } else {
        false
    }
}

/// 更新节点信息
pub async fn update_node(
    uid: &str,
    update: SmartNodeUpdate,
    pool: &Draft<ISmartNodePool>,
) -> bool {
    let found = {
        let data = pool.data_arc();
        data.nodes.iter().any(|n| n.uid == uid)
    };

    if !found {
        return false;
    }

    pool.edit_draft(|d| {
        if let Some(node) = d.nodes.iter_mut().find(|n| n.uid == uid) {
            if let Some(name) = update.name {
                node.name = name;
            }
            if let Some(uri) = update.uri {
                node.uri = Some(uri);
            }
            if let Some(tags) = update.tags {
                node.tags = tags;
            }
            node.updated_at = chrono::Local::now().to_rfc3339();
        }
        d.mark_updated();
    });
    pool.apply();

    let data = pool.data_arc();
    logging_error!(Type::Core, data.save_file().await);

    logging!(info, Type::Core, "SNC: updated node uid={}", uid);
    true
}

/// 获取单个节点
pub fn get_node(uid: &str, pool: &Draft<ISmartNodePool>) -> Option<SmartNode> {
    let data = pool.data_arc();
    data.nodes.iter().find(|n| n.uid == uid).cloned()
}

/// 获取 Pool 统计信息
pub fn get_pool_stats(pool: &Draft<ISmartNodePool>) -> PoolStats {
    pool.data_arc().get_stats()
}

/// 批量更新节点状态
pub fn update_node_status(
    uid: &str,
    status: NodeStatus,
    latency_ms: Option<u64>,
    pool: &Draft<ISmartNodePool>,
) {
    pool.edit_draft(|d| {
        if let Some(node) = d.nodes.iter_mut().find(|n| n.uid == uid) {
            node.status = status.clone();
            node.latency_ms = latency_ms;
            if status == NodeStatus::Healthy {
                node.fail_count = 0;
            } else if status == NodeStatus::Unhealthy {
                node.fail_count += 1;
            }
            node.updated_at = chrono::Local::now().to_rfc3339();
        }
        d.mark_updated();
    });
}
