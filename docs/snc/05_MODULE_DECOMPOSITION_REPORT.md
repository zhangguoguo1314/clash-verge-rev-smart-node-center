# Smart Node Center - 模块划分报告

> 版本：1.0 | 日期：2026-07-02 | 基于：Clash Verge Rev v2.5.2 源码分析

---

## 1. 模块划分原则

| 原则 | 说明 |
|---|---|
| 单一职责 | 每个模块只负责一个明确的功能域 |
| 物理隔离 | SNC 模块与官方模块物理隔离（独立目录） |
| 最小依赖 | 优先复用官方已有能力，新增依赖需论证 |
| 可独立启停 | 每个子功能可通过配置独立开关 |
| 便于测试 | 模块间低耦合，便于单元测试 |

---

## 2. 模块总览

```
Smart Node Center (SNC)
│
├── 后端模块 (Rust)
│   ├── M-SNC-CMD       IPC 命令层
│   ├── M-SNC-CONFIG    配置管理层
│   ├── M-SNC-POOL      Master Pool 管理层
│   ├── M-SNC-GROUP     分组管理层
│   ├── M-SNC-PARSER    格式解析引擎
│   ├── M-SNC-DETECT    协议检测引擎
│   ├── M-SNC-SPEED     测速引擎
│   ├── M-SNC-SWITCH    自动切换引擎
│   ├── M-SNC-HEALTH    健康管理引擎
│   ├── M-SNC-GROUP-DET 分组目标检测引擎
│   ├── M-SNC-HISTORY   历史数据引擎
│   ├── M-SNC-SCHED     调度器
│   ├── M-SNC-NOTIFY    事件通知
│   └── M-SNC-MGR       SNC 管理器（生命周期）
│
└── 前端模块 (React/TypeScript)
    ├── M-SNC-FE-PAGES     SNC 页面
    ├── M-SNC-FE-COMPS     SNC 组件
    ├── M-SNC-FE-HOOKS     SNC Hooks
    ├── M-SNC-FE-SERVICE   SNC 服务层
    ├── M-SNC-FE-PROVIDER  SNC Context Provider
    └── M-SNC-FE-TYPES     SNC 类型定义
```

---

## 3. 后端模块详细划分

### 3.1 M-SNC-CMD — IPC 命令层

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/cmd/smart_node/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `cmd/mod.rs` 的 `StringifyErr` trait 和 `CmdResult` 类型 |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**子模块**：

| 文件 | 功能 | 命令数量 |
|---|---|---|
| `mod.rs` | 模块导出、公共工具 | - |
| `pool.rs` | Master Pool 命令 | 7 |
| `group.rs` | 分组管理命令 | 5 |
| `speed_test.rs` | 测速命令 | 3 |
| `switch.rs` | 自动切换命令 | 3 |
| `health.rs` | 健康检测命令 | 2 |
| `history.rs` | 历史数据命令 | 3 |
| `dashboard.rs` | Dashboard 命令 | 4 |
| `import_export.rs` | 导入导出命令 | 3 |
| `media_detect.rs` | 流媒体检测命令 | 2 |
| `settings.rs` | SNC 设置命令 | 4 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 标准 Tauri 命令模式 |
| 性能风险 | 🟢 低 | 命令层薄封装，无重逻辑 |
| 官方兼容风险 | 🟢 低 | 独立目录 |
| 升级维护风险 | 🟢 低 | 与官方 cmd 无依赖 |

---

### 3.2 M-SNC-CONFIG — 配置管理层

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/config/smart_node/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `clash-verge-draft` crate 的 Draft 模式、`serde_yaml_ng`、`help::save_yaml` / `help::read_yaml` |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响（独立配置文件） |

**子模块**：

| 文件 | 功能 |
|---|---|
| `mod.rs` | 模块导出、配置初始化 |
| `snc_config.rs` | `ISmartNodeConfig` 结构体（SNC 全局配置） |
| `pool.rs` | `ISmartNodePool` 结构体（Master Pool 数据模型） |
| `groups.rs` | `ISmartNodeGroups` 结构体（分组数据模型） |
| `history.rs` | 历史数据配置常量 |

**配置文件**：

