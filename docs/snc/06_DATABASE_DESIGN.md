# Smart Node Center - 数据库设计文档

> 版本：1.0 | 日期：2026-07-02

---

## 1. 存储方案

采用**纯文件存储**，与 Clash Verge Rev 官方保持一致，不引入数据库。

## 2. 文件结构

```
{app_home}/smart_node/
├── config.yaml          # SNC 全局配置
├── pool.yaml            # Master Pool 节点数据
├── groups.yaml          # 分组配置
├── history/             # 历史数据目录（按天分文件）
│   ├── 2026-07-02.jsonl
│   └── ...
└── cache/               # 测速缓存目录
```

## 3. 配置兼容性

- SNC 配置独立目录 `smart_node/`，不修改官方配置
- 关闭 SNC 时可安全删除整个目录
- 配置文件包含 `version` 字段支持自动迁移

## 4. JSONL 历史记录格式

每行一条 JSON 记录：
```json
{"node_uid":"snc_001","timestamp":"2026-07-02T00:10:00Z","type":"speed_test","tcp_latency":120,"http_latency":150,"download_speed_mbps":50.0,"score":85.5}
```

## 5. 数据清理策略

- 7/30/90 天保留周期，每天凌晨 3:00 自动清理
- 每小时聚合一次数据，避免原始记录过多
