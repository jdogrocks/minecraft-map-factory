# Output Format

This document describes the structure and format of published Minecraft maps produced by the pipeline.

## Published Map Location

Completed maps are placed in the pipeline's published output directory:

```
{output_dir}/published/{location_name}/
```

Each location gets its own folder. The location name is sanitized to contain only alphanumeric characters and dashes (spaces and special characters are replaced with underscores).

## File Structure

Each published map is a standard Minecraft world folder containing Anvil region files:

```
published/times-square-nyc/
  region/
    r.0.0.mca
    r.0.-1.mca
    r.-1.0.mca
    ...
```

### Region Files (`.mca`)

Region files use Minecraft's Anvil format. Each `.mca` file contains a 32x32 grid of chunks (512x512 blocks). Key characteristics:

- Minimum file size: 8 KB (two 4096-byte lookup tables)
- Named by region coordinates: `r.{x}.{z}.mca`
- Compatible with Minecraft Java Edition 1.17+

The number of region files depends on the size of the requested bounding box. A small landmark may produce 1-2 region files; a large downtown area may produce dozens.

## Using Generated Maps

### Minecraft Java Edition

Copy the published location folder into your Minecraft saves directory:

```
# macOS
~/Library/Application Support/minecraft/saves/

# Windows
%APPDATA%\.minecraft\saves\

# Linux
~/.minecraft/saves/
```

The folder will appear as a selectable world in the Minecraft launcher.

### Minecraft Bedrock Edition

Bedrock Edition worlds require conversion from the Java Edition Anvil format. The generated maps are in Java Edition format by default.

## Quality Guarantees

Every published map has passed the pipeline's validation checks:

- At least one valid `.mca` region file is present
- Total output meets the minimum size threshold
- Each region file has valid Anvil format headers (when structure validation is enabled)

Maps that fail any validation check are not published. If a requested map is not present in the published output, it did not pass validation and may need to be re-requested with adjusted parameters (e.g., a smaller bounding box).
