# Marketing Interface

This directory is the contract between Minecraft Map Factory (the dev shop) and the marketing partner. It contains everything needed to request, receive, and monetize generated Minecraft maps. No code or build artifacts belong here.

For the full automation contract — schedule, delivery channels, and failure handling — see **[WORKFLOW.md](WORKFLOW.md)**.

## How Map Generation Works

Minecraft Map Factory converts real-world locations into playable Minecraft worlds. You provide a location and bounding box, the pipeline generates the map twice a day on GitHub Actions, and you receive a ready-to-use Minecraft world folder either as a CI artifact or committed to the repo.

## Request-to-Delivery Workflow

### 1. Submit a Map Request

Open a GitHub issue against this repo using the [Map Request Template](MAP_REQUEST_TEMPLATE.md): desired location, bounding box, priority tier, and any special notes.

### 2. Location Added to Database

Engineering merges a PR that appends your location to `pipeline/data/locations.toml`. Each location gets a name, bounding box, and tier (small, medium, or large) that determines processing priority.

### 3. Pipeline Processes the Request

The pipeline runs on a fixed schedule — **00:01 UTC and 12:01 UTC daily** — via the `Scheduled Pipeline` GitHub Actions workflow. Each run:

- **Schedules** locations by tier (small first, then medium, then large)
- **Generates** worlds from OpenStreetMap data with terrain, buildings, and entities
- **Validates** completeness and structural integrity
- **Publishes** validated maps to `pipeline/output/published/{name}/`

### 4. Receive Your Map

Two delivery channels are available:

- **CI artifact** — every scheduled run uploads `pipeline-output` to GitHub Actions (30-day retention). Best for previewing a single run.
- **Repo (canonical for monetization)** — after the `Commit Output Artifacts` workflow runs, the published maps are committed to `main`. `git pull` to retrieve them.

See [OUTPUT_FORMAT.md](OUTPUT_FORMAT.md) for the file structure and how to use the worlds.

## Turnaround

Processing time depends on the size of the requested area:

| Tier | Typical Area | Processing |
|------|-------------|------------|
| Small | Single landmark or intersection | Minutes |
| Medium | Neighborhood or district | Minutes to hours |
| Large | Downtown area or multiple blocks | Hours |

The pipeline processes multiple locations concurrently. Actual times depend on server load and queue depth.

## Files in This Directory

| File | Purpose |
|------|---------|
| [WORKFLOW.md](WORKFLOW.md) | Full dev ↔ marketing contract: schedule, delivery, failure handling |
| [MAP_REQUEST_TEMPLATE.md](MAP_REQUEST_TEMPLATE.md) | Template for submitting new map requests |
| [OUTPUT_FORMAT.md](OUTPUT_FORMAT.md) | Description of published map file structure and usage |
