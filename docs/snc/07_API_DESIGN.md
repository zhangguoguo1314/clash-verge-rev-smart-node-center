# Smart Node Center - API 设计文档

> 版本：1.0 | 日期：2026-07-02

---

## 1. IPC 命令完整列表

### Master Pool (4 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_get_pool | 无 | JSON (ISmartNodePool) |
| snc_add_nodes | nodes: Vec, sourceName: string | JSON (ImportResult) |
| snc_remove_node | uid: string | () |
| snc_get_pool_stats | 无 | JSON (PoolStats) |

### 导入导出 (2 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_import_nodes | content: string, format: string, sourceName: string | JSON (ImportResult) |
| snc_export_nodes | nodeUids: Vec, format: string, outputPath: string | () |

### 分组管理 (5 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_get_groups | 无 | JSON (ISmartNodeGroups) |
| snc_create_group | group: JSON | () |
| snc_update_group | id: string, group: JSON | () |
| snc_delete_group | id: string | () |
| snc_detect_group | groupId: string | JSON (GroupDetectResult) |

### 测速 (2 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_test_node_speed | uid: string | JSON (SpeedTestResult) |
| snc_test_all_nodes | 无 | JSON (HashMap) |

### 自动切换 (1 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_switch_group_node | groupId: string | JSON (SwitchResult) |

### 健康检测 (2 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_check_node_health | uid: string | JSON (HealthCheckResult) |
| snc_check_all_health | 无 | JSON (HashMap) |

### 历史数据 (2 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_get_today_history | 无 | JSON (Vec) |
| snc_cleanup_history | 无 | () |

### Dashboard (1 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_get_dashboard_data | 无 | JSON (DashboardData) |

### 设置 (4 个命令)
| 命令 | 参数 | 返回 |
|---|---|---|
| snc_is_enabled | 无 | bool |
| snc_toggle_enabled | enabled: bool | () |
| snc_get_config | 无 | JSON (ISmartNodeConfig) |
| snc_patch_config | config: JSON | () |

**总计：23 个 IPC 命令**

## 2. 事件通知列表

| 事件名 | 数据 | 触发场景 |
|---|---|---|
| snc://pool-updated | PoolStats | Master Pool 变更 |
| snc://speed-test-completed | results | 测速完成 |
| snc://node-switched | SwitchResult | 节点切换 |
| snc://health-status-changed | {uid, status} | 健康状态变化 |
| snc://group-detect-completed | GroupDetectResult | 分组检测完成 |
| snc://config-changed | config | 配置变更 |
