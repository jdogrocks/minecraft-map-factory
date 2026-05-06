//! Surface-diversity check (MIN-43 #4). A real map mixes surface blocks
//! (grass, dirt, stone, water, sand, asphalt) across its bbox. The
//! degenerate "everything is a road stripe with air below" failure mode
//! shows up as 1–2 distinct surface block names. We enforce a tunable
//! minimum count of distinct surface block types across sampled chunks.
//!
//! "Surface" is defined as the topmost non-air block at each (x,z) — we
//! ask fastanvil for the heightmap-driven surface_height per column and
//! read the block at that y.

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

    let target_samples = config.surface_diversity_sample_chunks.max(1);
    let per_region = target_samples.div_ceil(region_files.len());
    let stride = (1024 / per_region.max(1)).max(1);

    let mut distinct: HashSet<String> = HashSet::new();
    let mut sampled_chunks = 0usize;

    for rf in region_files {
        let mut taken_in_region = 0usize;
        anvil::for_each_chunk(rf, |cx, cz, chunk| {
            let chunk_idx = cz * 32 + cx;
            if chunk_idx % stride != 0 {
                return;
            }
            if taken_in_region >= per_region {
                return;
            }
            taken_in_region += 1;
            sampled_chunks += 1;
            collect_surface_blocks(chunk, &mut distinct);
        })?;
    }

    debug!(
        sampled_chunks,
        distinct_count = distinct.len(),
        "Surface-diversity sampling complete"
    );

    if distinct.len() < config.surface_diversity_min_distinct {
        let mut listed: Vec<String> = distinct.into_iter().collect();
        listed.sort();
        reasons.push(format!(
            "surface_diversity_low: {} distinct surface block types across {} sampled chunks \
             (minimum: {}). Found: {}",
            listed.len(),
            sampled_chunks,
            config.surface_diversity_min_distinct,
            listed.join(", ")
        ));
    }

    Ok(reasons)
}

fn collect_surface_blocks(chunk: &JavaChunk, into: &mut HashSet<String>) {
    for bx in 0..16 {
        for bz in 0..16 {
            // surface_height() returns the y of the first air-like block *above*
            // the surface (Minecraft's WORLD_SURFACE convention). The actual
            // surface block is one below that.
            let surface_y = chunk.surface_height(bx, bz, fastanvil::HeightMode::Calculate);
            let block_y = surface_y - 1;
            if let Some(block) = chunk.block(bx, block_y, bz) {
                let name = block.name();
                if !is_airy(name) {
                    into.insert(name.to_string());
                }
            }
        }
    }
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
    fn empty_region_list_passes() {
        let reasons = check(&ValidationConfig::default(), &[]).unwrap();
        assert!(reasons.is_empty());
    }
}
