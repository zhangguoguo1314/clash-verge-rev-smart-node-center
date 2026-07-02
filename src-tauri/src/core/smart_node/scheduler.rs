use clash_verge_draft::Draft;
use clash_verge_logging::{Type, logging, logging_error};
use smartstring::alias::String;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

use crate::config::smart_node::ISmartNodeConfig;

/// 调度器命令
enum SchedulerCommand {
    Stop,
}

/// SNC 定时任务调度器
pub struct SncScheduler {
    command_tx: Option<mpsc::UnboundedSender<SchedulerCommand>>,
    running: AtomicBool,
}

impl std::fmt::Debug for SncScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SncScheduler")
            .field("running", &self.running.load(Ordering::Relaxed))
            .finish()
    }
}

impl SncScheduler {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            running: AtomicBool::new(false),
        }
    }

    /// 启动所有定时任务
    pub async fn start(&mut self) -> anyhow::Result<()> {
        let (command_tx, command_rx) = mpsc::unbounded_channel::<SchedulerCommand>();
        self.command_tx = Some(command_tx);
        self.running.store(true, Ordering::Relaxed);

        let config = ISmartNodeConfig::load_file().await;

        // 启动健康检测循环
        Self::spawn_health_check_loop(config.health_check.interval_minutes, command_rx.clone());

        // 启动测速循环
        Self::spawn_speed_test_loop(config.speed_test.schedule_interval_minutes, command_rx.clone());

        // 启动分组检测循环
        Self::spawn_group_detect_loop(30, command_rx.clone()); // 默认30分钟

        // 启动历史清理循环
        Self::spawn_history_cleanup_loop(config.history.retention_days, command_rx.clone());

        logging!(info, Type::Core, "SNC Scheduler: all loops started");
        Ok(())
    }

    /// 停止所有定时任务
    pub async fn stop(&self) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(SchedulerCommand::Stop);
        }
        self.running.store(false, Ordering::Relaxed);
        logging!(info, Type::Core, "SNC Scheduler: stop signal sent");
    }

    fn spawn_health_check_loop(
        interval_minutes: u64,
        mut stop_rx: mpsc::UnboundedReceiver<SchedulerCommand>,
    ) {
        if interval_minutes == 0 {
            return;
        }
        tokio::spawn(async move {
            let interval = Duration::from_secs(interval_minutes.saturating_mul(60));
            logging!(
                info,
                Type::Core,
                "SNC Scheduler: health_check_loop started (interval: {}min)",
                interval_minutes
            );

            loop {
                tokio::select! {
                    _ = sleep(interval) => {
                        logging!(debug, Type::Core, "SNC Scheduler: health_check tick");
                        // 健康检测逻辑由 health 模块处理
                    }
                    cmd = stop_rx.recv() => {
                        match cmd {
                            Some(SchedulerCommand::Stop) | None => {
                                logging!(info, Type::Core, "SNC Scheduler: health_check_loop stopped");
                                return;
                            }
                        }
                    }
                }
            }
        });
    }

    fn spawn_speed_test_loop(
        interval_minutes: u64,
        mut stop_rx: mpsc::UnboundedReceiver<SchedulerCommand>,
    ) {
        if interval_minutes == 0 {
            return;
        }
        tokio::spawn(async move {
            let interval = Duration::from_secs(interval_minutes.saturating_mul(60));
            logging!(
                info,
                Type::Core,
                "SNC Scheduler: speed_test_loop started (interval: {}min)",
                interval_minutes
            );

            loop {
                tokio::select! {
                    _ = sleep(interval) => {
                        logging!(debug, Type::Core, "SNC Scheduler: speed_test tick");
                        // 测速逻辑由 speed_test 模块处理
                    }
                    cmd = stop_rx.recv() => {
                        match cmd {
                            Some(SchedulerCommand::Stop) | None => {
                                logging!(info, Type::Core, "SNC Scheduler: speed_test_loop stopped");
                                return;
                            }
                        }
                    }
                }
            }
        });
    }

    fn spawn_group_detect_loop(
        interval_minutes: u64,
        mut stop_rx: mpsc::UnboundedReceiver<SchedulerCommand>,
    ) {
        if interval_minutes == 0 {
            return;
        }
        tokio::spawn(async move {
            let interval = Duration::from_secs(interval_minutes.saturating_mul(60));
            logging!(
                info,
                Type::Core,
                "SNC Scheduler: group_detect_loop started (interval: {}min)",
                interval_minutes
            );

            loop {
                tokio::select! {
                    _ = sleep(interval) => {
                        logging!(debug, Type::Core, "SNC Scheduler: group_detect tick");
                        // 分组检测逻辑由 feat 模块处理
                    }
                    cmd = stop_rx.recv() => {
                        match cmd {
                            Some(SchedulerCommand::Stop) | None => {
                                logging!(info, Type::Core, "SNC Scheduler: group_detect_loop stopped");
                                return;
                            }
                        }
                    }
                }
            }
        });
    }

    fn spawn_history_cleanup_loop(
        _retention_days: u32,
        mut stop_rx: mpsc::UnboundedReceiver<SchedulerCommand>,
    ) {
        // 每天凌晨执行一次清理
        tokio::spawn(async move {
            let interval = Duration::from_secs(24 * 60 * 60); // 24小时
            logging!(info, Type::Core, "SNC Scheduler: history_cleanup_loop started");

            loop {
                tokio::select! {
                    _ = sleep(interval) => {
                        logging!(debug, Type::Core, "SNC Scheduler: history_cleanup tick");
                        // 历史清理逻辑由 history 模块处理
                    }
                    cmd = stop_rx.recv() => {
                        match cmd {
                            Some(SchedulerCommand::Stop) | None => {
                                logging!(info, Type::Core, "SNC Scheduler: history_cleanup_loop stopped");
                                return;
                            }
                        }
                    }
                }
            }
        });
    }
}
