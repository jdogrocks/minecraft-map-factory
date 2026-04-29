//! Quality validator for generated Minecraft maps.
//!
//! Replaces the legacy `validation/` module. The pre-MIN-43 validator only
//! checked region count, total size, and Anvil headers — none of which catch
//! the floating-buildings failure that motivated MIN-40. This module adds
//! four real checks plus the legacy structural sanity:
//!
//! 1. **Ground continuity** — sampled (x,z) columns must have non-air
//!    blocks from `ground_y_min` up to surface, with a small tolerance for
//!    caves/basements.
//! 2. **Interior populated** — sampled chunks containing door blocks must
//!    also contain furniture and a floor partition.
//! 3. **Region-file size sanity** — flag the 4,202,496-byte empty-chunks
//!    signature explicitly; bytes-per-chunk threshold is tunable.
//! 4. **Surface diversity** — sampled chunks must collectively expose at
//!    least N distinct surface block types (catches "everything is a road
//!    stripe with air below").
//!
//! Each failure produces a distinct, named string in `failure_reasons` so
//! the publisher can route, the metrics layer can bucket, and the CTO
//! reviewing a regression run can see exactly which check caught what.

mod anvil;
mod ground;
mod interior;
mod region_size;
mod structure;
mod surface_diversity;

use crate::config::ValidationConfig;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Per-map validation report. `is_valid = true` iff every enabled check
/// passes; otherwise `failure_reasons` lists every distinct failure across
/// all checks (one map can fail multiple checks at once and the report
/// surfaces all of them so the operator can see the full picture, not just
/// the first one to trip).
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub region_file_count: usize,
    pub total_size_bytes: u64,
    pub failure_reasons: Vec<String>,
}

impl ValidationReport {
    fn empty() -> Self {
        Self {
            is_valid: true,
            region_file_count: 0,
            total_size_bytes: 0,
            failure_reasons: Vec::new(),
        }
    }
}

/// Quality validator for generated maps. Construct once per pipeline run
/// (cheap; just clones config); call `validate(map_path)` per generated
/// map.
pub struct Validator {
    config: ValidationConfig,
}

impl Validator {
    pub fn new(config: &ValidationConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Validate a generated map at `map_path`. The path may either contain
    /// a `region/` directory directly or wrap one in a sub-directory (the
    /// generator emits `<world_name>/region/`); both layouts are handled.
    ///
    /// Errors here are reserved for I/O / parse failures the validator
    /// can't reason about. Quality failures (the four MIN-43 checks)
    /// surface in `report.failure_reasons` with `is_valid = false`.
    pub fn validate(
        &self,
        map_path: &Path,
    ) -> Result<ValidationReport, Box<dyn std::error::Error + Send + Sync>> {
        let mut report = ValidationReport::empty();

        let region_dir = match locate_region_dir(map_path)? {
            Some(dir) => dir,
            None => {
                report.is_valid = false;
                report
                    .failure_reasons
                    .push("no_region_directory: no region/*.mca dir found under map path".into());
                return Ok(report);
            }
        };

        debug!(region_dir = %region_dir.display(), "Found region dir");

        let region_files = anvil::list_region_files(&region_dir)?;
        report.region_file_count = region_files.len();
        report.total_size_bytes = region_files.iter().map(|rf| rf.size_bytes).sum();

        // ---- 0. Structural sanity (legacy MIN-7 / MIN-29) ----
        let mut reasons =
            structure::check(&self.config, &region_dir, &region_files, &report).unwrap_or_default();
        report.failure_reasons.append(&mut reasons);

        // ---- 1. Region-file size sanity (MIN-43 #3) ----
        let mut reasons = region_size::check(&self.config, &region_files);
        report.failure_reasons.append(&mut reasons);

        // ---- 2. Ground continuity (MIN-43 #1) ----
        match ground::check(&self.config, &region_files) {
            Ok(mut reasons) => report.failure_reasons.append(&mut reasons),
            Err(e) => {
                warn!(error = %e, "Ground-continuity check errored");
                report
                    .failure_reasons
                    .push(format!("ground_check_error: {e}"));
            }
        }

        // ---- 3. Interior populated (MIN-43 #2) ----
        match interior::check(&self.config, &region_files) {
            Ok(mut reasons) => report.failure_reasons.append(&mut reasons),
            Err(e) => {
                warn!(error = %e, "Interior check errored");
                report
                    .failure_reasons
                    .push(format!("interior_check_error: {e}"));
            }
        }

        // ---- 4. Surface diversity (MIN-43 #4) ----
        match surface_diversity::check(&self.config, &region_files) {
            Ok(mut reasons) => report.failure_reasons.append(&mut reasons),
            Err(e) => {
                warn!(error = %e, "Surface-diversity check errored");
                report
                    .failure_reasons
                    .push(format!("surface_diversity_check_error: {e}"));
            }
        }

        report.is_valid = report.failure_reasons.is_empty();
        if report.is_valid {
            info!(
                region_files = report.region_file_count,
                total_bytes = report.total_size_bytes,
                "Validation passed"
            );
        } else {
            warn!(
                region_files = report.region_file_count,
                total_bytes = report.total_size_bytes,
                reasons = ?report.failure_reasons,
                "Validation failed"
            );
        }
        Ok(report)
    }
}

/// Find the `region/` directory under `map_path`. The generator wraps the
/// world in a sub-directory (`<world_name>/region/`), so we walk up to two
/// levels deep before giving up.
fn locate_region_dir(
    map_path: &Path,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
    let direct = map_path.join("region");
    if direct.is_dir() {
        return Ok(Some(direct));
    }
    if !map_path.is_dir() {
        return Ok(None);
    }
    for entry in std::fs::read_dir(map_path)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let nested = entry.path().join("region");
        if nested.is_dir() {
            return Ok(Some(nested));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn locate_region_dir_finds_direct() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join("region")).unwrap();
        let found = locate_region_dir(tmp.path()).unwrap();
        assert_eq!(found.unwrap(), tmp.path().join("region"));
    }

    #[test]
    fn locate_region_dir_finds_nested() {
        let tmp = tempfile::tempdir().unwrap();
        let world = tmp.path().join("MMF World 1");
        fs::create_dir(&world).unwrap();
        fs::create_dir(world.join("region")).unwrap();
        let found = locate_region_dir(tmp.path()).unwrap();
        assert_eq!(found.unwrap(), world.join("region"));
    }

    #[test]
    fn locate_region_dir_missing_returns_none() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(locate_region_dir(tmp.path()).unwrap().is_none());
    }

    #[test]
    fn validate_missing_region_dir_fails_with_named_reason() {
        let tmp = tempfile::tempdir().unwrap();
        let validator = Validator::new(&ValidationConfig::default());
        let report = validator.validate(tmp.path()).unwrap();
        assert!(!report.is_valid);
        assert!(report
            .failure_reasons
            .iter()
            .any(|r| r.starts_with("no_region_directory")));
    }
}
