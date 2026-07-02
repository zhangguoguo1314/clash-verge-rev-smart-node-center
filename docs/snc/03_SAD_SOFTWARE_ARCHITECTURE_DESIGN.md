# Smart Node Center - 软件架构设计（SAD）

> 版本：1.0 | 日期：2026-07-02 | 基于：Clash Verge Rev v2.5.2 源码分析

---

## 1. 架构设计原则

### 1.1 核心原则

| 原则 | 描述 | 优先级 |
|---|---|---|
| 零侵入 | 不修改任何官方源码文件 | P0 |
| 插件化 | SNC 作为独立模块，可通过开关完全启停 | P0 |
| 物理隔离 | SNC 代码与官方代码物理隔离（独立目录） | P0 |
| 技术一致 | 复用官方技术栈（Tauri + React + MUI + SWR + YAML） | P0 |
| 配置隔离 | SNC 配置文件独立，不修改官方配置格式 | P0 |
| 可合并 | 后续同步官方更新时，SNC 代码无冲突 | P0 |

### 1.2 架构约束

| 约束 | 来源 | 说明 |
|---|---|---|
| 不修改 Mihomo 内核 | 需求文档 | 只通过 IPC 与 Mihomo 通信 |
| 不修改官方源码 | 需求文档 | SNC 全部代码为新增文件 |
| 保持 Tauri + Rust + React | 需求文档 | 不引入新框架 |
| 不引入无理由第三方库 | 需求文档 | 新依赖需论证必要性 |
| 遵循 Draft 配置模式 | 官方架构 | 新配置复用 Draft 模式 |

---

## 2. 整体架构

### 2.1 系统架构图

