use std::path::PathBuf;

/// Subdirectory name for tile cache within the OS cache directory
const TILE_CACHE_DIR_NAME: &str = "arnis-tile-cache";

/// Maximum age for cached tiles in days before they are cleaned up
const TILE_CACHE_MAX_AGE_DAYS: u64 = 7;

/// Returns the tile cache directory path for a specific provider.
/// Uses the OS-standard cache directory (e.g. AppData/Local on Windows, ~/.cache on Linux).
/// Falls back to ./arnis-tile-cache if the OS cache directory is unavailable.
pub fn get_cache_dir(provider_name: &str) -> PathBuf {
    let base = if let Some(cache_dir) = dirs::cache_dir() {
        cache_dir.join(TILE_CACHE_DIR_NAME)
    } else {
        PathBuf::from(format!("./{TILE_CACHE_DIR_NAME}"))
    };
    base.join(provider_name)
}

/// Returns the base tile cache directory path (without provider subdirectory).
pub fn get_base_cache_dir() -> PathBuf {
    if let Some(cache_dir) = dirs::cache_dir() {
        cache_dir.join(TILE_CACHE_DIR_NAME)
    } else {
        PathBuf::from(format!("./{TILE_CACHE_DIR_NAME}"))
    }
}

/// Summary of a cache-clear operation, returned to the GUI so it can
/// report "cleared N files, freed X MB" to the user.
#[derive(Clone, Copy, Debug, Default)]
pub struct CacheClearStats {
    pub files_deleted: u64,
    pub bytes_freed: u64,
    pub errors: u64,
}

impl CacheClearStats {
    /// Combine two stats values — used when we clean multiple caches
    /// (elevation + land-cover) in one UI operation.
    pub fn combined(self, other: Self) -> Self {
        Self {
            files_deleted: self.files_deleted + other.files_deleted,
            bytes_freed: self.bytes_freed + other.bytes_freed,
            errors: self.errors + other.errors,
        }
    }
}

/// Recursively remove everything inside `dir`, leaving `dir` itself in
/// place (so subsequent cache writes don't need to recreate the root
/// handle). Missing directory is a no-op.
///
/// Safety considerations implemented here:
/// - Symlinks are removed but not followed. We never recurse into an
///   arbitrary filesystem the user may have pointed at with a stray
///   symlink inside the cache.
/// - Unreadable entries contribute to the error count instead of
///   propagating — the GUI surfaces the error count as a warning.
/// - No panics: every fs call is matched; transient errors (e.g. a
///   file busy-locked by another reader) just increment `errors`.
pub fn clear_cache_dir(dir: &std::path::Path) -> CacheClearStats {
    let mut stats = CacheClearStats::default();
    // Use `symlink_metadata` so a symlink pointing to another directory
    // doesn't trick us into recursing through it — `read_dir` follows
    // symlinks, so even though `clear_recursive` doesn't walk across
    // individual symlinked children, an entire symlinked *root* would
    // still traverse a foreign filesystem. A non-existent root is a
    // no-op; any other error is counted and surfaced.
    let meta = match std::fs::symlink_metadata(dir) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return stats,
        Err(_) => {
            stats.errors += 1;
            return stats;
        }
    };
    if meta.file_type().is_symlink() {
        // Refuse to clear when the root itself is a symlink — we'd be
        // operating on whatever lives at the target, which may be far
        // outside the cache directory the user thinks they're wiping.
        stats.errors += 1;
        return stats;
    }
    clear_recursive(dir, &mut stats);
    stats
}

fn clear_recursive(dir: &std::path::Path, stats: &mut CacheClearStats) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => {
            stats.errors += 1;
            return;
        }
    };
    // Intentionally NOT using `.flatten()`, which would silently swallow
    // Err iterator items and under-report failures. The doc comment on
    // `clear_cache_dir` promises unreadable entries count as errors,
    // so we match each entry explicitly.
    for entry_result in entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => {
                stats.errors += 1;
                continue;
            }
        };
        let path = entry.path();
        // Use `entry.file_type()` (never follows symlinks, cross-platform)
        // before any call to `std::fs::metadata`, which DOES follow
        // symlinks and could otherwise make a link-to-directory look
        // like a real directory and cause us to recurse outside the
        // cache — the exact scenario this branch is guarding against.
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(_) => {
                stats.errors += 1;
                continue;
            }
        };
        if file_type.is_symlink() {
            // Remove the link itself; never traverse into its target.
            if std::fs::remove_file(&path).is_err() {
                stats.errors += 1;
            }
            continue;
        }
        if file_type.is_dir() {
            clear_recursive(&path, stats);
            // `remove_dir` only succeeds once the directory is empty;
            // if a nested clear left something behind that's already
            // reflected in `errors` so we just note another failure.
            if std::fs::remove_dir(&path).is_err() {
                stats.errors += 1;
            }
            continue;
        }
        if file_type.is_file() {
            // Only look up size *after* confirming this is a regular
            // file — symlink_metadata avoids accidentally resolving
            // anything surprising between the type check and the read.
            let size = std::fs::symlink_metadata(&path)
                .map(|m| m.len())
                .unwrap_or(0);
            match std::fs::remove_file(&path) {
                Ok(()) => {
                    stats.files_deleted += 1;
                    stats.bytes_freed += size;
                }
                Err(_) => stats.errors += 1,
            }
        }
    }
}

