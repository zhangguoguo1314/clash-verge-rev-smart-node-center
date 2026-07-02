use crate::config::smart_node::{NodeStatus, SmartNode};
use serde::Deserialize;

/// 切换策略枚举
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SwitchStrategy {
    /// 选择延迟最低的节点
    Fastest,
    /// 选择最稳定的节点 (评分最高)
    MostStable,
    /// 按顺序选择下一个
    Sequential,
    /// 随机选择
    Random,
    /// 加权选择
    Weighted,
    /// AI 推荐
    AIRecommend,
}

impl SwitchStrategy {
    /// 从字符串解析策略
    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fastest" => Self::Fastest,
            "most_stable" | "moststable" | "stable" => Self::MostStable,
            "sequential" => Self::Sequential,
            "random" => Self::Random,
            "weighted" => Self::Weighted,
            "ai_recommend" | "ai" | "ai-recommend" => Self::AIRecommend,
            _ => Self::Fastest,
        }
    }
}

/// 根据策略选择最佳节点
///
/// 返回选中的节点 UID。
pub fn select_node(
    strategy: &SwitchStrategy,
    candidates: &[SmartNode],
) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }

    // 优先选择健康的，如果没有则回退到全部
    let healthy: Vec<&SmartNode> = candidates
        .iter()
        .filter(|n| n.status == NodeStatus::Healthy || n.status == NodeStatus::Unknown)
        .collect();

    let pool = if healthy.is_empty() {
        candidates.iter().collect::<Vec<&SmartNode>>()
    } else {
        healthy
    };

    match strategy {
        SwitchStrategy::Fastest => select_fastest(&pool),
        SwitchStrategy::MostStable => select_most_stable(&pool),
        SwitchStrategy::Sequential => select_sequential(&pool),
        SwitchStrategy::Random => select_random(&pool),
        SwitchStrategy::Weighted => select_weighted(&pool),
        SwitchStrategy::AIRecommend => select_ai_recommend(&pool),
    }
}

/// 最快: 按 latency_ms ASC
fn select_fastest(candidates: &[&SmartNode]) -> Option<String> {
    let mut sorted = candidates.to_vec();
    sorted.sort_by_key(|n| n.latency_ms.unwrap_or(u64::MAX));
    sorted.first().map(|n| n.uid.clone())
}

/// 最稳定: 按 score DESC
fn select_most_stable(candidates: &[&SmartNode]) -> Option<String> {
    let mut sorted = candidates.to_vec();
    sorted.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    sorted.first().map(|n| n.uid.clone())
}

/// 顺序: 返回第一个
fn select_sequential(candidates: &[&SmartNode]) -> Option<String> {
    candidates.first().map(|n| n.uid.clone())
}

/// 随机: 使用时间戳哈希简单伪随机
fn select_random(candidates: &[&SmartNode]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }
    // 使用纳秒时间戳作为简单随机种子
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as usize;
    let idx = seed % candidates.len();
    candidates.get(idx).map(|n| n.uid.clone())
}

/// 加权: 按 score 作为权重
fn select_weighted(candidates: &[&SmartNode]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }

    let total_weight: f64 = candidates.iter().map(|n| n.score.max(1.0)).sum();
    if total_weight <= 0.0 {
        return select_random(candidates);
    }

    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as f64;
    let rand_val = (seed % (total_weight as u128) as f64).abs();

    let mut accumulated = 0.0;
    for node in candidates {
        accumulated += node.score.max(1.0);
        if accumulated >= rand_val {
            return Some(node.uid.clone());
        }
    }

    candidates.last().map(|n| n.uid.clone())
}

/// AI 推荐: 综合评分最高的 (当前实现等同 MostStable)
fn select_ai_recommend(candidates: &[&SmartNode]) -> Option<String> {
    select_most_stable(candidates)
}
