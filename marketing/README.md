# Marketing Interface

This directory contains documentation for requesting and receiving generated Minecraft maps. No code or build artifacts belong here.

## How Map Generation Works

Minecraft Map Factory converts real-world locations into playable Minecraft worlds. You provide a location and bounding box, the pipeline generates the map, and you receive a ready-to-use Minecraft world folder.

## Request-to-Delivery Workflow

### 1. Submit a Map Request

Fill out the [Map Request Template](MAP_REQUEST_TEMPLATE.md) with your desired location, bounding box coordinates, priority tier, and any special notes. Submit the completed request to the engineering team.

### 2. Location Added to Database

Engineering adds your location to the pipeline's locations database. Each location gets a name, geographic bounding box, and a size tier (small, medium, or large) that determines processing priority.

### 3. Pipeline Processes the Request

The automated pipeline handles everything from here:

- **Scheduling** -- your location is queued by tier (small locations process first)
- **Generation** -- real-world geographic data is fetched and converted into Minecraft blocks, terrain, and entities
- **Validation** -- the generated world is checked for completeness and structural integrity
- **Publishing** -- validated maps are placed in the published output directory

### 4. Receive Your Map

Once published, your map is available as a standard Minecraft world folder. See [Output Format](OUTPUT_FORMAT.md) for details on the file structure and how to use the generated maps.

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
| [MAP_REQUEST_TEMPLATE.md](MAP_REQUEST_TEMPLATE.md) | Template for submitting new map requests |
| [OUTPUT_FORMAT.md](OUTPUT_FORMAT.md) | Description of published map file structure and usage |