/// Clear every cached elevation tile across all providers. The root
/// cache directory itself is left in place.
pub fn clear_all_cached_tiles() -> CacheClearStats {
    clear_cache_dir(&get_base_cache_dir())
}

/// Cleans up old cached files from all provider cache directories.
/// Only deletes files older than TILE_CACHE_MAX_AGE_DAYS.
pub fn cleanup_old_cached_files() {
    let base_dir = get_base_cache_dir();

    // Mirror the `clear_cache_dir` safety check: refuse to walk when
    // the root is itself a symlink, since `read_dir` would follow it
    // out of the cache root. Missing directory is a silent no-op (the
    // startup path runs this best-effort on every launch).
    let meta = match std::fs::symlink_metadata(&base_dir) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
        Err(e) => {
            eprintln!("Warning: cannot stat cache dir {}: {e}", base_dir.display());
            return;
        }
    };
    if !meta.is_dir() || meta.file_type().is_symlink() {
        return;
    }

    let max_age = std::time::Duration::from_secs(TILE_CACHE_MAX_AGE_DAYS * 24 * 60 * 60);
    let now = std::time::SystemTime::now();
    let mut deleted_count = 0;
    let mut error_count = 0;

    // Walk all files in the cache directory tree
    cleanup_dir_recursive(
        &base_dir,
        max_age,
        now,
        &mut deleted_count,
        &mut error_count,
    );

    if deleted_count > 0 {
        println!(
            "Cleaned up {deleted_count} old cached elevation files (older than {TILE_CACHE_MAX_AGE_DAYS} days)"
        );
    }
    if error_count > 1 {
        eprintln!("Warning: Failed to delete {error_count} old cached files");
    }
}

