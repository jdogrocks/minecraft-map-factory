//! Integration-level fixture tests that run the validator against real
//! `.mca` artifacts checked into the repo. These exercise every chunk-
//! walking path in the four checks (the unit tests in `mod.rs`,
//! `anvil.rs`, `ground.rs`, etc. cover the pure-function pieces and
//! synthetic byte-level cases).
//!
//! The fixture in use is `pipeline/output/published/Times_Square__NYC/`,
//! one of the 20 floating maps that motivated MIN-40. It exists at PR
//! time to exercise:
//! - The structural / region-size happy path (Times Square's region file
//!   is 7.5 MB, well above thresholds — passes structure).
//! - The ground-continuity path (Times Square has dense urban geometry
//!   that mostly survives ground continuity; useful for the
//!   non-degenerate case).
//! - The surface-diversity path (Times Square has roads + a few surface
//!   types, currently below the v2 threshold).
//!
//! When MIN-41 lands, swap in the new clean fixture as a *passing* case.
//! Until then, this fixture is the regression target.

#[cfg(test)]
mod tests {
    use super::super::Validator;
    use crate::config::ValidationConfig;
    use std::path::PathBuf;

    fn fixture_path() -> Option<PathBuf> {
        // Walk up from the pipeline crate root to the repo root, then
        // down to the Times Square published fixture. If the fixture
        // isn't checked out at the expected path, skip the test rather
        // than fail — this lets `cargo test` run anywhere the repo
        // isn't fully cloned.
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let repo_root = manifest_dir.parent()?;
        let p = repo_root.join("pipeline/output/published/Times_Square__NYC/MMF World 1");
        if p.is_dir() {
            Some(p)
        } else {
            None
        }
    }

    /// Times Square is one of the 20 floating maps from MIN-40. It must
    /// fail v2 validation: ground discontinuity beneath the dense urban
    /// geometry, surface-diversity collapse (heightmap returns y=0 for
    /// most chunks because the ground fill never wrote), and an interior
    /// chunk where doors were placed without furniture/floor. This is
    /// the proof-of-correctness anchor for the regression run; if this
    /// test ever starts passing without MIN-41 also having landed,
    /// something in the validator regressed.
    #[test]
    fn times_square_floating_map_fails_with_named_reasons() {
        let Some(map_path) = fixture_path() else {
            eprintln!("Times Square fixture not present; skipping");
            return;
        };
        let validator = Validator::new(&ValidationConfig::default());
        let report = validator
            .validate(&map_path)
            .expect("validator should not error on a real fixture");

        eprintln!(
            "Times Square (MIN-43 evidence): is_valid={} region_files={} total_bytes={} reasons={:#?}",
            report.is_valid,
            report.region_file_count,
            report.total_size_bytes,
            report.failure_reasons,
        );

        assert!(
            !report.is_valid,
            "Times Square is one of the 20 known-broken maps and must fail validation"
        );
        assert!(
            report
                .failure_reasons
                .iter()
                .any(|r| r.starts_with("ground_discontinuity")),
            "expected ground_discontinuity in {:?}",
            report.failure_reasons
        );
        // Surface diversity is degenerate on a chunk-allocated-without-
        // terrain map (heightmap collapses to y=0 → blocks at y=0 are
        // air → 0 distinct surface types).
        assert!(
            report
                .failure_reasons
                .iter()
                .any(|r| r.starts_with("surface_diversity_low")),
            "expected surface_diversity_low in {:?}",
            report.failure_reasons
        );
    }

    /// Region-size check fires the "empty chunks signature" reason on a
    /// hypothetical region file with the empirical signature byte count.
    /// (We use a fake on-disk file because the Times Square fixture is
    /// 7.5 MB, not the empty signature; this hits the named-reason
    /// path.)
    #[test]
    fn region_size_signature_fires_named_reason() {
        let tmp = tempfile::tempdir().unwrap();
        let region = tmp.path().join("region");
        std::fs::create_dir(&region).unwrap();
        let mca = region.join("r.0.0.mca");
        // A 4,202,496-byte file — the exact MIN-40 empty-chunks
        // signature. The other checks will skip (no real chunks) but
        // the region-size check should trip.
        std::fs::write(&mca, vec![0u8; 4_202_496]).unwrap();

        let validator = Validator::new(&ValidationConfig::default());
        let report = validator.validate(tmp.path()).unwrap();
        assert!(!report.is_valid);
        assert!(
            report
                .failure_reasons
                .iter()
                .any(|r| r.starts_with("region_empty_chunks_signature")),
            "expected region_empty_chunks_signature in {:?}",
            report.failure_reasons
        );
    }

