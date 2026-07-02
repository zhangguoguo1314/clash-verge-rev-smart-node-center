use crate::cmd::StringifyErr;
use crate::config::smart_node::ISmartNodeConfig;
use crate::core::smart_node::{SncManager, notification::SncNotification};
use clash_verge_logging::{Type, logging};

/// 检查 SNC 是否启用
#[tauri::command]
pub async fn snc_is_enabled() -> crate::cmd::CmdResult<bool> {
    let config = SncManager::config().await;
    let enabled = config.data_arc().enabled.unwrap_or(false);
    Ok(enabled)
}

/// 切换 SNC 启用/禁用
#[tauri::command]
pub async fn snc_toggle_enabled(enabled: bool) -> crate::cmd::CmdResult<()> {
    let config = SncManager::config().await;
    config.edit_draft(|d| {
        d.enabled = Some(enabled);
    });
    config.apply();

    // 保存到文件
    let data = config.data_arc();
    data.save_file().await.map_err(|e| e.to_string())?;

    // 启动或停止服务
    if enabled {
        SncManager::global().start().await.map_err(|e| e.to_string())?;
    } else {
        SncManager::global().stop().await;
    }

    SncNotification::emit_config_changed();

    logging!(info, Type::Core, "SNC: enabled set to {}", enabled);
    Ok(())
}

/// 获取 SNC 配置
#[tauri::command]
pub async fn snc_get_config() -> crate::cmd::CmdResult<serde_json::Value> {
    let config = SncManager::config().await;
    let data = config.data_arc();
    serde_json::to_value(data.as_ref()).stringify_err()
}

/// 更新 SNC 配置 (部分更新)
#[tauri::command]
pub async fn snc_patch_config(
    patch: serde_json::Value,
) -> crate::cmd::CmdResult<()> {
    let config = SncManager::config().await;

    // 解析 patch 为部分配置
    let partial: ISmartNodeConfig =
        serde_json::from_value(patch.clone()).map_err(|e| e.to_string())?;

    // 提取需要更新的字段
    let enabled = partial.enabled;
    let speed_test = partial.speed_test.clone();
    let scoring = partial.scoring.clone();
    let auto_switch = partial.auto_switch.clone();
    let health_check = partial.health_check.clone();
    let history = partial.history.clone();
    let dashboard = partial.dashboard.clone();
    let patch_clone = patch.clone();

    config.edit_draft(move |d| {
        // patch enabled
        if let Some(en) = enabled {
            d.enabled = Some(en);
        }

        // 简单策略: 如果 patch JSON 中包含对应 key，就整体替换
        if let Some(st) = patch_clone.get("speed_test") {
            if st.is_object() {
                d.speed_test = speed_test;
            }
        }
        if let Some(sc) = patch_clone.get("scoring") {
            if sc.is_object() {
                d.scoring = scoring;
            }
        }
        if let Some(as_cfg) = patch_clone.get("auto_switch") {
            if as_cfg.is_object() {
                d.auto_switch = auto_switch;
            }
        }
        if let Some(hc) = patch_clone.get("health_check") {
            if hc.is_object() {
                d.health_check = health_check;
            }
        }
        if let Some(hi) = patch_clone.get("history") {
            if hi.is_object() {
                d.history = history;
            }
        }
        if let Some(db) = patch_clone.get("dashboard") {
            if db.is_object() {
                d.dashboard = dashboard;
            }
        }
    });
    config.apply();

    // 保存到文件
    let data = config.data_arc();
    data.save_file().await.map_err(|e| e.to_string())?;

    SncNotification::emit_config_changed();

    logging!(info, Type::Core, "SNC: config patched");
    Ok(())
}
