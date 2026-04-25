use sysinfo::System;
use tracing::debug;

/// Monitors system resource usage for throttling decisions.
pub struct ResourceMonitor {
    max_memory_mb: u64,
    max_cpu_percent: f64,
}

impl ResourceMonitor {
    pub fn new(max_memory_mb: u64, max_cpu_percent: f64) -> Self {
        Self {
            max_memory_mb,
            max_cpu_percent,
        }
    }

    /// Check if current resource usage exceeds configured limits.
    pub fn is_over_limit(&self) -> bool {
        let mut sys = System::new();
        sys.refresh_memory();
        sys.refresh_cpu_all();

        let used_memory_mb = sys.used_memory() / 1024 / 1024;
        let cpu_usage = sys.global_cpu_usage() as f64 / 100.0;

        let over = used_memory_mb > self.max_memory_mb || cpu_usage > self.max_cpu_percent;

        if over {
            debug!(
                used_memory_mb,
                max_memory_mb = self.max_memory_mb,
                cpu_usage,
                max_cpu_percent = self.max_cpu_percent,
                "Resource limit exceeded"
            );
        }

        over
    }

    /// Wait until resources drop below the threshold.
    pub async fn wait_for_capacity(&self) {
        while self.is_over_limit() {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}
