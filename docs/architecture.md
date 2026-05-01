# System Architecture

Minecraft Map Factory transforms real-world geographic data into playable Minecraft worlds through a multi-stage pipeline. This document describes the system components, data flow, and how they fit together.

## Components

### Arnis Core (`src/`)

The map generation engine. Converts OpenStreetMap geographic data into Minecraft Java Edition (1.17+) and Bedrock Edition worlds. Key subsystems:

- **OSM Parser** -- fetches and parses OpenStreetMap data for a bounding box
- **Element Processing** -- converts OSM elements (buildings, roads, water, vegetation) into Minecraft block types
- **Elevation** -- processes terrain height data for realistic topography
- **Entity Placement** -- spawns appropriate Minecraft entities within generated areas
- **World Editor** -- writes Minecraft Anvil region files (`.mca`) to disk
- **Coordinate System** -- translates geographic coordinates (lat/lng) to Minecraft block coordinates

### Pipeline (`pipeline/`)

An autonomous batch processor that schedules, generates, validates, and publishes maps at scale. Comprised of four stages described below.

### Locations Database (`pipeline/data/locations.toml`)

A TOML file defining target areas for map generation. Each entry includes a human-readable name, geographic bounding box, size tier, and optional tags. The pipeline processes locations in tier order: small first, then medium, then large.

Example entry:

```toml
[[location]]
name = "Times Square, NYC"
state = "NY"
bbox = [40.7565, -73.9882, 40.7600, -73.9842]
tier = "small"
tags = ["landmark", "urban"]
```

## Pipeline Stages

### 1. Scheduler

The orchestrator. Reads the locations database, prioritizes by tier, and dispatches concurrent generation jobs. Enforces resource limits defined in `pipeline.toml`:

- `scheduler.max_concurrency` -- parallel job limit (default: half of available CPUs)
- `scheduler.max_memory_mb` -- RSS memory threshold before throttling
- `scheduler.max_cpu_percent` -- CPU usage threshold

Between jobs, a self-tuner monitors success rates and resource usage. If the success rate drops below `tuning.min_success_rate` or memory exceeds `tuning.memory_threshold`, concurrency is automatically reduced (minimum of 1).

### 2. Generator

Invokes the Arnis core binary for each location with bounding-box parameters. Output lands in `{output_dir}/jobs/{location_name}_attempt{N}/`.

Failures are classified and handled by the retry strategy:

| Error Type | Behavior |
|------------|----------|
| **Transient** (timeout, HTTP 429/503) | Retry with exponential backoff (configured via `retry.initial_backoff_secs` and `retry.max_backoff_secs`) |
| **Resource** (OOM, disk full) | Shrink bounding box by `retry.bbox_shrink_factor` and retry |
| **Permanent** (invalid coordinates) | Fail immediately, no retry |

Maximum attempts per job: `retry.max_retries` (default: 3).

### 3. Validator

Checks generated map integrity before publishing:

- **Region file count** -- at least `validation.min_region_files` `.mca` files present
- **Total output size** -- meets `validation.min_map_size_bytes` threshold
- **Anvil structure** -- each `.mca` file is at least 8 KB with valid Anvil format headers (when `validation.validate_structure` is enabled)

Maps that fail validation are marked as failed with specific failure reasons.

### 4. Publisher

Copies validated maps to their final location at `{output_dir}/published/{sanitized_name}/`. Location names are sanitized to alphanumeric characters and dashes. If a previous output exists for that location, it is replaced.

## Data Flow

```
locations.toml
      |
      v
  Scheduler ──── Self-Tuner
      |               |
      v               |
  Generator ←── Retry Strategy
      |
      v
  Validator
      |
      v
  Publisher ──→ {output_dir}/published/{location_name}/
      |
      v
  Metrics Collector
```

1. The **Scheduler** reads locations from `locations.toml` and dispatches jobs in tier order
2. The **Generator** invokes the Arnis binary, producing raw Minecraft world files in `jobs/`
3. The **Validator** checks region file count, size, and Anvil structure
4. The **Publisher** copies validated output to `published/` for consumption
5. The **Metrics Collector** records duration, output size, and success/failure for each job

## Output Locations

All paths are relative to the `output_dir` setting in `pipeline.toml`. In the scheduled CI run, `output_dir` is `pipeline/output/`, so the canonical, repo-relative paths are:

| Path | Contents |
|------|----------|
| `pipeline/output/jobs/{name}_attempt{N}/` | Raw generator output per attempt |
| `pipeline/output/published/{name}/` | Validated, ready-to-use Minecraft worlds |

Validated, published worlds are auto-committed back to `main` by the commit-output workflow (see Automation & Scheduling), so anyone with repo access can `git pull` to retrieve the latest map set.

## Automation & Scheduling

The pipeline does not need to be invoked by hand. GitHub Actions drives the full cycle:

| Workflow | File | Trigger | Purpose |
|----------|------|---------|---------|
| **Scheduled Pipeline** | `.github/workflows/scheduled-pipeline.yml` | Cron `1 0 * * *` and `1 12 * * *` (UTC) — twice daily at 00:01 and 12:01 UTC; also `workflow_dispatch` | Build the pipeline in release mode, run it against `pipeline/data/locations.toml`, and upload `pipeline/output/` as a 30-day artifact |
| **Commit Output Artifacts** | `.github/workflows/commit-output.yml` | `workflow_dispatch` (input: `output-path`) | Stage and commit any new files under the given output path with a metadata-aware message, then push to `main` |
| **CI Build** | `.github/workflows/ci-build.yml` | Pull requests touching code/docs/marketing, plus push to `main` | Lint, test, and validate changes before merge |

End-to-end automated flow:

```
00:01 UTC / 12:01 UTC          (cron)
  └─► scheduled-pipeline.yml
        └─► cargo build --release -p minecraft-map-factory-pipeline
        └─► run pipeline → pipeline/output/published/{location}/
        └─► upload-artifact (pipeline-output, 30d retention)

manual / follow-up
  └─► commit-output.yml  (workflow_dispatch with output-path)
        └─► git add pipeline/output/...
        └─► git commit && git push   → main
```

Concurrency is gated by `concurrency: scheduled-pipeline` so two scheduled runs never overlap. If you need an off-schedule run, dispatch the workflow manually from the GitHub Actions tab.

## Configuration

Runtime behavior is controlled by `pipeline.toml`. See [pipeline-operations.md](pipeline-operations.md) for the full configuration reference, CLI overrides, and operational guidance.

## Metrics and Observability

The pipeline tracks per-job metrics (duration, output size, success/failure) and prints periodic summaries with p50/p95/p99 duration percentiles and failure breakdowns. Use `--json-logs` for structured output suitable for log aggregation. See [pipeline-operations.md](pipeline-operations.md) for details.
