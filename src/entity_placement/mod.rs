pub mod theme;

use crate::element_processing::buildings::BuildingCategory;
use crate::world_editor::WorldEditor;
use theme::ThemePack;

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
                // Place entity 1 block above the floor level (ground-relative Y)
                let y_offset = floor_y + 1;
                editor.add_entity(&entry.id, x, y_offset, z, None);
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
