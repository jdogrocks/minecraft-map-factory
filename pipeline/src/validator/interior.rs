//! Interior-populated check (MIN-43 #2). The validator can't see OSM
//! building footprints from a finished .mca, so we use the door blocks the
//! generator places at building entrances as a proxy for "this chunk
//! contains a building." For every sampled chunk that contains at least
//! one door block, we require the same chunk to also contain at least one
//! furniture block, and the chunk must contain a horizontal layer of
//! non-air blocks at the door's foot Y (the floor partition).
//!
//! Block-tag schema is coordinated with MIN-42 (Game Developer); see the
//! coordination comment posted to that issue. The matcher below is the
//! initial schema; if MIN-42 introduces more furniture block types this
//! list extends without breaking existing pass cases (more furniture
//! options can only make the check looser, not tighter).
//!
//! Empty maps with no doors at all aren't flagged here — that case is
//! caught by the surface-diversity and ground-continuity checks. The
//! interior check only catches the *partial* case where the generator
//! placed door blocks but no interior, which is exactly the WS-2 failure
//! mode we want to surface in regression.

use super::anvil;
use super::anvil::RegionFile;
use crate::config::ValidationConfig;
use fastanvil::{Chunk, JavaChunk};
use std::collections::HashSet;
use tracing::debug;

pub fn check(
    config: &ValidationConfig,
    region_files: &[RegionFile],
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut reasons = Vec::new();
    if region_files.is_empty() {
        return Ok(reasons);
    }

    let target_samples = config.interior_sample_chunks.max(1);
    // Spread the sample budget evenly across regions; round up so we
    // always touch every region at least once when sample > 0.
    let per_region = target_samples.div_ceil(region_files.len());
    let stride = (1024 / per_region.max(1)).max(1);

    let mut sampled_chunks = 0usize;
    let mut chunks_with_doors = 0usize;
    let mut chunks_failing = 0usize;
    let mut failure_examples: Vec<String> = Vec::new();

    for rf in region_files {
        let mut taken_in_region = 0usize;
        anvil::for_each_chunk(rf, |cx, cz, chunk| {
            let chunk_idx = cz * 32 + cx;
            // Deterministic stride sample: take every `stride`-th chunk
            // until we've grabbed `per_region`.
            if chunk_idx % stride != 0 {
                return;
            }
            if taken_in_region >= per_region {
                return;
            }
            taken_in_region += 1;
            sampled_chunks += 1;

            let summary = chunk_summary(chunk);
            let Some(door_y) = summary.door_y else {
                return; // No buildings in this sampled chunk; not a failure.
            };
            chunks_with_doors += 1;

            let mut chunk_reasons = Vec::new();
            if !summary.has_furniture {
                chunk_reasons.push("no_furniture".to_string());
            }
            if !has_floor_partition(chunk, door_y) {
                chunk_reasons.push("no_floor_partition".to_string());
            }

            if !chunk_reasons.is_empty() {
                chunks_failing += 1;
                if failure_examples.len() < 3 {
                    failure_examples.push(format!(
                        "{} chunk=({},{}) door_y={} reasons={}",
                        rf.path.display(),
                        cx,
                        cz,
                        door_y,
                        chunk_reasons.join("+")
                    ));
                }
            }
        })?;
    }

    debug!(
        sampled_chunks,
        chunks_with_doors, chunks_failing, "Interior-populated sampling complete"
    );

    if chunks_failing > 0 {
        reasons.push(format!(
            "interior_unpopulated: {}/{} sampled chunks contain a door but \
             no furniture/floor — buildings present without interior content. \
             Examples: {}",
            chunks_failing,
            chunks_with_doors,
            failure_examples.join("; ")
        ));
    }

    Ok(reasons)
}

/// Per-chunk summary used by the populated-interior check.
struct ChunkSummary {
    /// y-coordinate of the lower half of the lowest door block found.
    door_y: Option<i32>,
    has_furniture: bool,
}

fn chunk_summary(chunk: &JavaChunk) -> ChunkSummary {
    let mut door_y: Option<i32> = None;
    let mut has_furniture = false;
    let y_range = chunk.y_range();
    // Constrain sweep to the typical above-ground band to keep this cheap;
    // doors and furniture are placed by `element_processing/doors.rs` and
    // friends near surface, never deep underground.
    let y_lo = y_range.start.max(-32) as i32;
    let y_hi = y_range.end.min(192) as i32;

    for y in y_lo..y_hi {
        for bx in 0..16 {
            for bz in 0..16 {
                let block = match chunk.block(bx, y as isize, bz) {
                    Some(b) => b,
                    None => continue,
                };
                let name = block.name();
                if is_door_block(name) {
                    door_y = Some(door_y.map_or(y, |existing| existing.min(y)));
                }
                if !has_furniture && is_furniture_block(name) {
                    has_furniture = true;
                }
                // Early-exit when we've answered both questions.
                if door_y.is_some() && has_furniture {
                    return ChunkSummary {
                        door_y,
                        has_furniture,
                    };
                }
            }
        }
    }

    ChunkSummary {
        door_y,
        has_furniture,
    }
}

