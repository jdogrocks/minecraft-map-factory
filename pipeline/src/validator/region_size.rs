//! Region-file size sanity (MIN-43 #3). Size-based checks were removed in
//! MIN-44 because they produced false positives on legitimate sparse terrain
//! (parks, open areas) after the MIN-100 stone-variant entropy fix: sparse
//! chunks still compress to ≤4096 bytes (1 Anvil sector), making even
//! fully-populated maps hit the old 4,202,496-byte "empty" signature.
//! Content-based checks (ground_continuity, surface_diversity) now cover
//! the failure mode these heuristics were proxying for.

use super::anvil::RegionFile;
use crate::config::ValidationConfig;

pub fn check(_config: &ValidationConfig, _region_files: &[RegionFile]) -> Vec<String> {
    Vec::new()
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
    fn always_passes_regardless_of_size() {
        let cfg = ValidationConfig::default();
        // All of these previously false-positived on sparse-terrain maps.
        for size in [4_202_496, 4_200_000, 100, 7_753_728, 12_000_000] {
            let reasons = check(&cfg, &[rf("r.0.0.mca", size)]);
            assert!(reasons.is_empty(), "size {size} produced reasons: {reasons:?}");
        }
    }
}
