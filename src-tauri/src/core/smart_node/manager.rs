use crate::config::smart_node::ISmartNodeConfig;
use crate::core::smart_node::scheduler::SncScheduler;
use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging, logging_error};
use parking_lot::Mutex;
use smartstring::alias::String;
use std::sync::Arc;
use tokio::sync::OnceCell;

/// SNC 运行状态
#[derive(Debug, Clone, PartialEq)]
pub enum SncState {
    Disabled,
    Starting,
    Running,
    Stopping,
}

/// SNC 管理器 - Smart Node Center 的核心控制器
pub struct SncManager {
    state: Arc<Mutex<SncState>>,
    scheduler: Arc<Mutex<Option<SncScheduler>>>,
    config_draft: OnceCell<Draft<ISmartNodeConfig>>,
}

impl std::fmt::Debug for SncManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SncManager")
            .field("state", &*self.state.lock())
            .finish()
    }
}

singleton!(SncManager, SNC_MANAGER);

impl SncManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(SncState::Disabled)),
            scheduler: Arc::new(Mutex::new(None)),
            config_draft: OnceCell::new(),
        }
    }

    /// 获取全局配置 Draft
    pub async fn config() -> Draft<ISmartNodeConfig> {
        Self::global().get_or_init_config().await
    }

    async fn get_or_init_config(&self) -> Draft<ISmartNodeConfig> {
        self.config_draft
            .get_or_init(|| async {
                let config = ISmartNodeConfig::load_file().await;
                Draft::new(config)
            })
            .await
            .clone()
    }

    /// 初始化 SNC，如果 enabled 则自动启动
    pub async fn init() {
        logging!(info, Type::Core, "SNC: initializing Smart Node Center...");

        let global = Self::global();
        let config = global.get_or_init_config().await;
        let enabled = config.data_arc().enabled.unwrap_or(false);

        if enabled {
            logging!(info, Type::Core, "SNC: enabled, starting services...");
            logging_error!(Type::Core, Self::global().start().await);
        } else {
            logging!(info, Type::Core, "SNC: disabled, skip starting services");
        }
    }

    /// 启动所有 SNC 子服务
    pub async fn start() -> anyhow::Result<()> {
        {
            let mut state = self.state.lock();
            match *state {
                SncState::Running => {
                    logging!(info, Type::Core, "SNC: already running");
                    return Ok(());
                }
                SncState::Stopping => {
                    logging!(warn, Type::Core, "SNC: stopping, please wait");
                    return Err(anyhow::anyhow!("SNC is stopping"));
                }
                _ => {}
            }
            *state = SncState::Starting;
        }

        logging!(info, Type::Core, "SNC: starting scheduler...");

        let mut scheduler_lock = self.scheduler.lock();
        let mut scheduler = SncScheduler::new();
        scheduler.start().await?;
        *scheduler_lock = Some(scheduler);

        {
            let mut state = self.state.lock();
            *state = SncState::Running;
        }

        logging!(info, Type::Core, "SNC: all services started successfully");
        Ok(())
    }

    /// 停止所有 SNC 子服务
    pub async fn stop() {
        logging!(info, Type::Core, "SNC: stopping services...");

        {
            let mut state = self.state.lock();
            if *state == SncState::Disabled {
                logging!(info, Type::Core, "SNC: already disabled");
                return;
            }
            *state = SncState::Stopping;
        }

        {
            let mut scheduler_lock = self.scheduler.lock();
            if let Some(scheduler) = scheduler_lock.take() {
                scheduler.stop().await;
            }
        }

        {
            let mut state = self.state.lock();
            *state = SncState::Disabled;
        }

        logging!(info, Type::Core, "SNC: all services stopped");
    }

    /// SNC 是否正在运行
    pub fn is_running(&self) -> bool {
        let state = self.state.lock();
        *state == SncState::Running
    }

    /// 获取当前状态
    pub fn get_state(&self) -> SncState {
        self.state.lock().clone()
    }

    /// 保存配置并应用
    pub async fn save_config() -> anyhow::Result<()> {
        let config = Self::global().get_or_init_config().await;
        config.apply();
        let data = config.data_arc();
        data.save_file().await
    }
}
