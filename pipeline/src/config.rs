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

    /// Generator CLI flags.
    #[serde(default)]
    pub generator: GeneratorConfig,
}

/// Flags forwarded to the map generator binary on every invocation.
///
/// Keep this in sync with the generator's CLI surface in `src/args.rs`. The
/// pipeline passes each one explicitly so the generator's own defaults can
/// never silently change pipeline output (the MIN-40 regression).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    /// Enable terrain (DEM-driven elevation).
    #[serde(default = "default_true")]
    pub terrain: bool,

    /// Enable ESA WorldCover land-cover classification (forests, water, etc.).
    /// Requires `terrain = true` to take effect.
    #[serde(default = "default_true")]
    pub land_cover: bool,

    /// Enable interior generation for buildings.
    #[serde(default = "default_true")]
    pub interior: bool,

    /// Enable entity placement inside buildings.
    #[serde(default = "default_true")]
    pub entities: bool,

    /// Entity theme pack (e.g. "default", "fantasy").
    #[serde(default = "default_entity_theme")]
    pub entity_theme: String,

    /// Enable roof generation.
    #[serde(default = "default_true")]
    pub roof: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            terrain: true,
            land_cover: true,
            interior: true,
            entities: true,
            entity_theme: default_entity_theme(),
            roof: true,
        }
    }
}