```
┌──────────────────────────────────────────────────────────────────┐
│                        Clash Verge Rev (官方)                      │
│                                                                    │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                      前端 (React + MUI)                     │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │              官方页面 & 组件                          │   │  │
│  │  │  Home | Proxies | Profiles | Connections | Settings │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │         Smart Node Center (SNC) 前端模块 [新增]      │   │  │
│  │  │  Dashboard | NodeManager | GroupManager | SpeedTest │   │  │
│  │  │  HistoryChart | Settings                           │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  └─────────────────────────────────────────────────────────────┘  │
│                            ↕ Tauri IPC + Event                    │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                    后端 (Rust + Tauri)                       │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │              官方模块 (不修改)                        │   │  │
│  │  │  cmd/ | config/ | core/ | enhance/ | feat/ | utils/ │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │       Smart Node Center (SNC) 后端模块 [新增]       │   │  │
│  │  │  cmd/smart_node/ | feat/smart_node/                │   │  │
│  │  │  config/smart_node/ | core/smart_node/             │   │  │
│  │  │  module/smart_node/                                │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  └─────────────────────────────────────────────────────────────┘  │
│                            ↕ IPC (LocalSocket)                  │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                    Mihomo 内核 (不修改)                     │  │
│  └─────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

### 2.2 SNC 架构分层

```
┌─────────────────────────────────────────────────────┐
│                   SNC 前端层 (React)                  │
│  pages/smart-node/          ← SNC 页面               │
│  components/smart-node/     ← SNC 组件               │
│  hooks/use-smart-node/     ← SNC Hooks              │
│  services/smart-node/       ← SNC 服务/API 调用      │
│  providers/smart-node/     ← SNC Context            │
│  types/smart-node/          ← SNC 类型定义            │
├─────────────────────────────────────────────────────┤
│                   Tauri IPC 边界                       │
├─────────────────────────────────────────────────────┤
│                   SNC 后端层 (Rust)                    │
│  cmd/smart_node/            ← IPC 命令层             │
│  feat/smart_node/           ← 业务逻辑层             │
│  config/smart_node/         ← 配置管理层             │
│  core/smart_node/           ← 核心服务层             │
│  module/smart_node/         ← 功能模块层             │
├─────────────────────────────────────────────────────┤
│                   数据存储层                            │
│  smart_node_pool.yaml       ← Master Pool            │
│  smart_node_groups.yaml     ← 分组配置                │
│  smart_node_config.yaml     ← SNC 全局配置            │
│  smart_node_history/        ← 历史数据 (JSONL)         │
│  smart_node_cache/          ← 测速缓存               │
└─────────────────────────────────────────────────────┘
```

---

## 3. 技术架构

### 3.1 前端技术架构

| 层次 | 技术 | 说明 |
|---|---|---|
| 页面路由 | React Router v8 | 新增 `/smart-node/*` 路由，懒加载 |
| UI 组件 | MUI v9 | 复用官方 MUI 组件库 |
| 数据获取 | SWR v2 | 新增 SNC 专用 query keys |
| 状态管理 | React Context | 新增 SNC Provider |
| 代码编辑 | Monaco Editor | 复用官方封装 |
| 虚拟列表 | @tanstack/react-virtual | 复用官方 VirtualList |
| 图表 | 待确认 | 需新增图表库（如 recharts 或纯 Canvas） |
| 国际化 | i18next | 新增 SNC 专用命名空间 |
| 样式 | Emotion + SCSS | 复用官方样式方案 |

### 3.2 后端技术架构

| 层次 | 技术 | 说明 |
|---|---|---|
| IPC 命令 | Tauri Command | 新增 cmd/smart_node/ 模块 |
| 异步运行时 | Tokio | 复用官方 Tokio 运行时 |
| HTTP 客户端 | reqwest | 复用官方依赖，用于测速和检测 |
| 序列化 | serde + serde_yaml_ng | 复用官方依赖 |
| 加密 | aes-gcm | 复用官方依赖（可选） |
| 定时器 | tokio::sync::mpsc + DelayQueue | 复用 timer.rs 框架 |
| 日志 | flexi_logger / clash_verge_logger | 复用官方日志系统 |
| 配置管理 | Draft 模式 | 复用 clash-verge-draft crate |

### 3.3 新增第三方依赖评估

| 依赖 | 用途 | 是否必需 | 许可证 | 官方是否已有 | 维护状态 | 对升级影响 |
|---|---|---|---|---|---|---|
| recharts | Dashboard 图表 | 建议引入 | MIT | ❌ 无 | 活跃 | 无影响（仅前端） |
| rust_xlsxwriter | Excel 导出（Rust端） | 备选 | MIT | ❌ 无 | 活跃 | 无影响 |
| calamine | Excel 导入（Rust端） | 备选 | MIT | ❌ 无 | 活跃 | 无影响 |
| maxminddb | GeoIP 离线查询 | 备选 | MIT | ❌ 无 | 活跃 | 无影响 |
| uuid | 节点 UID 生成 | 可能需要 | MIT/Apache-2.0 | ❌ 无 | 活跃 | 无影响（Rust 生态标配） |

> **待确认**：图表库选择（recharts vs 纯 Canvas 自绘 vs 其他）、Excel 处理在 Rust 端还是前端端

---

## 4. 功能架构

### 4.1 SNC 功能模块图

```
Smart Node Center
│
├── 🔌 功能开关 (Feature Toggle)
│   └── 全局启用/关闭 → verge.yaml 或 smart_node_config.yaml
│
├── 📦 导入导出 (Import/Export)
│   ├── 格式解析器 (Parser)
│   │   ├── Base64 Parser
│   │   ├── YAML Parser (复用 serde_yaml_ng)
│   │   ├── JSON Parser (复用 serde_json)
│   │   ├── TXT Parser (行解析)
│   │   ├── CSV Parser (复用 serde + csv)
│   │   └── Excel Parser (新增)
│   ├── 协议识别 (Protocol Detector) [复用 uri-parser]
│   └── 去重引擎 (Dedup Engine)
│
├── 🏊 Master Pool (节点池)
│   ├── 节点 CRUD
│   ├── 节点引用计数
│   └── 持久化 (smart_node_pool.yaml)
│
├── 📋 分组管理 (Group Manager)
│   ├── 分组 CRUD
│   ├── 目标网址检测引擎
│   │   ├── HTTP Detector
│   │   ├── HTTPS Detector
│   │   ├── TCP Detector
│   │   └── API Detector
│   ├── 自动同步引擎
│   └── 智能补组引擎
│
├── ⚡ 测速系统 (Speed Test)
│   ├── 测速引擎
│   │   ├── TCP Ping
│   │   ├── HTTP Ping
│   │   ├── HTTPS Ping
│   │   ├── TTFB
│   │   ├── 下载速度测试
│   │   ├── 上传速度测试
│   │   ├── DNS 延迟
│   │   └── Jitter/Loss
│   ├── 评分引擎 (Scoring Engine)
│   ├── 调度器 (Scheduler)
│   │   ├── 定时测速
│   │   ├── 后台测速
│   │   ├── 异步并发控制
│   │   └── 缓存管理
│   └── Mihomo 延迟测试集成
│
├── 🔄 自动切换 (Auto Switch)
│   ├── 策略引擎
│   │   ├── 最快优先 (Fastest)
│   │   ├── 最稳定优先 (MostStable)
│   │   ├── 顺序切换 (Sequential)
│   │   ├── 随机切换 (Random)
│   │   ├── 权重切换 (Weighted)
│   │   └── AI 推荐 (AIRecommend)
│   ├── 切换执行器
│   └── 切换调度器
│
├── 🛡️ 健康管理 (Health Manager)
│   ├── 健康检测引擎
│   ├── 掉线检测 (3次失败)
│   ├── 自动切换触发
│   ├── 备用节点管理
│   ├── 恢复检测引擎
│   └── Master Pool 兜底
│
├── 📊 历史数据库 (History Database)
│   ├── 数据采集器
│   ├── 数据聚合器
│   ├── 数据清理器 (按周期)
│   └── 图表生成器
│
├── 📈 Dashboard
│   ├── 实时信息面板
│   │   ├── IP/国家/运营商 (复用+新增)
│   │   ├── 当前节点/延迟 (复用 mihomo-api)
│   │   ├── 流量统计 (复用 WebSocket)
│   │   ├── 系统信息 (复用 sysinfo)
│   │   └── SNC 统计 (新增)
│   ├── 流媒体检测面板
│   │   ├── 官方检测 (复用 media_unlock_checker)
│   │   └── 新增检测 (Steam/GitHub/Google/Cloudflare)
│   └── 自定义网址检测
│
└── ⚙️ SNC 设置
    ├── 全局开关
    ├── 测速配置
    ├── 切换策略配置
    ├── 历史保留策略
    └── Dashboard 配置
```

### 4.2 模块间依赖关系

```
功能开关 ──→ 控制所有模块启停
    │
导入导出 ──→ Master Pool (写入)
    │
Master Pool ←── 分组管理 (读取引用)
    │              │
    │              ├── 目标网址检测 ──→ 测速系统 (延迟测试)
    │              └── 自动同步 ──→ 健康管理
    │
测速系统 ──→ Master Pool (更新评分/状态)
    │
健康管理 ──→ 自动切换 (触发切换)
    │              │
    │              └── Mihomo API (select_node_for_group)
    │
历史数据库 ←── 测速系统 (写入记录)
              ←── 健康管理 (写入记录)
              ←── 自动切换 (写入记录)
    │
Dashboard ←── Master Pool (读取数据)
            ←── 历史数据库 (读取历史)
            ←── 流媒体检测 (读取状态)
            ←── 系统信息 (复用)
```

---

## 5. 数据流设计

### 5.1 节点导入数据流

```
用户输入/文件
    │
    ▼
[前端] 选择导入方式（URL/文件/粘贴）
    │
    ▼ invoke("snc_import_nodes")
    │
[后端] cmd/smart_node/import.rs
    │
    ├── 格式检测 (Base64/YAML/JSON/TXT/CSV/Excel)
    │
    ▼
    │   格式解析器
    │
    ├── URI 提取
    │
    ▼
    │   协议识别 (复用 uri-parser)
    │
    ├── SmartNode 结构构建
    │
    ▼
    │   去重引擎 (与 Master Pool 比对)
    │
    ├── 去重处理 (新节点入库, 重复跳过)
    │
    ▼
    │   Master Pool 写入 (Draft apply)
    │
    ├── smart_node_pool.yaml 持久化
    │
    ▼
    │   事件通知 ("snc://pool-updated")
    │
    ▼
[前端] SWR 自动刷新 Master Pool 数据
```

### 5.2 测速数据流

```
定时器触发 / 手动触发
    │
    ▼
[后端] core/smart_node/scheduler.rs
    │
    ├── 获取待测节点列表 (Master Pool)
    │
    ├── 并发控制 (Semaphore, 最多 20 并发)
    │
    ▼
    │   测速引擎
    │   ├── TCP Ping (tokio TcpStream)
    │   ├── HTTP Ping (reqwest)
    │   ├── HTTPS Ping (reqwest)
    │   └── 下载测试 (reqwest, 固定大小文件)
    │
    ▼
    │   评分计算
    │
    ├── 更新 Master Pool 节点评分
    │
    ├── 写入历史记录 (JSONL)
    │
    ├── 更新缓存
    │
    ▼
    │   事件通知 ("snc://speed-test-completed")
    │
    ▼
[前端] Dashboard / 节点列表自动更新
```

### 5.3 自动切换数据流

```
健康管理检测到掉线 / 定时切换触发
    │
    ▼
[后端] core/smart_node/switch.rs
    │
    ├── 获取当前分组节点列表
    │
    ├── 根据策略排序
    │   ├── 最快优先: 按 latency ASC
    │   ├── 最稳定优先: 按 score DESC
    │   ├── 顺序: 按 preset order
    │   └── ...
    │
    ├── 选择最优节点
    │
    ▼
    │   切换执行
    │   ├── mihomo.select_node_for_group()
    │   ├── 更新 Master Pool 当前节点标记
    │   ├── 写入切换记录到历史
    │   └── 事件通知 ("snc://node-switched")
    │
    ▼
[前端] Dashboard 当前节点信息更新
```

---

## 6. 目录结构设计

### 6.1 后端目录结构（Rust）

在 `src-tauri/src/` 下新增以下目录（不修改任何现有目录）：

```
src-tauri/src/
├── cmd/
│   └── smart_node/                    ← [新增] SNC IPC 命令层
│       ├── mod.rs                      # 命令模块导出
│       ├── pool.rs                     # Master Pool 命令
│       ├── group.rs                    # 分组管理命令
│       ├── speed_test.rs               # 测速命令
│       ├── switch.rs                   # 自动切换命令
│       ├── health.rs                   # 健康检测命令
│       ├── history.rs                  # 历史数据命令
│       ├── dashboard.rs               # Dashboard 命令
│       ├── import_export.rs           # 导入导出命令
│       ├── media_detect.rs            # 流媒体检测命令
│       └── settings.rs               # SNC 设置命令
│
├── config/
│   └── smart_node/                    ← [新增] SNC 配置管理
│       ├── mod.rs                      # 模块导出
│       ├── snc_config.rs              # ISmartNodeConfig 结构体
│       ├── pool.rs                    # Master Pool 配置
│       ├── groups.rs                   # 分组配置
│       └── history.rs                  # 历史数据配置
│
├── feat/
│   └── smart_node/                    ← [新增] SNC 业务逻辑层
│       ├── mod.rs                      # 模块导出
│       ├── pool.rs                    # Master Pool CRUD
│       ├── group.rs                    # 分组管理逻辑
│       ├── import_export.rs           # 导入导出逻辑
│       ├── parser/                    # 格式解析器
│       │   ├── mod.rs
│       │   ├── base64.rs
│       │   ├── yaml.rs
│       │   ├── json.rs
│       │   ├── txt.rs
│       │   ├── csv.rs
│       │   └── excel.rs
│       └── detector/                  # 协议检测
│           ├── mod.rs
│           └── protocol.rs            # 复用 uri-parser 逻辑
│
├── core/
│   └── smart_node/                    ← [新增] SNC 核心服务层
│       ├── mod.rs                      # 模块导出
│       ├── manager.rs                  # SNC 管理器（生命周期）
│       ├── speed_test/               # 测速引擎
│       │   ├── mod.rs
│       │   ├── tcp_ping.rs
│       │   ├── http_ping.rs
│       │   ├── download.rs
│       │   ├── upload.rs
│       │   ├── dns.rs
│       │   ├── jitter.rs
│       │   └── scorer.rs              # 评分引擎
│       ├── switch/                    # 自动切换引擎
│       │   ├── mod.rs
│       │   ├── strategy.rs            # 策略定义
│       │   ├── executor.rs            # 切换执行器
│       │   └── scheduler.rs           # 切换调度器
│       ├── health/                    # 健康管理引擎
│       │   ├── mod.rs
│       │   ├── checker.rs             # 健康检测器
│       │   ├── failover.rs            # 故障转移
│       │   └── recovery.rs             # 恢复检测
│       ├── group_detector/           # 分组检测引擎
│       │   ├── mod.rs
│       │   ├── http.rs
│       │   ├── https.rs
│       │   ├── tcp.rs
│       │   └── api.rs
│       ├── history/                    # 历史数据引擎
│       │   ├── mod.rs
│       │   ├── collector.rs          # 数据采集
│       │   ├── aggregator.rs         # 数据聚合
│       │   └── cleaner.rs             # 数据清理
│       ├── scheduler.rs               # 统一定时调度器
│       └── notification.rs           # SNC 事件通知
│
└── module/
    └── smart_node.rs                   ← [新增] SNC 高级模块入口
```

### 6.2 前端目录结构（React/TypeScript）

在 `src/` 下新增以下目录（不修改任何现有目录结构）：

```
src/
├── pages/
│   └── smart-node/                    ← [新增] SNC 页面
│       ├── index.tsx                   # SNC 入口页面（Tab 布局）
│       ├── dashboard.tsx              # Dashboard 页面
│       ├── pool.tsx                    # Master Pool 节点管理页
│       ├── groups.tsx                  # 分组管理页
│       ├── speed-test.tsx             # 测速页面
│       ├── history.tsx                # 历史数据页面
│       └── settings.tsx               # SNC 设置页面
│
├── components/
│   └── smart-node/                    ← [新增] SNC 组件
│       ├── dashboard/                  # Dashboard 组件
│       │   ├── info-card.tsx
│       │   ├── traffic-stats.tsx
│       │   ├── media-detect-card.tsx
│       │   └── system-stats.tsx
│       ├── pool/                      # Master Pool 组件
│       │   ├── node-list.tsx
│       │   ├── node-item.tsx
│       │   ├── node-detail.tsx
│       │   ├── node-filter.tsx
│       │   └── import-export-dialog.tsx
│       ├── groups/                     # 分组组件
│       │   ├── group-list.tsx
│       │   ├── group-item.tsx
│       │   ├── group-editor.tsx
│       │   ├── detection-config.tsx
│       │   └── group-nodes.tsx
│       ├── speed-test/               # 测速组件
│       │   ├── speed-test-panel.tsx
│       │   ├── speed-test-result.tsx
│       │   └── score-display.tsx
│       ├── history/                    # 历史组件
│       │   ├── history-chart.tsx
│       │   ├── history-table.tsx
│       │   └── history-filter.tsx
│       └── settings/                   # 设置组件
│           ├── general-settings.tsx
│           ├── speed-test-settings.tsx
│           ├── switch-settings.tsx
│           └── history-settings.tsx
│
├── hooks/
│   └── use-smart-node/               ← [新增] SNC Hooks
│       ├── use-smart-node-enabled.ts   # SNC 开关状态
│       ├── use-pool.ts                 # Master Pool 数据
│       ├── use-groups.ts              # 分组数据
│       ├── use-speed-test.ts          # 测速状态
│       ├── use-health.ts              # 健康检测状态
│       ├── use-switch.ts              # 切换状态
│       └── use-history.ts             # 历史数据
│
├── services/
│   └── smart-node.ts                  ← [新增] SNC IPC 命令封装
│
├── providers/
│   └── smart-node/                    ← [新增] SNC Context
│       ├── smart-node-context.ts      # Context 定义
│       └── smart-node-provider.tsx    # Provider 组件
│
└── types/
    └── smart-node/                    ← [新增] SNC 类型定义
        ├── pool.ts
        ├── group.ts
        ├── speed-test.ts
        ├── health.ts
        ├── switch.ts
        └── history.ts
```

### 6.3 配置文件结构

```
{app_home}/
├── verge.yaml                         # 官方配置（不修改，仅新增 enable_snc 字段）
├── config.yaml                        # 官方配置（不修改）
├── profiles.yaml                      # 官方配置（不修改）
└── smart_node/                        ← [新增] SNC 独立配置目录
    ├── config.yaml                    # SNC 全局配置
    ├── pool.yaml                      # Master Pool 节点数据
    ├── groups.yaml                    # 分组配置
    ├── history/                       # 历史数据目录
    │   ├── 2026-07-02.jsonl
    │   ├── 2026-07-01.jsonl
    │   └── ...
    └── cache/                         # 测速缓存目录
        ├── speed_cache.json
        └── health_cache.json
```

> **设计决策**：SNC 配置使用独立子目录 `smart_node/`，而非混入官方配置目录。这样：
> 1. 关闭 SNC 时可直接删除整个目录
> 2. 不污染官方配置目录
> 3. 官方更新不会影响 SNC 配置

### 6.4 与官方代码的集成点

虽然不修改官方源码，但需要在以下位置进行**最小化集成**（新增代码，不修改现有代码）：

| 集成点 | 位置 | 集成方式 | 影响 |
|---|---|---|---|
| SNC 命令注册 | `lib.rs` 的 `generate_handler![]` | **需在宏中追加** SNC 命令模块 | 最小影响：追加一行 `mod smart_node;` 和在 handler 列表中追加 |
| SNC 路由注册 | `pages/_routers.tsx` | **需追加** SNC 路由定义 | 最小影响：追加路由配置 |
| SNC Provider 注册 | `main.tsx` | **需追加** SNC Provider 包裹 | 最小影响：追加 Provider 组件 |
| SNC 导航菜单 | `pages/_layout.tsx` | **需追加** SNC 导航项 | 最小影响：追加菜单项 |
| SNC 开关字段 | `config/verge.rs` | **需追加** `enable_snc: Option<bool>` 字段 | 最小影响：追加一个可选字段，默认 false，不影响官方解析 |

> **重要说明**：以上 5 个集成点是**不可避免的**，但都是**纯追加式修改**（只增不减，不改变现有逻辑）。后续同步官方更新时，这些位置的冲突极低（因为只涉及追加代码）。

---

## 7. 配置文件设计

### 7.1 SNC 全局配置 (`smart_node/config.yaml`)

```yaml
# Smart Node Center 全局配置
enabled: true                          # 功能开关

# 测速配置
speed_test:
  concurrent_limit: 20                 # 并发测速上限
  tcp_timeout_ms: 5000                 # TCP 延迟超时
  http_timeout_ms: 5000                 # HTTP 延迟超时
  download_timeout_ms: 10000           # 下载测速超时
  download_url: "http://speedtest.tele2.net/1MB.zip"  # 下载测试 URL
  download_size: 1048576              # 下载测试大小 (bytes)
  upload_url: ""                       # 上传测试 URL（可选）
  upload_size: 1048576                # 上传测试大小 (bytes)
  cache_ttl_seconds: 300              # 缓存有效期
  schedule_interval_minutes: 30        # 定时测速间隔

# 评分权重
scoring:
  latency_weight: 0.3
  speed_weight: 0.25
  stability_weight: 0.25
  online_rate_weight: 0.2
  latency_formula: "inverse"           # inverse / linear / sigmoid
  # 延迟评分参考值
  latency_excellent: 100               # ≤100ms 为优秀
  latency_good: 300                    # ≤300ms 为良好
  latency_poor: 1000                   # ≤1000ms 为一般

# 自动切换配置
auto_switch:
  enabled: true
  default_strategy: "fastest"          # fastest / most_stable / sequential / random / weighted
  default_interval_minutes: 30
  fail_count_threshold: 3             # 失败次数阈值触发切换
  enable_recovery_switch_back: false   # 恢复后是否切回
  recovery_check_interval_minutes: 5  # 恢复检测间隔
  master_pool_fallback: true          # 分组全失效时从 Master Pool 兜底

# 健康检测配置
health_check:
  enabled: true
  interval_minutes: 5                # 检测间隔
  timeout_ms: 5000                    # 检测超时
  check_method: "tcp"                 # tcp / http / https
  check_url: "https://www.google.com/generate_204"

# 历史数据配置
history:
  retention_days: 30                  # 数据保留天数 (7/30/90)
  auto_cleanup: true                  # 自动清理
  cleanup_time: "03:00"              # 清理时间

# Dashboard 配置
dashboard:
  refresh_interval_seconds: 10        # 刷新间隔
  show_media_detect: true           # 显示流媒体检测
  custom_detect_urls: []              # 自定义检测网址

# 分组默认配置
group_defaults:
  auto_sync: true                     # 默认开启自动同步
  detection_timeout_ms: 5000          # 检测超时
```

### 7.2 Master Pool 配置 (`smart_node/pool.yaml`)

```yaml
# Master Pool 节点池
version: 1
updated_at: "2026-07-02T00:00:00Z"
total_count: 150

nodes:
  - uid: "snc_001"
    name: "香港-01-SS"
    protocol: "ss"
    address: "hk01.example.com"
    port: 443
    uri: "ss://..."                   # 原始 URI
    source: "订阅A"                    # 来源标识
    source_url: "https://..."         # 来源 URL（可选）
    tags: ["香港", "游戏"]             # 标签
    status: "healthy"                 # healthy / unhealthy / unknown
    score: 85
    latency_ms: 120
    download_speed_mbps: 50
    last_health_check: "2026-07-02T00:05:00Z"
    last_speed_test: "2026-07-02T00:10:00Z"
    created_at: "2026-07-02T00:00:00Z"
    updated_at: "2026-07-02T00:10:00Z"
    fail_count: 0                     # 连续失败次数
```

### 7.3 分组配置 (`smart_node/groups.yaml`)

```yaml
# 分组配置
version: 1
updated_at: "2026-07-02T00:00:00Z"

groups:
  - id: "grp_001"
    name: "Netflix 可用"
    description: "可访问 Netflix 的节点"
    auto_switch:
      enabled: true
      strategy: "fastest"
      interval_minutes: 30
    detection:
      urls:
        - url: "https://www.netflix.com/title/80018499"
          method: "https"
          expected_status: 200
          timeout_ms: 5000
        - url: "https://www.netflix.com/"
          method: "https"
          expected_status: 200
          timeout_ms: 5000
    auto_sync: true
    sync_interval_minutes: 60
    node_uids:
      - "snc_001"
      - "snc_002"
      - "snc_003"
    fallback_node_uids:              # 备用节点
      - "snc_100"
    created_at: "2026-07-02T00:00:00Z"
    updated_at: "2026-07-02T00:00:00Z"
```

---

## 8. IPC 命令设计

### 8.1 SNC 命令列表

#### Master Pool 命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_get_pool` | `async() -> CmdResult<SharedDraft<ISmartNodePool>>` | 获取 Master Pool |
| `snc_get_node` | `async(uid: String) -> CmdResult<SmartNode>` | 获取单个节点 |
| `snc_add_nodes` | `async(nodes: Vec<SmartNodeInput>) -> CmdResult<ImportResult>` | 批量添加节点 |
| `snc_remove_node` | `async(uid: String) -> CmdResult` | 删除节点 |
| `snc_update_node` | `async(uid: String, node: SmartNodeUpdate) -> CmdResult` | 更新节点 |
| `snc_clear_pool` | `async() -> CmdResult` | 清空 Master Pool |
| `snc_get_pool_stats` | `async() -> CmdResult<PoolStats>` | 获取 Pool 统计 |

#### 导入导出命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_import_nodes` | `async(content: String, format: String, source_name: String) -> CmdResult<ImportResult>` | 导入节点 |
| `snc_import_from_file` | `async(file_path: String) -> CmdResult<ImportResult>` | 从文件导入 |
| `snc_export_nodes` | `async(node_uids: Vec<String>, format: String, output_path: String) -> CmdResult` | 导出节点 |

#### 分组命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_get_groups` | `async() -> CmdResult<SharedDraft<ISmartNodeGroups>>` | 获取所有分组 |
| `snc_create_group` | `async(group: GroupInput) -> CmdResult` | 创建分组 |
| `snc_update_group` | `async(id: String, group: GroupUpdate) -> CmdResult` | 更新分组 |
| `snc_delete_group` | `async(id: String) -> CmdResult` | 删除分组 |
| `snc_detect_group` | `async(group_id: String) -> CmdResult<GroupDetectResult>` | 触发分组检测 |

#### 测速命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_start_speed_test` | `async(node_uids: Vec<String>, test_types: Vec<String>) -> CmdResult` | 启动测速 |
| `snc_get_speed_results` | `async() -> CmdResult<HashMap<String, SpeedTestResult>>` | 获取测速结果 |
| `snc_stop_speed_test` | `async() -> CmdResult` | 停止测速 |

#### 自动切换命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_get_switch_status` | `async() -> CmdResult<SwitchStatus>` | 获取切换状态 |
| `snc_trigger_switch` | `async(group_id: String) -> CmdResult<SwitchResult>` | 手动触发切换 |
| `snc_set_switch_strategy` | `async(group_id: String, strategy: String) -> CmdResult` | 设置切换策略 |

#### 历史数据命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_get_history` | `async(node_uid: String, days: u32) -> CmdResult<Vec<HistoryRecord>>` | 获取历史数据 |
| `snc_get_stats` | `async(node_uid: String, days: u32) -> CmdResult<NodeStats>` | 获取统计信息 |
| `snc_cleanup_history` | `async() -> CmdResult` | 手动清理历史 |

#### Dashboard 命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_get_dashboard_data` | `async() -> CmdResult<DashboardData>` | 获取 Dashboard 全部数据 |
| `snc_get_ip_info` | `async() -> CmdResult<IpInfo>` | 获取 IP/国家/运营商信息 |
| `snc_check_media_unlock` | `async(node_uid: String) -> CmdResult<Vec<MediaUnlockResult>>` | 流媒体检测 |
| `snc_check_custom_url` | `async(url: String) -> CmdResult<bool>` | 自定义网址检测 |

#### 设置命令

| 命令 | 签名 | 描述 |
|---|---|---|
| `snc_get_config` | `async() -> CmdResult<SharedDraft<ISmartNodeConfig>>` | 获取 SNC 配置 |
| `snc_patch_config` | `async(payload: ISmartNodeConfig) -> CmdResult` | 修改 SNC 配置 |
| `snc_toggle_enabled` | `async(enabled: bool) -> CmdResult` | 切换 SNC 开关 |
| `snc_is_enabled` | `async() -> CmdResult<bool>` | 获取 SNC 开关状态 |

### 8.2 SNC 事件列表

| 事件名 | 触发场景 | 数据 |
|---|---|---|
| `snc://pool-updated` | Master Pool 变更 | Pool 变更信息 |
| `snc://speed-test-progress` | 测速进度更新 | 进度百分比 |
| `snc://speed-test-completed` | 测速完成 | 测速结果 |
| `snc://node-switched` | 节点切换完成 | 切换前后节点信息 |
| `snc://health-status-changed` | 节点健康状态变化 | 节点 UID + 新状态 |
| `snc://group-detect-completed` | 分组检测完成 | 检测结果 |
| `snc://config-changed` | SNC 配置变更 | 配置变更信息 |

---

## 9. 后台任务设计

### 9.1 任务调度架构

```
SNC Scheduler (core/smart_node/scheduler.rs)
│
├── 健康检测定时器 (Health Check Timer)
│   ├── 间隔：5 分钟（可配置）
│   ├── 扫描 Master Pool 所有节点
│   └── 更新节点状态
│
├── 测速定时器 (Speed Test Timer)
│   ├── 间隔：30 分钟（可配置）
│   ├── 测试所有标记为 healthy 的节点
│   └── 更新评分
│
├── 分组检测定时器 (Group Detect Timer)
│   ├── 间隔：每个分组独立配置
│   ├── 对分组内节点执行目标网址检测
│   └── 更新分组节点列表
│
├── 自动切换定时器 (Auto Switch Timer)
│   ├── 间隔：每个分组独立配置
│   ├── 根据策略选择最优节点
│   └── 执行切换
│
├── 恢复检测定时器 (Recovery Check Timer)
│   ├── 间隔：5 分钟（可配置）
│   ├── 检测掉线节点是否恢复
│   └── 恢复后根据配置决定是否切回
│
├── 历史清理定时器 (History Cleanup Timer)
│   ├── 每天凌晨 3:00
│   ├── 清理超过保留周期的数据
│   └── 生成统计摘要
│
└── 订阅同步定时器 (Subscription Sync Timer)
    ├── 复用官方 timer.rs 框架
    ├── 跟随官方订阅更新
    └── 更新 Master Pool 节点信息
```

### 9.2 调度器实现方案

复用官方 `core/timer.rs` 的 `tokio::sync::mpsc` + `DelayQueue` 模式：

```rust
// 伪代码
pub struct SncScheduler {
    health_tx: mpsc::Sender<ScheduledTask>,
    speed_tx: mpsc::Sender<ScheduledTask>,
    group_tx: mpsc::Sender<ScheduledTask>,
    switch_tx: mpsc::Sender<ScheduledTask>,
    recovery_tx: mpsc::Sender<ScheduledTask>,
    cleanup_tx: mpsc::Sender<ScheduledTask>,
}
```

- SNC 启用时，初始化所有定时器
- SNC 关闭时，停止所有定时器
- 各定时器独立运行，互不阻塞

---

## 10. 状态机设计

### 10.1 SNC 全局状态机

```
                  ┌──────────┐
                  │ Disabled │ ← 初始状态/关闭后
                  └────┬─────┘
                       │ snc_toggle_enabled(true)
                       ▼
                  ┌──────────┐
         ┌───────│ Starting │
         │       └────┬─────┘
         │            │ 初始化完成
         │            ▼
         │       ┌──────────┐
    初始化失败│      │ Running  │ ← 正常运行
         │       └────┬─────┘
         │            │ snc_toggle_enabled(false)
         │            ▼
         │       ┌──────────┐
         └──────▶│ Stopping │
                  └────┬─────┘
                       │ 停止完成
                       ▼
                  ┌──────────┐
                  │ Disabled │
                  └──────────┘
```

### 10.2 节点状态机

```
              ┌─────────┐
              │ Unknown │ ← 新导入节点
              └────┬────┘
                   │ 首次健康检测
                   ▼
              ┌─────────┐
     ┌─────── │ Healthy │
     │        └────┬────┘
     │             │ 连续失败 3 次
     │             ▼
     │        ┌──────────┐
     │        │ Unhealthy│
     │        └────┬─────┘
     │             │ 恢复检测成功
     │             ▼
     │        ┌─────────┐
     └────────│ Healthy │
              └─────────┘
```

### 10.3 分组检测状态机

```
              ┌────────┐
              │  Idle  │
              └───┬────┘
                  │ 触发检测
                  ▼
              ┌────────────┐
              │ Detecting  │
              └──┬─────┬───┘
                 │     │
          全部完成 │     │ 超时/错误
                 ▼     ▼
           ┌────────┐ ┌──────────┐
           │  Done  │ │ Failed   │
           └────────┘ └──────────┘
               │           │
               └─────┬─────┘
                     │ 返回 Idle
                     ▼
                 ┌────────┐
                 │  Idle  │
                 └────────┘
```

---

## 11. 日志设计

### 11.1 日志策略

复用官方 `flexi_logger` + `clash_verge_logger`：

| 日志级别 | 用途 | 示例 |
|---|---|---|
| ERROR | 严重错误（测速引擎崩溃、数据损坏） | `SNC speed test engine panic: ...` |
| WARN | 警告（节点掉线、切换失败、检测超时） | `Node snc_001 marked unhealthy after 3 failures` |
| INFO | 重要事件（节点切换、分组更新、导入完成） | `Auto-switched group 'Netflix' from snc_001 to snc_002` |
| DEBUG | 调试信息（测速详情、检测结果） | `Speed test: snc_001 TCP latency=120ms HTTP=150ms` |
| TRACE | 详细追踪（请求/响应详情） | `Health check request to snc_001: 200 OK in 85ms` |

### 11.2 日志输出

- 复用官方日志目录 `{app_home}/logs/`
- SNC 日志写入 `logs/snc_latest.log`
- 日志前缀 `[SNC]` 便于过滤
- 日志轮转策略与官方一致

---

## 12. 升级兼容方案

### 12.1 官方更新同步策略

```
官方仓库更新
    │
    ▼
git fetch upstream
    │
    ▼
git merge upstream/main
    │
    ├── 冲突分析
    │   ├── lib.rs (SNC 命令注册) → 手动追加 SNC 模块
    │   ├── pages/_routers.tsx (SNC 路由) → 手动追加路由
    │   ├── main.tsx (SNC Provider) → 手动追加 Provider
    │   ├── pages/_layout.tsx (SNC 导航) → 手动追加菜单
    │   └── config/verge.rs (SNC 字段) → 手动追加字段
    │
    ├── 无冲突（大概率）
    │   ├── src-tauri/src/cmd/smart_node/     → 无冲突（独立目录）
    │   ├── src-tauri/src/feat/smart_node/    → 无冲突（独立目录）
    │   ├── src-tauri/src/config/smart_node/   → 无冲突（独立目录）
    │   ├── src-tauri/src/core/smart_node/     → 无冲突（独立目录）
    │   ├── src/pages/smart-node/              → 无冲突（独立目录）
    │   ├── src/components/smart-node/         → 无冲突（独立目录）
    │   └── src/hooks/use-smart-node/           → 无冲突（独立目录）
    │
    ▼
    验证编译 → 验证功能 → 发布
```

### 12.2 冲突风险评估

| 集成点 | 冲突概率 | 冲突复杂度 | 解决方案 |
|---|---|---|---|
| `lib.rs` handler 注册 | 低 | 低 | 追加 `mod smart_node;` 和 handler 列表项 |
| `_routers.tsx` 路由 | 低 | 低 | 追加路由配置对象 |
| `main.tsx` Provider | 低 | 低 | 追加 Provider 包裹 |
| `_layout.tsx` 导航 | 低 | 低 | 追加 navItems 条目 |
| `verge.rs` 字段 | 极低 | 低 | 追加 `enable_snc: Option<bool>` |

> **结论**：所有集成点都是追加式修改，冲突概率极低且解决简单。

### 12.3 配置文件兼容性

| 场景 | 处理方式 |
|---|---|
| 从官方版本升级到 SNC 版本 | 自动创建 `smart_node/` 目录和默认配置 |
| 从 SNC 版本降级到官方版本 | 忽略 `smart_node/` 目录，不影响官方功能 |
| SNC 配置版本升级 | 配置文件包含 `version` 字段，支持自动迁移 |

---

## 13. 待确认设计决策

| 编号 | 决策项 | 方案A | 方案B | 推荐 |
|---|---|---|---|---|
| D1 | 历史数据存储 | 纯文件（JSONL） | SQLite 嵌入式数据库 | **方案A**（保持与官方一致） |
| D2 | 图表库 | recharts（React 图表库） | 纯 Canvas 自绘 | **方案A**（开发效率高） |
| D3 | Excel 处理 | Rust 端（rust_xlsxwriter + calamine） | 前端端（SheetJS） | **方案B**（不增加 Rust 依赖） |
| D4 | GeoIP/ASN 查询 | 在线 API（ip-api.com 等） | 离线数据库（maxminddb） | **待确认** |
| D5 | SNC 开关存储 | verge.yaml 新增字段 | SNC 独立 config.yaml | **方案A**（统一管理） |
| D6 | 节点 UID 生成 | nanoid（Rust 移植） | UUID v4 | **UUID v4**（Rust 生态标配） |
