//! Shared helpers for reading Anvil region files. The validator's per-check
//! modules (`ground`, `interior`, `surface_diversity`) all need to enumerate
//! the .mca files in a region directory, parse out the (region_x, region_z)
//! coordinates from the filename, sample chunks, and walk block columns —
//! this module concentrates that boilerplate so the checks themselves can
//! stay focused on the rule they enforce.
//!
//! Built on `fastanvil` 0.32. We use `JavaChunk` (the typed pre-1.18-and-up
//! deserializer) rather than chasing raw NBT — fastanvil already handles the
//! pre13/pre18/post18 variants and exposes a uniform `Chunk` trait.

use fastanvil::{Chunk, JavaChunk, Region};
use std::fs::File;
use std::path::{Path, PathBuf};

/// Pointer to a region file plus the cached metadata we use across checks.
/// Splitting the path from the file keeps the file handles short-lived
/// (each check opens, reads, closes) and avoids holding hundreds of file
/// descriptors when a map has a lot of regions.
#[derive(Debug, Clone)]
pub struct RegionFile {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub rx: i32,
    pub rz: i32,
}

impl RegionFile {
    /// Number of chunks this region file *could* contain (always 32×32 in
    /// the Anvil format), used by the size-sanity check to compute a
    /// per-chunk byte budget.
    pub const MAX_CHUNKS: usize = 32 * 32;
}

/// Enumerate every `r.<x>.<z>.mca` file directly under `region_dir`.
/// Filenames that don't match the canonical Anvil pattern are skipped
/// silently rather than failing the whole validate() call — a stray
/// file in `region/` shouldn't sink an otherwise good map.
pub fn list_region_files(
    region_dir: &Path,
) -> Result<Vec<RegionFile>, Box<dyn std::error::Error + Send + Sync>> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(region_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("mca") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some((rx, rz)) = parse_region_coords(name) else {
            continue;
        };
        let size_bytes = entry.metadata()?.len();
        out.push(RegionFile {
            path,
            size_bytes,
            rx,
            rz,
        });
    }
    // Stable order: deterministic sample selection across runs.
    out.sort_by_key(|r| (r.rz, r.rx));
    Ok(out)
}

fn parse_region_coords(filename: &str) -> Option<(i32, i32)> {
    // Expected: "r.<rx>.<rz>.mca"
    let stem = filename.strip_suffix(".mca")?;
    let mut parts = stem.split('.');
    if parts.next()? != "r" {
        return None;
    }
    let rx: i32 = parts.next()?.parse().ok()?;
    let rz: i32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((rx, rz))
}

/// Open a region file for reading and produce a fastanvil `Region` over it.
pub fn open_region(
    rf: &RegionFile,
) -> Result<Region<File>, Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(&rf.path)?;
    Ok(Region::from_stream(file)?)
}

/// Iterate every chunk in a region, calling `f` on each successfully
/// parsed chunk and its (chunk_x_in_region, chunk_z_in_region) coords
/// (each in 0..32). Errors on individual chunks are logged and skipped
/// — one corrupt chunk should not block the whole check from running.
pub fn for_each_chunk<F>(
    rf: &RegionFile,
    mut f: F,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: FnMut(usize, usize, &JavaChunk),
{
    let mut region = open_region(rf)?;
    for chunk_data in region.iter() {
        let chunk_data = match chunk_data {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!(region = %rf.path.display(), error = %e, "Skipping unreadable chunk");
                continue;
            }
        };
        let chunk = match JavaChunk::from_bytes(&chunk_data.data) {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!(region = %rf.path.display(), x = chunk_data.x, z = chunk_data.z, error = %e, "Skipping unparseable chunk");
                continue;
            }
        };
        f(chunk_data.x, chunk_data.z, &chunk);
    }
    Ok(())
}

/// Walk a block column from `y_min` up to `y_max` inclusive at `(x, z)`
/// inside `chunk` (chunk-local coords; x,z each in 0..16) and return the
/// names of the blocks at each y. None entries mean the chunk has no
/// section covering that y — treat as air.
pub fn column_block_names(
    chunk: &JavaChunk,
    x: usize,
    z: usize,
    y_min: i32,
    y_max: i32,
) -> Vec<Option<String>> {
    let mut out = Vec::with_capacity((y_max - y_min + 1).max(0) as usize);
    for y in y_min..=y_max {
        let block = chunk.block(x, y as isize, z);
        out.push(block.map(|b| b.name().to_string()));
    }
    out
}

/// Surface height for `(x, z)` in chunk-local coords, using fastanvil's
/// motion-blocking heightmap. The result is the y-coordinate of the
/// topmost solid block (or the heightmap's best estimate). Returns
/// `None` if the heightmap can't be resolved.
pub fn surface_height(chunk: &JavaChunk, x: usize, z: usize) -> Option<i32> {
    let h = chunk.surface_height(x, z, fastanvil::HeightMode::Calculate);
    Some(h as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_region_coords_canonical() {
        assert_eq!(parse_region_coords("r.0.0.mca"), Some((0, 0)));
        assert_eq!(parse_region_coords("r.-1.2.mca"), Some((-1, 2)));
        assert_eq!(parse_region_coords("r.10.-3.mca"), Some((10, -3)));
    }

    #[test]
    fn parse_region_coords_rejects_bad() {
        assert_eq!(parse_region_coords("foo.mca"), None);
        assert_eq!(parse_region_coords("r.0.mca"), None);
        assert_eq!(parse_region_coords("r.0.0.0.mca"), None);
        assert_eq!(parse_region_coords("r.0.0"), None);
    }

    #[test]
    fn list_region_files_orders_deterministically() {
        let tmp = tempfile::tempdir().unwrap();
        for (rx, rz) in [(1, 0), (0, 1), (0, 0)] {
            let path = tmp.path().join(format!("r.{rx}.{rz}.mca"));
            std::fs::write(&path, vec![0u8; 8192]).unwrap();
        }
        let listed = list_region_files(tmp.path()).unwrap();
        let coords: Vec<(i32, i32)> = listed.iter().map(|r| (r.rx, r.rz)).collect();
        assert_eq!(coords, vec![(0, 0), (1, 0), (0, 1)]);
    }

    #[test]
    fn list_region_files_skips_non_mca() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("r.0.0.mca"), vec![0u8; 8192]).unwrap();
        std::fs::write(tmp.path().join("readme.txt"), b"ignore me").unwrap();
        std::fs::write(tmp.path().join("rn.0.0.mca.bak"), vec![0u8; 8192]).unwrap();
        let listed = list_region_files(tmp.path()).unwrap();
        assert_eq!(listed.len(), 1);
    }
}
