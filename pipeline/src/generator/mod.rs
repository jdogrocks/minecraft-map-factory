mod retry;

pub use retry::RetryStrategy;

use crate::config::{GeneratorConfig, PipelineConfig};
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
    flags: GeneratorConfig,
}

impl Generator {
    pub fn new(config: &PipelineConfig) -> Self {
        Self {
            arnis_binary: config.arnis_binary.clone(),
            retry_strategy: RetryStrategy::new(&config.retry),
            output_base: config.output_dir.clone(),
            flags: config.generator.clone(),
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

            match self
                .invoke_generator(
                    &bbox_str,
                    &output_dir,
                    location.spawn_lat,
                    location.spawn_lng,
                )
                .await
            {
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
        spawn_lat: Option<f64>,
        spawn_lng: Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut command = Command::new(&self.arnis_binary);
        // Each job runs in its own output directory so concurrent subprocesses
        // never share a working directory or write relative-path temporaries
        // to the same location (identical-worlds isolation, MIN-137).
        command
            .current_dir(output_dir)
            .arg("--bbox")
            .arg(bbox)
            .arg("--output-dir")
            .arg(output_dir);

        // Pass every generator flag explicitly so the generator's own defaults
        // can't silently change pipeline output (the MIN-40 regression where
        // bare `--terrain` left land_cover and ground-fill at whatever the
        // generator's defaults happened to be).
        //
        // `--terrain` is a SetTrue flag (no `=value` form), so emit it bare
        // when enabled and omit otherwise. The remaining flags accept the
        // explicit `--flag=true|false` form per `args.rs`.
        if self.flags.terrain {
            command.arg("--terrain");
        }
        let bool_flags: [(&str, bool); 4] = [
            ("--land-cover", self.flags.land_cover),
            ("--interior", self.flags.interior),
            ("--entities", self.flags.entities),
            ("--roof", self.flags.roof),
        ];
        for (flag, value) in bool_flags {
            command.arg(format!("{flag}={value}"));
        }
        command.arg("--entity-theme").arg(&self.flags.entity_theme);
        command
            .arg("--ground-level")
            .arg(self.flags.ground_level.to_string());

        if let (Some(lat), Some(lng)) = (spawn_lat, spawn_lng) {
            command
                .arg("--spawn-lat")
                .arg(lat.to_string())
                .arg("--spawn-lng")
                .arg(lng.to_string());
        }

        let output = command
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