| 文件 | 格式 | 说明 |
|---|---|---|
| `smart_node/config.yaml` | YAML | SNC 全局配置 |
| `smart_node/pool.yaml` | YAML | Master Pool 节点数据 |
| `smart_node/groups.yaml` | YAML | 分组配置 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 复用官方 Draft 模式 |
| 性能风险 | 🟡 中 | pool.yaml 可能较大，需要优化读写 |
| 官方兼容风险 | 🟢 低 | 独立文件 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 3.3 M-SNC-POOL — Master Pool 管理层

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/feat/smart_node/pool.rs` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `tauri-plugin-mihomo` 的节点信息获取能力 |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**功能**：

| 功能 | 描述 | 优先级 |
|---|---|---|
| 节点 CRUD | 添加、查询、更新、删除节点 | P0 |
| 去重引擎 | 基于（协议+地址+端口）去重 | P0 |
| 自动分类 | 根据协议/地域/运营商分类 | P1 |
| 引用计数 | 跟踪节点被多少分组引用 | P0 |
| 批量导入 | 支持批量导入节点 | P0 |
| 批量导出 | 支持批量导出节点 | P0 |
| Pool 统计 | 节点总数、可用数、分类统计 | P0 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 标准 CRUD 操作 |
| 性能风险 | 🟡 中 | 大量节点时的文件 IO 性能 |
| 官方兼容风险 | 🟢 低 | 不依赖官方模块 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 3.4 M-SNC-GROUP — 分组管理层

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/feat/smart_node/group.rs` |
| 类型 | 新增模块 |
| 复用官方 | 无 |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**功能**：

| 功能 | 描述 | 优先级 |
|---|---|---|
| 分组 CRUD | 创建、查询、更新、删除分组 | P0 |
| 节点关联 | 分组与 Master Pool 节点关联/解除 | P0 |
| 备用节点 | 配置分组的备用节点 | P0 |
| 重命名 | 分组重命名 | P1 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 标准 CRUD + 关联管理 |
| 性能风险 | 🟢 低 | 操作轻量 |
| 官方兼容风险 | 🟢 低 | 独立模块 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 3.5 M-SNC-PARSER — 格式解析引擎

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/feat/smart_node/parser/` |
| 类型 | 新增模块 |
| 复用官方 | YAML 复用 `serde_yaml_ng`，JSON 复用 `serde_json` |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**子模块**：

| 文件 | 格式 | 复用 | 新增依赖 |
|---|---|---|---|
| `mod.rs` | - | - | - |
| `base64.rs` | Base64 | `base64` (官方已有) | 无 |
| `yaml.rs` | YAML | `serde_yaml_ng` (官方已有) | 无 |
| `json.rs` | JSON | `serde_json` (官方已有) | 无 |
| `txt.rs` | TXT | - | 无 |
| `csv.rs` | CSV | - | `csv` crate (MIT) |
| `excel.rs` | Excel | - | 待确认（见 D3 决策） |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟡 中 | Excel 解析可能有兼容性问题 |
| 性能风险 | 🟢 低 | 导入为低频操作 |
| 官方兼容风险 | 🟢 低 | 独立模块 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 3.6 M-SNC-DETECT — 协议检测引擎

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/feat/smart_node/detector/` |
| 类型 | 新增模块 |
| 复用官方 | **复用前端** `src/utils/uri-parser/` 的协议识别逻辑（在前端完成 URI 解析，后端接收结构化数据） |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**说明**：URI 解析在前端完成（复用官方 uri-parser），后端接收解析后的结构化节点数据。后端主要负责协议类型验证。

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 复用官方 uri-parser |
| 性能风险 | 🟢 低 | 解析操作轻量 |
| 官方兼容风险 | 🟢 低 | 只读取，不修改 |
| 升级维护风险 | 🟢 低 | 官方 uri-parser 更新时跟随 |

---

### 3.7 M-SNC-SPEED — 测速引擎

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/speed_test/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `reqwest` (HTTP 测速)、`tokio` (TCP 测速)、`tauri-plugin-mihomo` (延迟测试 API) |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**子模块**：