fn cleanup_dir_recursive(
    dir: &std::path::Path,
    max_age: std::time::Duration,
    now: std::time::SystemTime,
    deleted_count: &mut u32,
    error_count: &mut u32,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    // Same reasoning as `clear_recursive`: `DirEntry::file_type()`
    // never follows symlinks on any platform, while `Path::is_dir()` /
    // `is_file()` DO. Following a symlink here could make a link-to-
    // directory look like a real directory and cause us to age-delete
    // files outside the cache root entirely.
    for entry_result in entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if file_type.is_symlink() {
            // Don't touch symlinks from the aging-cleanup path. The
            // explicit "Clear Tile Cache" button is the one place we
            // remove cache entries that happen to be links; from here
            // it's safer to leave them alone.
            continue;
        }

        if file_type.is_dir() {
            cleanup_dir_recursive(&path, max_age, now, deleted_count, error_count);
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let modified = match metadata.modified() {
            Ok(time) => time,
            Err(_) => continue,
        };

        let age = match now.duration_since(modified) {
            Ok(duration) => duration,
            Err(_) => continue,
        };

        if age > max_age {
            match std::fs::remove_file(&path) {
                Ok(()) => *deleted_count += 1,
                Err(e) => {
                    if *error_count == 0 {
                        eprintln!(
                            "Warning: Failed to delete old cached file {}: {e}",
                            path.display()
                        );
                    }
                    *error_count += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_missing_dir_is_noop() {
        let stats = clear_cache_dir(std::path::Path::new("this/path/does/not/exist/ever"));
        assert_eq!(stats.files_deleted, 0);
        assert_eq!(stats.bytes_freed, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn clear_populated_dir_counts_and_frees() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let sub = root.join("provider");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(sub.join("a.bin"), b"hello").unwrap();
        std::fs::write(sub.join("b.bin"), b"world!").unwrap();
        std::fs::write(root.join("c.bin"), b"x").unwrap();

        let stats = clear_cache_dir(root);
        assert_eq!(stats.files_deleted, 3);
        assert_eq!(stats.bytes_freed, 5 + 6 + 1);
        assert_eq!(stats.errors, 0);
        // Root itself must still exist; only its contents were wiped.
        assert!(root.exists());
        assert!(!sub.exists());
    }

    /// The root directory being a symlink is rejected wholesale, even
    /// if it happens to point at another valid cache-shaped directory.
    /// Regression: prior implementation would have happily `read_dir`ed
    /// the target, which is outside the cache root the caller named.
    /// Symlink creation on Windows usually requires dev-mode / admin,
    /// so this test is Unix-only.
    #[cfg(unix)]
    #[test]
    fn clear_refuses_symlink_root() {
        let tmp = tempfile::tempdir().unwrap();
        let real = tmp.path().join("real");
        std::fs::create_dir(&real).unwrap();
        std::fs::write(real.join("important.txt"), b"dont delete me").unwrap();
        let link = tmp.path().join("link");
        std::os::unix::fs::symlink(&real, &link).unwrap();

        let stats = clear_cache_dir(&link);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.files_deleted, 0);
        // Target must be untouched.
        assert!(real.join("important.txt").exists());
    }

    // ── get_cache_dir ──────────────────────────────────────────────

    #[test]
    fn cache_dir_includes_provider_name() {
        let dir = get_cache_dir("aws_terrain");
        let path_str = dir.to_string_lossy();
        assert!(
            path_str.contains("aws_terrain"),
            "Expected provider in path: {path_str}"
        );
        assert!(
            path_str.contains(TILE_CACHE_DIR_NAME),
            "Expected cache dir name in path: {path_str}"
        );
    }

    #[test]
    fn cache_dir_different_providers_differ() {
        let dir_a = get_cache_dir("provider_a");
        let dir_b = get_cache_dir("provider_b");
        assert_ne!(dir_a, dir_b);
    }

    // ── get_base_cache_dir ─────────────────────────────────────────

    #[test]
    fn base_cache_dir_contains_cache_name() {
        let dir = get_base_cache_dir();
        let path_str = dir.to_string_lossy();
        assert!(path_str.contains(TILE_CACHE_DIR_NAME));
    }

    // ── CacheClearStats ────────────────────────────────────────────

    #[test]
    fn cache_clear_stats_default() {
        let stats = CacheClearStats::default();
        assert_eq!(stats.files_deleted, 0);
        assert_eq!(stats.bytes_freed, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn cache_clear_stats_combined() {
        let a = CacheClearStats {
            files_deleted: 5,
            bytes_freed: 1000,
            errors: 1,
        };
        let b = CacheClearStats {
            files_deleted: 3,
            bytes_freed: 500,
            errors: 2,
        };
        let c = a.combined(b);
        assert_eq!(c.files_deleted, 8);
        assert_eq!(c.bytes_freed, 1500);
        assert_eq!(c.errors, 3);
    }

    // ── cleanup_old_cached_files ────────────────────────────────────

    #[test]
    fn cleanup_dir_recursive_removes_old_files() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        // Create a file and manually set its modified time to the past
        let old_file = root.join("old.bin");
        std::fs::write(&old_file, b"old data").unwrap();

        // Use a very short max_age so the file is "old"
        let max_age = std::time::Duration::from_secs(0);
        let now = std::time::SystemTime::now();
        let mut deleted = 0u32;
        let mut errors = 0u32;

        cleanup_dir_recursive(root, max_age, now, &mut deleted, &mut errors);

        assert!(deleted >= 1);
        assert_eq!(errors, 0);
    }

    #[test]
    fn cleanup_dir_recursive_keeps_new_files() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        let new_file = root.join("new.bin");
        std::fs::write(&new_file, b"new data").unwrap();

        // Use very large max_age so nothing is old
        let max_age = std::time::Duration::from_secs(365 * 24 * 60 * 60);
        let now = std::time::SystemTime::now();
        let mut deleted = 0u32;
        let mut errors = 0u32;

        cleanup_dir_recursive(root, max_age, now, &mut deleted, &mut errors);

        assert_eq!(deleted, 0);
        assert!(new_file.exists());
    }

    #[test]
    fn cleanup_dir_recursive_missing_dir() {
        let mut deleted = 0u32;
        let mut errors = 0u32;
        cleanup_dir_recursive(
            std::path::Path::new("/nonexistent/path"),
            std::time::Duration::from_secs(1),
            std::time::SystemTime::now(),
            &mut deleted,
            &mut errors,
        );
        assert_eq!(deleted, 0);
    }

    // ── clear_cache_dir nested dirs ────────────────────────────────

    #[test]
    fn clear_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let stats = clear_cache_dir(tmp.path());
        assert_eq!(stats.files_deleted, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn clear_deeply_nested_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let deep = tmp.path().join("a").join("b").join("c");
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(deep.join("file.txt"), b"data").unwrap();

        let stats = clear_cache_dir(tmp.path());
        assert_eq!(stats.files_deleted, 1);
        assert_eq!(stats.errors, 0);
    }
}
