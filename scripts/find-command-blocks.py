#!/usr/bin/env python3
"""Scan the live world NBT for command blocks and print their coordinates and commands.

Usage: python3 scripts/find-command-blocks.py

Reads directly from the host-mounted world directory. Intended for post-smoke-test
cleanup verification — any block with a trivial command (say test, etc.) is a leftover.
"""

import os
import sys

try:
    import nbt
except ImportError:
    print("ERROR: nbt library not found. Install with: pip3 install nbt", file=sys.stderr)
    sys.exit(1)

WORLD_DIR = "/home/jason/minecraft-server/data/Times_Square__NYC"
DIMENSIONS = [
    ("overworld", os.path.join(WORLD_DIR, "dimensions/minecraft/overworld/region")),
    ("nether",    os.path.join(WORLD_DIR, "dimensions/minecraft/the_nether/region")),
    ("end",       os.path.join(WORLD_DIR, "dimensions/minecraft/the_end/region")),
]

found = []

for dim_name, region_dir in DIMENSIONS:
    if not os.path.isdir(region_dir):
        continue
    for fname in sorted(os.listdir(region_dir)):
        if not fname.endswith(".mca"):
            continue
        fpath = os.path.join(region_dir, fname)
        try:
            region = nbt.region.RegionFile(fpath)
        except Exception as e:
            print(f"WARN: could not open {fpath}: {e}", file=sys.stderr)
            continue
        for cx in range(32):
            for cz in range(32):
                try:
                    chunk = region.get_nbt(cx, cz)
                    if chunk is None:
                        continue
                    for key in ("block_entities", "TileEntities"):
                        if key not in chunk:
                            continue
                        for be in chunk[key]:
                            be_id = str(be.get("id", ""))
                            if "command" not in be_id.lower():
                                continue
                            x = str(be.get("x", "?"))
                            y = str(be.get("y", "?"))
                            z = str(be.get("z", "?"))
                            cmd = str(be.get("Command", ""))
                            auto = str(be.get("auto", "?"))
                            found.append((dim_name, x, y, z, auto, cmd))
                except Exception:
                    pass

if not found:
    print("No command blocks found.")
    sys.exit(0)

print(f"Found {len(found)} command block(s):\n")
print(f"{'Dimension':<12} {'X':>6} {'Y':>4} {'Z':>6}  {'Always':>6}  Command")
print("-" * 80)
for (dim, x, y, z, auto, cmd) in found:
    always = "yes" if auto == "1" else "no"
    print(f"{dim:<12} {x:>6} {y:>4} {z:>6}  {always:>6}  {cmd!r}")