| 文件 | 功能 | 复用 | 优先级 |
|---|---|---|---|
| `mod.rs` | 测速引擎协调 | - | - |
| `tcp_ping.rs` | TCP 延迟测试 | `tokio::net::TcpStream` | P0 |
| `http_ping.rs` | HTTP/HTTPS 延迟测试 | `reqwest::Client` | P0 |
| `download.rs` | 下载速度测试 | `reqwest::Client` | P1 |
| `upload.rs` | 上传速度测试 | `reqwest::Client` | P2 |
| `dns.rs` | DNS 延迟测试 | `tokio::net::UdpSocket` | P2 |
| `jitter.rs` | Jitter/Loss 测试 | 连续多次 TCP ping | P2 |
| `scorer.rs` | 评分引擎 | - | P0 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟡 中 | 多种测速方式实现，网络环境复杂 |
| 性能风险 | 🟡 中 | 并发测速消耗资源（需限制并发数） |
| 官方兼容风险 | 🟢 低 | 独立模块，复用官方 reqwest |
| 升级维护风险 | 🟢 低 | 依赖稳定 |

---

### 3.8 M-SNC-SWITCH — 自动切换引擎

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/switch/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `tauri-plugin-mihomo` 的 `select_node_for_group` API |
| 影响现有逻辑 | ❌ 不影响（通过 Mihomo API 操作，不修改官方代理逻辑） |
| 影响配置兼容 | ❌ 不影响 |

**子模块**：

| 文件 | 功能 | 优先级 |
|---|---|---|
| `mod.rs` | 模块导出 | - |
| `strategy.rs` | 切换策略定义（Fastest/MostStable/Sequential/Random/Weighted/AIRecommend） | P0 |
| `executor.rs` | 切换执行器（调用 Mihomo API + 记录历史） | P0 |
| `scheduler.rs` | 切换调度器（定时触发） | P0 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟡 中 | 依赖 Mihomo API 稳定性 |
| 性能风险 | 🟢 低 | 切换操作轻量 |
| 官方兼容风险 | 🟡 中 | Mihomo API 变更可能影响 |
| 升级维护风险 | 🟡 中 | 需跟随 tauri-plugin-mihomo 更新 |

---

### 3.9 M-SNC-HEALTH — 健康管理引擎

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/health/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `reqwest`（HTTP 健康检测）、`tokio`（TCP 健康检测） |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**子模块**：

| 文件 | 功能 | 优先级 |
|---|---|---|
| `mod.rs` | 模块导出 | - |
| `checker.rs` | 健康检测器（TCP/HTTP/HTTPS） | P0 |
| `failover.rs` | 故障转移（掉线切换逻辑） | P0 |
| `recovery.rs` | 恢复检测（掉线节点恢复检查） | P0 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | TCP/HTTP 检测实现简单 |
| 性能风险 | 🟡 中 | 大量节点定期检测可能耗时 |
| 官方兼容风险 | 🟢 低 | 独立模块 |
| 升级维护风险 | 🟢 低 | 依赖稳定 |

---

### 3.10 M-SNC-GROUP-DET — 分组目标检测引擎

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/group_detector/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `reqwest`（HTTP/HTTPS/API 检测）、`tokio`（TCP 检测） |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**子模块**：

| 文件 | 功能 | 优先级 |
|---|---|---|
| `mod.rs` | 模块导出 | - |
| `http.rs` | HTTP 检测器 | P0 |
| `https.rs` | HTTPS 检测器 | P0 |
| `tcp.rs` | TCP 检测器 | P0 |
| `api.rs` | API 检测器（响应内容匹配） | P0 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 标准 HTTP/TCP 检测 |
| 性能风险 | 🟡 中 | 多网址 × 多节点并发 |
| 官方兼容风险 | 🟢 低 | 独立模块 |
| 升级维护风险 | 🟢 低 | 依赖稳定 |

---

### 3.11 M-SNC-HISTORY — 历史数据引擎

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/history/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `serde_json`（JSONL 序列化）、`chrono`（时间处理） |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**子模块**：

