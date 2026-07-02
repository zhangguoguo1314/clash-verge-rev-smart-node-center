use crate::config::smart_node::{
    GroupDetectResult, ISmartNodeGroups, ISmartNodePool, SmartGroup,
};
use crate::utils::help;
use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging, logging_error};
use serde::{Deserialize, Serialize};
use smartstring::alias::String;
use std::collections::HashSet;

/// 创建分组的输入参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateGroupInput {
    pub name: String,
    pub description: Option<String>,
    pub node_uids: Option<Vec<String>>,
    pub fallback_node_uids: Option<Vec<String>>,
}

/// 更新分组的输入参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateGroupInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub node_uids: Option<Vec<String>>,
    pub fallback_node_uids: Option<Vec<String>>,
    pub auto_switch: Option<GroupAutoSwitchInput>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupAutoSwitchInput {
    pub enabled: Option<bool>,
    pub strategy: Option<String>,
    pub interval_minutes: Option<u64>,
}

/// 创建分组
pub async fn create_group(
    input: CreateGroupInput,
    groups: &Draft<ISmartNodeGroups>,
) -> Result<String, String> {
    let id = crate::utils::help::get_uid("grp");
    let mut group = SmartGroup::new(id.clone(), input.name);
    group.description = input.description;
    if let Some(uids) = input.node_uids {
        group.node_uids = uids;
    }
    if let Some(uids) = input.fallback_node_uids {
        group.fallback_node_uids = uids;
    }

    let id_for_return = id.clone();

    groups.edit_draft(|d| {
        d.groups.push(group);
        d.mark_updated();
    });
    groups.apply();

    let data = groups.data_arc();
    logging_error!(Type::Core, data.save_file().await);

    logging!(info, Type::Core, "SNC: created group id={}", id_for_return);
    Ok(id_for_return)
}

/// 更新分组
pub async fn update_group(
    id: &str,
    update: UpdateGroupInput,
    groups: &Draft<ISmartNodeGroups>,
) -> bool {
    let found = {
        let data = groups.data_arc();
        data.groups.iter().any(|g| g.id == id)
    };

    if !found {
        return false;
    }

    groups.edit_draft(|d| {
        if let Some(group) = d.groups.iter_mut().find(|g| g.id == id) {
            if let Some(name) = update.name {
                group.name = name;
            }
            if let Some(desc) = update.description {
                group.description = Some(desc);
            }
            if let Some(uids) = update.node_uids {
                group.node_uids = uids;
            }
            if let Some(uids) = update.fallback_node_uids {
                group.fallback_node_uids = uids;
            }
            if let Some(auto_switch) = update.auto_switch {
                if let Some(enabled) = auto_switch.enabled {
                    group.auto_switch.enabled = enabled;
                }
                if let Some(strategy) = auto_switch.strategy {
                    group.auto_switch.strategy = strategy;
                }
                if let Some(interval) = auto_switch.interval_minutes {
                    group.auto_switch.interval_minutes = interval;
                }
            }
            group.updated_at = chrono::Local::now().to_rfc3339();
        }
        d.mark_updated();
    });
    groups.apply();

    let data = groups.data_arc();
    logging_error!(Type::Core, data.save_file().await);

    logging!(info, Type::Core, "SNC: updated group id={}", id);
    true
}

/// 删除分组
///
/// 同时解除节点对该分组的引用。
pub async fn delete_group(
    id: &str,
    groups: &Draft<ISmartNodeGroups>,
    _pool: &Draft<ISmartNodePool>,
) -> bool {
    let found = {
        let data = groups.data_arc();
        data.groups.iter().any(|g| g.id == id)
    };

    if !found {
        return false;
    }

    groups.edit_draft(|d| {
        d.groups.retain(|g| g.id != id);
        d.mark_updated();
    });
    groups.apply();

    let data = groups.data_arc();
    logging_error!(Type::Core, data.save_file().await);

    logging!(info, Type::Core, "SNC: deleted group id={}", id);
    true
}

/// 对分组中的节点执行检测
///
/// 根据分组的 detection 配置，对每个节点执行检测。
/// 成功的保留，失败的记录到结果中。
pub async fn detect_group(
    group_id: &str,
    groups: &Draft<ISmartNodeGroups>,
    pool: &Draft<ISmartNodePool>,
) -> GroupDetectResult {
    let mut result = GroupDetectResult::new(group_id.to_string());

    let (group_config, pool_data) = {
        let g = groups.data_arc();
        let p = pool.data_arc();
        let group = match g.groups.iter().find(|gr| gr.id == group_id) {
            Some(gr) => gr.clone(),
            None => {
                result.errors.push(format!("group {} not found", group_id));
                return result;
            }
        };
        let uids: Vec<String> = group.node_uids.clone();
        let pool_nodes: Vec<_> = p.nodes.iter().filter(|n| uids.contains(&n.uid)).cloned().collect();
        let detection_urls = group.detection.urls.clone();
        (pool_nodes, detection_urls)
    };

    // 如果没有配置检测 URL，使用 TCP 检测
    let timeout = std::time::Duration::from_secs(5);

    for node in &pool_data {
        let mut node_healthy = false;

        if pool_data.is_empty() {
            break;
        }

        if !result.group_id.is_empty() {
            // 检测节点
            let check_result = crate::core::smart_node::health::check_node_health(
                &node.address,
                node.port,
                "tcp",
                None,
                timeout,
            )
            .await;

            if check_result.healthy {
                node_healthy = true;
                // 更新节点状态
                crate::feat::smart_node::pool::update_node_status(
                    &node.uid,
                    crate::config::smart_node::NodeStatus::Healthy,
                    check_result.latency_ms,
                    pool,
                );
            } else {
                crate::feat::smart_node::pool::update_node_status(
                    &node.uid,
                    crate::config::smart_node::NodeStatus::Unhealthy,
                    None,
                    pool,
                );
                if let Some(err) = check_result.error {
                    result.errors.push(format!("node {}: {}", node.uid, err));
                }
            }
        }

        if node_healthy {
            result.successful_uids.push(node.uid.clone());
        } else {
            result.failed_uids.push(node.uid.clone());
        }
    }

    // 保存 pool 更新
    {
        let pool_data = pool.data_arc();
        logging_error!(Type::Core, pool_data.save_file().await);
    }

    logging!(
        info,
        Type::Core,
        "SNC: group detect completed: group={}, success={}, failed={}",
        group_id,
        result.successful_uids.len(),
        result.failed_uids.len()
    );

    result
}