/// At the y-level just below the door, require a horizontal run of at
/// least 4 contiguous non-air blocks somewhere in the chunk. This is a
/// loose proxy for "the building has a floor at door height" — strict
/// enough to fail a hollow shell, lenient enough not to fail on weird
/// floor-plan geometries the generator may emit.
fn has_floor_partition(chunk: &JavaChunk, door_y: i32) -> bool {
    let floor_y = (door_y - 1).max(-64);
    for bz in 0..16 {
        let mut run = 0usize;
        for bx in 0..16 {
            let solid = match chunk.block(bx, floor_y as isize, bz) {
                Some(b) => !is_airy(b.name()),
                None => false,
            };
            if solid {
                run += 1;
                if run >= 4 {
                    return true;
                }
            } else {
                run = 0;
            }
        }
    }
    // Same scan transposed (z-runs instead of x-runs) — building floors
    // can be axis-aligned either way.
    for bx in 0..16 {
        let mut run = 0usize;
        for bz in 0..16 {
            let solid = match chunk.block(bx, floor_y as isize, bz) {
                Some(b) => !is_airy(b.name()),
                None => false,
            };
            if solid {
                run += 1;
                if run >= 4 {
                    return true;
                }
            } else {
                run = 0;
            }
        }
    }
    false
}

pub(super) fn is_door_block(name: &str) -> bool {
    // Strip the namespace if present to be robust against future
    // generator changes (e.g. a custom namespace), and exclude trapdoors
    // because they're not an "entrance" signal.
    let bare = name.strip_prefix("minecraft:").unwrap_or(name);
    bare.ends_with("_door") && !bare.ends_with("_trapdoor")
}

pub(super) fn is_furniture_block(name: &str) -> bool {
    let bare = name.strip_prefix("minecraft:").unwrap_or(name);
    // Whitelist matches what `src/block_definitions.rs` palette emits as
    // of MIN-43; coordinate any additions with MIN-42.
    // IMPORTANT: glowstone is the primary ceiling-light block placed by
    // buildings_interior.rs (ceiling_abs-1) — it must be here or any
    // building that uses glowstone lighting will false-fail.
    let furniture_set: HashSet<&'static str> = [
        "crafting_table",
        "chest",
        "trapped_chest",
        "barrel",
        "bookshelf",
        "chiseled_bookshelf",
        "lectern",
        "smithing_table",
        "fletching_table",
        "cartography_table",
        "loom",
        "anvil",
        "furnace",
        "blast_furnace",
        "smoker",
        "campfire",
        "soul_campfire",
        "glowstone",
        "sea_lantern",
        "lantern",
        "soul_lantern",
        "torch",
        "wall_torch",
        "soul_torch",
        "redstone_torch",
        "ender_chest",
        "cauldron",
        "water_cauldron",
    ]
    .iter()
    .copied()
    .collect();

    if furniture_set.contains(bare) {
        return true;
    }
    bare.ends_with("_bed") || bare.ends_with("_carpet") || bare.ends_with("_candle")
}

fn is_airy(name: &str) -> bool {
    matches!(
        name,
        "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn door_matcher_accepts_known_doors() {
        assert!(is_door_block("minecraft:oak_door"));
        assert!(is_door_block("minecraft:dark_oak_door"));
        assert!(is_door_block("minecraft:spruce_door"));
        assert!(is_door_block("minecraft:iron_door"));
        // Stripped namespace also works (defensive).
        assert!(is_door_block("oak_door"));
    }

    #[test]
    fn door_matcher_rejects_trapdoors_and_non_doors() {
        assert!(!is_door_block("minecraft:oak_trapdoor"));
        assert!(!is_door_block("minecraft:dark_oak_trapdoor"));
        assert!(!is_door_block("minecraft:stone"));
        assert!(!is_door_block("minecraft:air"));
    }

    #[test]
    fn furniture_matcher_accepts_known_furniture() {
        assert!(is_furniture_block("minecraft:crafting_table"));
        assert!(is_furniture_block("minecraft:chest"));
        assert!(is_furniture_block("minecraft:bookshelf"));
        assert!(is_furniture_block("minecraft:red_bed"));
        assert!(is_furniture_block("minecraft:white_carpet"));
        assert!(is_furniture_block("minecraft:glowstone"));
        assert!(is_furniture_block("minecraft:cauldron"));
        assert!(is_furniture_block("minecraft:water_cauldron"));
        assert!(is_furniture_block("minecraft:sea_lantern"));
        assert!(is_furniture_block("minecraft:lantern"));
    }

    #[test]
    fn furniture_matcher_rejects_walls_and_floor_blocks() {
        assert!(!is_furniture_block("minecraft:stone"));
        assert!(!is_furniture_block("minecraft:oak_planks"));
        assert!(!is_furniture_block("minecraft:gray_concrete"));
        assert!(!is_furniture_block("minecraft:dirt"));
    }

    #[test]
    fn empty_region_list_passes() {
        let reasons = check(&ValidationConfig::default(), &[]).unwrap();
        assert!(reasons.is_empty());
    }
}