| 文件 | 功能 | 优先级 |
|---|---|---|
| `mod.rs` | 模块导出 | - |
| `collector.rs` | 数据采集器（写入 JSONL） | P0 |
| `aggregator.rs` | 数据聚合器（按小时/天聚合） | P1 |
| `cleaner.rs` | 数据清理器（按保留周期清理） | P0 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟡 中 | JSONL 文件膨胀问题 |
| 性能风险 | 🟡 中 | 大量历史数据读写性能 |
| 官方兼容风险 | 🟢 低 | 独立目录 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 3.12 M-SNC-SCHED — 调度器

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/scheduler.rs` |
| 类型 | 新增模块（扩展模块） |
| 复用官方 | 复用 `core/timer.rs` 的 DelayQueue 模式 |
| 影响现有逻辑 | ❌ 不影响（SNC 有独立调度器实例） |
| 影响配置兼容 | ❌ 不影响 |

**功能**：

- 统一管理 SNC 所有定时任务
- 复用 `tokio::sync::mpsc` + `DelayQueue` 模式
- SNC 启用时启动所有定时器
- SNC 关闭时停止所有定时器

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 复用官方成熟模式 |
| 性能风险 | 🟢 低 | 定时器开销极小 |
| 官方兼容风险 | 🟢 低 | 独立实例 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 3.13 M-SNC-NOTIFY — 事件通知

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/notification.rs` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `core/notification.rs` 的 `NotificationSystem` |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**说明**：SNC 通过官方 NotificationSystem 向前端发送事件，使用 `snc://` 前缀区分。

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 复用官方机制 |
| 性能风险 | 🟢 低 | 事件通知开销极小 |
| 官方兼容风险 | 🟢 低 | 使用官方 API |
| 升级维护风险 | 🟢 低 | 跟随官方更新 |

---

### 3.14 M-SNC-MGR — SNC 管理器

| 属性 | 说明 |
|---|---|
| 目录 | `src-tauri/src/core/smart_node/manager.rs` |
| 类型 | 新增模块 |
| 复用官方 | 参考 `core/manager/mod.rs` 的单例模式 |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**功能**：

- SNC 全局生命周期管理（初始化、启动、停止、销毁）
- 协调所有 SNC 子模块
- 全局开关控制
- 错误隔离（catch_unwind）

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 标准生命周期管理 |
| 性能风险 | 🟢 低 | 管理器开销极小 |
| 官方兼容风险 | 🟢 低 | 独立模块 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

## 4. 前端模块详细划分

### 4.1 M-SNC-FE-PAGES — SNC 页面

| 属性 | 说明 |
|---|---|
| 目录 | `src/pages/smart-node/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 MUI 组件、BasePage 模式 |
| 影响现有逻辑 | ❌ 不影响（独立页面） |
| 影响配置兼容 | ❌ 不影响 |

**页面**：

| 文件 | 功能 | 优先级 |
|---|---|---|
| `index.tsx` | SNC 入口页面（Tab 布局） | P0 |
| `dashboard.tsx` | Dashboard 页面 | P0 |
| `pool.tsx` | Master Pool 节点管理 | P0 |
| `groups.tsx` | 分组管理 | P0 |
| `speed-test.tsx` | 测速页面 | P0 |
| `history.tsx` | 历史数据页面 | P1 |
| `settings.tsx` | SNC 设置页面 | P0 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 标准 React 页面 |
| 性能风险 | 🟢 低 | 懒加载，按需渲染 |
| 官方兼容风险 | 🟢 低 | 独立目录 |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 4.2 M-SNC-FE-COMPS — SNC 组件

| 属性 | 说明 |
|---|---|
| 目录 | `src/components/smart-node/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 `components/base/` 的基础组件 |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**子组件目录**：`dashboard/`、`pool/`、`groups/`、`speed-test/`、`history/`、`settings/`

**风险评估**：🟢 所有维度均为低风险。

---

### 4.3 M-SNC-FE-HOOKS — SNC Hooks

| 属性 | 说明 |
|---|---|
| 目录 | `src/hooks/use-smart-node/` |
| 类型 | 新增模块 |
| 复用官方 | 复用 SWR useQuery 模式（`services/query-client.ts`） |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**Hooks**：

| 文件 | 功能 |
|---|---|
| `use-smart-node-enabled.ts` | SNC 开关状态（从 verge config 读取） |
| `use-pool.ts` | Master Pool 数据（SWR query key: `['sncGetPool']`） |
| `use-groups.ts` | 分组数据（SWR query key: `['sncGetGroups']`） |
| `use-speed-test.ts` | 测速状态 |
| `use-health.ts` | 健康检测状态 |
| `use-switch.ts` | 切换状态 |
| `use-history.ts` | 历史数据 |

