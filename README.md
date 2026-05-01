# Minecraft Map Factory

Autonomous pipeline for generating real-world Minecraft Java Edition (1.17+) and Bedrock Edition maps from geographic data. Originally derived from [Arnis](https://github.com/louis-e/arnis), Minecraft Map Factory adds a multi-stage pipeline that schedules, generates, validates, and publishes maps at scale.

## Features

- **Batch generation** -- process a locations database and produce Minecraft worlds automatically
- **Concurrent scheduling** -- resource-aware parallelism with CPU and memory throttling
- **Retry with bbox shrinking** -- transient failures retry with exponential backoff; resource errors shrink the bounding box
- **Anvil validation** -- checks region file count, total size, and internal structure before publishing
- **Self-tuning** -- dynamically reduces concurrency when success rates drop or resources are constrained
- **Metrics** -- per-job tracking with p50/p95/p99 duration, output size, success rate, and failure breakdown

## Quick Start

### Prerequisites

- Rust 1.75+ (2021 edition)
- A built `minecraft-map-factory` binary (the core map generator)

### Build

```bash
# Build the full workspace (GUI app + pipeline)
cargo build --release

# Build only the pipeline
cargo build --release -p minecraft-map-factory-pipeline
```

### Run the Pipeline

```bash
# With default config
minecraft-map-factory-pipeline --config pipeline.toml

# Override output directory and concurrency
minecraft-map-factory-pipeline --config pipeline.toml \
  --output-dir ./maps \
  --max-concurrency 4

# Validate config without generating
minecraft-map-factory-pipeline --config pipeline.toml --dry-run

# JSON-structured logs
minecraft-map-factory-pipeline --config pipeline.toml --json-logs
```

See [docs/pipeline-operations.md](docs/pipeline-operations.md) for full pipeline operations documentation, [docs/architecture.md](docs/architecture.md) for the system architecture overview (including the twice-daily CI schedule and auto-commit flow), and [marketing/WORKFLOW.md](marketing/WORKFLOW.md) for the dev ↔ marketing handoff contract.

### Run the GUI (Arnis)

```bash
# GUI build (default features)
cargo run

# CLI-only build
cargo run --no-default-features -- \
  --terrain \
  --path="/path/to/.minecraft/saves/worldname" \
  --bbox="min_lat,min_lng,max_lat,max_lng"
```

## Project Structure

```
minecraft-map-factory/
  src/              # Arnis core -- map generation engine
  pipeline/         # Autonomous pipeline (scheduler, generator, validator, publisher)
  docs/             # Architecture and operations documentation
  marketing/        # Map request templates and output format guides
  assets/           # GUI and documentation assets
  tests/            # Integration tests
```

## License

Copyright (c) 2022-2026 Louis Erbkamm (louis-e)

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

This project is a derivative work of [Arnis](https://github.com/louis-e/arnis). See [NOTICE](NOTICE) for attribution.

NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.
