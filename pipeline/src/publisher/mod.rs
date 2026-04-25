use crate::locations::Location;
use std::path::{Path, PathBuf};
use tracing::info;

/// Publishes validated maps to a configured output target.
pub struct Publisher {
    output_dir: PathBuf,
}

impl Publisher {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_path_buf(),
        }
    }

    /// Publish a generated map to the output directory.
    pub fn publish(
        &self,
        source: &Path,
        location: &Location,
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let dest_name = Self::sanitize_name(&location.name);
        let dest = self.output_dir.join("published").join(&dest_name);

        if dest.exists() {
            std::fs::remove_dir_all(&dest)?;
        }

        std::fs::create_dir_all(dest.parent().unwrap_or(&self.output_dir))?;
        Self::copy_dir_recursive(source, &dest)?;

        info!(
            source = %source.display(),
            dest = %dest.display(),
            "Map published"
        );

        Ok(dest)
    }

    fn sanitize_name(name: &str) -> String {
        name.chars()
            .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '_' })
            .collect()
    }

    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
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
    fn test_publish_copies_files() {
        let tmp = tempfile::tempdir().unwrap();
        let source = tmp.path().join("source");
        let output = tmp.path().join("output");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("test.txt"), b"hello").unwrap();

        let loc = Location {
            name: "Test City".into(),
            state: "CA".into(),
            bbox: [0.0, 0.0, 1.0, 1.0],
            tier: "small".into(),
            tags: vec![],
        };

        let publisher = Publisher::new(&output);
        let dest = publisher.publish(&source, &loc).unwrap();

        assert!(dest.exists());
        assert!(dest.join("test.txt").exists());
        let content = fs::read_to_string(dest.join("test.txt")).unwrap();
        assert_eq!(content, "hello");
    }

    #[test]
    fn test_sanitize_name() {
        assert_eq!(Publisher::sanitize_name("New York City"), "New_York_City");
        assert_eq!(Publisher::sanitize_name("St. Louis, MO"), "St__Louis__MO");
    }
}
