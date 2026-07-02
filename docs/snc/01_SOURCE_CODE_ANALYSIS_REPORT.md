# Clash Verge Rev 源码分析报告

> 版本：v2.5.2 | 分析日期：2026-07-02 | 分析依据：官方 GitHub 仓库最新源码（浅克隆 depth=1）

---

## 1. 项目概览

| 项目信息 | 详情 |
|---|---|
| 项目名称 | Clash Verge Rev |
| 版本 | 2.5.2 |
| 许可证 | GPL-3.0-only |
| 技术定位 | 基于 Tauri 2 的 Clash Meta (Mihomo) GUI 客户端 |
| 前端框架 | React 19.2.7 + TypeScript 6.x |
| 后端框架 | Rust 1.95 (edition 2024) + Tauri 2.11.3 |
| UI 库 | MUI v9 + Emotion |
| 构建工具 | Vite 8（前端）+ Cargo（Rust） |
| 包管理 | pnpm 11.3.0（前端）+ Cargo（Rust） |

---

## 2. 项目目录结构

```
clash-verge-rev/
├── src/                          # 前端源码
│   ├── pages/                    # 9个页面（home, proxies, profiles, connections, rules, logs, settings, test, unlock）
│   ├── components/               # UI 组件（base, home, proxy, profile, connection, setting, rule, log, test, layout, shared）
│   ├── hooks/                    # 27个自定义 Hooks
│   ├── services/                 # 服务层（cmds.ts, api.ts, delay.ts, i18n.ts 等）
│   ├── providers/                # React Context（AppData, ChainProxy, Window）
│   ├── locales/                  # 13种语言国际化
│   ├── types/                    # TypeScript 类型定义
│   ├── utils/                    # 工具函数（含 uri-parser/ 协议解析器）
│   ├── assets/                   # 静态资源（字体、图标、SCSS）
│   ├── polyfills/                # 浏览器兼容
│   ├── main.tsx                  # 应用入口
│   └── index.html               # HTML 入口
├── src-tauri/                    # Rust 后端源码
│   ├── src/
│   │   ├── cmd/                  # Tauri IPC 命令层（14个模块，60+ 命令）
│   │   ├── config/               # 配置管理（config.yaml, verge.yaml, profiles.yaml, runtime）
│   │   ├── core/                 # 核心管理（内核生命周期、服务、托盘、日志、定时器、验证）
│   │   ├── enhance/              # 配置增强引擎（merge, script, chain, tun, builtin）
│   │   ├── feat/                 # 功能实现层（backup, clash, config, icon, profile, proxy, window）
│   │   ├── module/               # 高级功能模块（auto_backup, lightweight）
│   │   ├── process/              # 异步处理工具
│   │   ├── utils/                # 基础工具（DNS, 网络, 窗口, 速度, 单例等）
│   │   ├── main.rs               # Tokio 运行时初始化
│   │   ├── lib.rs                # Tauri 应用主逻辑、插件注册
│   │   └── constants.rs          # 常量定义
│   ├── capabilities/             # Tauri capabilities 配置
│   ├── tauri.conf.json           # Tauri 核心配置
│   └── Cargo.toml                # Rust 依赖
├── crates/                       # Rust workspace 子 crate（7个）
│   ├── clash-verge-draft/        # 核心数据类型/Draft 模式
│   ├── clash-verge-logging/      # 日志库
│   ├── clash-verge-signal/       # 信号处理
│   ├── clash-verge-i18n/         # Rust 端国际化
│   ├── clash-verge-limiter/      # 限流
│   └── tauri-plugin-clash-verge-sysinfo/  # 系统信息 Tauri 插件
├── package.json                  # 前端配置
├── Cargo.toml                    # Rust workspace 配置
├── vite.config.mts              # Vite 构建配置
└── Makefile.toml                 # 任务自动化
```

---

## 3. 技术栈详解

### 3.1 前端技术栈

