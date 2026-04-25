use crate::config::ValidationConfig;
use std::path::Path;
use tracing::debug;

/// Report from validating a generated map.
#[derive(Debug)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub region_file_count: usize,
    pub total_size_bytes: u64,
    pub failure_reasons: Vec<String>,
}

/// Validates generated Minecraft maps.
pub struct Validator {
    config: ValidationConfig,
}

impl Validator {
    pub fn new(config: &ValidationConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Validate a generated map at the given path.
    pub fn validate(&self, map_path: &Path) -> Result<ValidationReport, Box<dyn std::error::Error + Send + Sync>> {
        let mut reasons = Vec::new();

        // Count region files
        let region_dir = map_path.join("region");
        let region_file_count = if region_dir.exists() {
            Self::count_region_files(&region_dir)?
        } else {
            // Check if map_path itself contains region files (world may be nested)
            Self::find_region_count(map_path)?
        };

        debug!(region_file_count, "Region files found");

        if region_file_count < self.config.min_region_files {
            reasons.push(format!(
                "Too few region files: {} (minimum: {})",
                region_file_count, self.config.min_region_files
            ));
        }

        // Check total size
        let total_size_bytes = Self::dir_size(map_path)?;
        debug!(total_size_bytes, "Total map size");

        if total_size_bytes < self.config.min_map_size_bytes {
            reasons.push(format!(
                "Map too small: {} bytes (minimum: {})",
                total_size_bytes, self.config.min_map_size_bytes
            ));
        }

        // Validate region file structure if configured
        if self.config.validate_structure && region_file_count > 0 {
            let region_path = if region_dir.exists() {
                region_dir
            } else {
                Self::find_region_dir(map_path).unwrap_or(region_dir)
            };

            if let Err(e) = Self::validate_region_structure(&region_path) {
                reasons.push(format!("Region structure invalid: {e}"));
            }
        }

        Ok(ValidationReport {
            is_valid: reasons.is_empty(),
            region_file_count,
            total_size_bytes,
            failure_reasons: reasons,
        })
    }

    fn count_region_files(region_dir: &Path) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let count = std::fs::read_dir(region_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .is_some_and(|ext| ext == "mca")
            })
            .count();
        Ok(count)
    }

    fn find_region_count(map_path: &Path) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(region_dir) = Self::find_region_dir(map_path) {
            Self::count_region_files(&region_dir)
        } else {
            Ok(0)
        }
    }

    fn find_region_dir(base: &Path) -> Option<std::path::PathBuf> {
        // Look for a "region" directory recursively (up to 3 levels deep)
        for depth_entry in std::fs::read_dir(base).ok()?.flatten() {
            let path = depth_entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "region") {
                    return Some(path);
                }
                // Check one level deeper
                for sub in std::fs::read_dir(&path).ok()?.flatten() {
                    let sub_path = sub.path();
                    if sub_path.is_dir() && sub_path.file_name().is_some_and(|n| n == "region") {
                        return Some(sub_path);
                    }
                }
            }
        }
        None
    }

    fn dir_size(path: &Path) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut total = 0u64;
        if path.is_file() {
            return Ok(std::fs::metadata(path)?.len());
        }
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            if meta.is_file() {
                total += meta.len();
            } else if meta.is_dir() {
                total += Self::dir_size(&entry.path())?;
            }
        }
        Ok(total)
    }

    fn validate_region_structure(region_dir: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Validate Anvil region file format:
        // Each .mca file must be at least 8192 bytes (two 4096-byte tables)
        for entry in std::fs::read_dir(region_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "mca") {
                let meta = std::fs::metadata(&path)?;
                if meta.len() < 8192 {
                    return Err(format!(
                        "Region file {} is too small ({} bytes, minimum 8192)",
                        path.display(),
                        meta.len()
                    )
                    .into());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validate_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let validator = Validator::new(&ValidationConfig::default());
        let report = validator.validate(tmp.path()).unwrap();
        assert!(!report.is_valid);
        assert_eq!(report.region_file_count, 0);
    }

    #[test]
    fn test_validate_with_region_files() {
        let tmp = tempfile::tempdir().unwrap();
        let region_dir = tmp.path().join("region");
        fs::create_dir(&region_dir).unwrap();

        // Create a valid-sized region file
        let region_file = region_dir.join("r.0.0.mca");
        let data = vec![0u8; 8192];
        fs::write(&region_file, &data).unwrap();

        let config = ValidationConfig {
            min_region_files: 1,
            min_map_size_bytes: 1024,
            validate_structure: true,
        };
        let validator = Validator::new(&config);
        let report = validator.validate(tmp.path()).unwrap();
        assert!(report.is_valid);
        assert_eq!(report.region_file_count, 1);
        assert!(report.total_size_bytes >= 8192);
    }

    #[test]
    fn test_validate_corrupt_region() {
        let tmp = tempfile::tempdir().unwrap();
        let region_dir = tmp.path().join("region");
        fs::create_dir(&region_dir).unwrap();

        // Create a too-small region file
        let region_file = region_dir.join("r.0.0.mca");
        fs::write(&region_file, b"tiny").unwrap();

        let config = ValidationConfig {
            min_region_files: 1,
            min_map_size_bytes: 1,
            validate_structure: true,
        };
        let validator = Validator::new(&config);
        let report = validator.validate(tmp.path()).unwrap();
        assert!(!report.is_valid);
        assert!(report.failure_reasons.iter().any(|r| r.contains("too small")));
    }

}
