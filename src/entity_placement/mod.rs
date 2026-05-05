pub mod theme;

use crate::element_processing::buildings::BuildingCategory;
use crate::world_editor::WorldEditor;
use theme::ThemePack;

/// Context for outdoor entity placement, determining entity types and density.
#[derive(Copy, Clone)]
pub enum OutdoorContext {
    /// Open park / garden / nature_reserve areas: wolves (w=40), cats (w=30), villagers (w=30).
    Park,
    /// Pedestrian footways and paths: villagers (w=60), wolves (w=40).
    Footway,
    /// Outdoor yards adjacent to detached / residential houses: cats (w=50), wolves (w=50).
    ResidentialYard,
}

/// Place entities in outdoor areas using deterministic seeding.
///
/// Mirrors the seeding strategy of `place_building_entities` for cross-run reproducibility.
pub fn place_outdoor_entities(
    editor: &mut WorldEditor,
    positions: &[(i32, i32)],
    context: OutdoorContext,
    seed: u64,
) {
    if positions.is_empty() {
        return;
    }

    // Cumulative weight table: (entity_id, cumulative_weight) summing to 100
    let entities: &[(&str, u64)] = match context {
        OutdoorContext::Park => &[
            ("minecraft:wolf", 40),
            ("minecraft:cat", 70),       // 40+30
            ("minecraft:villager", 100), // 40+30+30
        ],
        OutdoorContext::Footway => &[
            ("minecraft:villager", 60),
            ("minecraft:wolf", 100), // 60+40
        ],
        OutdoorContext::ResidentialYard => &[
            ("minecraft:cat", 50),
            ("minecraft:wolf", 100), // 50+50
        ],
    };

    // How many entities to target for this area
    let max_count: usize = match context {
        OutdoorContext::Park => (positions.len() / 15).max(1),
        OutdoorContext::Footway => (positions.len() / 8).max(1),
        OutdoorContext::ResidentialYard => (seed.wrapping_mul(2654435761) % 3) as usize,
    };

    if max_count == 0 {
        return;
    }

    let step = (positions.len() / (max_count + 1)).max(1);
    for (i, &(x, z)) in positions.iter().enumerate() {
        if i % step != step / 2 {
            continue;
        }
        let entity_seed = seed.wrapping_mul(2654435761).wrapping_add(i as u64);
        let roll = entity_seed % 100;
        for &(entity_id, cum_weight) in entities {
            if roll < cum_weight {
                editor.add_entity(entity_id, x, 1, z, None);
                break;
            }
        }
    }
}

/// Maps a BuildingCategory to a theme context string.
pub fn category_to_context(category: BuildingCategory) -> &'static str {
    match category {
        BuildingCategory::House | BuildingCategory::Residential => "residential",
        BuildingCategory::Farm => "farm",
        BuildingCategory::Commercial | BuildingCategory::Hotel => "commercial",
        BuildingCategory::Office | BuildingCategory::TallBuilding => "commercial",
        BuildingCategory::GlassySkyscraper | BuildingCategory::ModernSkyscraper => "commercial",
        BuildingCategory::School | BuildingCategory::Hospital => "public",
        BuildingCategory::Religious => "religious",
        BuildingCategory::Industrial | BuildingCategory::Warehouse => "industrial",
        BuildingCategory::Historic | BuildingCategory::Tower => "public",
        BuildingCategory::Garage | BuildingCategory::Shed | BuildingCategory::Greenhouse => {
            "residential"
        }
        BuildingCategory::Default => "residential",
    }
}

/// Place entities inside a building based on its category and the active theme pack.
///
/// Entities are placed at open interior positions on each floor level.
/// Uses deterministic seeding based on building ID and coordinates
/// for reproducible placement.
pub fn place_building_entities(
    editor: &mut WorldEditor,
    theme: &ThemePack,
    category: BuildingCategory,
    floor_area: &[(i32, i32)],
    floor_levels: &[i32],
    building_id: u64,
) {
    let context = category_to_context(category);
    let max_entities = theme.max_per_floor(context);
    if max_entities == 0 {
        return;
    }

    // Skip very small buildings
    if floor_area.len() < 20 {
        return;
    }

    for (floor_idx, &floor_y) in floor_levels.iter().enumerate() {
        // Deterministic seed per floor
        let floor_seed = building_id
            .wrapping_mul(2654435761)
            .wrapping_add(floor_idx as u64);

        // Pick spawn positions from interior area — sample every N tiles
        let step = (floor_area.len() / (max_entities as usize + 1)).max(1);
        let mut placed = 0u32;

        for (i, &(x, z)) in floor_area.iter().enumerate() {
            if placed >= max_entities {
                break;
            }
            // Deterministic spacing: pick positions that are well-distributed
            if i % step != step / 2 {
                continue;
            }

            let entity_seed = floor_seed.wrapping_add(i as u64);
            if let Some(entry) = theme.select_entity(context, entity_seed) {
                editor.add_entity_absolute(&entry.id, x, floor_y + 1, z, None);
                placed += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_to_context() {
        assert_eq!(category_to_context(BuildingCategory::House), "residential");
        assert_eq!(
            category_to_context(BuildingCategory::Residential),
            "residential"
        );
        assert_eq!(category_to_context(BuildingCategory::Farm), "farm");
        assert_eq!(
            category_to_context(BuildingCategory::Commercial),
            "commercial"
        );
        assert_eq!(category_to_context(BuildingCategory::Office), "commercial");
        assert_eq!(category_to_context(BuildingCategory::School), "public");
        assert_eq!(category_to_context(BuildingCategory::Hospital), "public");
        assert_eq!(
            category_to_context(BuildingCategory::Religious),
            "religious"
        );
        assert_eq!(
            category_to_context(BuildingCategory::Industrial),
            "industrial"
        );
    }
}
