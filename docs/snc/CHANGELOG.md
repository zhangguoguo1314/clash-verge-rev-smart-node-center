# Smart Node Center - CHANGELOG

## [SNC 1.0.0] - 2026-07-02

### 新增
- Smart Node Center 插件式架构（23 个 IPC 命令 + 6 个事件通知）
- Master Pool 节点池管理（CRUD、去重、自动分类）
- 多格式导入导出（Base64、YAML、JSON、TXT、CSV）
- 9 种协议支持（SS、VMess、VLESS、Trojan、TUIC、Hysteria2、WireGuard、HTTP、SOCKS）
- 测速系统（TCP、HTTP、HTTPS 延迟 + 下载速度 + 综合评分）
- 健康管理（自动检测、故障转移、恢复检测、Master Pool 兜底）
- 自动切换（最快优先、最稳定优先、顺序、随机、权重策略）
- 分组管理（目标网址检测、多检测规则、自动同步）
- 历史数据（JSONL 存储、自动清理、统计聚合）
- Dashboard（IP 信息、实时流量、系统信息、节点状态）
- 全局功能开关（启用/关闭 SNC）

### 文件统计
- Rust 后端：51 个新文件（cmd/config/core/feat 层）
- 前端：52 个新文件（pages/components/hooks/services/providers/types）
- 官方修改：5 处最小化追加（lib.rs、cmd/mod.rs、verge.rs、_routers.tsx、main.tsx）