| 层面 | 技术 | 版本 | 用途 |
|---|---|---|---|
| UI 框架 | React | 19.2.7 | 组件化开发 |
| 语言 | TypeScript | 6.0.3 | 类型安全 |
| UI 库 | MUI (Material UI) | v9 | 组件库 |
| CSS | Emotion + SCSS | 11.14 | CSS-in-JS + 全局样式 |
| 状态/数据 | SWR | 2.4.2 | 数据请求与缓存 |
| 路由 | React Router | v8 | SPA 路由 |
| 代码编辑 | Monaco Editor | 0.55.1 | YAML 编辑器 |
| 拖拽 | @dnd-kit | 6.3.1 | 列表排序 |
| 虚拟列表 | @tanstack/react-virtual | 3.13.25 | 高性能渲染 |
| 表单 | react-hook-form | 7.76.1 | 表单管理 |
| Hooks 工具 | ahooks | 3.9.7 | useLockFn 等 |
| 国际化 | i18next + react-i18next | 26.2/17.0 | 多语言 |
| 构建 | Vite | 8.0.14 | 前端构建 |
| 代码规范 | ESLint + Biome | 10.4/2.4 | Lint + Format |
| Git Hooks | Husky + lint-staged | 9.1/17.0 | 提交检查 |

### 3.2 后端技术栈（Rust）

| 库 | 版本 | 用途 |
|---|---|---|
| tauri | 2.11.3 | 应用框架 |
| tokio | 1.52.3 | 异步运行时（多线程） |
| serde | - | 序列化框架 |
| warp | 0.4.3 | HTTP 服务（本地 API） |
| reqwest | 0.13.4 | HTTP 客户端 |
| reqwest_dav | 0.3.3 | WebDAV 客户端 |
| boa_engine | 0.21.1 | JavaScript 引擎（配置增强） |
| serde_yaml_ng | - | YAML 解析 |
| serde_json | - | JSON 解析 |
| aes-gcm | - | AES-256-GCM 加密 |
| sysproxy | - | 系统代理设置 |
| network-interface | - | 网络接口管理 |
| zip | 8.6.0 | 备份压缩/解压 |
| regex | - | 正则匹配 |
| chrono | - | 时间处理 |
| parking_lot | - | 高性能锁 |
| anyhow | - | 错误处理 |

---

## 4. 配置管理架构

### 4.1 Draft 模式

项目使用 `clash-verge-draft` crate 实现的 **Draft 模式** 管理配置，支持事务性修改：

```
edit_draft() → 修改草案 → apply() 提交 / discard() 丢弃
```

### 4.2 三个持久化配置文件

| 文件 | 结构体 | 存储路径 | 用途 |
|---|---|---|---|
| `config.yaml` | `IClashTemp(Mapping)` | `{app_home}/config.yaml` | Clash 内核基础配置 |
| `verge.yaml` | `IVerge`（80+ 字段） | `{app_home}/verge.yaml` | 应用设置 |
| `profiles.yaml` | `IProfiles` | `{app_home}/profiles.yaml` | 订阅列表 |

- 序列化格式：YAML
- 敏感字段（WebDAV 凭据）使用 AES-256-GCM 加密
- **不使用数据库**，完全基于文件存储

### 4.3 配置更新流程

```
Config::generate() → enhance::enhance() → CoreConfigValidator::validate()
→ Config::generate_file() → mihomo reload_config / restart_core
```

- 使用 `config_update_in_progress` 原子标志防止并发
- 支持 debounce（300ms）避免频繁更新
- 验证失败自动回滚到默认配置

---

## 5. 订阅管理机制

### 5.1 订阅类型

| 类型 | 前缀 | 描述 |
|---|---|---|
| remote | `R` | 远程订阅（URL 下载） |
| local | `L` | 本地配置文件 |
| merge | `m` | 合并配置（YAML overlay） |
| script | `s` | JavaScript 增强（Boa 引擎） |
| rules | `r` | 规则扩展 |
| proxies | `p` | 代理扩展 |
| groups | `g` | 代理组扩展 |

### 5.2 订阅更新流程

三重降级策略：
1. 直接下载（无代理）
2. 通过 Clash 代理下载（`self_proxy = true`）
3. 通过系统代理下载（`with_proxy = true`）

### 5.3 定时更新

`core/timer.rs` 使用 `tokio::sync::mpsc` + `DelayQueue` 实现，支持：
- 订阅级别的自动更新间隔
- 启动时检查过期订阅
- 更新完成后自动重新调度

---

## 6. Tauri 命令层（IPC 接口）

所有命令统一返回 `CmdResult<T>` = `Result<T, String>`。

### 6.1 命令分类统计

