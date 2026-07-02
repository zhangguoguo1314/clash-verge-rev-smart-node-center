use crate::config::smart_node::ScoringConfig;

/// 计算节点综合评分
///
/// 使用加权平均算法，各项指标归一化后加权求和。
/// - latency: 反比例函数评分 (越低越好)
/// - speed: 归一化评分 (越高越好)
/// - stability: 直接使用 (0-100)
/// - online_rate: 直接使用 (0-100)
pub fn calculate_score(
    config: &ScoringConfig,
    latency: Option<u64>,
    speed: Option<f64>,
    stability: Option<f64>,
    online_rate: Option<f64>,
) -> f64 {
    let mut total_score = 0.0;
    let mut total_weight = 0.0;

    // 延迟评分 (反比例函数)
    if let Some(lat) = latency {
        let lat_score = calculate_latency_score(config.latency_excellent, lat);
        total_score += lat_score * config.latency_weight;
        total_weight += config.latency_weight;
    }

    // 速度评分 (归一化到 0-100，假设 100Mbps 为满分)
    if let Some(spd) = speed {
        let speed_score = (spd / 100.0).min(1.0) * 100.0;
        total_score += speed_score * config.speed_weight;
        total_weight += config.speed_weight;
    }

    // 稳定性评分 (直接使用 0-100)
    if let Some(stab) = stability {
        let clamped = stab.clamp(0.0, 100.0);
        total_score += clamped * config.stability_weight;
        total_weight += config.stability_weight;
    }

    // 在线率评分 (直接使用 0-100)
    if let Some(rate) = online_rate {
        let clamped = rate.clamp(0.0, 100.0);
        total_score += clamped * config.online_rate_weight;
        total_weight += config.online_rate_weight;
    }

    if total_weight > 0.0 {
        total_score / total_weight
    } else {
        0.0
    }
}

/// 延迟评分函数
///
/// 使用反比例函数: score = 100 * excellent / (excellent + latency - 1)
/// 当 latency <= excellent 时返回 100
fn calculate_latency_score(excellent: u64, latency: u64) -> f64 {
    if latency <= excellent {
        return 100.0;
    }
    let excellent = excellent as f64;
    let latency = latency as f64;
    100.0 * excellent / (excellent + latency - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latency_score_excellent() {
        let config = ScoringConfig::default();
        assert_eq!(calculate_latency_score(config.latency_excellent, 50), 100.0);
        assert_eq!(calculate_latency_score(config.latency_excellent, 100), 100.0);
    }

    #[test]
    fn test_latency_score_poor() {
        let score = calculate_latency_score(100, 1000);
        assert!(score < 100.0);
        assert!(score > 0.0);
    }

    #[test]
    fn test_calculate_score_all_metrics() {
        let config = ScoringConfig::default();
        let score = calculate_score(
            &config,
            Some(100),
            Some(50.0),
            Some(80.0),
            Some(90.0),
        );
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_calculate_score_partial() {
        let config = ScoringConfig::default();
        let score = calculate_score(&config, Some(200), None, None, None);
        assert!(score > 0.0);
    }

    #[test]
    fn test_calculate_score_none() {
        let config = ScoringConfig::default();
        let score = calculate_score(&config, None, None, None, None);
        assert_eq!(score, 0.0);
    }
}