fn default_entity_theme() -> String {
    "default".to_string()
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

/// Quality validator configuration. Each block of fields corresponds to one
/// of the four checks the validator runs (structure/size sanity, ground
/// continuity, interior populated, surface diversity) plus the legacy
/// minimums that predate MIN-43.
///
/// Defaults are picked to (a) reject the existing 4,202,496-byte
/// empty-chunks signature, and (b) reject the 20 floating maps already on
/// disk while leaving room to tighten thresholds empirically once a
/// terrain-fixed map exists (MIN-41).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    // ---- Structural sanity (legacy MIN-7 / MIN-29 checks) ----
    /// Minimum number of region files for a valid map.
    #[serde(default = "default_min_region_files")]
    pub min_region_files: usize,

    /// Minimum total map size in bytes (sum across `region/*.mca`).
    #[serde(default = "default_min_map_size_bytes")]
    pub min_map_size_bytes: u64,

    /// Whether to validate Anvil region file structure (>= 8 KiB header).
    #[serde(default = "default_true")]
    pub validate_structure: bool,

    // ---- Region-file size sanity (MIN-43 #3) ----
    /// Region file sizes that exactly match this byte count are flagged as
    /// the "empty chunks" signature with a named failure reason. Default
    /// 4,202,496 — observed on the 20 rural maps where chunks were
    /// allocated but not filled with terrain (see MIN-40 diagnostic).
    #[serde(default = "default_region_empty_signature_bytes")]
    pub region_empty_signature_bytes: u64,

    /// Minimum bytes per occupied chunk in a region file. The 4,202,496 B
    /// empty signature works out to ~4,104 B/chunk; a populated map runs
    /// well above that. Tunable per region area instead of a hardcoded
    /// total threshold.
    #[serde(default = "default_region_min_bytes_per_chunk")]
    pub region_min_bytes_per_chunk: u64,

    // ---- Ground continuity (MIN-43 #1) ----
    /// Number of (x,z) columns sampled per region for the ground-continuity
    /// scan. Sampled on a deterministic stride so the same map produces
    /// the same report on repeated runs.
    #[serde(default = "default_ground_sample_columns_per_region")]
    pub ground_sample_columns_per_region: usize,

    /// Bottom of the y-range a sampled column must be ground-filled to.
    /// Default y=-60 per CTO note (one block above the typical world
    /// floor of y=-64; gives a small bedrock margin).
    #[serde(default = "default_ground_y_min")]
    pub ground_y_min: i32,

    /// Maximum air gaps tolerated below the surface in a sampled column.
    /// Caves are real, so a small number of air blocks is fine; a column
    /// that is mostly air below the surface is the floating-buildings
    /// failure we are trying to catch.
    #[serde(default = "default_ground_max_air_gap_blocks")]
    pub ground_max_air_gap_blocks: usize,

    /// Upper y-bound for the ground-continuity scan. The check walks from
    /// `ground_y_min` up to `min(surface_height, ground_y_scan_cap)` rather
    /// than all the way to the topmost block. Capping the scan just above
    /// the expected terrain level (e.g. ground_level + 16) avoids counting
    /// building-interior air as a ground discontinuity — buildings can be
    /// hundreds of blocks tall, each floor adding legitimate air gaps that
    /// would otherwise exceed `ground_max_air_gap_blocks`.
    ///
    /// Set this to roughly `ground_level + 16` in pipeline.toml whenever
    /// the generator's `--ground-level` is changed from the Minecraft
    /// default of y=64.
    #[serde(default = "default_ground_y_scan_cap")]
    pub ground_y_scan_cap: i32,

    // ---- Interior populated (MIN-43 #2) ----
    /// Number of chunks sampled across the map for the interior check.
    #[serde(default = "default_interior_sample_chunks")]
    pub interior_sample_chunks: usize,

    /// Minimum number of door-containing chunks that must fail the
    /// furniture+floor check before the interior_unpopulated reason is
    /// emitted. A threshold of 2 avoids false positives from buildings
    /// that straddle chunk boundaries (the door lands in the edge chunk
    /// while all furniture is in the adjacent chunk).
    #[serde(default = "default_interior_min_failing_chunks")]
    pub interior_min_failing_chunks: usize,

    // ---- Surface diversity (MIN-43 #4) ----
    /// Minimum distinct surface block types required across sampled
    /// chunks. A road-stripe-only map has 1–2 (asphalt + air); a real
    /// map mixes grass/dirt/stone/water/sand and lands well above this.
    #[serde(default = "default_surface_diversity_min_distinct")]
    pub surface_diversity_min_distinct: usize,

    /// Number of chunks sampled across the map for the surface-diversity
    /// check.
    #[serde(default = "default_surface_diversity_sample_chunks")]
    pub surface_diversity_sample_chunks: usize,
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
            region_empty_signature_bytes: default_region_empty_signature_bytes(),
            region_min_bytes_per_chunk: default_region_min_bytes_per_chunk(),
            ground_sample_columns_per_region: default_ground_sample_columns_per_region(),
            ground_y_min: default_ground_y_min(),
            ground_max_air_gap_blocks: default_ground_max_air_gap_blocks(),
            ground_y_scan_cap: default_ground_y_scan_cap(),
            interior_sample_chunks: default_interior_sample_chunks(),
            interior_min_failing_chunks: default_interior_min_failing_chunks(),
            surface_diversity_min_distinct: default_surface_diversity_min_distinct(),
            surface_diversity_sample_chunks: default_surface_diversity_sample_chunks(),
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

fn default_region_empty_signature_bytes() -> u64 {
    // Empirical signature observed across the 20 floating maps on the
    // mini PC: a region file with mostly empty chunks (just enough
    // non-air blocks for road/building outlines to allocate the chunks
    // but no terrain volume below) lands at exactly 4,202,496 B.
    4_202_496
}

fn default_region_min_bytes_per_chunk() -> u64 {
    // 4,202,496 / 1024 chunks = ~4,104 B/chunk. Anything at or below
    // that floor implies the chunk has no terrain. Default 4,200 B
    // to leave a little headroom; tighten once we have a fleet of
    // known-good (terrain-fixed) maps to calibrate against.
    4_200
}

fn default_ground_sample_columns_per_region() -> usize {
    // 64 columns on a deterministic stride covers the region cheaply.
    // 32×32 chunks × 16×16 columns/chunk = 262,144 columns; 64
    // samples is a 0.024% sample — cheap enough to run on every map
    // and dense enough to catch a region-wide ground gap.
    64
}

fn default_ground_y_min() -> i32 {
    // CTO spec: scan from y=-60 up to surface. World floor is y=-64
    // in 1.18+; the 4-block margin avoids treating bedrock as a
    // ground gap.
    -60
}

fn default_ground_max_air_gap_blocks() -> usize {
    // Caves and OSM-driven basements are legitimate; a column with a
    // handful of air blocks between bedrock and surface is fine. The
    // floating-buildings failure mode is hundreds of blocks of air,
    // so a generous tolerance here still catches it.
    16
}

fn default_ground_y_scan_cap() -> i32 {
    // Cap the ground-continuity scan at 16 blocks above the default
    // ground_level (64), so building interiors above y=80 are not counted
    // as air gaps. For the floating-buildings failure mode, the air gap
    // starts below y=64 and the scan catches it well within this cap.
    80
}

fn default_interior_sample_chunks() -> usize {
    32
}

fn default_interior_min_failing_chunks() -> usize {
    2
}

fn default_surface_diversity_min_distinct() -> usize {
    // 4 picked as a starting floor: a real map mixes grass/dirt/
    // stone/water at minimum. Tune empirically once MIN-41 lands a
    // known-good map and we can sample it.
    4
}

fn default_surface_diversity_sample_chunks() -> usize {
    16
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
        let mut config: Self = toml::from_str(&content)?;
        // Resolve relative paths relative to the config file's directory so
        // the pipeline can be invoked from any working directory.
        if let Some(base) = path.parent() {
            if config.locations_file.is_relative() {
                config.locations_file = base.join(&config.locations_file);
            }
            if config.arnis_binary.is_relative() {
                config.arnis_binary = base.join(&config.arnis_binary);
            }
        }
        Ok(config)
    }
}