| 类别 | 数量 | 关键命令 |
|---|---|---|
| 系统信息 | 7 | get_sys_proxy, get_network_interfaces_info |
| 应用操作 | 12 | open_app_dir, restart_app, get_auto_launch_status |
| Clash 核心 | 16 | get_clash_info, patch_clash_config, test_delay, start_core |
| Verge 配置 | 2 | get_verge_config, patch_verge_config |
| 订阅管理 | 12 | get_profiles, import_profile, update_profile |
| 运行时 | 6 | get_runtime_config, get_runtime_yaml |
| 服务管理 | 5 | install_service, uninstall_service |
| 备份 | 12 | create_local_backup, restore_webdav_backup |
| 流媒体检测 | 2 | get_unlock_items, check_media_unlock |

---

## 7. 前端通信架构

### 7.1 三种通信方式

| 方式 | 技术 | 场景 |
|---|---|---|
| IPC 命令调用 | `@tauri-apps/api/core invoke()` | 前端→后端结构化请求 |
| 事件通知 | `@tauri-apps/api/event listen()` | 后端→前端推送（配置变更、订阅更新等） |
| WebSocket | `tauri-plugin-mihomo-api` | 实时数据流（流量、连接、内存、日志） |

### 7.2 SWR 数据管理

前端使用 SWR + 自定义 `useQuery` hook 管理服务端状态：

| Query Key | 数据源 |
|---|---|
| `['getProfiles']` | 订阅列表 |
| `['getVergeConfig']` | 应用配置 |
| `['getRuntimeConfig']` | 运行时配置 |
| `['getProxies']` | 代理节点数据 |
| `['getProxyProviders']` | 代理 Provider |
| `['getRules']` | 规则列表 |
| `['getSystemProxy']` | 系统代理状态 |
| `['getRunningMode']` | 运行模式 |

---

## 8. 核心模块关系

```
┌─────────────────────────────────────────────────────┐
│                    lib.rs (主入口)                     │
│  插件注册 → Setup → 事件循环                          │
├─────────────────────────────────────────────────────┤
│  cmd/ (IPC 入口)  ←→  feat/ (业务桥接)               │
│                          ↓                           │
│  config/ (配置管理 Draft模式)                         │
│    ├── config.yaml    (Clash 基础配置)               │
│    ├── verge.yaml     (应用设置)                      │
│    ├── profiles.yaml  (订阅列表)                     │
│    └── runtime        (运行时配置)                     │
│                          ↓                           │
│  core/ (核心管理)                                     │
│    ├── manager/       (CoreManager 内核生命周期)      │
│    ├── service.rs     (系统服务管理)                   │
│    ├── sysopt.rs      (系统代理设置)                   │
│    ├── timer.rs       (定时器)                         │
│    ├── tray/          (系统托盘)                      │
│    └── notification.rs (事件通知)                     │
│                          ↓                           │
│  enhance/ (配置增强引擎)                               │
│    ├── merge.rs       (YAML 合并)                     │
│    ├── script.rs      (JS 脚本执行)                   │
│    ├── chain.rs       (增强链)                        │
│    ├── tun.rs         (TUN 配置)                     │
│    └── builtin/       (内置脚本)                      │
├─────────────────────────────────────────────────────┤
│  tauri-plugin-mihomo (Mihomo 内核 IPC)               │
│  LocalSocket 协议, 连接池 3-32                        │
└─────────────────────────────────────────────────────┘
```

---

## 9. 数据持久化方式

**无数据库**，完全基于文件存储：

| 文件/目录 | 格式 | 用途 |
|---|---|---|
| `verge.yaml` | YAML | 应用设置 |
| `profiles.yaml` | YAML | 订阅列表 |
| `config.yaml` | YAML | Clash 基础配置 |
| `clash-verge.yaml` | YAML | 运行时最终配置 |
| `profiles/*.yaml` | YAML | 各订阅配置文件 |
| `profiles/*.js` | JS | 脚本增强 |
| `.encryption_key` | 原始字节 | 加密密钥 |
| `window_state.json` | JSON | 窗口状态 |
| `icons/` | PNG/ICO | 图标缓存 |
| `{BACKUP_DIR}/` | ZIP | 备份文件 |
| `logs/` | 文本 | 应用日志 |

---

## 10. 前端路由结构

