use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level pipeline configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Path to the locations database file.
    pub locations_file: PathBuf,

    /// Output directory for generated maps.
    pub output_dir: PathBuf,

    /// Path to the map generator binary.
    #[serde(default = "default_arnis_path")]
    pub arnis_binary: PathBuf,

    /// Scheduler configuration.
    #[serde(default)]
    pub scheduler: SchedulerConfig,

    /// Retry configuration.
    #[serde(default)]
    pub retry: RetryConfig,

    /// Quality validation configuration.
    #[serde(default)]
    pub validation: ValidationConfig,

    /// Metrics configuration.
    #[serde(default)]
    pub metrics: MetricsConfig,

    /// Self-tuning configuration.
    #[serde(default)]
    pub tuning: TuningConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    /// Maximum number of concurrent generation jobs.
    #[serde(default = "default_max_concurrency")]
    pub max_concurrency: usize,

    /// Maximum RSS memory (in MB) before throttling.
    #[serde(default = "default_max_memory_mb")]
    pub max_memory_mb: u64,

    /// Maximum CPU usage percentage before throttling (0.0 - 1.0).
    #[serde(default = "default_max_cpu_percent")]
    pub max_cpu_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries per job.
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Initial backoff duration in seconds.
    #[serde(default = "default_initial_backoff_secs")]
    pub initial_backoff_secs: u64,

    /// Maximum backoff duration in seconds.
    #[serde(default = "default_max_backoff_secs")]
    pub max_backoff_secs: u64,

    /// Whether to reduce bbox on retry.
    #[serde(default = "default_true")]
    pub shrink_bbox_on_retry: bool,

    /// Factor to shrink bbox by on each retry (0.0 - 1.0).
    #[serde(default = "default_bbox_shrink_factor")]
    pub bbox_shrink_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Minimum number of region files for a valid map.
    #[serde(default = "default_min_region_files")]
    pub min_region_files: usize,

    /// Minimum total map size in bytes.
    #[serde(default = "default_min_map_size_bytes")]
    pub min_map_size_bytes: u64,

    /// Whether to validate Anvil region file structure.
    #[serde(default = "default_true")]
    pub validate_structure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Number of jobs between metrics summary reports.
    #[serde(default = "default_summary_interval")]
    pub summary_interval: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningConfig {
    /// Enable self-tuning of concurrency and parameters.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Minimum success rate before reducing concurrency (0.0 - 1.0).
    #[serde(default = "default_min_success_rate")]
    pub min_success_rate: f64,

    /// Memory usage threshold to trigger bbox reduction (0.0 - 1.0).
    #[serde(default = "default_memory_threshold")]
    pub memory_threshold: f64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_concurrency: default_max_concurrency(),
            max_memory_mb: default_max_memory_mb(),
            max_cpu_percent: default_max_cpu_percent(),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: default_max_retries(),
            initial_backoff_secs: default_initial_backoff_secs(),
            max_backoff_secs: default_max_backoff_secs(),
            shrink_bbox_on_retry: true,
            bbox_shrink_factor: default_bbox_shrink_factor(),
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            min_region_files: default_min_region_files(),
            min_map_size_bytes: default_min_map_size_bytes(),
            validate_structure: true,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            summary_interval: default_summary_interval(),
        }
    }
}

impl Default for TuningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_success_rate: default_min_success_rate(),
            memory_threshold: default_memory_threshold(),
        }
    }
}

fn default_arnis_path() -> PathBuf {
    PathBuf::from("minecraft-map-factory")
}

fn default_max_concurrency() -> usize {
    let cpus = num_cpus();
    std::cmp::max(1, cpus / 2)
}

fn default_max_memory_mb() -> u64 {
    4096
}

fn default_max_cpu_percent() -> f64 {
    0.8
}

fn default_max_retries() -> u32 {
    3
}

fn default_initial_backoff_secs() -> u64 {
    1
}

fn default_max_backoff_secs() -> u64 {
    300
}

fn default_bbox_shrink_factor() -> f64 {
    0.8
}

fn default_min_region_files() -> usize {
    1
}

fn default_min_map_size_bytes() -> u64 {
    1024
}

fn default_summary_interval() -> usize {
    10
}

fn default_min_success_rate() -> f64 {
    0.5
}

fn default_memory_threshold() -> f64 {
    0.8
}

fn default_true() -> bool {
    true
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

impl PipelineConfig {
    pub fn from_file(
        path: &std::path::Path,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
