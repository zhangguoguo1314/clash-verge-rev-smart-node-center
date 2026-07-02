export interface HistoryRecord {
  node_uid: string
  timestamp: string
  type: 'speed_test' | 'health_check' | 'switch' | 'group_detect'
  data: Record<string, unknown>
}

export interface NodeStats {
  uid: string
  success_rate: number
  failure_rate: number
  avg_latency_ms: number
  avg_speed_mbps: number
  online_rate: number
  total_tests: number
}
