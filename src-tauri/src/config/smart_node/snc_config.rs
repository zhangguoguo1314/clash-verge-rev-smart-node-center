use crate::utils::{dirs, help};
use anyhow::Result;
use clash_verge_logging::{Type, logging};
use serde::{Deserialize, Serialize};
use smartstring::alias::String;
use std::path::PathBuf;

/// 测速配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct SpeedTestConfig {
    pub concurrent_limit: u32,
    pub tcp_timeout_ms: u64,
    pub http_timeout_ms: u64,
    pub download_timeout_ms: u64,
    pub download_url: String,
    pub download_size: u64,
    pub cache_ttl_seconds: u64,
    pub schedule_interval_minutes: u64,
}

impl Default for SpeedTestConfig {
    fn default() -> Self {
        Self {
            concurrent_limit: 20,
            tcp_timeout_ms: 5000,
            http_timeout_ms: 5000,
            download_timeout_ms: 10000,
            download_url: String::from("https://www.google.com/generate_204"),
            download_size: 1048576,
            cache_ttl_seconds: 300,
            schedule_interval_minutes: 30,
        }
    }
}

/// 评分配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ScoringConfig {
    pub latency_weight: f64,
    pub speed_weight: f64,
    pub stability_weight: f64,
    pub online_rate_weight: f64,
    pub latency_excellent: u64,
    pub latency_good: u64,
    pub latency_poor: u64,
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            latency_weight: 0.3,
            speed_weight: 0.25,
            stability_weight: 0.25,
            online_rate_weight: 0.2,
            latency_excellent: 100,
            latency_good: 300,
            latency_poor: 1000,
        }
    }
}

/// 自动切换配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct AutoSwitchConfig {
    pub enabled: bool,
    pub default_strategy: String,
    pub default_interval_minutes: u64,
    pub fail_count_threshold: u32,
    pub enable_recovery_switch_back: bool,
    pub recovery_check_interval_minutes: u64,
    pub master_pool_fallback: bool,
}

impl Default for AutoSwitchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_strategy: String::from("fastest"),
            default_interval_minutes: 30,
            fail_count_threshold: 3,
            enable_recovery_switch_back: false,
            recovery_check_interval_minutes: 5,
            master_pool_fallback: true,
        }
    }
}

/// 健康检测配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub interval_minutes: u64,
    pub timeout_ms: u64,
    pub check_method: String,
    pub check_url: String,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_minutes: 5,
            timeout_ms: 5000,
            check_method: String::from("tcp"),
            check_url: String::from("https://www.google.com/generate_204"),
        }
    }
}

/// 历史数据配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct HistoryConfig {
    pub retention_days: u32,
    pub auto_cleanup: bool,
    pub cleanup_time: String,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            retention_days: 30,
            auto_cleanup: true,
            cleanup_time: String::from("03:00"),
        }
    }
}

/// Dashboard 配置
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct DashboardConfig {
    pub refresh_interval_seconds: u64,
    pub show_media_detect: bool,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            refresh_interval_seconds: 10,
            show_media_detect: true,
        }
    }
}

/// Smart Node Center 主配置结构体
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ISmartNodeConfig {
    /// 功能总开关
    pub enabled: Option<bool>,

    /// 测速配置
    #[serde(default)]
    pub speed_test: SpeedTestConfig,

    /// 评分配置
    #[serde(default)]
    pub scoring: ScoringConfig,

    /// 自动切换配置
    #[serde(default)]
    pub auto_switch: AutoSwitchConfig,

    /// 健康检测配置
    #[serde(default)]
    pub health_check: HealthCheckConfig,

    /// 历史数据配置
    #[serde(default)]
    pub history: HistoryConfig,

    /// Dashboard 配置
    #[serde(default)]
    pub dashboard: DashboardConfig,
}

impl ISmartNodeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取 SNC 配置文件路径
    pub fn config_dir() -> Result<PathBuf> {
        let dir = dirs::app_home_dir()?.join("smart_node");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }

    fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.yaml"))
    }

    /// 从文件加载配置
    pub async fn load_file() -> Self {
        match Self::config_path() {
            Ok(path) => match help::read_yaml::<Self>(&path).await {
                Ok(config) => config,
                Err(err) => {
                    logging!(error, Type::Config, "Failed to load SNC config: {}", err);
                    Self::default()
                }
            },
            Err(err) => {
                logging!(error, Type::Config, "Failed to get SNC config path: {}", err);
                Self::default()
            }
        }
    }

    /// 保存配置到文件
    pub async fn save_file(&self) -> Result<()> {
        let path = Self::config_path()?;
        help::save_yaml(&path, self, Some("# Smart Node Center Config")).await
    }
}
