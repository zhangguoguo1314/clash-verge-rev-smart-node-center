use crate::cmd::StringifyErr;
use crate::config::smart_node::{ISmartNodeGroups, ISmartNodePool};
use crate::feat::smart_node::group::{
    CreateGroupInput, UpdateGroupInput, create_group, delete_group, detect_group, update_group,
};
use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging};
use tokio::sync::OnceCell as TokioOnceCell;

async fn get_groups_draft() -> Draft<ISmartNodeGroups> {
    static GROUPS_DRAFT: TokioOnceCell<Draft<ISmartNodeGroups>> = TokioOnceCell::const_new();
    GROUPS_DRAFT
        .get_or_init(|| async {
            let groups = ISmartNodeGroups::load_file().await;
            Draft::new(groups)
        })
        .await
        .clone()
}

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

/// 获取所有分组
#[tauri::command]
pub async fn snc_get_groups() -> crate::cmd::CmdResult<serde_json::Value> {
    let groups = get_groups_draft().await;
    let data = groups.data_arc();
    serde_json::to_value(data.as_ref()).stringify_err()
}

/// 创建分组
#[tauri::command]
pub async fn snc_create_group(input: serde_json::Value) -> crate::cmd::CmdResult<serde_json::Value> {
    let input: CreateGroupInput = serde_json::from_value(input).map_err(|e| e.to_string())?;
    let groups = get_groups_draft().await;
    let id = create_group(input, &groups).await.map_err(|e| e)?;
    Ok(serde_json::json!({ "id": id }))
}

/// 更新分组
#[tauri::command]
pub async fn snc_update_group(
    id: String,
    update: serde_json::Value,
) -> crate::cmd::CmdResult<()> {
    let update: UpdateGroupInput = serde_json::from_value(update).map_err(|e| e.to_string())?;
    let groups = get_groups_draft().await;
    update_group(&id, update, &groups).await;
    Ok(())
}

/// 删除分组
#[tauri::command]
pub async fn snc_delete_group(id: String) -> crate::cmd::CmdResult<()> {
    let groups = get_groups_draft().await;
    let pool = get_pool_draft().await;
    delete_group(&id, &groups, &pool).await;
    Ok(())
}

/// 对分组执行检测
#[tauri::command]
pub async fn snc_detect_group(
    group_id: String,
) -> crate::cmd::CmdResult<serde_json::Value> {
    let groups = get_groups_draft().await;
    let pool = get_pool_draft().await;
    let result = detect_group(&group_id, &groups, &pool).await;
    serde_json::to_value(&result).stringify_err()
}