| 路由 | 页面 | 加载方式 |
|---|---|---|
| `/` | 首页（仪表盘） | 直接 import |
| `/proxies` | 代理管理 | 懒加载 |
| `/profile` | 订阅管理 | 懒加载 |
| `/connections` | 连接监控 | 懒加载 |
| `/rules` | 规则查看 | 懒加载 |
| `/logs` | 日志 | 懒加载（覆盖层） |
| `/settings` | 设置 | 懒加载 |
| `/test` | 流媒体检测 | 懒加载 |
| `/unlock` | 解锁页面 | 懒加载 |

---

## 11. 现有流媒体解锁检测

`cmd/media_unlock_checker/` 已支持以下服务检测：

| 服务 | 文件 | 状态 |
|---|---|---|
| Netflix | netflix.rs | ✅ 已实现 |
| Disney+ | disney_plus.rs | ✅ 已实现 |
| YouTube | youtube.rs | ✅ 已实现 |
| TikTok | tiktok.rs | ✅ 已实现 |
| ChatGPT | chatgpt.rs | ✅ 已实现 |
| Claude | claude.rs | ✅ 已实现 |
| Gemini | gemini.rs | ✅ 已实现 |
| Spotify | spotify.rs | ✅ 已实现 |
| Bilibili | bilibili.rs | ✅ 已实现 |
| Bahamut | bahamut.rs | ✅ 已实现 |
| Prime Video | prime_video.rs | ✅ 已实现 |

---

## 12. 可复用的现有能力

| 能力 | 位置 | 可复用程度 |
|---|---|---|
| 订阅管理（CRUD） | cmd/profile.rs + feat/profile.rs | 高 - 可直接复用 |
| 配置 Draft 模式 | config/ + clash-verge-draft crate | 高 - 新配置可采用相同模式 |
| 定时器系统 | core/timer.rs | 中 - 需扩展支持更多调度策略 |
| 事件通知系统 | core/notification.rs | 高 - 可直接复用 |
| 流媒体检测 | cmd/media_unlock_checker/ | 高 - 可直接复用 |
| URI 协议解析 | src/utils/uri-parser/ | 高 - 支持 SS/VMess/VLESS/Trojan/TUIC/Hysteria/WireGuard |
| SWR 数据层 | services/query-client.ts | 高 - 新数据可直接接入 |
| 系统信息 | tauri-plugin-clash-verge-sysinfo | 高 - CPU/内存等可复用 |
| 虚拟列表 | components/base/virtual-list.tsx | 高 - 大量节点展示可复用 |
| Monaco Editor | components/base/monaco-editor.tsx | 高 - 配置编辑可复用 |

---

## 13. 关键设计约束

1. **无数据库**：所有数据存储基于 YAML/JSON 文件
2. **Draft 模式**：配置修改使用事务性 Draft 模式
3. **Mihomo 不修改**：内核通过 IPC 通信，不直接修改内核代码
4. **YAML 序列化**：配置统一使用 serde_yaml_ng
5. **文件存储路径**：通过 `utils/dirs.rs` 统一管理
6. **错误处理**：Tauri 命令层统一 `CmdResult<T>` = `Result<T, String>`
7. **Clippy 严格规则**：Rust 代码遵守 50+ 条 Clippy lint 规则
8. **前端 Hook 模式**：数据获取通过 SWR useQuery，状态通过 React Context

---

## 14. 对 Smart Node Center 的影响分析

| 方面 | 影响 | 说明 |
|---|---|---|
| 前端路由 | 需新增路由 | 需在 `_routers.tsx` 添加 `/smart-node` 等路由 |
| 状态管理 | 需扩展 | 新增 SNC 相关的 SWR query keys 和 Context |
| 配置存储 | 需新增文件 | Smart Node Center 独立配置文件 |
| 后端命令 | 需新增 cmd 模块 | 新增 `cmd/smart_node.rs` 等 |
| 定时器 | 需扩展 | 复用 timer.rs 框架，新增 SNC 定时任务 |
| 通知系统 | 可直接复用 | 使用现有 NotificationSystem |
| URI 解析 | 可直接复用 | utils/uri-parser/ 已支持全部协议 |
| 流媒体检测 | 可直接复用 | media_unlock_checker 已实现主要服务 |
| 系统信息 | 可直接复用 | tauri-plugin-clash-verge-sysinfo |
