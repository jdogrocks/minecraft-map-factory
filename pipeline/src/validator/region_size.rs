//! Region-file size sanity (MIN-43 #3). Catches the `4,202,496-byte`
//! empty-chunks signature explicitly with a named failure reason, plus a
//! tunable per-region bytes-per-chunk floor for the more general case
//! where a region file is small but not exactly the signature size.

use super::anvil::RegionFile;
use crate::config::ValidationConfig;

pub fn check(config: &ValidationConfig, region_files: &[RegionFile]) -> Vec<String> {
    let mut reasons = Vec::new();

    for rf in region_files {
        // 1) Exact 4,202,496-byte signature — call this out by name. The
        //    CTO singled this out as the diagnostic that motivated MIN-40
        //    and it should never get blurred into a generic "too small"
        //    bucket.
        if rf.size_bytes == config.region_empty_signature_bytes {
            reasons.push(format!(
                "region_empty_chunks_signature: {} is exactly {} bytes — \
                 chunks were allocated but contain no terrain (MIN-40)",
                rf.path.display(),
                config.region_empty_signature_bytes
            ));
            // Don't double-report on the per-chunk threshold below for
            // this same file — the signature reason already explains it.
            continue;
        }

        // 2) Per-region bytes-per-chunk floor. A region file is 32×32
        //    chunks; tiny region files imply most chunks have no terrain
        //    even if the byte count happens not to land on the empirical
        //    signature.
        let per_chunk = rf.size_bytes / RegionFile::MAX_CHUNKS as u64;
        if per_chunk < config.region_min_bytes_per_chunk {
            reasons.push(format!(
                "region_too_small_per_chunk: {} is {} bytes ({} B/chunk \
                 across {} chunks), below the {} B/chunk floor",
                rf.path.display(),
                rf.size_bytes,
                per_chunk,
                RegionFile::MAX_CHUNKS,
                config.region_min_bytes_per_chunk
            ));
        }
    }

    reasons
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn rf(name: &str, size: u64) -> RegionFile {
        RegionFile {
            path: PathBuf::from(name),
            size_bytes: size,
            rx: 0,
            rz: 0,
        }
    }

    #[test]
    fn flags_exact_4_202_496_signature() {
        let cfg = ValidationConfig::default();
        let reasons = check(&cfg, &[rf("r.0.0.mca", 4_202_496)]);
        assert_eq!(reasons.len(), 1);
        assert!(reasons[0].starts_with("region_empty_chunks_signature"));
        assert!(reasons[0].contains("4202496"));
    }

    #[test]
    fn passes_when_well_above_threshold() {
        // Times Square: 7,753,728 B → ~7.5 KiB/chunk, well above 4,200.
        let cfg = ValidationConfig::default();
        let reasons = check(&cfg, &[rf("r.0.0.mca", 7_753_728)]);
        assert!(reasons.is_empty(), "unexpected reasons: {reasons:?}");
    }

    #[test]
    fn flags_per_chunk_floor_for_non_signature_size() {
        // 4,200,000 B → ~4,101 B/chunk: below the 4,200 default floor
        // and not the exact signature.
        let cfg = ValidationConfig::default();
        let reasons = check(&cfg, &[rf("r.0.0.mca", 4_200_000)]);
        assert_eq!(reasons.len(), 1);
        assert!(reasons[0].starts_with("region_too_small_per_chunk"));
    }

    #[test]
    fn signature_match_does_not_double_report_per_chunk() {
        let cfg = ValidationConfig::default();
        let reasons = check(&cfg, &[rf("r.0.0.mca", 4_202_496)]);
        assert!(!reasons
            .iter()
            .any(|r| r.starts_with("region_too_small_per_chunk")));
    }

    #[test]
    fn aggregates_reasons_across_multiple_region_files() {
        let cfg = ValidationConfig::default();
        let reasons = check(
            &cfg,
            &[
                rf("r.0.0.mca", 4_202_496),
                rf("r.1.0.mca", 4_202_496),
                rf("r.2.0.mca", 12_000_000),
            ],
        );
        assert_eq!(reasons.len(), 2);
    }
}
