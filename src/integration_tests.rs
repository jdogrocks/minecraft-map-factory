//! Integration tests: end-to-end small area generation (MIN-14)
//!
//! These tests exercise the full generation pipeline on small reference areas:
//! OSM fixture → parse → transform → generate world → validate output.

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
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // ── Helpers ─────────────────────────────────────────────────────

    /// Bounding box that encompasses the small_area.json fixture nodes.
    /// Nodes range from ~54.6295–54.6310 lat, ~9.9290–9.9320 lon.
    fn fixture_llbbox() -> LLBBox {
        LLBBox::new(54.6290, 9.9280, 54.6315, 9.9330).unwrap()
    }

    /// Construct a minimal Args suitable for headless integration testing.
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
        }
    }

    /// Load the small_area.json fixture and parse it through the OSM pipeline.
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

    // ═══════════════════════════════════════════════════════════════
    //  1. Full pipeline: parse → generate → validate output files
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn full_pipeline_generates_region_files() {
        let (elements, xzbbox, llbbox) = load_and_parse_fixture();
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let world_dir = tmp.path().join("test_world");

        fs::create_dir_all(&world_dir).expect("create world dir");
        let args = test_args(llbbox, Some(world_dir.clone()));
        let ground = Ground::new_flat(args.ground_level);

        let options = GenerationOptions {
            path: world_dir.clone(),
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };

        let result = generate_world_with_options(elements, xzbbox, llbbox, ground, &args, options);
        assert!(
            result.is_ok(),
            "World generation failed: {:?}",
            result.err()
        );

        let output_path = result.unwrap();
        // Verify region directory and .mca files were created
        let region_dir = output_path.join("region");
        assert!(region_dir.exists(), "Region directory should exist");

        let mca_files: Vec<_> = std::fs::read_dir(&region_dir)
            .expect("Failed to read region dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "mca"))
            .collect();

        assert!(
            !mca_files.is_empty(),
            "At least one .mca region file should be generated"
        );

        // Verify metadata.json was created
        let metadata_path = output_path.join("metadata.json");
        assert!(
            metadata_path.exists(),
            "metadata.json should be created in the world directory"
        );

        // Validate metadata content
        let metadata_str =
            std::fs::read_to_string(&metadata_path).expect("Failed to read metadata.json");
        let metadata: serde_json::Value =
            serde_json::from_str(&metadata_str).expect("metadata.json should be valid JSON");

        assert!(
            metadata.get("minMcX").is_some(),
            "metadata should contain minMcX"
        );
        assert!(
            metadata.get("maxMcX").is_some(),
            "metadata should contain maxMcX"
        );
        assert!(
            metadata.get("minMcZ").is_some(),
            "metadata should contain minMcZ"
        );
        assert!(
            metadata.get("maxMcZ").is_some(),
            "metadata should contain maxMcZ"
        );
        assert!(
            metadata.get("minGeoLat").is_some(),
            "metadata should contain minGeoLat"
        );
        assert!(
            metadata.get("maxGeoLon").is_some(),
            "metadata should contain maxGeoLon"
        );
    }

    // ═══════════════════════════════════════════════════════════════
    //  2. Round-trip: write world → read back region → verify blocks
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn round_trip_region_files_are_valid_anvil() {
        let (elements, xzbbox, llbbox) = load_and_parse_fixture();
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let world_dir = tmp.path().join("roundtrip_world");

        fs::create_dir_all(&world_dir).expect("create world dir");
        let args = test_args(llbbox, Some(world_dir.clone()));
        let ground = Ground::new_flat(args.ground_level);

        let options = GenerationOptions {
            path: world_dir.clone(),
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };

        let output_path =
            generate_world_with_options(elements, xzbbox, llbbox, ground, &args, options)
                .expect("World generation should succeed");

        // Read back each .mca file and verify it can be parsed by fastanvil
        let region_dir = output_path.join("region");
        for entry in std::fs::read_dir(&region_dir).expect("read region dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "mca") {
                let file = std::fs::File::open(&path).expect("Failed to open region file");
                let mut region = fastanvil::Region::from_stream(file)
                    .expect("Region file should be valid Anvil format");

                // Iterate chunks to verify at least some are populated
                let mut chunk_count = 0u32;
                for chunk_data in region.iter().flatten() {
                    // Verify chunk data is valid NBT by parsing it
                    let _nbt: fastnbt::Value = fastnbt::from_bytes(chunk_data.data.as_slice())
                        .expect("Chunk NBT should be parseable");
                    chunk_count += 1;
                }
                assert!(
                    chunk_count > 0,
                    "Region file {} should contain at least one chunk",
                    path.display()
                );
            }
        }
    }

    #[test]
    fn round_trip_chunks_contain_expected_sections() {
        let (elements, xzbbox, llbbox) = load_and_parse_fixture();
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let world_dir = tmp.path().join("sections_world");

        fs::create_dir_all(&world_dir).expect("create world dir");
        let args = test_args(llbbox, Some(world_dir.clone()));
        let ground = Ground::new_flat(args.ground_level);

        let options = GenerationOptions {
            path: world_dir.clone(),
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };

        let output_path =
            generate_world_with_options(elements, xzbbox, llbbox, ground, &args, options)
                .expect("World generation should succeed");

        let region_dir = output_path.join("region");
        let mut found_sections = false;

        for entry in std::fs::read_dir(&region_dir).expect("read region dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "mca") {
                let file = std::fs::File::open(&path).expect("open region file");
                let mut region = fastanvil::Region::from_stream(file).expect("valid region");

                for chunk in region.iter().flatten() {
                    let nbt: fastnbt::Value = fastnbt::from_bytes(chunk.data.as_slice()).unwrap();
                    if let fastnbt::Value::Compound(map) = &nbt {
                        // Chunks should have a "sections" array
                        if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                            if !sections.is_empty() {
                                found_sections = true;
                            }
                        }
                    }
                }
            }
        }

        assert!(
            found_sections,
            "At least one chunk should contain non-empty sections (block data)"
        );
    }

    // ═══════════════════════════════════════════════════════════════
    //  3. OSM parsing layer integration
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn parse_fixture_produces_expected_elements() {
        let (elements, xzbbox, _) = load_and_parse_fixture();

        // The fixture has: 1 building way, 1 highway way, 1 tree node
        let building_count = elements
            .iter()
            .filter(|e| e.tags().contains_key("building"))
            .count();
        let highway_count = elements
            .iter()
            .filter(|e| e.tags().contains_key("highway"))
            .count();
        let tree_count = elements
            .iter()
            .filter(|e| {
                e.tags()
                    .get("natural")
                    .map(|v| v == "tree")
                    .unwrap_or(false)
            })
            .count();

        assert_eq!(building_count, 1, "Should parse exactly 1 building");
        assert_eq!(highway_count, 1, "Should parse exactly 1 highway");
        assert_eq!(tree_count, 1, "Should parse exactly 1 tree");

        // XZ bounding box should have positive dimensions
        assert!(
            xzbbox.max_x() > xzbbox.min_x(),
            "XZ bbox should have positive width"
        );
        assert!(
            xzbbox.max_z() > xzbbox.min_z(),
            "XZ bbox should have positive depth"
        );
    }

    #[test]
    fn parse_fixture_elements_sorted_by_priority() {
        let (elements, _, _) = load_and_parse_fixture();

        // Verify elements are sorted by priority (lower index = higher priority)
        let priorities: Vec<usize> = elements.iter().map(osm_parser::get_priority).collect();

        for window in priorities.windows(2) {
            assert!(
                window[0] <= window[1],
                "Elements should be sorted by priority: {} should come before {}",
                window[0],
                window[1]
            );
        }
    }

    // ═══════════════════════════════════════════════════════════════
    //  4. Error path tests
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn corrupt_osm_file_returns_error() {
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/corrupt_osm.json"
        );
        let result = retrieve_data::fetch_data_from_file(fixture_path);

        // The corrupt file has no "elements" field; serde should fail
        // or the data should be marked as empty
        match result {
            Err(_) => {} // Expected: deserialization failure
            Ok(data) => {
                assert!(
                    data.is_empty(),
                    "Corrupt OSM data should either error or produce empty elements"
                );
            }
        }
    }

    #[test]
    fn empty_osm_produces_no_elements() {
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/empty_area.json"
        );
        let osm_data = retrieve_data::fetch_data_from_file(fixture_path)
            .expect("Empty OSM file should still parse");

        assert!(osm_data.is_empty(), "Empty fixture should have no elements");

        let llbbox = fixture_llbbox();
        let (elements, _xzbbox) = osm_parser::parse_osm_data(osm_data, llbbox, 1.0, false);
        assert!(
            elements.is_empty(),
            "Parsing empty OSM data should produce no elements"
        );
    }

    #[test]
    fn empty_elements_generate_world_without_panic() {
        let llbbox = fixture_llbbox();
        let xzbbox = XZBBox::rect_from_xz_lengths(50.0, 50.0).unwrap();
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let world_dir = tmp.path().join("empty_world");

        fs::create_dir_all(&world_dir).expect("create world dir");
        let args = test_args(llbbox, Some(world_dir.clone()));
        let ground = Ground::new_flat(args.ground_level);

        let options = GenerationOptions {
            path: world_dir.clone(),
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };

        // Should succeed without panic even with zero elements
        let result =
            generate_world_with_options(Vec::new(), xzbbox, llbbox, ground, &args, options);
        assert!(
            result.is_ok(),
            "Empty element list should not cause generation to fail: {:?}",
            result.err()
        );
    }

    #[test]
    fn out_of_bounds_nodes_are_clipped() {
        // Create a very small bbox that excludes most fixture nodes
        let narrow_bbox = LLBBox::new(54.6299, 9.9299, 54.6301, 9.9301).unwrap();
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/small_area.json"
        );
        let osm_data =
            retrieve_data::fetch_data_from_file(fixture_path).expect("Failed to load fixture");

        let (elements, _xzbbox) = osm_parser::parse_osm_data(osm_data, narrow_bbox, 1.0, false);

        // With such a narrow bbox, most elements should be clipped out
        // or have coordinates snapped to the bbox boundary
        // The tree node at (54.6302, 9.9295) is outside this bbox
        let tree_count = elements
            .iter()
            .filter(|e| {
                e.tags()
                    .get("natural")
                    .map(|v| v == "tree")
                    .unwrap_or(false)
            })
            .count();

        assert_eq!(
            tree_count, 0,
            "Tree node outside the narrow bbox should be clipped"
        );
    }

    #[test]
    fn nonexistent_file_returns_error() {
        let result = retrieve_data::fetch_data_from_file("/nonexistent/path/to/file.json");
        assert!(result.is_err(), "Nonexistent file should return an error");
    }

    // ═══════════════════════════════════════════════════════════════
    //  5. Generation options and formats
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn generation_with_custom_scale() {
        let (_, _, llbbox) = load_and_parse_fixture();

        // Re-parse at scale 2.0 — should produce larger XZ coordinates
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/small_area.json"
        );
        let osm_data = retrieve_data::fetch_data_from_file(fixture_path).unwrap();
        let (_, xzbbox_2x) = osm_parser::parse_osm_data(osm_data, llbbox, 2.0, false);

        let osm_data_1x = retrieve_data::fetch_data_from_file(fixture_path).unwrap();
        let (_, xzbbox_1x) = osm_parser::parse_osm_data(osm_data_1x, llbbox, 1.0, false);

        // At 2x scale, the world should be roughly twice as wide
        let width_1x = xzbbox_1x.max_x() - xzbbox_1x.min_x();
        let width_2x = xzbbox_2x.max_x() - xzbbox_2x.min_x();

        assert!(
            width_2x > width_1x,
            "2x scale should produce wider world: 1x={}, 2x={}",
            width_1x,
            width_2x
        );
    }

    #[test]
    fn generation_with_flat_ground_produces_level_output() {
        let (elements, xzbbox, llbbox) = load_and_parse_fixture();
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let world_dir = tmp.path().join("flat_ground_world");
        fs::create_dir_all(&world_dir).expect("create world dir");

        let args = test_args(llbbox, Some(world_dir.clone()));
        let ground = Ground::new_flat(-62);

        let options = GenerationOptions {
            path: world_dir.clone(),
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };

        let result = generate_world_with_options(elements, xzbbox, llbbox, ground, &args, options);
        assert!(result.is_ok(), "Flat ground generation should succeed");

        // Verify world was created
        let output = result.unwrap();
        assert!(output.exists(), "Output directory should exist");
    }

    // ═══════════════════════════════════════════════════════════════
    //  6. Determinism: same input → same output
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn generation_is_deterministic() {
        let fixture_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/small_area.json"
        );
        let llbbox = fixture_llbbox();

        // Run 1
        let osm_data_1 = retrieve_data::fetch_data_from_file(fixture_path).unwrap();
        let (mut elements_1, xzbbox_1) = osm_parser::parse_osm_data(osm_data_1, llbbox, 1.0, false);
        elements_1.sort_by_key(osm_parser::get_priority);

        let tmp1 = TempDir::new().unwrap();
        let dir1 = tmp1.path().join("det_world_1");
        fs::create_dir_all(&dir1).expect("create dir1");
        let args1 = test_args(llbbox, Some(dir1.clone()));
        let ground1 = Ground::new_flat(args1.ground_level);
        let opts1 = GenerationOptions {
            path: dir1.clone(),
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };
        let out1 =
            generate_world_with_options(elements_1, xzbbox_1, llbbox, ground1, &args1, opts1)
                .expect("Run 1 should succeed");

        // Run 2
        let osm_data_2 = retrieve_data::fetch_data_from_file(fixture_path).unwrap();
        let (mut elements_2, xzbbox_2) = osm_parser::parse_osm_data(osm_data_2, llbbox, 1.0, false);
        elements_2.sort_by_key(osm_parser::get_priority);

        let tmp2 = TempDir::new().unwrap();
        let dir2 = tmp2.path().join("det_world_2");
        fs::create_dir_all(&dir2).expect("create dir2");
        let args2 = test_args(llbbox, Some(dir2.clone()));
        let ground2 = Ground::new_flat(args2.ground_level);
        let opts2 = GenerationOptions {
            path: dir2.clone(),
            format: WorldFormat::JavaAnvil,
            level_name: None,
            spawn_point: None,
        };
        let out2 =
            generate_world_with_options(elements_2, xzbbox_2, llbbox, ground2, &args2, opts2)
                .expect("Run 2 should succeed");

        // Compare metadata files — they should be byte-identical
        let meta1 = std::fs::read_to_string(out1.join("metadata.json")).expect("metadata run 1");
        let meta2 = std::fs::read_to_string(out2.join("metadata.json")).expect("metadata run 2");
        assert_eq!(meta1, meta2, "Metadata should be identical across runs");

        // Compare region file sets
        let mut regions1: Vec<String> = std::fs::read_dir(out1.join("region"))
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        regions1.sort();

        let mut regions2: Vec<String> = std::fs::read_dir(out2.join("region"))
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        regions2.sort();

        assert_eq!(
            regions1, regions2,
            "Same region files should be produced across runs"
        );
    }
}
