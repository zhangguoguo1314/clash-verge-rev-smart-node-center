export interface DetectionUrl {
  url: string
  method: 'http' | 'https' | 'tcp' | 'api'
  expected_status?: number
  timeout_ms: number
}

export interface GroupAutoSwitch {
  enabled: boolean
  strategy: string
  interval_minutes: number
}

export interface SmartGroup {
  id: string
  name: string
  description?: string
  auto_switch: GroupAutoSwitch
  detection: {
    urls: DetectionUrl[]
  }
  auto_sync: boolean
  sync_interval_minutes: number
  node_uids: string[]
  fallback_node_uids: string[]
  created_at: string
  updated_at: string
}

export interface GroupDetectResult {
  group_id: string
  successful_uids: string[]
  failed_uids: string[]
  errors: string[]
}
