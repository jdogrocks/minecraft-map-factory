//! Ground-continuity check (MIN-43 #1). The 20 floating maps in
//! `pipeline/output/published/` had rural chunks that were chunk-allocated
//! but never ground-filled — the column from y=-60 to surface was almost
//! entirely air. This check samples a configurable number of (x,z)
//! columns per region and verifies each has continuous non-air blocks
//! from `ground_y_min` up to surface, with a small tolerance for caves
//! and basements.
//!
//! Sampling strategy: deterministic stride across the region's 16×16×32×32
//! column grid so re-running the validator on the same .mca produces the
//! same column set. We pick the same indices regardless of map content,
//! which is what the regression run wants.

use super::anvil;
use super::anvil::RegionFile;
use crate::config::ValidationConfig;
use fastanvil::JavaChunk;
use tracing::debug;

const COLUMNS_PER_CHUNK: usize = 16 * 16;
const CHUNKS_PER_REGION: usize = 32 * 32;

pub fn check(
    config: &ValidationConfig,
    region_files: &[RegionFile],
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut reasons = Vec::new();
    if region_files.is_empty() {
        return Ok(reasons);
    }

    let columns_per_region = config.ground_sample_columns_per_region.max(1);
    let stride = (CHUNKS_PER_REGION * COLUMNS_PER_CHUNK / columns_per_region).max(1);

    let mut total_sampled = 0usize;
    let mut total_failed = 0usize;
    let mut first_failure_examples: Vec<String> = Vec::new();

    for rf in region_files {
        // Build the deterministic per-region column index list.
        let global_indices: Vec<usize> = (0..columns_per_region)
            .map(|i| (i * stride) % (CHUNKS_PER_REGION * COLUMNS_PER_CHUNK))
            .collect();

        // Group by chunk so we only parse each chunk once even if multiple
        // sample columns fall in it.
        let mut by_chunk: std::collections::BTreeMap<(usize, usize), Vec<(usize, usize)>> =
            std::collections::BTreeMap::new();
        for &gi in &global_indices {
            let chunk_idx = gi / COLUMNS_PER_CHUNK;
            let col_idx = gi % COLUMNS_PER_CHUNK;
            let cx = chunk_idx % 32;
            let cz = chunk_idx / 32;
            let bx = col_idx % 16;
            let bz = col_idx / 16;
            by_chunk.entry((cx, cz)).or_default().push((bx, bz));
        }

        anvil::for_each_chunk(rf, |cx, cz, chunk| {
            let Some(cols) = by_chunk.get(&(cx, cz)) else {
                return;
            };
            for &(bx, bz) in cols {
                total_sampled += 1;
                if let Some(reason) = column_failure(chunk, bx, bz, config, rf, cx, cz) {
                    total_failed += 1;
                    if first_failure_examples.len() < 3 {
                        first_failure_examples.push(reason);
                    }
                }
            }
        })?;
    }

    debug!(
        total_sampled,
        total_failed, "Ground-continuity sampling complete"
    );

    if total_failed > 0 {
        let summary = format!(
            "ground_discontinuity: {}/{} sampled columns have an air gap below surface from y={} \
             (max allowed gap: {} blocks). Examples: {}",
            total_failed,
            total_sampled,
            config.ground_y_min,
            config.ground_max_air_gap_blocks,
            first_failure_examples.join("; ")
        );
        reasons.push(summary);
    }

    Ok(reasons)
}

/// Returns `Some(reason)` if the column at chunk-local (bx, bz) violates
/// the ground-continuity rule. The rule: walking from `y_min` up to the
/// reported surface height, the total air-block count must not exceed
/// `ground_max_air_gap_blocks`. Floating buildings (the failure mode we
/// are catching) put hundreds of air blocks here.
fn column_failure(
    chunk: &JavaChunk,
    bx: usize,
    bz: usize,
    config: &ValidationConfig,
    rf: &RegionFile,
    cx: usize,
    cz: usize,
) -> Option<String> {
    let surface = anvil::surface_height(chunk, bx, bz)?;
    if surface <= config.ground_y_min {
        // Below sea-floor or empty column — surface should be above the
        // ground floor. If not, treat the whole column as missing.
        return Some(format!(
            "{} chunk=({},{}) col=({},{}) surface={} <= y_min",
            rf.path.display(),
            cx,
            cz,
            bx,
            bz,
            surface
        ));
    }

    let names = anvil::column_block_names(chunk, bx, bz, config.ground_y_min, surface);
    let mut air_blocks = 0usize;
    for name in &names {
        let is_air = match name.as_deref() {
            None => true,
            Some(n) => is_airy(n),
        };
        if is_air {
            air_blocks += 1;
        }
    }

    if air_blocks > config.ground_max_air_gap_blocks {
        let world_x = (rf.rx as i64) * 512 + (cx as i64) * 16 + bx as i64;
        let world_z = (rf.rz as i64) * 512 + (cz as i64) * 16 + bz as i64;
        return Some(format!(
            "world=({},{}) air={} surface={}",
            world_x, world_z, air_blocks, surface
        ));
    }

    None
}

/// Block names treated as air for the purpose of ground continuity.
/// Matches fastanvil's `BlockArchetype::Airy` set.
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
    fn is_airy_recognises_all_air_variants() {
        assert!(is_airy("minecraft:air"));
        assert!(is_airy("minecraft:cave_air"));
        assert!(is_airy("minecraft:void_air"));
        assert!(!is_airy("minecraft:stone"));
        assert!(!is_airy("minecraft:water"));
    }

    #[test]
    fn empty_region_list_passes() {
        let reasons = check(&ValidationConfig::default(), &[]).unwrap();
        assert!(reasons.is_empty());
    }
}
