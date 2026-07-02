use crate::cmd::StringifyErr;
use crate::config::smart_node::{ISmartNodePool, PoolStats};
use crate::core::smart_node::notification::SncNotification;
use crate::feat::smart_node::pool::{
    SmartNodeInput, SmartNodeUpdate, add_nodes, get_node, get_pool_stats, remove_node, update_node,
};
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

/// 获取 Master Pool 数据
#[tauri::command]
pub async fn snc_get_pool() -> crate::cmd::CmdResult<serde_json::Value> {
    let pool = get_pool_draft().await;
    let data = pool.data_arc();
    serde_json::to_value(data.as_ref()).stringify_err()
}

/// 添加节点到 Master Pool
#[tauri::command]
pub async fn snc_add_nodes(
    nodes: Vec<serde_json::Value>,
    source_name: String,
) -> crate::cmd::CmdResult<serde_json::Value> {
    let inputs: Vec<SmartNodeInput> = serde_json::from_value(serde_json::Value::Array(nodes))
        .map_err(|e| e.to_string())?;

    let pool = get_pool_draft().await;
    let result = add_nodes(inputs, &pool).await;

    // 通知前端 pool 更新
    let stats = get_pool_stats(&pool);
    SncNotification::emit_pool_updated(&serde_json::to_value(&stats).unwrap_or_default());

    serde_json::to_value(&result).stringify_err()
}

/// 从 Master Pool 移除节点
#[tauri::command]
pub async fn snc_remove_node(uid: String) -> crate::cmd::CmdResult<()> {
    let pool = get_pool_draft().await;
    remove_node(&uid, &pool).await;
    Ok(())
}

/// 更新节点信息
#[tauri::command]
pub async fn snc_update_node(
    uid: String,
    update: serde_json::Value,
) -> crate::cmd::CmdResult<()> {
    let update: SmartNodeUpdate = serde_json::from_value(update)
        .map_err(|e| e.to_string())?;

    let pool = get_pool_draft().await;
    update_node(&uid, update, &pool).await;
    Ok(())
}

/// 获取单个节点详情
#[tauri::command]
pub async fn snc_get_node(uid: String) -> crate::cmd::CmdResult<serde_json::Value> {
    let pool = get_pool_draft().await;
    let node = get_node(&uid, &pool).ok_or_else(|| "node not found")?;
    serde_json::to_value(&node).stringify_err()
}

/// 获取 Pool 统计信息
#[tauri::command]
pub async fn snc_get_pool_stats() -> crate::cmd::CmdResult<serde_json::Value> {
    let pool = get_pool_draft().await;
    let stats = get_pool_stats(&pool);
    serde_json::to_value(&stats).stringify_err()
}
