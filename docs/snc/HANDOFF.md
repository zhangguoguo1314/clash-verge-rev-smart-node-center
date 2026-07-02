# Smart Node Center - 项目交接文档

> 生成时间：2026-07-02 | 作者：AI 架构师 | 状态：核心代码已完成，待编译验证

---

## 1. 项目概述

### 1.1 项目定义

在 [Clash Verge Rev v2.5.2](https://github.com/clash-verge-rev/clash-verge-rev) 基础上，以**插件式架构**新增 **Smart Node Center（智能节点中心）** 功能模块。

### 1.2 核心约束（不可违反）

| 约束 | 说明 |
|---|---|
| 不修改 Mihomo 内核 | 只通过 IPC API 与内核通信 |
| 不破坏官方功能 | 所有新功能可独立启停 |
| 配置文件兼容 | SNC 使用独立配置目录 |
| 可同步官方更新 | SNC 代码与官方物理隔离 |
| 保持官方技术栈 | Tauri 2 + Rust + React 19 + MUI 9 + SWR |

### 1.3 新增功能清单

- **9 种协议支持**：SS/VMess/VLESS/Trojan/TUIC/Hysteria2/WireGuard/HTTP/SOCKS
- **多格式导入导出**：Base64/YAML/JSON/TXT/CSV
- **Master Pool 节点池**：全局去重、自动分类、引用式分组
- **测速系统**：TCP/HTTP/HTTPS 延迟 + 下载速度 + 综合评分
- **健康管理**：自动检测、3次失败切换、备用节点、Master Pool 兜底
- **自动切换**：最快/最稳定/顺序/随机/权重策略
- **分组管理**：目标网址检测（HTTP/HTTPS/TCP/API）、自动同步
- **历史数据**：JSONL 存储、自动清理、统计聚合
- **Dashboard**：IP信息、流量、系统信息、节点状态
- **全局开关**：关闭后与官方版本完全一致

---

## 2. 已完成工作

### 2.1 时间线

| 阶段 | 内容 | 状态 |
|---|---|---|
| 2026-07-02 阶段一 | 源码分析、PRD、SAD、风险分析、模块划分 | ✅ 完成 |
| 2026-07-02 阶段二 | 数据库设计、API设计 | ✅ 完成 |
| 2026-07-02 阶段三 | 测试方案、开发计划 | ✅ 完成 |
| 2026-07-02 阶段四 | Rust 后端 51 个文件 | ✅ 完成 |
| 2026-07-02 阶段五 | 前端 52 个文件 | ✅ 完成 |
| 2026-07-02 集成 | 5 处官方代码追加式修改 | ✅ 完成 |
| — | 编译验证 (cargo check) | ⏳ 待完成 |
| — | 前端构建 (pnpm web:build) | ⏳ 待完成 |
| — | 功能测试 | 🔜 待完成 |

### 2.2 交付文件清单

#### 设计文档（10 份，位于 `/workspace/`）

| 文件 | 说明 |
|---|---|
| `00_INDEX_AND_PROJECT_STATUS.md` | 项目索引、进度、下一步 |
| `01_SOURCE_CODE_ANALYSIS_REPORT.md` | Clash Verge Rev v2.5.2 完整源码分析 |
| `02_PRD_PRODUCT_REQUIREMENTS_DOCUMENT.md` | 产品需求文档（功能需求、优先级、验收标准） |
| `03_SAD_SOFTWARE_ARCHITECTURE_DESIGN.md` | 软件架构设计（整体架构、技术架构、数据流、配置设计、IPC命令、状态机） |
| `04_RISK_ANALYSIS_REPORT.md` | 风险分析（12项风险 + 缓解方案） |
| `05_MODULE_DECOMPOSITION_REPORT.md` | 模块划分（20个模块 + 依赖矩阵 + 风险评估） |
| `06_DATABASE_DESIGN.md` | 数据库设计（文件存储方案、JSONL格式） |
| `07_API_DESIGN.md` | API设计（23个IPC命令 + 6个事件通知） |
| `08_TEST_PLAN.md` | 测试方案（单元测试 + 集成测试 + 测试用例） |
| `CHANGELOG.md` | 更新日志 |
| `HANDOFF.md` | 本文档（交接文档） |

#### 代码文件

**Rust 后端（51 个新文件）**

```
src-tauri/src/
├── cmd/smart_node/              # IPC 命令层 (9 个文件)
│   ├── mod.rs                   # 模块导出 + re-export
│   ├── pool.rs                  # Master Pool 命令 (4个)
│   ├── group.rs                 # 分组命令 (5个)
│   ├── speed_test.rs            # 测速命令 (2个)
│   ├── switch.rs                # 切换命令 (1个)
│   ├── health.rs               # 健康检测命令 (2个)
│   ├── history.rs               # 历史数据命令 (2个)
│   ├── dashboard.rs             # Dashboard 命令 (1个)
│   └── settings.rs              # 设置命令 (4个)
│
├── config/smart_node/           # 配置模型 (4 个文件)
│   ├── mod.rs
│   ├── snc_config.rs            # ISmartNodeConfig (全局配置)
│   ├── pool.rs                  # ISmartNodePool, SmartNode, NodeStatus
│   └── groups.rs                # ISmartNodeGroups, SmartGroup
│
├── core/smart_node/             # 核心服务层 (15 个文件)
│   ├── mod.rs
│   ├── manager.rs               # SncManager (全局单例, 生命周期)
│   ├── scheduler.rs             # SncScheduler (4个定时任务循环)
│   ├── notification.rs         # SNC 事件通知
│   ├── speed_test/
│   │   ├── mod.rs
│   │   ├── tcp_ping.rs          # TCP 延迟测试
│   │   ├── http_ping.rs         # HTTP/HTTPS 延迟测试
│   │   ├── download.rs          # 下载速度测试
│   │   └── scorer.rs            # 评分引擎 (加权平均)
│   ├── health/
│   │   ├── mod.rs
│   │   ├── checker.rs           # 健康检测 (TCP/HTTP)
│   │   ├── failover.rs          # 故障转移 (备用→分组→MasterPool)
│   │   └── recovery.rs          # 恢复检测
│   ├── switch/
│   │   ├── mod.rs
│   │   ├── strategy.rs          # 6种切换策略
│   │   └── executor.rs          # 切换执行 (调Mihomo API)
│   └── history/
│       ├── mod.rs
│       ├── collector.rs         # JSONL 数据采集
│       └── cleaner.rs           # 按天数清理
│
└── feat/smart_node/             # 业务逻辑层 (12 个文件)
    ├── mod.rs
    ├── pool.rs                  # Master Pool CRUD + 去重
    ├── group.rs                 # 分组 CRUD + 检测
    ├── import_export.rs         # 多格式导入导出
    └── parser/
        ├── mod.rs               # ParsedNode 结构
        ├── base64.rs            # Base64 解析
        ├── yaml.rs              # YAML Clash配置解析
        ├── json.rs              # JSON 解析
        ├── txt.rs               # TXT URI逐行解析
        └── csv.rs               # CSV 解析
```

**前端（52 个新文件）**

```
src/
├── types/smart-node/            # TypeScript 类型 (8 个文件)
│   ├── index.ts
│   ├── config.ts               # ISmartNodeConfig, SpeedTestConfig, ScoringConfig...
│   ├── pool.ts                 # SmartNode, PoolStats, ImportResult, SmartNodeInput
│   ├── group.ts                # SmartGroup, DetectionUrl, GroupDetectResult
│   ├── speed-test.ts           # SpeedTestResult
│   ├── switch.ts               # SwitchResult
│   ├── history.ts              # HistoryRecord, NodeStats
│   └── dashboard.ts            # DashboardData, IpInfo
│
├── services/smart-node.ts       # IPC 命令封装 (28个函数)
│
├── hooks/use-smart-node/        # 自定义 Hooks (9 个文件)
│   ├── index.ts
│   ├── use-smart-node-enabled.ts
│   ├── use-pool.ts
│   ├── use-groups.ts
│   ├── use-speed-test.ts
│   ├── use-health.ts
│   ├── use-dashboard.ts
│   ├── use-history.ts
│   └── use-config.ts
│
├── providers/smart-node/       # React Context (3 个文件)
│   ├── index.ts
│   ├── smart-node-context.ts
│   └── smart-node-provider.tsx
│
├── pages/smart-node/           # 页面 (7 个文件)
│   ├── index.tsx               # 入口 (MUI Tabs)
│   ├── dashboard.tsx            # Dashboard
│   ├── pool.tsx                 # 节点池管理
│   ├── groups.tsx               # 分组管理
│   ├── speed-test.tsx           # 测速
│   ├── history.tsx              # 历史数据
│   └── settings.tsx             # SNC 设置
│
└── components/smart-node/       # UI 组件 (16 个文件)
    ├── index.ts
    ├── dashboard/
    │   ├── index.ts
    │   ├── info-card.tsx
    │   ├── traffic-card.tsx
    │   └── system-card.tsx
    ├── pool/
    │   ├── index.ts
    │   ├── node-list.tsx        # VirtualList
    │   ├── node-item.tsx
    │   └── import-dialog.tsx
    ├── groups/
    │   ├── index.ts
    │   ├── group-list.tsx
    │   └── group-editor.tsx
    └── settings/
        ├── index.ts
        ├── general-settings.tsx
        └── speed-settings.tsx
```

#### 官方代码修改（5 处，全部为追加式，不改变现有逻辑）

| 文件 | 修改内容 |
|---|---|
| `src-tauri/src/lib.rs` | `generate_handler![]` 宏中追加 23 个 `cmd::snc_*` 命令 |
| `src-tauri/src/cmd/mod.rs` | 追加 `pub mod smart_node;` 和 `pub use smart_node::*;` |
| `src-tauri/src/config/verge.rs` | IVerge 结构体末尾追加 `enable_snc: Option<bool>` |
| `src/pages/_routers.tsx` | navItems 数组末尾追加 Smart Node Center 路由 |
| `src/main.tsx` | 追加 `SncProvider` 包裹在 WindowProvider 和 AppDataProvider 之间 |

---

## 3. 技术架构要点

### 3.1 后端关键设计

| 要点 | 说明 |
|---|---|
| 配置管理 | 复用 `clash_verge_draft::Draft` 模式，SNC 配置独立目录 `smart_node/` |
| 全局单例 | `SncManager` 使用 `std::sync::LazyLock` |
| 定时任务 | `SncScheduler` 使用 `tokio::spawn` + `tokio::select!` 监听停止信号 |
| 事件通知 | 通过 `core::handle::Handle` 发送 `snc://` 前缀事件 |
| UUID 生成 | 复用项目已有的 UID 生成机制 |
| 错误处理 | 所有命令返回 `CmdResult<T>` = `Result<T, String>` |
| 字符串类型 | 自定义结构体字段使用 `smartstring::alias::String` |
| 序列化 | `serde_yaml_ng` (YAML), `serde_json` (JSON/IPC), CSV 使用自定义解析 |

### 3.2 前端关键设计

| 要点 | 说明 |
|---|---|
| 状态管理 | SWR + 自定义 `useQuery` hook（复用 `services/query-client.ts`） |
| IPC 调用 | `@tauri-apps/api/core` 的 `invoke()` |
| 异步锁 | `ahooks/lib/useLockFn` 防重入 |
| 虚拟列表 | 复用 `components/base/virtual-list.tsx` |
| UI 组件 | 复用 `BaseDialog`, `BaseFieldset`, `BaseSearchBox`, `EnhancedCard` |
| 国际化 | i18next 命名空间 `smartNode` |
| 代码风格 | 单引号、2空格缩进、80行宽（Biome） |

### 3.3 IPC 命令完整列表

共 23 个命令，全部在 `lib.rs` 的 `generate_handler![]` 中注册：

```
cmd::snc_get_pool            → 获取 Master Pool
cmd::snc_add_nodes          → 批量添加节点
cmd::snc_remove_node        → 删除节点
cmd::snc_get_pool_stats     → 获取 Pool 统计
cmd::snc_import_nodes       → 导入节点（多格式）
cmd::snc_export_nodes       → 导出节点
cmd::snc_get_groups         → 获取分组列表
cmd::snc_create_group       → 创建分组
cmd::snc_update_group       → 更新分组
cmd::snc_delete_group       → 删除分组
cmd::snc_detect_group       → 触发分组检测
cmd::snc_test_node_speed    → 测试单节点速度
cmd::snc_test_all_nodes     → 测试所有节点
cmd::snc_switch_group_node  → 切换分组节点
cmd::snc_check_node_health  → 检测节点健康
cmd::snc_check_all_health   → 检测所有节点健康
cmd::snc_get_today_history  → 获取今日历史
cmd::snc_cleanup_history    → 清理历史数据
cmd::snc_get_dashboard_data → 获取 Dashboard 数据
cmd::snc_is_enabled         → SNC 是否启用
cmd::snc_toggle_enabled     → 切换 SNC 开关
cmd::snc_get_config         → 获取 SNC 配置
cmd::snc_patch_config       → 修改 SNC 配置
```

---

## 4. 已知问题和待办

### 4.1 编译待验证

**Rust 后端可能的问题**（因为未在真实环境中编译，可能存在）：

| 问题 | 可能原因 | 修复方向 |
|---|---|---|
| 类型不匹配 | `smartstring::alias::String` vs `std::string::String` | 检查所有字符串传递边界 |
| 缺少 import | 子代理可能遗漏某些 use 语句 | cargo check 输出会明确提示 |
| Draft API 用法 | Draft 模式的 `edit_draft`/`apply` 可能需要调整 | 参考 `feat/profile.rs` 的用法 |
| UID 生成 | 可能需要使用项目实际的 UID 生成函数 | 检查 `utils::help` 中是否有 `get_uid` |
| Mihomo API 调用 | `tauri_plugin_mihomo` 的实际 API 签名可能不同 | 检查插件文档 |
| 文件路径 | `utils::dirs` 中的路径函数需要确认 | 检查实际可用的路径函数 |

**前端可能的问题**：

| 问题 | 修复方向 |
|---|---|
| useQuery 参数格式 | 确认 `services/query-client.ts` 的实际 useQuery 签名 |
| 组件 prop 类型 | 确认 BaseDialog/BaseFieldset 等组件的 props |
| import 路径 | 确认 `@/` 别名配置 |

### 4.2 功能待完善

| 功能 | 优先级 | 状态 |
|---|---|---|
| Excel 导入导出 | P1 | 🔜 前端 SheetJS 集成 |
| 下载速度测速 UI | P1 | 🔜 |
| 分组自动同步 UI | P1 | 🔜 |
| Dashboard 图表 | P1 | 🔜 需要 recharts |
| 流媒体检测集成 | P1 | 🔜 复用官方 media_unlock_checker |
| AI 推荐策略 | P2 | 🔜 |
| Jitter/Loss 测速 | P2 | 🔜 |
| 智能补组 | P2 | 🔜 |
| 国际化翻译 | P2 | 🔜 13种语言 |
| 自定义网址检测 | P1 | 🔜 |

---

## 5. 快速上手指南

### 5.1 环境准备

```bash
# 克隆仓库
git clone <repo-url>
cd clash-verge-rev-snc

# 安装前端依赖
pnpm install

# 预构建（处理 mihomo sidecar）
pnpm run prebuild
```

### 5.2 编译验证

```bash
# Rust 编译检查（关键步骤！）
cd src-tauri
cargo check 2>&1 | head -100

# 根据错误输出修复，常见问题：
# 1. use 语句缺失 → 添加 import
# 2. 类型不匹配 → 调整类型转换
# 3. 未实现的 trait → 检查 serde 属性
```

### 5.3 前端构建

```bash
# 前端类型检查
pnpm run lint

# 前端构建
pnpm run web:build
```

### 5.4 运行项目

```bash
# 开发模式
pnpm dev
```

### 5.5 关键文件快速定位

| 要修改什么 | 文件位置 |
|---|---|
| SNC 功能开关 | `config/verge.rs` → `enable_snc` |
| IPC 命令注册 | `lib.rs` → `generate_handler![]` |
| 前端路由 | `pages/_routers.tsx` → `navItems` |
| SNC Provider | `main.tsx` → `<SncProvider>` |
| SNC 配置模型 | `config/smart_node/snc_config.rs` |
| Master Pool 模型 | `config/smart_node/pool.rs` |
| 分组模型 | `config/smart_node/groups.rs` |
| 测速引擎 | `core/smart_node/speed_test/` |
| 自动切换 | `core/smart_node/switch/` |
| 健康管理 | `core/smart_node/health/` |
| 导入导出 | `feat/smart_node/import_export.rs` |
| 前端 Dashboard | `pages/smart-node/dashboard.tsx` |
| 前端节点池 | `pages/smart-node/pool.tsx` |

---

## 6. 同步官方更新方法

```bash
# 1. 添加官方仓库为 upstream
git remote add upstream https://github.com/clash-verge-rev/clash-verge-rev.git

# 2. 拉取官方更新
git fetch upstream

# 3. 合并
git merge upstream/main

# 4. 冲突只可能在以下 5 个文件（都是追加式，解决简单）：
#    - src-tauri/src/lib.rs         → 追加 SNC 命令注册
#    - src-tauri/src/cmd/mod.rs     → 追加 smart_node 模块
#    - src-tauri/src/config/verge.rs → 追加 enable_snc 字段
#    - src/pages/_routers.tsx        → 追加 SNC 路由
#    - src/main.tsx                  → 追加 SncProvider
```

---

## 7. 设计决策记录

| 决策 | 选择 | 理由 |
|---|---|---|
| 数据存储 | 纯文件 (YAML + JSONL) | 与官方一致，零新依赖 |
| 历史记录格式 | JSONL (每行一条 JSON) | 简单高效，支持流式追加 |
| Excel 处理 | 前端 SheetJS | 不增加 Rust 依赖 |
| GeoIP/ASN | 在线 API | 不增加离线数据库 |
| SNC 开关位置 | verge.yaml 追加字段 | 统一管理 |
| 节点 UID | 项目已有 UID 机制 | 复用，不引入 uuid crate |
| 定时器 | tokio spawn + select! | 复用官方 timer.rs 思路 |
| 图表库 | recharts (建议) | React 生态主流 |

---

## 8. 联系与文档参考

- 官方仓库：https://github.com/clash-verge-rev/clash-verge-rev
- Clash Verge Rev 文档：`docs/` 目录下的多语言 README
- Tauri 2 文档：https://v2.tauri.app/
- MUI v9 文档：https://mui.com/
- SWR 文档：https://swr.vercel.app/
