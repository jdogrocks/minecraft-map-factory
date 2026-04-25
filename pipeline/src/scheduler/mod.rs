mod queue;
mod resource;

pub use resource::ResourceMonitor;

use crate::config::{PipelineConfig, TuningConfig};
use crate::generator::Generator;
use crate::locations::{LocationDatabase, LocationStatus};
use crate::metrics::MetricsCollector;
use crate::publisher::Publisher;
use crate::validation::Validator;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tracing::{error, info, warn};

/// Orchestrates concurrent map generation jobs.
pub struct Scheduler {
    config: PipelineConfig,
    db: Arc<Mutex<LocationDatabase>>,
    metrics: Arc<Mutex<MetricsCollector>>,
    resource_monitor: ResourceMonitor,
}

impl Scheduler {
    pub fn new(config: PipelineConfig, db: LocationDatabase, metrics: MetricsCollector) -> Self {
        let resource_monitor = ResourceMonitor::new(
            config.scheduler.max_memory_mb,
            config.scheduler.max_cpu_percent,
        );
        Self {
            config,
            db: Arc::new(Mutex::new(db)),
            metrics: Arc::new(Mutex::new(metrics)),
            resource_monitor,
        }
    }

    /// Run the pipeline to completion.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let concurrency = self.effective_concurrency().await;
        let semaphore = Arc::new(Semaphore::new(concurrency));

        info!(max_concurrency = concurrency, "Pipeline started");

        let mut handles = Vec::new();

        loop {
            let job = {
                let mut db = self.db.lock().await;
                match db.next_pending() {
                    Some((idx, _loc)) => {
                        db.set_status(idx, LocationStatus::InProgress);
                        Some(idx)
                    }
                    None => None,
                }
            };

            let Some(location_idx) = job else {
                break;
            };

            // Wait for resource availability
            if self.resource_monitor.is_over_limit() {
                info!("Resource limits reached, waiting for capacity");
                self.resource_monitor.wait_for_capacity().await;
            }

            let permit = semaphore.clone().acquire_owned().await?;
            let config = self.config.clone();
            let db = self.db.clone();
            let metrics = self.metrics.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit;
                Self::run_job(config, db, metrics, location_idx).await;
            });

            handles.push(handle);
        }

        // Wait for all in-flight jobs to complete
        for handle in handles {
            if let Err(e) = handle.await {
                error!(error = %e, "Job task panicked");
            }
        }

        // Print final summary
        let metrics = self.metrics.lock().await;
        metrics.print_summary();

        let db = self.db.lock().await;
        let summary = db.status_summary();
        info!(
            completed = summary.completed,
            failed = summary.failed,
            skipped = summary.skipped,
            total = db.total(),
            "Pipeline finished"
        );

        Ok(())
    }

    async fn run_job(
        config: PipelineConfig,
        db: Arc<Mutex<LocationDatabase>>,
        metrics: Arc<Mutex<MetricsCollector>>,
        location_idx: usize,
    ) {
        let location = {
            let db = db.lock().await;
            db.get_location(location_idx).cloned()
        };

        let Some(location) = location else {
            error!(location_idx, "Location not found");
            return;
        };

        info!(name = %location.name, state = %location.state, tier = %location.tier, "Starting generation");

        let generator = Generator::new(&config);
        let start = std::time::Instant::now();

        match generator.generate(&location).await {
            Ok(output_path) => {
                let duration = start.elapsed();

                // Validate the generated map
                let validator = Validator::new(&config.validation);
                match validator.validate(&output_path) {
                    Ok(report) if report.is_valid => {
                        info!(
                            name = %location.name,
                            duration_secs = duration.as_secs_f64(),
                            region_files = report.region_file_count,
                            total_bytes = report.total_size_bytes,
                            "Generation succeeded, validation passed"
                        );

                        // Publish
                        let publisher = Publisher::new(&config.output_dir);
                        let publish_result = publisher.publish(&output_path, &location);
                        let mut db = db.lock().await;
                        match publish_result {
                            Ok(dest) => {
                                info!(name = %location.name, dest = %dest.display(), "Published");
                                db.set_status(location_idx, LocationStatus::Completed);
                            }
                            Err(e) => {
                                let msg = format!("Publish failed: {e}");
                                error!(name = %location.name, error = %msg, "Publish failed");
                                db.set_status(
                                    location_idx,
                                    LocationStatus::Failed {
                                        attempts: 1,
                                        last_error: msg,
                                    },
                                );
                            }
                        }
                        drop(db);

                        let mut m = metrics.lock().await;
                        m.record_success(duration, report.total_size_bytes, &location);
                    }
                    Ok(report) => {
                        warn!(
                            name = %location.name,
                            reasons = ?report.failure_reasons,
                            "Validation failed"
                        );
                        let mut db = db.lock().await;
                        db.set_status(
                            location_idx,
                            LocationStatus::Failed {
                                attempts: 1,
                                last_error: format!(
                                    "Validation: {}",
                                    report.failure_reasons.join(", ")
                                ),
                            },
                        );

                        let mut m = metrics.lock().await;
                        m.record_failure(duration, &location, "validation_failed");
                    }
                    Err(e) => {
                        error!(name = %location.name, error = %e, "Validation error");
                        let mut db = db.lock().await;
                        db.set_status(
                            location_idx,
                            LocationStatus::Failed {
                                attempts: 1,
                                last_error: format!("Validation error: {e}"),
                            },
                        );

                        let mut m = metrics.lock().await;
                        m.record_failure(duration, &location, "validation_error");
                    }
                }
            }
            Err(e) => {
                let duration = start.elapsed();
                error!(name = %location.name, error = %e, "Generation failed");
                let mut db = db.lock().await;
                db.set_status(
                    location_idx,
                    LocationStatus::Failed {
                        attempts: 1,
                        last_error: e.to_string(),
                    },
                );

                let mut m = metrics.lock().await;
                m.record_failure(duration, &location, "generation_failed");
            }
        }

        // Check if we should print periodic metrics
        let m = metrics.lock().await;
        if m.should_print_summary(&config.metrics) {
            m.print_summary();
        }
    }

    async fn effective_concurrency(&self) -> usize {
        if !self.config.tuning.enabled {
            return self.config.scheduler.max_concurrency;
        }
        self.tune_concurrency(&self.config.tuning).await
    }

    async fn tune_concurrency(&self, tuning: &TuningConfig) -> usize {
        let metrics = self.metrics.lock().await;
        let success_rate = metrics.success_rate();

        let base = self.config.scheduler.max_concurrency;
        if success_rate < tuning.min_success_rate && metrics.total_jobs() > 5 {
            let reduced = std::cmp::max(1, base / 2);
            warn!(
                base_concurrency = base,
                reduced_concurrency = reduced,
                success_rate,
                "Reducing concurrency due to low success rate"
            );
            reduced
        } else {
            base
        }
    }
}
