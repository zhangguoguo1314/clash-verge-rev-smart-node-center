use crate::core::handle::Handle;
use crate::utils::window_manager::WindowManager;
use clash_verge_logging::{Type, logging};
use serde::Serialize;
use tauri::Emitter as _;

/// SNC 事件通知模块
pub struct SncNotification;

impl SncNotification {
    /// 发送 SNC 事件到前端
    pub fn emit_event(event_name: &str, payload: impl Serialize) {
        if Handle::global().is_exiting() {
            return;
        }

        if let Some(window) = WindowManager::get_main_window() {
            if let Err(e) = window.emit(event_name, payload) {
                logging!(warn, Type::Frontend, "SNC event emit failed: {}", e);
            }
        }
    }

    /// Pool 更新事件
    pub fn emit_pool_updated(stats: &serde_json::Value) {
        Self::emit_event("snc://pool-updated", stats);
    }

    /// 测速完成事件
    pub fn emit_speed_test_completed(result: &serde_json::Value) {
        Self::emit_event("snc://speed-test-completed", result);
    }

    /// 节点切换事件
    pub fn emit_node_switched(info: &serde_json::Value) {
        Self::emit_event("snc://node-switched", info);
    }

    /// 健康检测完成事件
    pub fn emit_health_check_completed(result: &serde_json::Value) {
        Self::emit_event("snc://health-check-completed", result);
    }

    /// 配置变更事件
    pub fn emit_config_changed() {
        Self::emit_event("snc://config-changed", serde_json::json!({ "status": "ok" }));
    }
}
