use crate::utils::{dirs, help};
use anyhow::Result;
use clash_verge_logging::{Type, logging};
use serde::{Deserialize, Serialize};
use smartstring::alias::String;
use std::path::PathBuf;

/// 节点状态枚举
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    #[default]
    Unknown,
    Healthy,
    Unhealthy,
}

/// 智能节点
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct SmartNode {
    /// 唯一标识符 (UUID v4)
    pub uid: String,
    /// 节点名称
    pub name: String,
    /// 协议类型 (ss, ssr, vmess, vless, trojan, hysteria2, etc.)
    pub protocol: String,
    /// 服务器地址
    pub address: String,
    /// 服务器端口
    pub port: u16,
    /// 完整 URI
    pub uri: Option<String>,
    /// 来源名称
    pub source: Option<String>,
    /// 来源 URL
    pub source_url: Option<String>,
    /// 标签列表
    #[serde(default)]
    pub tags: Vec<String>,
    /// 节点状态
    pub status: NodeStatus,
    /// 评分 (0-100)
    #[serde(default)]
    pub score: f64,
    /// 延迟 (ms)
    pub latency_ms: Option<u64>,
    /// 下载速度 (Mbps)
    pub download_speed_mbps: Option<f64>,
    /// 最后健康检测时间
    pub last_health_check: Option<String>,
    /// 最后测速时间
    pub last_speed_test: Option<String>,
    /// 创建时间 (ISO 8601)
    pub created_at: String,
    /// 更新时间 (ISO 8601)
    pub updated_at: String,
    /// 连续失败计数
    #[serde(default)]
    pub fail_count: u32,
}

impl SmartNode {
    pub fn new(
        uid: String,
        name: String,
        protocol: String,
        address: String,
        port: u16,
    ) -> Self {
        let now = chrono::Local::now().to_rfc3339();
        Self {
            uid,
            name,
            protocol,
            address,
            port,
            uri: None,
            source: None,
            source_url: None,
            tags: Vec::new(),
            status: NodeStatus::Unknown,
            score: 0.0,
            latency_ms: None,
            download_speed_mbps: None,
            last_health_check: None,
            last_speed_test: None,
            created_at: now.clone(),
            updated_at: now,
            fail_count: 0,
        }
    }

    /// 生成去重键: protocol+address+port
    pub fn dedup_key(&self) -> String {
        format!("{}:{}:{}", self.protocol, self.address, self.port)
    }
}

/// Master Pool 统计信息
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct PoolStats {
    pub total_count: usize,
    pub healthy_count: usize,
    pub unhealthy_count: usize,
    pub unknown_count: usize,
}

/// 导入结果
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub duplicated: usize,
    pub failed: usize,
    #[serde(default)]
    pub errors: Vec<String>,
}

/// Master Pool 数据模型
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ISmartNodePool {
    /// 数据版本号
    pub version: u32,
    /// 最后更新时间
    pub updated_at: Option<String>,
    /// 节点列表
    #[serde(default)]
    pub nodes: Vec<SmartNode>,
}

impl ISmartNodePool {
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取 Pool 配置文件路径
    pub fn config_dir() -> Result<PathBuf> {
        let dir = dirs::app_home_dir()?.join("smart_node");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }

    fn pool_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("pool.yaml"))
    }

    /// 从文件加载 Pool
    pub async fn load_file() -> Self {
        match Self::pool_path() {
            Ok(path) => match help::read_yaml::<Self>(&path).await {
                Ok(pool) => pool,
                Err(err) => {
                    logging!(
                        error,
                        Type::Config,
                        "Failed to load SNC pool: {}",
                        err
                    );
                    Self::default()
                }
            },
            Err(err) => {
                logging!(
                    error,
                    Type::Config,
                    "Failed to get SNC pool path: {}",
                    err
                );
                Self::default()
            }
        }
    }

    /// 保存 Pool 到文件
    pub async fn save_file(&self) -> Result<()> {
        let path = Self::pool_path()?;
        help::save_yaml(&path, self, Some("# Smart Node Center Master Pool")).await
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> PoolStats {
        let mut stats = PoolStats::default();
        stats.total_count = self.nodes.len();
        for node in &self.nodes {
            match node.status {
                NodeStatus::Healthy => stats.healthy_count += 1,
                NodeStatus::Unhealthy => stats.unhealthy_count += 1,
                NodeStatus::Unknown => stats.unknown_count += 1,
            }
        }
        stats
    }

    /// 更新版本号和更新时间
    pub fn mark_updated(&mut self) {
        self.version += 1;
        self.updated_at = Some(chrono::Local::now().to_rfc3339());
    }
}
