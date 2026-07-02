import { invoke } from '@tauri-apps/api/core'

import type {
  DashboardData,
  GroupDetectResult,
  HistoryRecord,
  ImportResult,
  IpInfo,
  NodeStats,
  PoolStats,
  SmartGroup,
  SmartNode,
  SmartNodeConfig,
  SpeedTestResult,
  SwitchResult,
} from '@/types/smart-node'

// Pool commands
export const sncGetPool = () =>
  invoke<SmartNode[]>('snc_get_pool')

export const sncAddNodes = (
  nodes: SmartNode[],
  sourceName: string,
) =>
  invoke<ImportResult>('snc_add_nodes', {
    nodes,
    sourceName,
  })

export const sncRemoveNode = (uid: string) =>
  invoke<void>('snc_remove_node', { uid })

export const sncUpdateNode = (
  uid: string,
  update: Partial<SmartNode>,
) =>
  invoke<void>('snc_update_node', { uid, update })

export const sncGetPoolStats = () =>
  invoke<PoolStats>('snc_get_pool_stats')

// Import/Export
export const sncImportNodes = (
  content: string,
  format: string,
  sourceName: string,
) =>
  invoke<ImportResult>('snc_import_nodes', {
    content,
    format,
    sourceName,
  })

export const sncExportNodes = (
  nodeUids: string[],
  format: string,
  outputPath: string,
) =>
  invoke<void>('snc_export_nodes', {
    nodeUids,
    format,
    outputPath,
  })

// Group commands
export const sncGetGroups = () =>
  invoke<SmartGroup[]>('snc_get_groups')

export const sncCreateGroup = (group: Partial<SmartGroup>) =>
  invoke<SmartGroup>('snc_create_group', { group })

export const sncUpdateGroup = (
  id: string,
  group: Partial<SmartGroup>,
) =>
  invoke<void>('snc_update_group', { id, group })

export const sncDeleteGroup = (id: string) =>
  invoke<void>('snc_delete_group', { id })

export const sncDetectGroup = (groupId: string) =>
  invoke<GroupDetectResult>('snc_detect_group', {
    groupId,
  })

// Speed test
export const sncStartSpeedTest = (
  nodeUids: string[],
  testTypes: string[],
) =>
  invoke<void>('snc_start_speed_test', {
    nodeUids,
    testTypes,
  })

export const sncGetSpeedResults = () =>
  invoke<SpeedTestResult[]>('snc_get_speed_results')

export const sncStopSpeedTest = () =>
  invoke<void>('snc_stop_speed_test')

// Switch
export const sncGetSwitchStatus = () =>
  invoke<SwitchResult[]>('snc_get_switch_status')

export const sncTriggerSwitch = (groupId: string) =>
  invoke<SwitchResult>('snc_trigger_switch', {
    groupId,
  })

// Health
export const sncStartHealthCheck = (nodeUids?: string[]) =>
  invoke<void>('snc_start_health_check', { nodeUids })

export const sncGetHealthStatus = () =>
  invoke<Record<string, string>>('snc_get_health_status')

// History
export const sncGetHistory = (
  nodeUid: string,
  days: number,
) =>
  invoke<HistoryRecord[]>('snc_get_history', {
    nodeUid,
    days,
  })

export const sncGetStats = (
  nodeUid: string,
  days: number,
) =>
  invoke<NodeStats>('snc_get_stats', {
    nodeUid,
    days,
  })

export const sncCleanupHistory = () =>
  invoke<void>('snc_cleanup_history')

// Dashboard
export const sncGetDashboardData = () =>
  invoke<DashboardData>('snc_get_dashboard_data')

export const sncGetIpInfo = () =>
  invoke<IpInfo>('snc_get_ip_info')

export const sncCheckMediaUnlock = (nodeUid?: string) =>
  invoke<Record<string, boolean>>(
    'snc_check_media_unlock',
    { nodeUid },
  )

export const sncCheckCustomUrl = (url: string) =>
  invoke<boolean>('snc_check_custom_url', { url })

// Settings
export const sncGetConfig = () =>
  invoke<SmartNodeConfig>('snc_get_config')

export const sncPatchConfig = (
  config: Partial<SmartNodeConfig>,
) =>
  invoke<void>('snc_patch_config', { config })

export const sncToggleEnabled = (enabled: boolean) =>
  invoke<void>('snc_toggle_enabled', { enabled })

export const sncIsEnabled = () =>
  invoke<boolean>('snc_is_enabled')
