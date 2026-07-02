export interface SpeedTestConfig {
  concurrent_limit: number
  tcp_timeout_ms: number
  http_timeout_ms: number
  download_timeout_ms: number
  download_url: string
  download_size: number
  cache_ttl_seconds: number
  schedule_interval_minutes: number
}

export interface ScoringConfig {
  latency_weight: number
  speed_weight: number
  stability_weight: number
  online_rate_weight: number
  latency_excellent: number
  latency_good: number
  latency_poor: number
}

export interface AutoSwitchConfig {
  enabled: boolean
  default_strategy:
    | 'fastest'
    | 'most_stable'
    | 'sequential'
    | 'random'
    | 'weighted'
    | 'ai_recommend'
  default_interval_minutes: number
  fail_count_threshold: number
  enable_recovery_switch_back: boolean
  recovery_check_interval_minutes: number
  master_pool_fallback: boolean
}

export interface HealthCheckConfig {
  enabled: boolean
  interval_minutes: number
  timeout_ms: number
  check_method: string
  check_url: string
}

export interface HistoryConfig {
  retention_days: number
  auto_cleanup: boolean
  cleanup_time: string
}

export interface DashboardConfig {
  refresh_interval_seconds: number
  show_media_detect: boolean
}

export interface SmartNodeConfig {
  enabled: boolean
  speed_test: SpeedTestConfig
  scoring: ScoringConfig
  auto_switch: AutoSwitchConfig
  health_check: HealthCheckConfig
  history: HistoryConfig
  dashboard: DashboardConfig
}
