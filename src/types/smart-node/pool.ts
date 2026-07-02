export type NodeStatus = 'unknown' | 'healthy' | 'unhealthy'

export interface SmartNode {
  uid: string
  name: string
  protocol: string
  address: string
  port: number
  uri?: string
  source?: string
  source_url?: string
  tags: string[]
  status: NodeStatus
  score: number
  latency_ms?: number
  download_speed_mbps?: number
  last_health_check?: string
  last_speed_test?: string
  created_at: string
  updated_at: string
  fail_count: number
}

export interface PoolStats {
  total_count: number
  healthy_count: number
  unhealthy_count: number
  unknown_count: number
}

export interface ImportResult {
  imported: number
  duplicated: number
  failed: number
  errors: string[]
}

export interface SmartNodeInput {
  name?: string
  protocol: string
  address: string
  port: number
  uri: string
  source?: string
  source_url?: string
  tags?: string[]
}
