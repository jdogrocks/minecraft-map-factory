mod tuning;

use crate::config::MetricsConfig;
use crate::locations::Location;
use serde::Serialize;
use std::time::Duration;
use tracing::info;

/// Collects and reports pipeline metrics.
pub struct MetricsCollector {
    successes: Vec<JobMetric>,
    failures: Vec<FailureMetric>,
    jobs_since_summary: usize,
}

#[derive(Debug, Serialize)]
struct JobMetric {
    location_name: String,
    location_tier: String,
    duration_secs: f64,
    output_size_bytes: u64,
}

#[derive(Debug, Serialize)]
struct FailureMetric {
    location_name: String,
    location_tier: String,
    duration_secs: f64,
    reason: String,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            successes: Vec::new(),
            failures: Vec::new(),
            jobs_since_summary: 0,
        }
    }

    pub fn record_success(&mut self, duration: Duration, size_bytes: u64, location: &Location) {
        self.successes.push(JobMetric {
            location_name: location.name.clone(),
            location_tier: location.tier.clone(),
            duration_secs: duration.as_secs_f64(),
            output_size_bytes: size_bytes,
        });
        self.jobs_since_summary += 1;
    }

    pub fn record_failure(&mut self, duration: Duration, location: &Location, reason: &str) {
        self.failures.push(FailureMetric {
            location_name: location.name.clone(),
            location_tier: location.tier.clone(),
            duration_secs: duration.as_secs_f64(),
            reason: reason.to_string(),
        });
        self.jobs_since_summary += 1;
    }

    pub fn total_jobs(&self) -> usize {
        self.successes.len() + self.failures.len()
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.total_jobs();
        if total == 0 {
            return 1.0;
        }
        self.successes.len() as f64 / total as f64
    }

    pub fn should_print_summary(&self, config: &MetricsConfig) -> bool {
        self.jobs_since_summary >= config.summary_interval
    }

    pub fn print_summary(&self) {
        let total = self.total_jobs();
        if total == 0 {
            info!("No jobs completed yet");
            return;
        }

        let success_rate = self.success_rate();
        let durations: Vec<f64> = self.successes.iter().map(|m| m.duration_secs).collect();
        let total_sizes: u64 = self.successes.iter().map(|m| m.output_size_bytes).sum();

        let (p50, p95, p99) = if durations.is_empty() {
            (0.0, 0.0, 0.0)
        } else {
            percentiles(&durations)
        };

        info!(
            total_jobs = total,
            successes = self.successes.len(),
            failures = self.failures.len(),
            success_rate = format!("{:.1}%", success_rate * 100.0),
            duration_p50_secs = format!("{:.1}", p50),
            duration_p95_secs = format!("{:.1}", p95),
            duration_p99_secs = format!("{:.1}", p99),
            total_output_mb = format!("{:.1}", total_sizes as f64 / 1024.0 / 1024.0),
            "Pipeline metrics summary"
        );

        // Log failure breakdown
        if !self.failures.is_empty() {
            let mut reasons: std::collections::HashMap<&str, usize> =
                std::collections::HashMap::new();
            for f in &self.failures {
                *reasons.entry(&f.reason).or_insert(0) += 1;
            }
            for (reason, count) in &reasons {
                info!(reason, count, "Failure breakdown");
            }
        }
    }
}

fn percentiles(values: &[f64]) -> (f64, f64, f64) {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = sorted.len();

    let p50 = sorted[len / 2];
    let p95 = sorted[std::cmp::min(len - 1, (len as f64 * 0.95) as usize)];
    let p99 = sorted[std::cmp::min(len - 1, (len as f64 * 0.99) as usize)];

    (p50, p95, p99)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc(name: &str, tier: &str) -> Location {
        Location {
            name: name.into(),
            state: "CA".into(),
            bbox: [0.0, 0.0, 1.0, 1.0],
            tier: tier.into(),
            tags: vec![],
        }
    }

    #[test]
    fn test_empty_metrics() {
        let m = MetricsCollector::new();
        assert_eq!(m.total_jobs(), 0);
        assert!((m.success_rate() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_success_tracking() {
        let mut m = MetricsCollector::new();
        m.record_success(Duration::from_secs(10), 1024, &loc("A", "small"));
        m.record_success(Duration::from_secs(20), 2048, &loc("B", "medium"));
        assert_eq!(m.total_jobs(), 2);
        assert!((m.success_rate() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_failure_tracking() {
        let mut m = MetricsCollector::new();
        m.record_success(Duration::from_secs(10), 1024, &loc("A", "small"));
        m.record_failure(Duration::from_secs(5), &loc("B", "large"), "timeout");
        assert_eq!(m.total_jobs(), 2);
        assert!((m.success_rate() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_percentiles() {
        let values: Vec<f64> = (1..=100).map(|i| i as f64).collect();
        let (p50, p95, p99) = percentiles(&values);
        // p50 of 1..=100 is index 50 = 51.0
        assert!((p50 - 51.0).abs() < 1.0);
        assert!((p95 - 96.0).abs() < 2.0);
        assert!((p99 - 100.0).abs() < 2.0);
    }
}
