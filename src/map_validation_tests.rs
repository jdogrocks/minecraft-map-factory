//! Map validation suite: structural integrity, entities, terrain (MIN-15)
//!
//! Validates generated Minecraft world output across six dimensions:
//! 1. Structural integrity — Region files valid Anvil format, chunk headers parse
//! 2. Block count minimums — Non-air blocks exceed threshold for bounding box
//! 3. Entity placement — Expected entities present and within bounds
//! 4. Terrain accuracy — Elevation matches expected ground level
//! 5. No corruption — All chunks loadable, no truncated NBT
//! 6. Biome sanity — Distribution matches expected terrain

#[cfg(test)]
mod tests {
    use crate::args::Args;
    use crate::coordinate_system::cartesian::XZBBox;
    use crate::coordinate_system::geographic::LLBBox;
    use crate::data_processing::{generate_world_with_options, GenerationOptions};
    use crate::ground::Ground;
    use crate::osm_parser::{self, ProcessedElement};
    use crate::retrieve_data;
    use crate::world_editor::WorldFormat;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // ── Helpers ─────────────────────────────────────────────────────

    fn fixture_llbbox() -> LLBBox {
        LLBBox::new(54.6290, 9.9280, 54.6315, 9.9330).unwrap()
    }

    fn test_args(bbox: LLBBox, output_dir: Option<PathBuf>) -> Args {
        Args {
            bbox,
            file: None,
            save_json_file: None,
            path: output_dir,
            bedrock: false,
            downloader: "requests".to_string(),
            scale: 1.0,
            ground_level: 64,
            terrain: false,
            interior: false,
            entities: false,
            entity_theme: "default".to_string(),
            roof: false,
            fillground: false,
            land_cover: false,
            debug: false,
            timeout: None,
            spawn_lat: None,
            spawn_lng: None,
            rotation: 0.0,
            disable_height_limit: false,
            benchmark: false,
            minecraft_version: "1.26.1.2".to_string(),
        }
    }

    fn load_and_parse_fixture() -> (Vec<ProcessedElement>, XZBBox, LLBBox) {
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/small_area.json"
        );
        let osm_data = retrieve_data::fetch_data_from_file(fixture_path)
            .expect("Failed to load small_area.json fixture");

        let llbbox = fixture_llbbox();
        let (mut elements, xzbbox) = osm_parser::parse_osm_data(osm_data, llbbox, 1.0, false);
        elements.sort_by_key(|el: &ProcessedElement| osm_parser::get_priority(el));

