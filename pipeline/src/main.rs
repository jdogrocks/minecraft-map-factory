mod config;
mod generator;
mod locations;
mod metrics;
mod publisher;
mod scheduler;
mod validator;

use clap::Parser;
use config::PipelineConfig;
use locations::LocationDatabase;
use metrics::MetricsCollector;
use scheduler::Scheduler;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

/// Autonomous map generation pipeline for Minecraft Map Factory.
#[derive(Parser, Debug)]
#[command(name = "minecraft-map-factory-pipeline", version, about)]
struct Cli {
    /// Path to the pipeline configuration file (TOML).
    #[arg(long, default_value = "pipeline.toml")]
    config: PathBuf,

    /// Override output directory.
    #[arg(long)]
    output_dir: Option<PathBuf>,

    /// Override max concurrency.
    #[arg(long)]
    max_concurrency: Option<usize>,

    /// Enable JSON log output.
    #[arg(long)]
    json_logs: bool,

    /// Dry run: validate config and locations, but don't generate maps.
    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    // Set up logging
    if cli.json_logs {
        fmt()
            .json()
            .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
            .init();
    } else {
        fmt()
            .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
            .init();
    }

    info!(config_path = %cli.config.display(), "Loading pipeline configuration");

    // Load configuration
    let mut config = PipelineConfig::from_file(&cli.config)?;

    // Apply CLI overrides
    if let Some(output_dir) = cli.output_dir {
        config.output_dir = output_dir;
    }
    if let Some(max_concurrency) = cli.max_concurrency {
        config.scheduler.max_concurrency = max_concurrency;
    }

    info!(
        locations_file = %config.locations_file.display(),
        output_dir = %config.output_dir.display(),
        arnis_binary = %config.arnis_binary.display(),
        max_concurrency = config.scheduler.max_concurrency,
        "Configuration loaded"
    );

    // Load location database
    let db = LocationDatabase::from_file(&config.locations_file)?;
    let summary = db.status_summary();
    info!(
        total = db.total(),
        pending = summary.pending,
        "Location database loaded"
    );

    if cli.dry_run {
        info!("Dry run mode — configuration and locations validated. Exiting.");
        return Ok(());
    }

    // Ensure output directory exists
    std::fs::create_dir_all(&config.output_dir)?;

    // Run the pipeline
    let metrics = MetricsCollector::new();
    let scheduler = Scheduler::new(config, db, metrics);
    scheduler.run().await?;

    Ok(())
}
