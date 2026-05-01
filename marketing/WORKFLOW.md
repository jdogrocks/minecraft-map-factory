# Dev ↔ Marketing Automated Workflow

This document defines the contract between Minecraft Map Factory (the dev shop) and the marketing partner. The handoff is intentionally narrow so both sides can run independently and asynchronously.

## Roles

| Role | Owner | Responsibilities |
|------|-------|------------------|
| **Marketing** | external partner | Researches in-demand locations, files map requests, distributes and monetizes the published outputs |
| **Engineering** | this repo | Maintains the pipeline, accepts requests via `pipeline/data/locations.toml`, and publishes validated worlds |
| **Pipeline** | GitHub Actions | Generates and validates maps on a fixed schedule, no manual coordination needed |

## End-to-End Flow

```
Marketing research
        │
        ▼
GitHub issue (Map Request)         ──►  filed against this repo using
        │                               marketing/MAP_REQUEST_TEMPLATE.md
        ▼
Engineering adds entry to
pipeline/data/locations.toml       ──►  one PR per batch of new locations
        │
        ▼
Scheduled Pipeline (cron)          ──►  runs at 00:01 and 12:01 UTC daily
        │                               (.github/workflows/scheduled-pipeline.yml)
        ▼
Validated worlds land in
pipeline/output/published/{name}/  ──►  Anvil region files, Java Edition 1.17+
        │
        ▼
Commit Output Artifacts            ──►  pushes published maps to main
(workflow_dispatch)                     (.github/workflows/commit-output.yml)
        │
        ▼
Marketing pulls / monetizes        ──►  `git pull` to fetch the new worlds,
                                        upload to distribution channels
```

## Inputs (Marketing → Engineering)

### 1. File a Map Request

Open a GitHub issue in this repo using the `marketing/MAP_REQUEST_TEMPLATE.md` template. Required fields:

- **Location name** — human-readable (e.g., "Times Square, NYC")
- **Bounding box** — `min_lat, min_lng, max_lat, max_lng`
- **Tier** — `small`, `medium`, or `large`

Optional fields: state/region, tags, and notes (any constraints engineering should know).

### 2. Batch and Track

Marketing can submit requests one at a time or in batches. Engineering groups them into a PR that appends new entries to `pipeline/data/locations.toml`. The next scheduled run picks them up automatically.

### 3. SLA

| Tier | Typical area | Generation time |
|------|--------------|-----------------|
| Small | Single landmark or intersection | Minutes |
| Medium | Neighborhood or district | Minutes to hours |
| Large | Downtown area or multiple blocks | Hours |

Times are per-job and depend on CI capacity. The pipeline runs at 00:01 and 12:01 UTC, so a request merged before 23:00 UTC will normally appear in the next morning's output.

## Outputs (Engineering → Marketing)

### 1. Where Maps Live

Validated worlds are written to:

```
pipeline/output/published/{sanitized_location_name}/
```

Inside each location folder you'll find a Minecraft world directory with:

```
{location}/
  region/
    r.0.0.mca
    r.0.-1.mca
    ...
```

Names are sanitized to alphanumeric characters and dashes. See [OUTPUT_FORMAT.md](OUTPUT_FORMAT.md) for the complete file structure.

### 2. How Marketing Pulls Outputs

Two delivery channels are supported:

1. **Repo (default for monetization)** — once the `Commit Output Artifacts` workflow has been dispatched against the latest pipeline run, the published maps are committed to `main`. Marketing clones or pulls the repo and reads from `pipeline/output/published/`. This is the canonical path for monetizable deliverables because it carries Git history.
2. **CI artifact (transient)** — every scheduled run also uploads `pipeline-output` to GitHub Actions with 30-day retention. Useful for previewing a single run without waiting for the commit.

### 3. Quality Guarantees

Every map under `pipeline/output/published/` has passed pipeline validation:

- ≥1 valid `.mca` region file
- Total output ≥ `validation.min_map_size_bytes` (default 1024)
- Each `.mca` ≥ 8 KB with valid Anvil headers (when `validate_structure` is enabled)

If a requested location is not present in `published/`, it failed validation and should be re-requested with adjusted parameters (typically a smaller bounding box).

## Failure Handling

| Symptom | Owner | Action |
|---------|-------|--------|
| Request not in `published/` after the next scheduled run | Engineering | Inspect pipeline logs for the location, classify the failure, retry with a tighter bbox if needed |
| Marketing-reported quality issue (missing buildings, wrong terrain) | Engineering | Open an issue against the responsible work stream (e.g., terrain integrity, interiors). Reference the affected location |
| Schedule miss (no run at the expected window) | Engineering | Check the `Scheduled Pipeline` workflow status on the Actions tab; rerun via `workflow_dispatch` |
| Commit-output workflow not dispatched | Engineering | Run it manually with `output-path: pipeline/output/published` |

## Files in This Directory

| File | Purpose |
|------|---------|
| [README.md](README.md) | High-level overview of the marketing interface |
| [MAP_REQUEST_TEMPLATE.md](MAP_REQUEST_TEMPLATE.md) | Template marketing fills out per request |
| [OUTPUT_FORMAT.md](OUTPUT_FORMAT.md) | Specification of published map files |
| [WORKFLOW.md](WORKFLOW.md) | This document — the dev ↔ marketing contract |

## Related Engineering Docs

- [Architecture](../docs/architecture.md) — system components, data flow, automation
- [Pipeline operations](../docs/pipeline-operations.md) — configuration, scheduled runs, observability
