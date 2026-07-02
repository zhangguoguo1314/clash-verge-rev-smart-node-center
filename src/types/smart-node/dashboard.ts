export interface DashboardData {
  ip_info?: IpInfo
  current_node?: string
  current_latency?: number
  download_speed: number
  upload_speed: number
  cpu_usage: number
  memory_usage: number
  uptime_minutes: number
  switch_count: number
  speed_test_count: number
  subscription_count: number
  node_count: number
  availability_rate: number
  avg_score: number
  media_unlock?: Record<string, boolean>
}

export interface IpInfo {
  ip: string
  country?: string
  country_code?: string
  isp?: string
  city?: string
}
