use crate::config::TuningConfig;
use crate::metrics::MetricsCollector;
use crate::scheduler::ResourceMonitor;
use tracing::{info, warn};

/// Self-tuning engine that adjusts pipeline parameters based on metrics.
pub struct SelfTuner {
    config: TuningConfig,
}

impl SelfTuner {
    pub fn new(config: &TuningConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Recommend a concurrency level based on current metrics and resource usage.
    pub fn recommend_concurrency(
        &self,
        base_concurrency: usize,
        metrics: &MetricsCollector,
        resource_monitor: &ResourceMonitor,
    ) -> usize {
        if !self.config.enabled {
            return base_concurrency;
        }

        let mut recommended = base_concurrency;

        // Reduce concurrency if success rate is too low
        if metrics.total_jobs() > 5 && metrics.success_rate() < self.config.min_success_rate {
            recommended = std::cmp::max(1, recommended / 2);
            warn!(
                base = base_concurrency,
                recommended,
                success_rate = metrics.success_rate(),
                "Reducing concurrency due to low success rate"
            );
        }

        // Reduce concurrency if memory pressure is high
        if resource_monitor.is_over_limit() {
            recommended = std::cmp::max(1, recommended / 2);
            info!(recommended, "Reducing concurrency due to resource pressure");
        }

        recommended
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::locations::Location;
    use std::time::Duration;

    #[test]
    fn test_tuner_disabled() {
        let config = TuningConfig {
            enabled: false,
            ..Default::default()
        };
        let tuner = SelfTuner::new(&config);
        let metrics = MetricsCollector::new();
        let rm = ResourceMonitor::new(999999, 1.0);

        assert_eq!(tuner.recommend_concurrency(4, &metrics, &rm), 4);
    }

    #[test]
    fn test_tuner_reduces_on_failures() {
        let config = TuningConfig::default();
        let tuner = SelfTuner::new(&config);
        let mut metrics = MetricsCollector::new();
        let rm = ResourceMonitor::new(999999, 1.0);

        let loc = Location {
            name: "Test".into(),
            state: "CA".into(),
            bbox: [0.0, 0.0, 1.0, 1.0],
            tier: "small".into(),
            tags: vec![],
        };

        // Record mostly failures
        for _ in 0..8 {
            metrics.record_failure(Duration::from_secs(5), &loc, "timeout");
        }
        for _ in 0..2 {
            metrics.record_success(Duration::from_secs(10), 1024, &loc);
        }

        // Success rate is 20%, below threshold — should reduce
        let recommended = tuner.recommend_concurrency(4, &metrics, &rm);
        assert!(recommended < 4);
    }
}
