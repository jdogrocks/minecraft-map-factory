//! Legacy structural sanity (region count / map size / Anvil header).
//! Carried over from the pre-MIN-43 validator so existing thresholds keep
//! working; the new region-file-size-sanity check (`region_size`) handles
//! the per-file empty-chunks signature that this didn't catch.

use super::anvil::RegionFile;
use crate::config::ValidationConfig;
use std::path::Path;

/// Returns one named failure reason per failure encountered. Empty vec
/// means everything looked fine (or the check was disabled).
pub fn check(
    config: &ValidationConfig,
    region_dir: &Path,
    region_files: &[RegionFile],
    report: &super::ValidationReport,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut reasons = Vec::new();

    if region_files.len() < config.min_region_files {
        reasons.push(format!(
            "structure_too_few_region_files: {} found, minimum {}",
            region_files.len(),
            config.min_region_files
        ));
    }

    if report.total_size_bytes < config.min_map_size_bytes {
        reasons.push(format!(
            "structure_map_too_small: {} bytes total, minimum {}",
            report.total_size_bytes, config.min_map_size_bytes
        ));
    }

    if config.validate_structure && !region_files.is_empty() {
        if let Err(e) = validate_anvil_headers(region_dir, region_files) {
            reasons.push(format!("structure_anvil_header_invalid: {e}"));
        }
    }

    Ok(reasons)
}

fn validate_anvil_headers(
    _region_dir: &Path,
    region_files: &[RegionFile],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Anvil region files have two 4 KiB tables (location + timestamp) at
    // the head, so any file under 8 KiB is structurally invalid.
    for rf in region_files {
        if rf.size_bytes < 8192 {
            return Err(format!(
                "{} is {} bytes (minimum 8192 for Anvil header)",
                rf.path.display(),
                rf.size_bytes
            )
            .into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn rf(path: std::path::PathBuf, size: u64) -> RegionFile {
        RegionFile {
            path,
            size_bytes: size,
            rx: 0,
            rz: 0,
        }
    }

    #[test]
    fn passes_when_thresholds_met() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("r.0.0.mca");
        fs::write(&p, vec![0u8; 8192]).unwrap();
        let region_files = vec![rf(p, 8192)];
        let report = super::super::ValidationReport {
            is_valid: true,
            region_file_count: 1,
            total_size_bytes: 8192,
            failure_reasons: vec![],
        };
        let reasons = check(
            &ValidationConfig::default(),
            tmp.path(),
            &region_files,
            &report,
        )
        .unwrap();
        assert!(reasons.is_empty());
    }

    #[test]
    fn flags_too_few_region_files() {
        let tmp = tempfile::tempdir().unwrap();
        let region_files: Vec<RegionFile> = vec![];
        let report = super::super::ValidationReport {
            is_valid: true,
            region_file_count: 0,
            total_size_bytes: 0,
            failure_reasons: vec![],
        };
        let reasons = check(
            &ValidationConfig::default(),
            tmp.path(),
            &region_files,
            &report,
        )
        .unwrap();
        assert!(reasons
            .iter()
            .any(|r| r.starts_with("structure_too_few_region_files")));
    }

    #[test]
    fn flags_too_small_anvil_header() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("r.0.0.mca");
        fs::write(&p, vec![0u8; 100]).unwrap();
        let region_files = vec![rf(p, 100)];
        let report = super::super::ValidationReport {
            is_valid: true,
            region_file_count: 1,
            total_size_bytes: 100,
            failure_reasons: vec![],
        };
        let reasons = check(
            &ValidationConfig::default(),
            tmp.path(),
            &region_files,
            &report,
        )
        .unwrap();
        assert!(reasons
            .iter()
            .any(|r| r.starts_with("structure_anvil_header_invalid")));
    }
}
