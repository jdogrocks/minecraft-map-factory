# Pipeline Operations

The Minecraft Map Factory pipeline automates map generation through four stages: **Schedule**, **Generate**, **Validate**, and **Publish**. It runs as a standalone binary (`minecraft-map-factory-pipeline`) that processes a locations database and produces validated Minecraft worlds.

## Architecture

```
Locations DB (TOML)
       |
  Scheduler ──── Self-Tuner
       |              |
  Generator ←── Retry Strategy
       |
  Validator
       |
  Publisher ──→ published/{location}/
       |
  Metrics Collector
```

Each job flows through these stages sequentially. The scheduler runs multiple jobs concurrently up to a configurable limit, governed by resource constraints.

## Stages

### Scheduler

Orchestrates the pipeline. Pulls pending locations from the database, spawns concurrent generation tasks using a tokio semaphore, and routes results through validation and publishing.

- Respects `max_concurrency`, `max_memory_mb`, and `max_cpu_percent` limits
- Prioritizes locations by tier: small, medium, large
- Invokes the self-tuner between jobs to adjust concurrency dynamically

### Generator

Invokes the `arnis` binary with bounding-box parameters for each location. Handles failures through the retry strategy.

- Output lands in `jobs/{location_name}_attempt{N}/`
- On transient errors: retries with exponential backoff (1s -> 2s -> 4s, capped at 300s)
- On resource errors: shrinks the bounding box by `bbox_shrink_factor` (default 0.8) before retrying
- On permanent errors: fails immediately, no retry

### Validator

Checks generated map integrity before publishing:

- **Region file count** -- at least `min_region_files` `.mca` files (default: 1)
- **Total size** -- meets `min_map_size_bytes` threshold (default: 1024 bytes)
- **Anvil structure** -- each `.mca` file is at least 8 KB and has valid Anvil format headers (when `validate_structure` is enabled)

Failed validation marks the location as failed with the specific reason.

### Publisher

Copies validated maps to `published/{sanitized_location_name}/`, overwriting any previous output. Updates the location status to Completed.

## Configuration

The pipeline reads a TOML configuration file (default: `pipeline.toml`).

### Full Configuration Reference

```toml
# Required: path to the locations database
locations_file = "locations.toml"

# Required: output directory for generated and published maps
output_dir = "./output"

# Path to the arnis binary (default: "arnis" on PATH)
arnis_binary = "./target/release/arnis"

[scheduler]
max_concurrency = 4        # default: num_cpus / 2
max_memory_mb = 4096       # RSS limit before throttling (default: 4096)
max_cpu_percent = 0.8      # CPU threshold 0.0-1.0 (default: 0.8)

[retry]
max_retries = 3            # per job (default: 3)
initial_backoff_secs = 1   # first backoff delay (default: 1)
max_backoff_secs = 300     # backoff cap (default: 300)
shrink_bbox_on_retry = true
bbox_shrink_factor = 0.8   # multiply bbox dimensions on retry (default: 0.8)

[validation]
min_region_files = 1       # minimum .mca files (default: 1)
min_map_size_bytes = 1024  # minimum total output size (default: 1024)
validate_structure = true  # check Anvil format headers (default: true)

[metrics]
summary_interval = 10      # print summary every N jobs (default: 10)

[tuning]
enabled = true             # enable self-tuning (default: true)
min_success_rate = 0.5     # reduce concurrency below this (default: 0.5)
memory_threshold = 0.8     # reduce concurrency above this memory usage (default: 0.8)
```

### CLI Overrides

| Flag | Description |
|------|-------------|
| `--config <path>` | Path to TOML config file (default: `pipeline.toml`) |
| `--output-dir <path>` | Override `output_dir` from config |
| `--max-concurrency <N>` | Override `scheduler.max_concurrency` |
| `--json-logs` | Emit structured JSON logs |
| `--dry-run` | Validate config and locations, then exit |

## Locations Database

Locations are defined in a TOML file. Each entry specifies a name, bounding box, and optional tier for priority ordering.

Jobs are processed in tier order: `small` first, then `medium`, then `large`. The pipeline tracks each location's status: Pending, InProgress, Completed, Failed, or Skipped.

## Metrics and Observability

### Structured Logging

Set `RUST_LOG` to control log verbosity:

```bash
RUST_LOG=info minecraft-map-factory-pipeline --config pipeline.toml
RUST_LOG=debug minecraft-map-factory-pipeline --config pipeline.toml
```

Use `--json-logs` for machine-parseable output suitable for log aggregators.

### Metrics Summary

The pipeline prints a metrics summary every `summary_interval` jobs (default: 10), including:

| Metric | Description |
|--------|-------------|
| `total_jobs` | Jobs completed so far |
| `successes` / `failures` | Count and success rate |
| `duration_p50` / `p95` / `p99` | Generation time percentiles |
| `total_output_mb` | Cumulative published map size |
| `failure_reasons` | Breakdown by error category |

### Self-Tuning

When enabled, the self-tuner monitors success rate and resource usage between jobs:

- If `success_rate < min_success_rate` (default: 0.5), concurrency is reduced
- If memory usage exceeds `memory_threshold` (default: 80%), concurrency is reduced
- Concurrency never drops below 1

## Running End-to-End

1. **Prepare locations** -- create a `locations.toml` with your target areas
2. **Configure** -- create `pipeline.toml` pointing to your locations file, output dir, and arnis binary
3. **Validate** -- run with `--dry-run` to check configuration
4. **Execute** -- run without `--dry-run` to start generation
5. **Monitor** -- watch structured logs and periodic metrics summaries
6. **Collect** -- find validated maps in `{output_dir}/published/{location_name}/`

```bash
# Example end-to-end run
minecraft-map-factory-pipeline \
  --config pipeline.toml \
  --json-logs 2>&1 | tee pipeline.log
```
