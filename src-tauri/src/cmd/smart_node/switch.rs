use crate::cmd::StringifyErr;
use crate::config::smart_node::ISmartNodePool;
use crate::core::smart_node::switch::{SwitchStrategy, execute_switch, select_node};
use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging};
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

/// 对指定分组执行自动切换
#[tauri::command]
pub async fn snc_switch_group_node(
    group_name: String,
    strategy: String,
) -> crate::cmd::CmdResult<serde_json::Value> {
    let pool = get_pool_draft().await;

    // 获取所有健康节点作为候选
    let candidates: Vec<_> = pool
        .data_arc()
        .nodes
        .iter()
        .filter(|n| {
            n.status == crate::config::smart_node::NodeStatus::Healthy
                || n.status == crate::config::smart_node::NodeStatus::Unknown
        })
        .cloned()
        .collect();

    let switch_strategy = SwitchStrategy::from_str_lossy(&strategy);

    let selected_uid = select_node(&switch_strategy, &candidates)
        .ok_or_else(|| "no suitable node found for switching")?;

    // 查找节点名称
    let selected_node = candidates
        .iter()
        .find(|n| n.uid == selected_uid)
        .ok_or_else(|| "selected node not found")?;

    let node_name = selected_node.name.clone();
    let uid = selected_uid.clone();

    // 执行切换
    execute_switch(&group_name, &uid, &node_name)
        .await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "group_name": group_name,
        "node_uid": uid,
        "node_name": node_name,
        "strategy": strategy,
    }))
}