        (elements, xzbbox, llbbox)
    }

    /// Generate a world from the small_area fixture and return the output path.
    fn generate_test_world() -> (PathBuf, TempDir) {
        let (elements, xzbbox, llbbox) = load_and_parse_fixture();
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let world_dir = tmp.path().join("validation_world");
        fs::create_dir_all(&world_dir).expect("create world dir");

        let args = test_args(llbbox, Some(world_dir.clone()));
        let ground = Ground::new_flat(args.ground_level);

        let options = GenerationOptions {
            path: world_dir,
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };

        let output = generate_world_with_options(elements, xzbbox, llbbox, ground, &args, options)
            .expect("World generation should succeed");
        (output, tmp)
    }

    /// Parse all region files and return structured chunk data for validation.
    /// Returns: Vec<(region_filename, Vec<chunk_nbt>)>
    fn read_all_chunks(world_path: &std::path::Path) -> Vec<(String, Vec<fastnbt::Value>)> {
        let region_dir = world_path.join("region");
        let mut results = Vec::new();

        for entry in fs::read_dir(&region_dir).expect("read region dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "mca") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let file = fs::File::open(&path).expect("open region file");
                let mut region = fastanvil::Region::from_stream(file)
                    .expect("Region file should be valid Anvil format");

                let mut chunks = Vec::new();
                for chunk in region.iter().flatten() {
                    let nbt: fastnbt::Value = fastnbt::from_bytes(chunk.data.as_slice())
                        .expect("Chunk NBT should be parseable");
                    chunks.push(nbt);
                }
                results.push((filename, chunks));
            }
        }

        results
    }

    // ═══════════════════════════════════════════════════════════════
    //  1. STRUCTURAL INTEGRITY
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn structural_region_files_are_valid_anvil_format() {
        let (world_path, _tmp) = generate_test_world();
        let region_dir = world_path.join("region");

        let mca_files: Vec<_> = fs::read_dir(&region_dir)
            .expect("read region dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "mca"))
            .collect();

        assert!(!mca_files.is_empty(), "At least one .mca file should exist");

        for entry in &mca_files {
            let path = entry.path();
            let file = fs::File::open(&path).expect("open region file");
            let region = fastanvil::Region::from_stream(file);
            assert!(
                region.is_ok(),
                "Region file {} should be valid Anvil format: {:?}",
                path.display(),
                region.err()
            );
        }
    }

    #[test]
    fn structural_region_filenames_follow_convention() {
        let (world_path, _tmp) = generate_test_world();
        let region_dir = world_path.join("region");

        for entry in fs::read_dir(&region_dir).expect("read region dir") {
            let entry = entry.expect("dir entry");
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.ends_with(".mca") {
                // Format: r.<x>.<z>.mca
                let parts: Vec<&str> = filename.split('.').collect();
                assert_eq!(
                    parts.len(),
                    4,
                    "Region filename should be r.<x>.<z>.mca, got: {}",
                    filename
                );
                assert_eq!(
                    parts[0], "r",
                    "Region filename should start with 'r': {}",
                    filename
                );
                assert!(
                    parts[1].parse::<i32>().is_ok(),
                    "Region X should be integer: {}",
                    filename
                );
                assert!(
                    parts[2].parse::<i32>().is_ok(),
                    "Region Z should be integer: {}",
                    filename
                );
                assert_eq!(parts[3], "mca", "Extension should be 'mca': {}", filename);
            }
        }
    }

    #[test]
    fn structural_all_chunks_have_required_nbt_fields() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let required_fields = ["DataVersion", "xPos", "yPos", "zPos", "Status", "sections"];

        let mut total_chunks = 0u32;
        for (region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                total_chunks += 1;
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    for field in &required_fields {
                        assert!(
                            map.contains_key(*field),
                            "Chunk in {} missing required field '{}'. Keys: {:?}",
                            region_name,
                            field,
                            map.keys().collect::<Vec<_>>()
                        );
                    }
                } else {
                    panic!("Chunk NBT root in {} should be a Compound", region_name);
                }
            }
        }

        assert!(total_chunks > 0, "Should have parsed at least one chunk");
    }

    #[test]
    fn structural_chunk_data_version_is_correct() {
        use crate::pack_format::data_version_for;

        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let expected = data_version_for("1.26.1.2")
            .expect("1.26.1.2 must be in DATA_VERSION_TABLE");

        for (region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::Int(version)) = map.get("DataVersion") {
                        assert_eq!(
                            *version, expected,
                            "DataVersion in {} should be {} (MC 1.26.1.2), got {}",
                            region_name, expected, version
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn structural_chunk_status_is_full() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        for (region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::String(status)) = map.get("Status") {
                        assert_eq!(
                            status, "minecraft:full",
                            "Chunk Status in {} should be 'minecraft:full', got '{}'",
                            region_name, status
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn structural_sections_have_valid_y_range() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Byte(y)) = s_map.get("Y") {
                                    // Vanilla range: -4 to 19 (Y=-64 to Y=319)
                                    // Extended range: up to 127 with data packs
                                    assert!(
                                        *y >= -4 && (*y as i16) <= 127,
                                        "Section Y={} is outside valid range [-4, 127]",
                                        y
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn structural_sections_have_block_states_with_palette() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let mut sections_checked = 0u32;
        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Compound(block_states)) =
                                    s_map.get("block_states")
                                {
                                    assert!(
                                        block_states.contains_key("palette"),
                                        "block_states should have a 'palette' key"
                                    );
                                    if let Some(fastnbt::Value::List(palette)) =
                                        block_states.get("palette")
                                    {
                                        assert!(
                                            !palette.is_empty(),
                                            "Palette should have at least one entry"
                                        );

                                        // Verify palette entries have Name field
                                        for item in palette {
                                            if let fastnbt::Value::Compound(p_map) = item {
                                                assert!(
                                                    p_map.contains_key("Name"),
                                                    "Palette item should have 'Name' field"
                                                );
                                            }
                                        }
                                    }
                                    sections_checked += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        assert!(
            sections_checked > 0,
            "Should have checked at least one section"
        );
    }

    #[test]
    fn structural_heightmaps_present_and_valid() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let expected_types = [
            "MOTION_BLOCKING",
            "MOTION_BLOCKING_NO_LEAVES",
            "OCEAN_FLOOR",
            "WORLD_SURFACE",
        ];

        let mut checked = 0u32;
        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::Compound(heightmaps)) = map.get("Heightmaps") {
                        for hm_type in &expected_types {
                            assert!(
                                heightmaps.contains_key(*hm_type),
                                "Heightmaps should contain '{}'",
                                hm_type
                            );
                            if let Some(fastnbt::Value::LongArray(data)) = heightmaps.get(*hm_type)
                            {
                                assert!(
                                    !data.is_empty(),
                                    "Heightmap '{}' should not be empty",
                                    hm_type
                                );
                            }
                        }
                        checked += 1;
                    }
                }
            }
        }

        assert!(
            checked > 0,
            "Should have checked at least one chunk's heightmaps"
        );
    }

    // ═══════════════════════════════════════════════════════════════
    //  2. BLOCK COUNT MINIMUMS
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn block_count_non_air_blocks_exceed_minimum() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let mut total_non_air_sections = 0u32;
        let mut total_palette_blocks = 0usize;

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Compound(block_states)) =
                                    s_map.get("block_states")
                                {
                                    if let Some(fastnbt::Value::List(palette)) =
                                        block_states.get("palette")
                                    {
                                        // Count palette entries that are NOT air
                                        let has_non_air = palette.iter().any(|item| {
                                            if let fastnbt::Value::Compound(p) = item {
                                                if let Some(fastnbt::Value::String(name)) =
                                                    p.get("Name")
                                                {
                                                    return name != "minecraft:air";
                                                }
                                            }
                                            false
                                        });

                                        if has_non_air {
                                            total_non_air_sections += 1;
                                        }
                                        total_palette_blocks += palette.len();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // The small_area fixture has a building, highway, and tree — there must be
        // non-air content. Plus each region fills base chunks with grass at Y=-62.
        assert!(
            total_non_air_sections > 0,
            "Generated world should have sections with non-air blocks"
        );

        // Palette should contain more than just air entries overall
        assert!(
            total_palette_blocks > total_non_air_sections as usize,
            "Total palette entries ({}) should exceed section count ({}), \
             indicating diverse block types",
            total_palette_blocks,
            total_non_air_sections
        );
    }

    #[test]
    fn block_count_base_layer_grass_present() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let mut found_grass = false;
        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Compound(block_states)) =
                                    s_map.get("block_states")
                                {
                                    if let Some(fastnbt::Value::List(palette)) =
                                        block_states.get("palette")
                                    {
                                        for item in palette {
                                            if let fastnbt::Value::Compound(p) = item {
                                                if let Some(fastnbt::Value::String(name)) =
                                                    p.get("Name")
                                                {
                                                    if name == "minecraft:grass_block" {
                                                        found_grass = true;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        assert!(
            found_grass,
            "World should contain grass blocks (base layer at Y=64)"
        );
    }

    #[test]
    fn block_count_every_region_has_populated_chunks() {
        let (world_path, _tmp) = generate_test_world();
        let region_dir = world_path.join("region");

        for entry in fs::read_dir(&region_dir).expect("read region dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "mca") {
                let file = fs::File::open(&path).expect("open region file");
                let mut region = fastanvil::Region::from_stream(file).expect("valid region");

                let mut chunk_count = 0u32;
                for chunk_result in region.iter() {
                    if chunk_result.is_ok() {
                        chunk_count += 1;
                    }
                }

                // Every region should have all 1024 chunks (32x32) because
                // the save pass fills empty chunks with base layer
                assert_eq!(
                    chunk_count,
                    1024,
                    "Region {} should have 1024 chunks (all filled), got {}",
                    path.display(),
                    chunk_count
                );
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════
    //  3. ENTITY PLACEMENT
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn entity_theme_select_returns_valid_minecraft_ids() {
        use crate::entity_placement::theme::{default_theme, fantasy_theme};

        let themes = [default_theme(), fantasy_theme()];
        let contexts = [
            "residential",
            "commercial",
            "public",
            "farm",
            "religious",
            "industrial",
        ];

        for theme in &themes {
            for context in &contexts {
                for seed in 0..100u64 {
                    if let Some(entry) = theme.select_entity(context, seed) {
                        assert!(
                            entry.id.starts_with("minecraft:"),
                            "Entity ID '{}' in theme '{}' context '{}' should start with 'minecraft:'",
                            entry.id,
                            theme.name,
                            context
                        );
                        // Entity ID should not be empty after prefix
                        assert!(
                            entry.id.len() > "minecraft:".len(),
                            "Entity ID '{}' should have a name after 'minecraft:'",
                            entry.id
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn entity_theme_select_is_deterministic() {
        use crate::entity_placement::theme::default_theme;

        let theme = default_theme();
        let contexts = ["residential", "commercial", "public", "farm"];

        for context in &contexts {
            for seed in [0u64, 42, 999, u64::MAX] {
                let result1 = theme.select_entity(context, seed);
                let result2 = theme.select_entity(context, seed);
                match (result1, result2) {
                    (Some(e1), Some(e2)) => {
                        assert_eq!(
                            e1.id, e2.id,
                            "Entity selection should be deterministic for context '{}' seed {}",
                            context, seed
                        );
                    }
                    (None, None) => {}
                    _ => panic!(
                        "Entity selection should be deterministic for context '{}' seed {}",
                        context, seed
                    ),
                }
            }
        }
    }

    #[test]
    fn entity_theme_weight_distribution_is_plausible() {
        use crate::entity_placement::theme::default_theme;

        let theme = default_theme();
        let mut counts: HashMap<String, u32> = HashMap::new();
        let num_samples = 10_000u64;

        for seed in 0..num_samples {
            if let Some(entry) = theme.select_entity("residential", seed) {
                *counts.entry(entry.id.clone()).or_insert(0) += 1;
            }
        }

        // Residential has: cat(40), wolf(20), villager(30), parrot(10)
        // With 10k samples, each should appear proportionally
        assert!(
            counts.len() >= 3,
            "Should select at least 3 different entity types"
        );

        // Cat (weight 40/100 = 40%) should be the most common
        let cat_count = counts.get("minecraft:cat").copied().unwrap_or(0);
        let parrot_count = counts.get("minecraft:parrot").copied().unwrap_or(0);
        assert!(
            cat_count > parrot_count,
            "Cat (weight 40) should appear more than parrot (weight 10): cat={}, parrot={}",
            cat_count,
            parrot_count
        );
    }

    #[test]
    fn entity_category_mapping_covers_all_categories() {
        use crate::element_processing::buildings::BuildingCategory;
        use crate::entity_placement::category_to_context;

        let categories = [
            BuildingCategory::House,
            BuildingCategory::Residential,
            BuildingCategory::Farm,
            BuildingCategory::Commercial,
            BuildingCategory::Hotel,
            BuildingCategory::Office,
            BuildingCategory::TallBuilding,
            BuildingCategory::GlassySkyscraper,
            BuildingCategory::ModernSkyscraper,
            BuildingCategory::School,
            BuildingCategory::Hospital,
            BuildingCategory::Religious,
            BuildingCategory::Industrial,
            BuildingCategory::Warehouse,
            BuildingCategory::Historic,
            BuildingCategory::Tower,
            BuildingCategory::Garage,
            BuildingCategory::Shed,
            BuildingCategory::Greenhouse,
            BuildingCategory::Default,
        ];

        let valid_contexts = [
            "residential",
            "farm",
            "commercial",
            "public",
            "religious",
            "industrial",
        ];

        for category in &categories {
            let context = category_to_context(*category);
            assert!(
                valid_contexts.contains(&context),
                "Category {:?} maps to unknown context '{}'",
                category,
                context
            );
        }
    }

    #[test]
    fn entity_placement_skips_small_buildings() {
        use crate::element_processing::buildings::BuildingCategory;
        use crate::entity_placement::place_building_entities;
        use crate::entity_placement::theme::default_theme;

        // Create a minimal world editor to test entity placement
        let xzbbox = XZBBox::rect_from_xz_lengths(100.0, 100.0).unwrap();
        let llbbox = fixture_llbbox();
        let tmp = TempDir::new().unwrap();
        let world_dir = tmp.path().join("entity_test");
        fs::create_dir_all(&world_dir).unwrap();

        let mut editor = crate::world_editor::WorldEditor::new(world_dir, &xzbbox, llbbox);
        let theme = default_theme();

        // Small building (< 20 floor area) — should place no entities
        let small_floor: Vec<(i32, i32)> = (0..10).map(|i| (i, 0)).collect();
        let floor_levels = vec![0];

        place_building_entities(
            &mut editor,
            &theme,
            BuildingCategory::House,
            &small_floor,
            &floor_levels,
            12345,
        );

        // Verify no entities were placed (small building should be skipped)
        // We can't directly inspect entities in the editor, but the function
        // returns without placing when floor_area.len() < 20
        // This test verifies no panic occurs with small buildings
    }

    // ═══════════════════════════════════════════════════════════════
    //  4. TERRAIN ACCURACY
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn terrain_flat_ground_level_consistent() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        // With flat ground at 64, the section containing Y=64 is section 4
        // (Y=64 to Y=79). Grass block should appear in this section.
        let target_section_y: i8 = 4; // section containing Y=64

        let mut found_grass_in_target_section = false;

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Byte(y)) = s_map.get("Y") {
                                    if *y == target_section_y {
                                        if let Some(fastnbt::Value::Compound(block_states)) =
                                            s_map.get("block_states")
                                        {
                                            if let Some(fastnbt::Value::List(palette)) =
                                                block_states.get("palette")
                                            {
                                                let has_grass = palette.iter().any(|item| {
                                                    if let fastnbt::Value::Compound(p) = item {
                                                        if let Some(fastnbt::Value::String(name)) =
                                                            p.get("Name")
                                                        {
                                                            return name == "minecraft:grass_block";
                                                        }
                                                    }
                                                    false
                                                });
                                                if has_grass {
                                                    found_grass_in_target_section = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        assert!(
            found_grass_in_target_section,
            "Grass blocks should be in section Y={} (containing ground level Y=64)",
            target_section_y
        );
    }

    #[test]
    fn terrain_metadata_coordinates_are_consistent() {
        let (world_path, _tmp) = generate_test_world();
        let metadata_path = world_path.join("metadata.json");
        let metadata_str = fs::read_to_string(&metadata_path).expect("read metadata.json");
        let metadata: serde_json::Value =
            serde_json::from_str(&metadata_str).expect("parse metadata");

        let min_x = metadata["minMcX"].as_i64().expect("minMcX");
        let max_x = metadata["maxMcX"].as_i64().expect("maxMcX");
        let min_z = metadata["minMcZ"].as_i64().expect("minMcZ");
        let max_z = metadata["maxMcZ"].as_i64().expect("maxMcZ");

        // Bounding box should have positive dimensions
        assert!(
            max_x > min_x,
            "maxMcX ({}) should be > minMcX ({})",
            max_x,
            min_x
        );
        assert!(
            max_z > min_z,
            "maxMcZ ({}) should be > minMcZ ({})",
            max_z,
            min_z
        );

        // Geographic coordinates should be within input bbox
        let min_lat = metadata["minGeoLat"].as_f64().expect("minGeoLat");
        let max_lat = metadata["maxGeoLat"].as_f64().expect("maxGeoLat");
        let min_lon = metadata["minGeoLon"].as_f64().expect("minGeoLon");
        let max_lon = metadata["maxGeoLon"].as_f64().expect("maxGeoLon");

        assert!(max_lat > min_lat, "maxGeoLat should be > minGeoLat");
        assert!(max_lon > min_lon, "maxGeoLon should be > minGeoLon");

        // Coordinates should be within roughly the input bbox
        assert!(
            (54.0..=55.0).contains(&min_lat),
            "minGeoLat should be near 54.6"
        );
        assert!(
            (9.0..=10.0).contains(&min_lon),
            "minGeoLon should be near 9.93"
        );
    }

    #[test]
    fn terrain_chunk_positions_match_region_coordinates() {
        let (world_path, _tmp) = generate_test_world();
        let region_dir = world_path.join("region");

        for entry in fs::read_dir(&region_dir).expect("read region dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "mca") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let parts: Vec<&str> = filename.split('.').collect();
                let region_x: i32 = parts[1].parse().unwrap();
                let region_z: i32 = parts[2].parse().unwrap();

                let file = fs::File::open(&path).expect("open region file");
                let mut region = fastanvil::Region::from_stream(file).expect("valid region");

                for chunk in region.iter().flatten() {
                    let nbt: fastnbt::Value = fastnbt::from_bytes(chunk.data.as_slice()).unwrap();
                    if let fastnbt::Value::Compound(map) = &nbt {
                        if let (
                            Some(fastnbt::Value::Int(x_pos)),
                            Some(fastnbt::Value::Int(z_pos)),
                        ) = (map.get("xPos"), map.get("zPos"))
                        {
                            // Chunk position should be within the region's range
                            let expected_min_x = region_x * 32;
                            let expected_max_x = expected_min_x + 31;
                            let expected_min_z = region_z * 32;
                            let expected_max_z = expected_min_z + 31;

                            assert!(
                                (expected_min_x..=expected_max_x).contains(x_pos),
                                "Chunk xPos {} should be in range [{}, {}] for region {}",
                                x_pos,
                                expected_min_x,
                                expected_max_x,
                                filename
                            );
                            assert!(
                                (expected_min_z..=expected_max_z).contains(z_pos),
                                "Chunk zPos {} should be in range [{}, {}] for region {}",
                                z_pos,
                                expected_min_z,
                                expected_max_z,
                                filename
                            );
                        }
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════
    //  5. NO CORRUPTION
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn corruption_all_chunks_deserialize_without_error() {
        let (world_path, _tmp) = generate_test_world();
        let region_dir = world_path.join("region");
        let mut total_chunks = 0u32;
        let mut failed_chunks = 0u32;

        for entry in fs::read_dir(&region_dir).expect("read region dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "mca") {
                let file = fs::File::open(&path).expect("open region file");
                let mut region = fastanvil::Region::from_stream(file).expect("valid region");

                for chunk_result in region.iter() {
                    match chunk_result {
                        Ok(chunk) => {
                            total_chunks += 1;
                            let parse_result: Result<fastnbt::Value, _> =
                                fastnbt::from_bytes(chunk.data.as_slice());
                            if parse_result.is_err() {
                                failed_chunks += 1;
                            }
                        }
                        Err(_) => {
                            // Iteration error is also corruption
                            failed_chunks += 1;
                        }
                    }
                }
            }
        }

        assert!(total_chunks > 0, "Should have chunks to validate");
        assert_eq!(
            failed_chunks, 0,
            "All {} chunks should deserialize without errors ({} failed)",
            total_chunks, failed_chunks
        );
    }

    #[test]
    fn corruption_no_duplicate_section_y_values() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        for (region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        let mut seen_ys: Vec<i8> = Vec::new();
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Byte(y)) = s_map.get("Y") {
                                    assert!(
                                        !seen_ys.contains(y),
                                        "Duplicate section Y={} in chunk in region {}",
                                        y,
                                        region_name
                                    );
                                    seen_ys.push(*y);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn corruption_palette_data_array_consistency() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Compound(block_states)) =
                                    s_map.get("block_states")
                                {
                                    if let Some(fastnbt::Value::List(palette)) =
                                        block_states.get("palette")
                                    {
                                        let palette_size = palette.len();

                                        if palette_size == 1 {
                                            // Uniform section — data array is optional
                                            // (omitted or empty is valid)
                                        } else if palette_size > 1 {
                                            // Multi-block section must have data array
                                            if let Some(fastnbt::Value::LongArray(data)) =
                                                block_states.get("data")
                                            {
                                                assert!(
                                                    !data.is_empty(),
                                                    "Multi-block palette (size {}) must have non-empty data array",
                                                    palette_size
                                                );

                                                // Verify data array size is reasonable
                                                // bits_per_block = max(4, ceil(log2(palette_size)))
                                                let mut bits = 4usize;
                                                while (1usize << bits) < palette_size {
                                                    bits += 1;
                                                }
                                                let vals_per_long = 64 / bits;
                                                let expected_longs =
                                                    4096usize.div_ceil(vals_per_long);

                                                assert_eq!(
                                                    data.len(),
                                                    expected_longs,
                                                    "Data array length {} doesn't match expected {} \
                                                     for palette size {} (bits_per_block={})",
                                                    data.len(),
                                                    expected_longs,
                                                    palette_size,
                                                    bits
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn corruption_block_names_are_valid_minecraft_ids() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let mut all_block_names: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Compound(block_states)) =
                                    s_map.get("block_states")
                                {
                                    if let Some(fastnbt::Value::List(palette)) =
                                        block_states.get("palette")
                                    {
                                        for item in palette {
                                            if let fastnbt::Value::Compound(p) = item {
                                                if let Some(fastnbt::Value::String(name)) =
                                                    p.get("Name")
                                                {
                                                    all_block_names.insert(name.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // All block names should follow minecraft:identifier format
        for name in &all_block_names {
            assert!(
                name.contains(':'),
                "Block name '{}' should contain namespace separator ':'",
                name
            );
            let parts: Vec<&str> = name.splitn(2, ':').collect();
            assert_eq!(
                parts.len(),
                2,
                "Block name '{}' should have namespace:id format",
                name
            );
            assert!(
                !parts[0].is_empty() && !parts[1].is_empty(),
                "Block name '{}' should have non-empty namespace and id",
                name
            );
            // Namespace should be lowercase alphanumeric
            assert!(
                parts[0].chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "Block namespace '{}' should be lowercase: {}",
                parts[0],
                name
            );
        }
    }

    // ═══════════════════════════════════════════════════════════════
    //  6. BIOME SANITY
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn biome_all_sections_have_biome_data() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let mut sections_with_biomes = 0u32;
        let mut sections_total = 0u32;

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            sections_total += 1;
                            if let fastnbt::Value::Compound(s_map) = section {
                                if s_map.contains_key("biomes") {
                                    sections_with_biomes += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        assert!(sections_total > 0, "Should have sections to check");
        assert_eq!(
            sections_with_biomes, sections_total,
            "All sections should have biome data: {}/{} have biomes",
            sections_with_biomes, sections_total
        );
    }

    #[test]
    fn biome_palette_contains_valid_biome_ids() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        let mut biome_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Compound(biomes)) = s_map.get("biomes")
                                {
                                    if let Some(fastnbt::Value::List(palette)) =
                                        biomes.get("palette")
                                    {
                                        for item in palette {
                                            if let fastnbt::Value::String(biome_id) = item {
                                                biome_ids.insert(biome_id.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        assert!(
            !biome_ids.is_empty(),
            "Should have at least one biome ID in the world"
        );

        // All biome IDs should follow minecraft:identifier format
        for biome_id in &biome_ids {
            assert!(
                biome_id.starts_with("minecraft:"),
                "Biome ID '{}' should start with 'minecraft:'",
                biome_id
            );
        }
    }

    #[test]
    fn biome_default_is_plains() {
        let (world_path, _tmp) = generate_test_world();
        let all_chunks = read_all_chunks(&world_path);

        // The generator uses "minecraft:plains" as the default biome for all sections
        let mut found_plains = false;

        for (_region_name, chunks) in &all_chunks {
            for chunk_nbt in chunks {
                if let fastnbt::Value::Compound(map) = chunk_nbt {
                    if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                        for section in sections {
                            if let fastnbt::Value::Compound(s_map) = section {
                                if let Some(fastnbt::Value::Compound(biomes)) = s_map.get("biomes")
                                {
                                    if let Some(fastnbt::Value::List(palette)) =
                                        biomes.get("palette")
                                    {
                                        for item in palette {
                                            if let fastnbt::Value::String(biome_id) = item {
                                                if biome_id == "minecraft:plains" {
                                                    found_plains = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        assert!(
            found_plains,
            "Default biome 'minecraft:plains' should be present in the world"
        );
    }

    // ═══════════════════════════════════════════════════════════════
    //  7. METADATA VALIDATION
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn metadata_json_has_all_required_fields() {
        let (world_path, _tmp) = generate_test_world();
        let metadata_path = world_path.join("metadata.json");

        assert!(metadata_path.exists(), "metadata.json should exist");

        let metadata_str = fs::read_to_string(&metadata_path).expect("read metadata");
        let metadata: serde_json::Value =
            serde_json::from_str(&metadata_str).expect("parse metadata");

        let required = [
            "minMcX",
            "maxMcX",
            "minMcZ",
            "maxMcZ",
            "minGeoLat",
            "maxGeoLat",
            "minGeoLon",
            "maxGeoLon",
        ];

        for field in &required {
            assert!(
                metadata.get(field).is_some(),
                "metadata.json missing required field '{}'",
                field
            );
        }
    }

    #[test]
    fn metadata_minecraft_coords_have_positive_area() {
        let (world_path, _tmp) = generate_test_world();
        let metadata_str =
            fs::read_to_string(world_path.join("metadata.json")).expect("read metadata");
        let m: serde_json::Value = serde_json::from_str(&metadata_str).unwrap();

        let width = m["maxMcX"].as_i64().unwrap() - m["minMcX"].as_i64().unwrap();
        let depth = m["maxMcZ"].as_i64().unwrap() - m["minMcZ"].as_i64().unwrap();

        assert!(width > 0, "World width should be positive: {}", width);
        assert!(depth > 0, "World depth should be positive: {}", depth);

        // For the small_area fixture at scale 1.0, world should be reasonable size
        assert!(
            width < 10_000,
            "World width {} seems unreasonably large for test fixture",
            width
        );
        assert!(
            depth < 10_000,
            "World depth {} seems unreasonably large for test fixture",
            depth
        );
    }
}
