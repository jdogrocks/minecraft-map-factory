use crate::block_definitions::{
    Block, BRICK, COBBLESTONE, CYAN_TERRACOTTA, DIRT, GRASS_BLOCK, GRAVEL, GRAY_CONCRETE_POWDER,
    OAK_PLANKS, PODZOL, RED_TERRACOTTA, SAND, TERRACOTTA,
};
use crate::osm_parser::ProcessedWay;

pub fn get_blocks_for_surface(surface_type: &str) -> Option<&'static [Block]> {
    match surface_type {
        "clay" => Some(&[TERRACOTTA]),
        "sand" => Some(&[SAND]),
        "tartan" => Some(&[RED_TERRACOTTA]),
        "grass" => Some(&[GRASS_BLOCK]),
        "dirt" | "ground" | "earth" => Some(&[DIRT]),
        "mulch" => Some(&[PODZOL]),
        "pebblestone" | "cobblestone" | "unhewn_cobblestone" => Some(&[COBBLESTONE]),
        // Paving-stones, sett and poured concrete roads render with the
        // same asphalt mix as `surface=asphalt`. Using the mix directly
        // (rather than a palette that also includes stone_bricks /
        // light_gray_concrete) is what guarantees these surfaces never
        // place L/S blocks that could later show up as islands inside
        // adjacent major roads — the road-overwrite blacklist already
        // protects the asphalt mix, so overlap resolves cleanly.
        "paving_stones" | "sett" => Some(&[GRAY_CONCRETE_POWDER, CYAN_TERRACOTTA]),
        "bricks" => Some(&[BRICK]),
        "wood" => Some(&[OAK_PLANKS]),
        "asphalt" => Some(&[GRAY_CONCRETE_POWDER, CYAN_TERRACOTTA]),
        "gravel" | "fine_gravel" => Some(&[GRAVEL]),
        "concrete" => Some(&[GRAY_CONCRETE_POWDER, CYAN_TERRACOTTA]),
        _ => None,
    }
}

/// Returns the block slice for a way's `surface=*` tag, or `default` when
/// the tag is missing or unknown. Takes and returns `&[Block]` so the hot
/// paths don't allocate — the tables in `get_blocks_for_surface` are all
/// `&'static [Block]`.
pub fn get_blocks_for_surface_way<'a>(way: &ProcessedWay, default: &'a [Block]) -> &'a [Block] {
    way.tags
        .get("surface")
        .and_then(|s| get_blocks_for_surface(s))
        .unwrap_or(default)
}

/// Pick a surface block deterministically from `block_types` based on
/// coordinates. The same `(x, z)` always returns the same block (so a
/// later overwrite pass sees a stable result), while adjacent cells
/// scatter across the palette for a varied, speckled look.
/// A 1-element slice effectively returns that single block everywhere.
#[inline]
pub fn semirandom_surface(x: i32, z: i32, block_types: &[Block]) -> Block {
    // Combine coordinates into a single value and apply bit mixing for a scattered look
    let mut h = (x as u32).wrapping_mul(0x9E3779B9) ^ (z as u32).wrapping_mul(0x517CC1B7);
    h ^= h >> 16;
    h = h.wrapping_mul(0x45D9F3B);
    h ^= h >> 16;
    block_types[(h as usize) % block_types.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn get_blocks_for_known_surfaces() {
        assert!(get_blocks_for_surface("clay").is_some());
        assert!(get_blocks_for_surface("sand").is_some());
        assert!(get_blocks_for_surface("grass").is_some());
        assert!(get_blocks_for_surface("dirt").is_some());
        assert!(get_blocks_for_surface("ground").is_some());
        assert!(get_blocks_for_surface("earth").is_some());
        assert!(get_blocks_for_surface("gravel").is_some());
        assert!(get_blocks_for_surface("fine_gravel").is_some());
        assert!(get_blocks_for_surface("asphalt").is_some());
        assert!(get_blocks_for_surface("concrete").is_some());
        assert!(get_blocks_for_surface("wood").is_some());
        assert!(get_blocks_for_surface("bricks").is_some());
        assert!(get_blocks_for_surface("cobblestone").is_some());
        assert!(get_blocks_for_surface("paving_stones").is_some());
        assert!(get_blocks_for_surface("sett").is_some());
        assert!(get_blocks_for_surface("tartan").is_some());
        assert!(get_blocks_for_surface("mulch").is_some());
        assert!(get_blocks_for_surface("pebblestone").is_some());
        assert!(get_blocks_for_surface("unhewn_cobblestone").is_some());
    }

    #[test]
    fn get_blocks_for_unknown_surface() {
        assert!(get_blocks_for_surface("magical_surface").is_none());
        assert!(get_blocks_for_surface("").is_none());
    }

    #[test]
    fn get_blocks_for_surface_way_with_tag() {
        let mut tags = HashMap::new();
        tags.insert("surface".to_string(), "sand".to_string());
        let way = ProcessedWay {
            id: 1,
            nodes: vec![],
            tags,
        };
        let default = &[DIRT];
        let result = get_blocks_for_surface_way(&way, default);
        assert_eq!(result, get_blocks_for_surface("sand").unwrap());
    }

    #[test]
    fn get_blocks_for_surface_way_without_tag() {
        let way = ProcessedWay {
            id: 1,
            nodes: vec![],
            tags: HashMap::new(),
        };
        let default = &[DIRT];
        let result = get_blocks_for_surface_way(&way, default);
        assert_eq!(result, default);
    }

    #[test]
    fn get_blocks_for_surface_way_unknown_tag() {
        let mut tags = HashMap::new();
        tags.insert("surface".to_string(), "unknown_surface".to_string());
        let way = ProcessedWay {
            id: 1,
            nodes: vec![],
            tags,
        };
        let default = &[DIRT];
        let result = get_blocks_for_surface_way(&way, default);
        assert_eq!(result, default);
    }

    #[test]
    fn semirandom_surface_deterministic() {
        let blocks = &[DIRT, SAND, GRAVEL];
        let b1 = semirandom_surface(10, 20, blocks);
        let b2 = semirandom_surface(10, 20, blocks);
        assert_eq!(b1, b2);
    }

    #[test]
    fn semirandom_surface_single_block() {
        let blocks = &[DIRT];
        let result = semirandom_surface(0, 0, blocks);
        assert_eq!(result, DIRT);
    }

    #[test]
    fn semirandom_surface_different_coords() {
        let blocks = &[DIRT, SAND, GRAVEL, COBBLESTONE];
        for x in 0..10 {
            for z in 0..10 {
                let b = semirandom_surface(x, z, blocks);
                assert!(blocks.contains(&b));
            }
        }
    }

    #[test]
    fn semirandom_surface_negative_coords() {
        let blocks = &[DIRT, SAND];
        let b = semirandom_surface(-100, -200, blocks);
        assert!(blocks.contains(&b));
    }
}