**风险评估**：🟢 所有维度均为低风险。

---

### 4.4 M-SNC-FE-SERVICE — SNC 服务层

| 属性 | 说明 |
|---|---|
| 目录 | `src/services/smart-node.ts` |
| 类型 | 新增模块 |
| 复用官方 | 参考 `services/cmds.ts` 的 invoke 封装模式 |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**风险评估**：🟢 所有维度均为低风险。

---

### 4.5 M-SNC-FE-PROVIDER — SNC Context Provider

| 属性 | 说明 |
|---|---|
| 目录 | `src/providers/smart-node/` |
| 类型 | 新增模块 |
| 复用官方 | 参考 `providers/app-data-provider.tsx` 模式 |
| 影响现有逻辑 | ❌ 不影响（独立 Provider，在 main.tsx 中追加包裹） |
| 影响配置兼容 | ❌ 不影响 |

**风险评估**：

| 风险类型 | 等级 | 说明 |
|---|---|---|
| 技术风险 | 🟢 低 | 标准 React Context |
| 性能风险 | 🟢 低 | 仅在 SNC 启用时挂载 |
| 官方兼容风险 | 🟢 低 | 独立 Provider |
| 升级维护风险 | 🟢 低 | 独立模块 |

---

### 4.6 M-SNC-FE-TYPES — SNC 类型定义

| 属性 | 说明 |
|---|---|
| 目录 | `src/types/smart-node/` |
| 类型 | 新增模块 |
| 复用官方 | 无 |
| 影响现有逻辑 | ❌ 不影响 |
| 影响配置兼容 | ❌ 不影响 |

**文件**：`pool.ts`、`group.ts`、`speed-test.ts`、`health.ts`、`switch.ts`、`history.ts`、`config.ts`、`dashboard.ts`

**风险评估**：🟢 所有维度均为低风险。

---

## 5. 模块依赖矩阵

### 5.1 后端模块依赖

```
M-SNC-MGR (管理器)
    ├── M-SNC-CONFIG (配置管理)
    ├── M-SNC-SCHED (调度器)
    ├── M-SNC-NOTIFY (事件通知)
    │
    ├── M-SNC-POOL (Master Pool)
    │   └── M-SNC-CONFIG
    │
    ├── M-SNC-GROUP (分组管理)
    │   └── M-SNC-POOL (引用节点)
    │
    ├── M-SNC-PARSER (格式解析)
    │   └── M-SNC-DETECT (协议检测)
    │
    ├── M-SNC-SPEED (测速引擎)
    │   └── M-SNC-HISTORY (写入记录)
    │
    ├── M-SNC-SWITCH (自动切换)
    │   ├── M-SNC-POOL (选择节点)
    │   ├── M-SNC-GROUP (分组信息)
    │   └── M-SNC-HISTORY (写入记录)
    │
    ├── M-SNC-HEALTH (健康管理)
    │   ├── M-SNC-POOL (更新状态)
    │   ├── M-SNC-SWITCH (触发切换)
    │   └── M-SNC-HISTORY (写入记录)
    │
    ├── M-SNC-GROUP-DET (分组目标检测)
    │   ├── M-SNC-GROUP (更新分组)
    │   └── M-SNC-SPEED (延迟测试)
    │
    └── M-SNC-HISTORY (历史数据)
        └── M-SNC-CONFIG (配置)
```

### 5.2 前端模块依赖

```
M-SNC-FE-PROVIDER (Context)
    └── M-SNC-FE-SERVICE (API 调用)

M-SNC-FE-PAGES (页面)
    ├── M-SNC-FE-COMPS (组件)
    ├── M-SNC-FE-HOOKS (Hooks)
    │   └── M-SNC-FE-SERVICE (API 调用)
    └── M-SNC-FE-PROVIDER (Context)

M-SNC-FE-COMPS (组件)
    └── M-SNC-FE-TYPES (类型)

M-SNC-FE-HOOKS (Hooks)
    └── M-SNC-FE-TYPES (类型)

M-SNC-FE-SERVICE (API)
    └── M-SNC-FE-TYPES (类型)
```

