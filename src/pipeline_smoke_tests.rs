//! Pipeline smoke test: generated map validation before publish (MIN-17)
//!
//! Runs the full generation pipeline on a small reference area, then validates
//! the output across structural, content, and size dimensions. Produces a JSON
//! validation report suitable for CI artifact upload.

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
    use std::path::{Path, PathBuf};
    use std::time::Instant;
    use tempfile::TempDir;

    // ── Constants ───────────────────────────────────────────────────

    /// Minimum total world size in bytes (region files + metadata).
    const MIN_WORLD_SIZE_BYTES: u64 = 1_024;
    /// Maximum total world size in bytes for the small fixture (guard against bloat).
    const MAX_WORLD_SIZE_BYTES: u64 = 50 * 1024 * 1024; // 50 MB
    /// Minimum number of region files expected for the small fixture.
    const MIN_REGION_FILES: usize = 1;
    /// Maximum number of region files expected for the small fixture.
    const MAX_REGION_FILES: usize = 4;
    /// Minimum non-air block types (grass, stone, building materials, etc.).
    const MIN_BLOCK_TYPES: usize = 2;

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
            ground_level: -62,
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

    /// Generate a world and return (output_path, temp_dir_guard).
    fn generate_smoke_world() -> (PathBuf, TempDir) {
        let (elements, xzbbox, llbbox) = load_and_parse_fixture();
        let tmp = TempDir::new().expect("Failed to create temp dir");
        let world_dir = tmp.path().join("smoke_world");
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
            .expect("Smoke test: world generation should succeed");
        (output, tmp)
    }

    /// Calculate total size of all files in a directory tree.
    fn dir_total_size(path: &Path) -> u64 {
        let mut total = 0u64;
        if path.is_file() {
            return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        }
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    total += dir_total_size(&p);
                } else {
                    total += fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                }
            }
        }
        total
    }

    /// Collect all unique block names from the generated world.
    fn collect_block_names(world_path: &Path) -> std::collections::HashSet<String> {
        let region_dir = world_path.join("region");
        let mut names = std::collections::HashSet::new();

        for entry in fs::read_dir(&region_dir)
            .expect("read region dir")
            .flatten()
        {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mca") {
                let file = fs::File::open(&path).expect("open region file");
                let mut region = fastanvil::Region::from_stream(file).expect("valid Anvil region");

                for chunk_result in region.iter() {
                    if let Ok(chunk) = chunk_result {
                        let nbt: fastnbt::Value =
                            fastnbt::from_bytes(chunk.data.as_slice()).unwrap();
                        if let fastnbt::Value::Compound(map) = &nbt {
                            if let Some(fastnbt::Value::List(sections)) = map.get("sections") {
                                for section in sections {
                                    if let fastnbt::Value::Compound(s_map) = section {
                                        if let Some(fastnbt::Value::Compound(bs)) =
                                            s_map.get("block_states")
                                        {
                                            if let Some(fastnbt::Value::List(palette)) =
                                                bs.get("palette")
                                            {
                                                for item in palette {
                                                    if let fastnbt::Value::Compound(p) = item {
                                                        if let Some(fastnbt::Value::String(name)) =
                                                            p.get("Name")
                                                        {
                                                            names.insert(name.clone());
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
        }
        names
    }

    /// Count total chunks and parseable chunks in the world.
    fn count_chunks(world_path: &Path) -> (u32, u32) {
        let region_dir = world_path.join("region");
        let mut total = 0u32;
        let mut parseable = 0u32;

        for entry in fs::read_dir(&region_dir)
            .expect("read region dir")
            .flatten()
        {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mca") {
                let file = fs::File::open(&path).expect("open region file");
                let mut region = fastanvil::Region::from_stream(file).expect("valid Anvil region");

                for chunk_result in region.iter() {
                    total += 1;
                    if let Ok(chunk) = chunk_result {
                        let parse: Result<fastnbt::Value, _> =
                            fastnbt::from_bytes(chunk.data.as_slice());
                        if parse.is_ok() {
                            parseable += 1;
                        }
                    }
                }
            }
        }
        (total, parseable)
    }

    // ═══════════════════════════════════════════════════════════════
    //  PIPELINE SMOKE TEST — single test that validates everything
    //  and writes a JSON report
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn pipeline_smoke_test() {
        let start = Instant::now();

        // ── Step 1: Run the full generation pipeline ────────────
        let gen_start = Instant::now();
        let (world_path, _tmp) = generate_smoke_world();
        let generation_ms = gen_start.elapsed().as_millis();

        // ── Step 2: Validate output structure ───────────────────
        let region_dir = world_path.join("region");
        assert!(region_dir.exists(), "Region directory must exist");

        let mca_files: Vec<_> = fs::read_dir(&region_dir)
            .expect("read region dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "mca"))
            .collect();

        let region_count = mca_files.len();
        assert!(
            region_count >= MIN_REGION_FILES,
            "Expected at least {} region file(s), got {}",
            MIN_REGION_FILES,
            region_count
        );
        assert!(
            region_count <= MAX_REGION_FILES,
            "Expected at most {} region file(s), got {} (possible generation runaway)",
            MAX_REGION_FILES,
            region_count
        );

        // Region filenames must follow r.<x>.<z>.mca convention
        for entry in &mca_files {
            let name = entry.file_name().to_string_lossy().to_string();
            let parts: Vec<&str> = name.split('.').collect();
            assert_eq!(
                parts.len(),
                4,
                "Region filename format: r.<x>.<z>.mca, got: {}",
                name
            );
            assert_eq!(parts[0], "r");
            assert!(
                parts[1].parse::<i32>().is_ok(),
                "Region X must be integer: {}",
                name
            );
            assert!(
                parts[2].parse::<i32>().is_ok(),
                "Region Z must be integer: {}",
                name
            );
        }

        // ── Step 3: Validate metadata.json ──────────────────────
        let metadata_path = world_path.join("metadata.json");
        assert!(metadata_path.exists(), "metadata.json must exist");

        let metadata_str = fs::read_to_string(&metadata_path).expect("read metadata");
        let metadata: serde_json::Value =
            serde_json::from_str(&metadata_str).expect("metadata must be valid JSON");

        let required_fields = [
            "minMcX",
            "maxMcX",
            "minMcZ",
            "maxMcZ",
            "minGeoLat",
            "maxGeoLat",
            "minGeoLon",
            "maxGeoLon",
        ];
        for field in &required_fields {
            assert!(
                metadata.get(field).is_some(),
                "metadata missing field '{}'",
                field
            );
        }

        let min_x = metadata["minMcX"].as_i64().unwrap();
        let max_x = metadata["maxMcX"].as_i64().unwrap();
        let min_z = metadata["minMcZ"].as_i64().unwrap();
        let max_z = metadata["maxMcZ"].as_i64().unwrap();
        assert!(max_x > min_x, "World must have positive width");
        assert!(max_z > min_z, "World must have positive depth");

        // ── Step 4: Validate file sizes ─────────────────────────
        let total_size = dir_total_size(&world_path);
        assert!(
            total_size >= MIN_WORLD_SIZE_BYTES,
            "World size {} bytes is below minimum {} bytes",
            total_size,
            MIN_WORLD_SIZE_BYTES
        );
        assert!(
            total_size <= MAX_WORLD_SIZE_BYTES,
            "World size {} bytes exceeds maximum {} bytes (possible generation bloat)",
            total_size,
            MAX_WORLD_SIZE_BYTES
        );

        let mut region_sizes: HashMap<String, u64> = HashMap::new();
        for entry in &mca_files {
            let name = entry.file_name().to_string_lossy().to_string();
            let size = fs::metadata(entry.path()).map(|m| m.len()).unwrap_or(0);
            assert!(size > 0, "Region file {} must not be empty", name);
            region_sizes.insert(name, size);
        }

        // ── Step 5: Validate chunk integrity ────────────────────
        let (total_chunks, parseable_chunks) = count_chunks(&world_path);
        assert!(total_chunks > 0, "World must contain chunks");
        assert_eq!(
            total_chunks, parseable_chunks,
            "All chunks must be parseable: {}/{} succeeded",
            parseable_chunks, total_chunks
        );

        // ── Step 6: Validate block content ──────────────────────
        let block_names = collect_block_names(&world_path);
        let non_air_blocks: Vec<_> = block_names
            .iter()
            .filter(|n| *n != "minecraft:air")
            .collect();

        assert!(
            non_air_blocks.len() >= MIN_BLOCK_TYPES,
            "Expected at least {} non-air block types, got {} ({:?})",
            MIN_BLOCK_TYPES,
            non_air_blocks.len(),
            non_air_blocks
        );

        // Grass block must be present (base terrain layer)
        assert!(
            block_names.contains("minecraft:grass_block"),
            "World must contain grass blocks (base layer)"
        );

        let total_ms = start.elapsed().as_millis();

        // ── Step 7: Write validation report ─────────────────────
        let report = serde_json::json!({
            "smoke_test": "pipeline_smoke_test",
            "status": "PASS",
            "fixture": "small_area.json",
            "timings": {
                "generation_ms": generation_ms,
                "total_ms": total_ms,
            },
            "world": {
                "total_size_bytes": total_size,
                "region_file_count": region_count,
                "region_sizes": region_sizes,
                "total_chunks": total_chunks,
                "parseable_chunks": parseable_chunks,
                "chunk_integrity_pct": 100.0,
            },
            "blocks": {
                "unique_block_types": block_names.len(),
                "non_air_block_types": non_air_blocks.len(),
                "has_grass_block": true,
                "sample_blocks": non_air_blocks.iter().take(20).collect::<Vec<_>>(),
            },
            "metadata": {
                "world_width": max_x - min_x,
                "world_depth": max_z - min_z,
                "min_mc_x": min_x,
                "max_mc_x": max_x,
                "min_mc_z": min_z,
                "max_mc_z": max_z,
            },
            "thresholds": {
                "min_world_size_bytes": MIN_WORLD_SIZE_BYTES,
                "max_world_size_bytes": MAX_WORLD_SIZE_BYTES,
                "min_region_files": MIN_REGION_FILES,
                "max_region_files": MAX_REGION_FILES,
                "min_block_types": MIN_BLOCK_TYPES,
            }
        });

        // Write report to a well-known location for CI artifact upload.
        // Use SMOKE_REPORT_DIR env var if set (CI), otherwise use the temp dir.
        let report_dir = std::env::var("SMOKE_REPORT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| world_path.parent().unwrap().to_path_buf());

        fs::create_dir_all(&report_dir).expect("create report dir");
        let report_path = report_dir.join("smoke-test-report.json");
        let report_str = serde_json::to_string_pretty(&report).expect("serialize report");
        fs::write(&report_path, &report_str).expect("write smoke test report");

        // Print report to stdout for CI --nocapture visibility
        let separator = "=".repeat(60);
        println!("\n{}", separator);
        println!("  PIPELINE SMOKE TEST REPORT");
        println!("{}", separator);
        println!("{}", report_str);
        println!("{}\n", separator);
        println!("Report written to: {}", report_path.display());
    }

    // ═══════════════════════════════════════════════════════════════
    //  Additional focused smoke checks
    // ═══════════════════════════════════════════════════════════════

    #[test]
    fn smoke_no_empty_region_files() {
        let (world_path, _tmp) = generate_smoke_world();
        let region_dir = world_path.join("region");

        for entry in fs::read_dir(&region_dir)
            .expect("read region dir")
            .flatten()
        {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mca") {
                let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                assert!(
                    size > 8192,
                    "Region file {} size ({} bytes) is suspiciously small (header alone is 8192 bytes)",
                    path.file_name().unwrap().to_string_lossy(),
                    size
                );
            }
        }
    }

    #[test]
    fn smoke_metadata_geo_coordinates_in_range() {
        let (world_path, _tmp) = generate_smoke_world();
        let metadata_str =
            fs::read_to_string(world_path.join("metadata.json")).expect("read metadata");
        let m: serde_json::Value = serde_json::from_str(&metadata_str).unwrap();

        let min_lat = m["minGeoLat"].as_f64().unwrap();
        let max_lat = m["maxGeoLat"].as_f64().unwrap();
        let min_lon = m["minGeoLon"].as_f64().unwrap();
        let max_lon = m["maxGeoLon"].as_f64().unwrap();

        // Must be within the fixture input bbox (Arnis, Germany area)
        assert!(
            min_lat >= 54.0 && max_lat <= 55.0,
            "Latitude out of range: {}-{}",
            min_lat,
            max_lat
        );
        assert!(
            min_lon >= 9.0 && max_lon <= 11.0,
            "Longitude out of range: {}-{}",
            min_lon,
            max_lon
        );
        assert!(max_lat > min_lat, "Latitude must have positive span");
        assert!(max_lon > min_lon, "Longitude must have positive span");
    }
}
