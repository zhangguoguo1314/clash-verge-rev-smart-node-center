use crate::utils::{dirs, help};
use anyhow::Result;
use clash_verge_logging::{Type, logging};
use serde::{Deserialize, Serialize};
use smartstring::alias::String;
use std::path::PathBuf;

/// 分组自动切换配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct GroupAutoSwitch {
    pub enabled: bool,
    pub strategy: String,
    pub interval_minutes: u64,
}

/// 检测 URL 配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DetectionUrl {
    pub url: String,
    pub method: String,
    pub expected_status: Option<u16>,
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_timeout_ms() -> u64 {
    5000
}

/// 智能分组
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct SmartGroup {
    /// 分组 ID
    pub id: String,
    /// 分组名称
    pub name: String,
    /// 分组描述
    pub description: Option<String>,
    /// 自动切换配置
    #[serde(default)]
    pub auto_switch: GroupAutoSwitch,
    /// 检测 URL 列表
    #[serde(default)]
    pub detection: GroupDetection,
    /// 是否自动同步
    #[serde(default)]
    pub auto_sync: bool,
    /// 同步间隔（分钟）
    #[serde(default)]
    pub sync_interval_minutes: u64,
    /// 节点 UID 列表
    #[serde(default)]
    pub node_uids: Vec<String>,
    /// 备用节点 UID 列表
    #[serde(default)]
    pub fallback_node_uids: Vec<String>,
    /// 创建时间 (ISO 8601)
    pub created_at: String,
    /// 更新时间 (ISO 8601)
    pub updated_at: String,
}

impl SmartGroup {
    pub fn new(id: String, name: String) -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            id,
            name,
            description: None,
            auto_switch: GroupAutoSwitch {
                enabled: true,
                strategy: String::from("fastest"),
                interval_minutes: 30,
            },
            detection: GroupDetection::default(),
            auto_sync: false,
            sync_interval_minutes: 60,
            node_uids: Vec::new(),
            fallback_node_uids: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

/// 分组检测配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetection {
    #[serde(default)]
    pub urls: Vec<DetectionUrl>,
}

/// 分组检测结果
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct GroupDetectResult {
    pub group_id: String,
    #[serde(default)]
    pub successful_uids: Vec<String>,
    #[serde(default)]
    pub failed_uids: Vec<String>,
    #[serde(default)]
    pub errors: Vec<String>,
}

impl GroupDetectResult {
    pub fn new(group_id: String) -> Self {
        Self {
            group_id,
            successful_uids: Vec::new(),
            failed_uids: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// 分组集合数据模型
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ISmartNodeGroups {
    /// 数据版本号
    pub version: u32,
    /// 最后更新时间
    pub updated_at: Option<String>,
    /// 分组列表
    #[serde(default)]
    pub groups: Vec<SmartGroup>,
}

impl ISmartNodeGroups {
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取 Groups 配置文件路径
    pub fn config_dir() -> Result<PathBuf> {
        let dir = dirs::app_home_dir()?.join("smart_node");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }

    fn groups_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("groups.yaml"))
    }

    /// 从文件加载 Groups
    pub async fn load_file() -> Self {
        match Self::groups_path() {
            Ok(path) => match help::read_yaml::<Self>(&path).await {
                Ok(groups) => groups,
                Err(err) => {
                    logging!(
                        error,
                        Type::Config,
                        "Failed to load SNC groups: {}",
                        err
                    );
                    Self::default()
                }
            },
            Err(err) => {
                logging!(
                    error,
                    Type::Config,
                    "Failed to get SNC groups path: {}",
                    err
                );
                Self::default()
            }
        }
    }

    /// 保存 Groups 到文件
    pub async fn save_file(&self) -> Result<()> {
        let path = Self::groups_path()?;
        help::save_yaml(&path, self, Some("# Smart Node Center Groups")).await
    }

    /// 更新版本号和更新时间
    pub fn mark_updated(&mut self) {
        self.version += 1;
        self.updated_at = Some(chrono::Local::now().to_rfc3339());
    }
}