---

## 6. 各模块风险汇总

| 模块 | 技术风险 | 性能风险 | 兼容风险 | 维护风险 | 新增依赖 | 建议 |
|---|---|---|---|---|---|---|
| M-SNC-CMD | 🟢 | 🟢 | 🟢 | 🟢 | 无 | 标准实现 |
| M-SNC-CONFIG | 🟢 | 🟡 | 🟢 | 🟢 | 无 | 注意大文件性能 |
| M-SNC-POOL | 🟢 | 🟡 | 🟢 | 🟢 | 无 | 注意大量节点性能 |
| M-SNC-GROUP | 🟢 | 🟢 | 🟢 | 🟢 | 无 | 标准实现 |
| M-SNC-PARSER | 🟡 | 🟢 | 🟢 | 🟢 | csv crate (可选) | Excel 兼容性测试 |
| M-SNC-DETECT | 🟢 | 🟢 | 🟢 | 🟢 | 无 | 复用官方 uri-parser |
| M-SNC-SPEED | 🟡 | 🟡 | 🟢 | 🟢 | 无 | 并发控制 |
| M-SNC-SWITCH | 🟡 | 🟢 | 🟡 | 🟡 | 无 | 版本检测 |
| M-SNC-HEALTH | 🟢 | 🟡 | 🟢 | 🟢 | 无 | 并发控制 |
| M-SNC-GROUP-DET | 🟢 | 🟡 | 🟢 | 🟢 | 无 | 并发控制 |
| M-SNC-HISTORY | 🟡 | 🟡 | 🟢 | 🟢 | 无 | 数据膨胀处理 |
| M-SNC-SCHED | 🟢 | 🟢 | 🟢 | 🟢 | 无 | 复用官方模式 |
| M-SNC-NOTIFY | 🟢 | 🟢 | 🟢 | 🟢 | 无 | 复用官方机制 |
| M-SNC-MGR | 🟢 | 🟢 | 🟢 | 🟢 | 无 | catch_unwind 隔离 |
| M-SNC-FE-* | 🟢 | 🟢 | 🟢 | 🟢 | recharts (建议) | 懒加载 |

---

## 7. 开发优先级和阶段

### 7.1 P0 阶段（核心功能）

| 模块 | 优先级 | 预估工作量 |
|---|---|---|
| M-SNC-CONFIG | P0 | 中 |
| M-SNC-CMD | P0 | 中 |
| M-SNC-POOL | P0 | 中 |
| M-SNC-DETECT | P0 | 小 |
| M-SNC-PARSER (Base64/YAML/JSON/TXT) | P0 | 中 |
| M-SNC-SPEED (TCP/HTTP/HTTPS + 评分) | P0 | 大 |
| M-SNC-SWITCH (最快/最稳定) | P0 | 中 |
| M-SNC-HEALTH | P0 | 中 |
| M-SNC-GROUP-DET | P0 | 中 |
| M-SNC-MGR | P0 | 中 |
| M-SNC-SCHED | P0 | 中 |
| M-SNC-NOTIFY | P0 | 小 |
| M-SNC-HISTORY (基础) | P0 | 中 |
| M-SNC-FE-* (基础页面) | P0 | 大 |

### 7.2 P1 阶段（增强功能）

| 模块 | 优先级 | 预估工作量 |
|---|---|---|
| M-SNC-SPEED (下载/TTFB) | P1 | 中 |
| M-SNC-HISTORY (聚合/图表) | P1 | 中 |
| M-SNC-PARSER (CSV/Excel) | P1 | 中 |
| M-SNC-GROUP (自动同步/重命名) | P1 | 小 |
| M-SNC-FE-HISTORY | P1 | 中 |

### 7.3 P2 阶段（扩展功能）

| 模块 | 优先级 | 预估工作量 |
|---|---|---|
| M-SNC-SPEED (上传/DNS/Jitter/Loss) | P2 | 中 |
| M-SNC-SWITCH (顺序/随机/权重/AI推荐) | P2 | 大 |
| M-SNC-HEALTH (恢复切回) | P2 | 小 |
| M-SNC-GROUP (智能补组) | P2 | 中 |
| M-SNC-POOL (自动修复/自动导出/自动分组) | P2 | 大 |
