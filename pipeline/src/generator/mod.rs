mod retry;

pub use retry::RetryStrategy;

use crate::config::PipelineConfig;
use crate::locations::{Location, LocationDatabase};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, error, info, warn};

/// Wraps map generator CLI invocation with retry logic.
pub struct Generator {
    arnis_binary: PathBuf,
    retry_strategy: RetryStrategy,
    output_base: PathBuf,
}

impl Generator {
    pub fn new(config: &PipelineConfig) -> Self {
        Self {
            arnis_binary: config.arnis_binary.clone(),
            retry_strategy: RetryStrategy::new(&config.retry),
            output_base: config.output_dir.clone(),
        }
    }

    /// Generate a map for the given location, with automatic retry on failure.
    pub async fn generate(
        &self,
        location: &Location,
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let mut attempt = 0u32;
        let mut current_bbox = location.bbox;

        loop {
            attempt += 1;
            let output_dir = self.job_output_dir(location, attempt);
            std::fs::create_dir_all(&output_dir)?;

            let bbox_str = format!(
                "{},{},{},{}",
                current_bbox[0], current_bbox[1], current_bbox[2], current_bbox[3]
            );

            info!(
                name = %location.name,
                attempt,
                bbox = %bbox_str,
                "Invoking map generator"
            );

            match self.invoke_generator(&bbox_str, &output_dir).await {
                Ok(()) => {
                    info!(name = %location.name, attempt, "Map generator completed successfully");
                    return Ok(output_dir);
                }
                Err(e) => {
                    let classification = self.retry_strategy.classify_error(&*e);
                    warn!(
                        name = %location.name,
                        attempt,
                        error = %e,
                        classification = ?classification,
                        "Map generator invocation failed"
                    );

                    match self.retry_strategy.should_retry(attempt, &classification) {
                        retry::RetryDecision::Retry { backoff } => {
                            info!(
                                name = %location.name,
                                backoff_secs = backoff.as_secs(),
                                "Retrying after backoff"
                            );
                            tokio::time::sleep(backoff).await;

                            // Shrink bbox if configured
                            if self.retry_strategy.should_shrink_bbox() {
                                current_bbox = LocationDatabase::shrink_bbox(
                                    &Location {
                                        bbox: current_bbox,
                                        ..location.clone()
                                    },
                                    self.retry_strategy.shrink_factor(),
                                );
                                debug!(
                                    name = %location.name,
                                    new_bbox = ?current_bbox,
                                    "Shrunk bbox for retry"
                                );
                            }
                        }
                        retry::RetryDecision::GiveUp => {
                            error!(
                                name = %location.name,
                                attempts = attempt,
                                "Exhausted retries"
                            );
                            return Err(e);
                        }
                    }
                }
            }
        }
    }

    async fn invoke_generator(
        &self,
        bbox: &str,
        output_dir: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new(&self.arnis_binary)
            .arg("--bbox")
            .arg(bbox)
            .arg("--output-dir")
            .arg(output_dir)
            .arg("--interior")
            .arg("--entities")
            .arg("--terrain")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            debug!(stdout = %stdout, "generator stdout");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            Err(format!(
                "Generator exited with status {}: stderr={}, stdout={}",
                output.status, stderr, stdout
            )
            .into())
        }
    }

    fn job_output_dir(&self, location: &Location, attempt: u32) -> PathBuf {
        let sanitized_name: String = location
            .name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        self.output_base
            .join("jobs")
            .join(format!("{}_attempt{}", sanitized_name, attempt))
    }
}