    /// Region-size check fires the per-chunk floor reason on a region
    /// file that's small but doesn't match the exact signature.
    #[test]
    fn region_size_per_chunk_floor_fires_for_undersized_file() {
        let tmp = tempfile::tempdir().unwrap();
        let region = tmp.path().join("region");
        std::fs::create_dir(&region).unwrap();
        let mca = region.join("r.0.0.mca");
        // ~3.9 MB / 1024 chunks ~= 3,900 B/chunk, below the 4,200 floor
        // and not the exact signature.
        std::fs::write(&mca, vec![0u8; 4_000_000]).unwrap();

        let validator = Validator::new(&ValidationConfig::default());
        let report = validator.validate(tmp.path()).unwrap();
        assert!(!report.is_valid);
        assert!(
            report
                .failure_reasons
                .iter()
                .any(|r| r.starts_with("region_too_small_per_chunk")),
            "expected region_too_small_per_chunk in {:?}",
            report.failure_reasons
        );
    }

    /// Structural sanity fires the named reason for a region file that
    /// is too small to even contain the Anvil header tables.
    #[test]
    fn structure_anvil_header_invalid_fires_for_truncated_file() {
        let tmp = tempfile::tempdir().unwrap();
        let region = tmp.path().join("region");
        std::fs::create_dir(&region).unwrap();
        let mca = region.join("r.0.0.mca");
        // 100 bytes is well below the 8 KiB Anvil header floor.
        std::fs::write(&mca, b"truncated").unwrap();

        let validator = Validator::new(&ValidationConfig::default());
        let report = validator.validate(tmp.path()).unwrap();
        assert!(!report.is_valid);
        assert!(
            report
                .failure_reasons
                .iter()
                .any(|r| r.starts_with("structure_anvil_header_invalid")),
            "expected structure_anvil_header_invalid in {:?}",
            report.failure_reasons
        );
    }

    /// Empty `region/` directory fails on `structure_too_few_region_files`.
    /// This is the cheapest "no map at all" case the validator catches.
    #[test]
    fn structure_too_few_region_files_fires_for_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("region")).unwrap();

        let validator = Validator::new(&ValidationConfig::default());
        let report = validator.validate(tmp.path()).unwrap();
        assert!(!report.is_valid);
        assert!(
            report
                .failure_reasons
                .iter()
                .any(|r| r.starts_with("structure_too_few_region_files")),
            "expected structure_too_few_region_files in {:?}",
            report.failure_reasons
        );
    }

    /// On-demand regression run against every map directory found at
    /// `MMF_REGRESSION_FIXTURES_DIR` (env var). Each immediate subdir is
    /// treated as a candidate map root; the validator runs against it and
    /// the report is printed to stderr. Marked `#[ignore]` so `cargo
    /// test` skips it by default (the fixtures live on a specific
    /// machine); run with `cargo test -- --ignored --nocapture` and the
    /// env var set to the broken-maps directory to capture evidence for
    /// the MIN-43 acceptance gate.
    #[test]
    #[ignore]
    fn regression_run_against_floating_maps() {
        let Ok(dir) = std::env::var("MMF_REGRESSION_FIXTURES_DIR") else {
            eprintln!(
                "set MMF_REGRESSION_FIXTURES_DIR to the directory \
                 containing the 20 floating maps"
            );
            return;
        };
        let root = std::path::PathBuf::from(&dir);
        let mut entries: Vec<_> = std::fs::read_dir(&root)
            .expect("regression fixtures dir must exist")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();
        entries.sort_by_key(|e| e.file_name());

        let validator = Validator::new(&ValidationConfig::default());
        let mut failed = 0usize;
        let mut passed = 0usize;
        for entry in &entries {
            let path = entry.path();
            // Map artifacts often live under `<dir>/<world_name>/region/`
            // (the generator's nested layout). The validator's
            // `locate_region_dir` walks one level deep.
            let report = match validator.validate(&path) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("[ERROR] {}: {e}", path.display());
                    failed += 1;
                    continue;
                }
            };
            if report.is_valid {
                eprintln!("[PASS] {}", path.display());
                passed += 1;
            } else {
                eprintln!(
                    "[FAIL] {} ({} regions, {} B): {}",
                    path.display(),
                    report.region_file_count,
                    report.total_size_bytes,
                    report.failure_reasons.join(" | ")
                );
                failed += 1;
            }
        }
        eprintln!(
            "\nMIN-43 regression summary: {} maps total — {} failed, {} passed",
            entries.len(),
            failed,
            passed,
        );
        // The acceptance criterion is "fails all 20 floating maps" — the
        // assertion below codifies that. If a future run comes in green
        // (because MIN-41 has been applied to the fixture set) the
        // expectation flips and this assertion needs updating.
        assert!(
            passed == 0,
            "regression fixtures are expected to all fail pre-MIN-41; \
             {} passed unexpectedly",
            passed
        );
    }
}
