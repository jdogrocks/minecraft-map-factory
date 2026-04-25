//! Common data structures for world modification.
//!
//! This module contains the internal data structures used to track block changes
//! before they are written to either Java or Bedrock format.

use crate::block_definitions::*;

/// Minimum Y coordinate in Minecraft (1.18+)
pub const MIN_Y: i32 = -64;
/// Maximum Y coordinate in Minecraft (data pack maximum: 2031)
/// Vanilla limit is 319, but data packs can extend this up to 2031.
/// The world editor supports the full range; the elevation system controls
/// the actual heights used based on the disable_height_limit setting.
const MAX_Y: i32 = 2031;
use fastnbt::{LongArray, Value};
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};

/// Chunk structure for Java Edition NBT format
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Chunk {
    pub sections: Vec<Section>,
    pub x_pos: i32,
    pub z_pos: i32,
    #[serde(default)]
    pub is_light_on: u8,
    #[serde(flatten)]
    pub other: FnvHashMap<String, Value>,
}

/// Section within a chunk (16x16x16 blocks)
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Section {
    pub block_states: Blockstates,
    #[serde(rename = "Y")]
    pub y: i8,
    #[serde(flatten)]
    pub other: FnvHashMap<String, Value>,
}

/// Block states within a section
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Blockstates {
    pub palette: Vec<PaletteItem>,
    pub data: Option<LongArray>,
    #[serde(flatten)]
    pub other: FnvHashMap<String, Value>,
}

/// Palette item for block state encoding
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct PaletteItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<Value>,
}

/// Block storage strategy for a 16×16×16 section.
///
/// **Memory optimisation**: instead of always allocating a 4 096-byte array,
/// we distinguish two cases:
///
/// * `Uniform(block)` – every position holds the same block (1 byte).
///   This covers freshly-created (all-AIR) sections, and sections that were
///   entirely filled with one type (e.g. STONE underground with `--fillground`).
///
/// * `Full(Vec<Block>)` – the general case, equivalent to the old `[Block; 4096]`
///   but heap-allocated via `Vec` so the *inline* size inside the parent
///   `FnvHashMap` entry is only 24 bytes (pointer + length + capacity) instead
///   of 4 096 bytes.  This eliminates huge HashMap-slot waste from unused
///   capacity slots.
pub(crate) enum BlockStorage {
    /// Every position is the same block (commonly AIR).
    Uniform(Block),
    /// Mixed blocks – always exactly 4 096 entries.
    Full(Vec<Block>),
}

impl BlockStorage {
    /// Read block at flat `index` (0..4095).
    #[inline(always)]
    pub fn get(&self, index: usize) -> Block {
        match self {
            BlockStorage::Uniform(b) => *b,
            BlockStorage::Full(v) => v[index],
        }
    }

    /// Write block at flat `index`.
    /// Promotes `Uniform` → `Full` on the first differing write.
    #[inline]
    pub fn set(&mut self, index: usize, block: Block) {
        match self {
            BlockStorage::Uniform(b) if *b == block => {
                // No-op – writing the same value.
            }
            BlockStorage::Uniform(base) => {
                let base = *base;
                let mut v = vec![base; 4096];
                v[index] = block;
                *self = BlockStorage::Full(v);
            }
            BlockStorage::Full(v) => {
                v[index] = block;
            }
        }
    }

    /// Iterate over all 4 096 blocks.
    #[inline]
    pub fn iter(&self) -> BlockStorageIter<'_> {
        match self {
            BlockStorage::Uniform(b) => BlockStorageIter::Uniform(*b, 0),
            BlockStorage::Full(v) => BlockStorageIter::Full(v.iter()),
        }
    }

    /// Try to collapse a `Full` vec back to `Uniform` if every entry
    /// is the same block.  Frees the 4 KiB heap allocation.
    pub fn try_compact(&mut self) {
        if let BlockStorage::Full(v) = self {
            if let Some(&first) = v.first() {
                if v.iter().all(|&b| b == first) {
                    *self = BlockStorage::Uniform(first);
                }
            }
        }
    }
}

/// Iterator returned by [`BlockStorage::iter`].
pub(crate) enum BlockStorageIter<'a> {
    Uniform(Block, usize),
    Full(std::slice::Iter<'a, Block>),
}

impl<'a> Iterator for BlockStorageIter<'a> {
    type Item = Block;

    #[inline]
    fn next(&mut self) -> Option<Block> {
        match self {
            BlockStorageIter::Uniform(b, count) => {
                if *count < 4096 {
                    *count += 1;
                    Some(*b)
                } else {
                    None
                }
            }
            BlockStorageIter::Full(it) => it.next().copied(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = match self {
            BlockStorageIter::Uniform(_, c) => 4096 - *c,
            BlockStorageIter::Full(it) => it.len(),
        };
        (rem, Some(rem))
    }
}

impl ExactSizeIterator for BlockStorageIter<'_> {}

/// A section being modified (16x16x16 blocks)
pub(crate) struct SectionToModify {
    pub storage: BlockStorage,
    /// Store properties for blocks that have them, indexed by the same index as blocks array
    pub properties: FnvHashMap<usize, Value>,
}

impl SectionToModify {
    #[inline]
    pub fn get_block(&self, x: u8, y: u8, z: u8) -> Option<Block> {
        let b = self.storage.get(Self::index(x, y, z));
        if b == AIR {
            return None;
        }
        Some(b)
    }

    #[inline]
    pub fn set_block(&mut self, x: u8, y: u8, z: u8, block: Block) {
        let index = Self::index(x, y, z);
        self.storage.set(index, block);
        self.properties.remove(&index);
    }

    #[inline]
    pub fn set_block_with_properties(
        &mut self,
        x: u8,
        y: u8,
        z: u8,
        block_with_props: BlockWithProperties,
    ) {
        let index = Self::index(x, y, z);
        self.storage.set(index, block_with_props.block);

        // Store properties if they exist
        if let Some(props) = block_with_props.properties {
            self.properties.insert(index, props);
        } else {
            // Remove any existing properties for this position
            self.properties.remove(&index);
        }
    }

    /// Read block at a raw flat index (used by Bedrock serialiser).
    #[inline(always)]
    pub fn get_block_at_index(&self, index: usize) -> Block {
        self.storage.get(index)
    }

    /// Calculate index from coordinates (YZX order)
    #[inline(always)]
    pub fn index(x: u8, y: u8, z: u8) -> usize {
        usize::from(y) % 16 * 256 + usize::from(z) * 16 + usize::from(x)
    }

    /// Try to collapse the block array back to `Uniform` if every entry
    /// is the same block and there are no properties.
    pub fn compact(&mut self) {
        if self.properties.is_empty() {
            self.storage.try_compact();
        }
    }

    /// Convert to Java Edition section format
    pub fn to_section(&self, y: i8) -> Section {
        // Fast path: Uniform section → single palette entry, no data array needed.
        // Only valid when no per-index properties exist, otherwise we must
        // fall through to the general path so every index is checked.
        if self.properties.is_empty() {
            if let BlockStorage::Uniform(block) = &self.storage {
                let palette_item = PaletteItem {
                    name: format!("{}:{}", block.namespace(), block.name()),
                    properties: block.properties(),
                };
                return Section {
                    block_states: Blockstates {
                        palette: vec![palette_item],
                        data: None,
                        other: FnvHashMap::default(),
                    },
                    y,
                    other: FnvHashMap::default(),
                };
            }
        }

        // General path: mixed blocks.
        // Create a map of unique block+properties combinations to palette indices
        let mut unique_blocks: Vec<(Block, Option<Value>)> = Vec::new();
        let mut palette_lookup: FnvHashMap<(Block, Option<String>), usize> = FnvHashMap::default();

        // Build unique block combinations and lookup table
        for (i, block) in self.storage.iter().enumerate() {
            let properties = self.properties.get(&i).cloned();

            // Create a key for the lookup (block + properties hash)
            let props_key = properties.as_ref().map(|p| format!("{p:?}"));
            let lookup_key = (block, props_key);

            if let std::collections::hash_map::Entry::Vacant(e) = palette_lookup.entry(lookup_key) {
                let palette_index = unique_blocks.len();
                e.insert(palette_index);
                unique_blocks.push((block, properties));
            }
        }

        let mut bits_per_block = 4; // minimum allowed
        while (1 << bits_per_block) < unique_blocks.len() {
            bits_per_block += 1;
        }

        let mut data = vec![];
        let mut cur = 0;
        let mut cur_idx = 0;

        for (i, block) in self.storage.iter().enumerate() {
            let properties = self.properties.get(&i).cloned();
            let props_key = properties.as_ref().map(|p| format!("{p:?}"));
            let lookup_key = (block, props_key);
            let p = palette_lookup[&lookup_key] as i64;

            if cur_idx + bits_per_block > 64 {
                data.push(cur);
                cur = 0;
                cur_idx = 0;
            }

            cur |= p << cur_idx;
            cur_idx += bits_per_block;
        }

        if cur_idx > 0 {
            data.push(cur);
        }

        let palette = unique_blocks
            .iter()
            .map(|(block, stored_props)| PaletteItem {
                name: format!("{}:{}", block.namespace(), block.name()),
                properties: stored_props.clone().or_else(|| block.properties()),
            })
            .collect();

        Section {
            block_states: Blockstates {
                palette,
                data: Some(LongArray::new(data)),
                other: FnvHashMap::default(),
            },
            y,
            other: FnvHashMap::default(),
        }
    }
}

impl Default for SectionToModify {
    fn default() -> Self {
        Self {
            storage: BlockStorage::Uniform(AIR),
            properties: FnvHashMap::default(),
        }
    }
}

/// A chunk being modified (16x384x16 blocks, divided into sections)
#[derive(Default)]
pub(crate) struct ChunkToModify {
    pub sections: FnvHashMap<i8, SectionToModify>,
    pub other: FnvHashMap<String, Value>,
}

impl ChunkToModify {
    #[inline]
    pub fn get_block(&self, x: u8, y: i32, z: u8) -> Option<Block> {
        // Clamp Y to valid Minecraft range to prevent TryFromIntError
        let y = y.clamp(MIN_Y, MAX_Y);
        let section_idx: i8 = (y >> 4) as i8;
        let section = self.sections.get(&section_idx)?;
        section.get_block(x, (y & 15) as u8, z)
    }

    #[inline]
    pub fn set_block(&mut self, x: u8, y: i32, z: u8, block: Block) {
        // Clamp Y to valid Minecraft range to prevent TryFromIntError
        let y = y.clamp(MIN_Y, MAX_Y);
        let section_idx: i8 = (y >> 4) as i8;
        let section = self.sections.entry(section_idx).or_default();
        section.set_block(x, (y & 15) as u8, z, block);
    }

    #[inline]
    pub fn set_block_with_properties(
        &mut self,
        x: u8,
        y: i32,
        z: u8,
        block_with_props: BlockWithProperties,
    ) {
        // Clamp Y to valid Minecraft range to prevent TryFromIntError
        let y = y.clamp(MIN_Y, MAX_Y);
        let section_idx: i8 = (y >> 4) as i8;
        let section = self.sections.entry(section_idx).or_default();
        section.set_block_with_properties(x, (y & 15) as u8, z, block_with_props);
    }

    pub fn sections(&self) -> impl Iterator<Item = Section> + '_ {
        self.sections.iter().map(|(y, s)| s.to_section(*y))
    }
}

/// A region being modified (32x32 chunks)
#[derive(Default)]
pub(crate) struct RegionToModify {
    pub chunks: FnvHashMap<(i32, i32), ChunkToModify>,
}

impl RegionToModify {
    #[inline]
    pub fn get_or_create_chunk(&mut self, x: i32, z: i32) -> &mut ChunkToModify {
        self.chunks.entry((x, z)).or_default()
    }

    #[inline]
    pub fn get_chunk(&self, x: i32, z: i32) -> Option<&ChunkToModify> {
        self.chunks.get(&(x, z))
    }
}

/// The entire world being modified.
#[derive(Default)]
pub(crate) struct WorldToModify {
    pub regions: FnvHashMap<(i32, i32), RegionToModify>,
}

impl WorldToModify {
    #[inline]
    pub fn get_or_create_region(&mut self, x: i32, z: i32) -> &mut RegionToModify {
        self.regions.entry((x, z)).or_default()
    }

    #[inline]
    pub fn get_region(&self, x: i32, z: i32) -> Option<&RegionToModify> {
        self.regions.get(&(x, z))
    }

    #[inline]
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        let chunk_x: i32 = x >> 4;
        let chunk_z: i32 = z >> 4;
        let region_x: i32 = chunk_x >> 5;
        let region_z: i32 = chunk_z >> 5;

        let region: &RegionToModify = self.get_region(region_x, region_z)?;
        let chunk: &ChunkToModify = region.get_chunk(chunk_x & 31, chunk_z & 31)?;
        chunk.get_block(
            (x & 15).try_into().unwrap(),
            y,
            (z & 15).try_into().unwrap(),
        )
    }

    #[inline]
    pub fn set_block_with_properties(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        block_with_props: BlockWithProperties,
    ) {
        let chunk_x: i32 = x >> 4;
        let chunk_z: i32 = z >> 4;
        let region_x: i32 = chunk_x >> 5;
        let region_z: i32 = chunk_z >> 5;

        let region: &mut RegionToModify = self.get_or_create_region(region_x, region_z);
        let chunk: &mut ChunkToModify = region.get_or_create_chunk(chunk_x & 31, chunk_z & 31);
        chunk.set_block_with_properties(
            (x & 15).try_into().unwrap(),
            y,
            (z & 15).try_into().unwrap(),
            block_with_props,
        );
    }

    /// Set a block only if the position is currently empty (AIR / absent).
    ///
    /// This avoids the double HashMap traversal of `get_block()` + `set_block()`
    /// which is the hot path in ground generation and many element processors.
    #[inline]
    pub fn set_block_if_absent(&mut self, x: i32, y: i32, z: i32, block: Block) {
        let chunk_x: i32 = x >> 4;
        let chunk_z: i32 = z >> 4;
        let region_x: i32 = chunk_x >> 5;
        let region_z: i32 = chunk_z >> 5;

        let region = self.regions.entry((region_x, region_z)).or_default();
        let chunk = region
            .chunks
            .entry((chunk_x & 31, chunk_z & 31))
            .or_default();

        // Clamp Y
        let y = y.clamp(MIN_Y, MAX_Y);
        let section_idx: i8 = (y >> 4) as i8;
        let section = chunk.sections.entry(section_idx).or_default();

        let local_x = (x & 15) as u8;
        let local_y = (y & 15) as u8;
        let local_z = (z & 15) as u8;
        let idx = SectionToModify::index(local_x, local_y, local_z);

        // Only write if the current block is AIR
        if section.storage.get(idx) == AIR {
            section.storage.set(idx, block);
            // Clear any stale properties from a previous block at this position
            section.properties.remove(&idx);
        }
    }

    /// Fill an entire column (single x, z) from y_min to y_max with the same block,
    /// resolving region/chunk only once.  Used by ground generation.
    #[inline]
    pub fn fill_column(
        &mut self,
        x: i32,
        z: i32,
        y_min: i32,
        y_max: i32,
        block: Block,
        skip_existing: bool,
    ) {
        let chunk_x: i32 = x >> 4;
        let chunk_z: i32 = z >> 4;
        let region_x: i32 = chunk_x >> 5;
        let region_z: i32 = chunk_z >> 5;

        let region = self.regions.entry((region_x, region_z)).or_default();
        let chunk = region
            .chunks
            .entry((chunk_x & 31, chunk_z & 31))
            .or_default();

        let local_x = (x & 15) as u8;
        let local_z = (z & 15) as u8;

        let y_min = y_min.clamp(MIN_Y, MAX_Y);
        let y_max = y_max.clamp(MIN_Y, MAX_Y);

        for y in y_min..=y_max {
            let section_idx: i8 = (y >> 4) as i8;
            let section = chunk.sections.entry(section_idx).or_default();
            let local_y = (y & 15) as u8;
            let idx = SectionToModify::index(local_x, local_y, local_z);

            if skip_existing {
                if section.storage.get(idx) == AIR {
                    section.storage.set(idx, block);
                    section.properties.remove(&idx);
                }
            } else {
                section.storage.set(idx, block);
                section.properties.remove(&idx);
            }
        }
    }

    /// Scan every section and collapse any that are entirely one block type
    /// from `Full(Vec)` back to `Uniform(Block)`, freeing the 4 KiB allocation.
    pub fn compact_sections(&mut self) {
        for region in self.regions.values_mut() {
            for chunk in region.chunks.values_mut() {
                for section in chunk.sections.values_mut() {
                    if matches!(&section.storage, BlockStorage::Full(_)) {
                        section.compact();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── BlockStorage ────────────────────────────────────────────────

    #[test]
    fn uniform_storage_returns_same_block() {
        let storage = BlockStorage::Uniform(STONE);
        for i in 0..4096 {
            assert_eq!(storage.get(i), STONE);
        }
    }

    #[test]
    fn uniform_noop_on_same_block_write() {
        let mut storage = BlockStorage::Uniform(AIR);
        storage.set(100, AIR);
        assert!(matches!(storage, BlockStorage::Uniform(b) if b == AIR));
    }

    #[test]
    fn uniform_promotes_to_full_on_different_write() {
        let mut storage = BlockStorage::Uniform(AIR);
        storage.set(42, STONE);
        assert!(matches!(storage, BlockStorage::Full(_)));
        assert_eq!(storage.get(42), STONE);
        assert_eq!(storage.get(0), AIR);
        assert_eq!(storage.get(4095), AIR);
    }

    #[test]
    fn full_storage_set_and_get() {
        let mut storage = BlockStorage::Full(vec![AIR; 4096]);
        storage.set(0, STONE);
        storage.set(4095, DIRT);
        assert_eq!(storage.get(0), STONE);
        assert_eq!(storage.get(4095), DIRT);
        assert_eq!(storage.get(100), AIR);
    }

    #[test]
    fn storage_iter_uniform() {
        let storage = BlockStorage::Uniform(GRASS_BLOCK);
        let items: Vec<_> = storage.iter().collect();
        assert_eq!(items.len(), 4096);
        assert!(items.iter().all(|&b| b == GRASS_BLOCK));
    }

    #[test]
    fn storage_iter_full() {
        let mut v = vec![AIR; 4096];
        v[0] = STONE;
        let storage = BlockStorage::Full(v);
        let items: Vec<_> = storage.iter().collect();
        assert_eq!(items.len(), 4096);
        assert_eq!(items[0], STONE);
        assert_eq!(items[1], AIR);
    }

    #[test]
    fn storage_try_compact_all_same() {
        let mut storage = BlockStorage::Full(vec![STONE; 4096]);
        storage.try_compact();
        assert!(matches!(storage, BlockStorage::Uniform(b) if b == STONE));
    }

    #[test]
    fn storage_try_compact_mixed_remains_full() {
        let mut v = vec![STONE; 4096];
        v[0] = AIR;
        let mut storage = BlockStorage::Full(v);
        storage.try_compact();
        assert!(matches!(storage, BlockStorage::Full(_)));
    }

    // ── SectionToModify ─────────────────────────────────────────────

    #[test]
    fn section_default_is_all_air() {
        let section = SectionToModify::default();
        assert!(section.get_block(0, 0, 0).is_none()); // AIR returns None
        assert!(section.get_block(15, 15, 15).is_none());
    }

    #[test]
    fn section_set_and_get_block() {
        let mut section = SectionToModify::default();
        section.set_block(5, 7, 3, STONE);
        assert_eq!(section.get_block(5, 7, 3), Some(STONE));
        // Other positions remain AIR
        assert!(section.get_block(0, 0, 0).is_none());
    }

    #[test]
    fn section_set_block_with_properties() {
        let mut section = SectionToModify::default();
        let bwp = BlockWithProperties::new(STONE, Some(fastnbt::Value::Byte(1)));
        section.set_block_with_properties(1, 2, 3, bwp);
        assert_eq!(section.get_block(1, 2, 3), Some(STONE));
        let idx = SectionToModify::index(1, 2, 3);
        assert!(section.properties.contains_key(&idx));
    }

    #[test]
    fn section_set_block_clears_properties() {
        let mut section = SectionToModify::default();
        let bwp = BlockWithProperties::new(STONE, Some(fastnbt::Value::Byte(1)));
        section.set_block_with_properties(1, 2, 3, bwp);
        // Now set plain block — properties should be removed
        section.set_block(1, 2, 3, DIRT);
        let idx = SectionToModify::index(1, 2, 3);
        assert!(!section.properties.contains_key(&idx));
        assert_eq!(section.get_block(1, 2, 3), Some(DIRT));
    }

    #[test]
    fn section_index_yzx_ordering() {
        // Index formula: y%16 * 256 + z * 16 + x
        assert_eq!(SectionToModify::index(0, 0, 0), 0);
        assert_eq!(SectionToModify::index(1, 0, 0), 1);
        assert_eq!(SectionToModify::index(0, 0, 1), 16);
        assert_eq!(SectionToModify::index(0, 1, 0), 256);
        assert_eq!(SectionToModify::index(15, 15, 15), 15 * 256 + 15 * 16 + 15);
    }

    #[test]
    fn section_compact_uniform_after_reverting() {
        let mut section = SectionToModify::default();
        // Write a block then write AIR back → can compact
        section.set_block(5, 5, 5, STONE);
        section.set_block(5, 5, 5, AIR);
        section.compact();
        assert!(matches!(section.storage, BlockStorage::Uniform(b) if b == AIR));
    }

    #[test]
    fn section_to_section_uniform_air() {
        let section = SectionToModify::default();
        let nbt_section = section.to_section(0);
        assert_eq!(nbt_section.y, 0);
        assert_eq!(nbt_section.block_states.palette.len(), 1);
        assert!(nbt_section.block_states.palette[0].name.contains("air"));
        assert!(nbt_section.block_states.data.is_none());
    }

    #[test]
    fn section_to_section_mixed_blocks() {
        let mut section = SectionToModify::default();
        section.set_block(0, 0, 0, STONE);
        section.set_block(1, 0, 0, DIRT);
        let nbt_section = section.to_section(3);
        assert_eq!(nbt_section.y, 3);
        // Palette should have AIR + STONE + DIRT = 3 entries
        assert_eq!(nbt_section.block_states.palette.len(), 3);
        assert!(nbt_section.block_states.data.is_some());
    }

    #[test]
    fn section_get_block_at_index() {
        let mut section = SectionToModify::default();
        section.set_block(3, 4, 5, STONE);
        let idx = SectionToModify::index(3, 4, 5);
        assert_eq!(section.get_block_at_index(idx), STONE);
    }

    // ── ChunkToModify ───────────────────────────────────────────────

    #[test]
    fn chunk_default_has_no_blocks() {
        let chunk = ChunkToModify::default();
        assert!(chunk.get_block(0, 0, 0).is_none());
        assert!(chunk.get_block(0, 64, 0).is_none());
    }

    #[test]
    fn chunk_set_and_get_block() {
        let mut chunk = ChunkToModify::default();
        chunk.set_block(5, 64, 10, STONE);
        assert_eq!(chunk.get_block(5, 64, 10), Some(STONE));
    }

    #[test]
    fn chunk_y_clamping() {
        let mut chunk = ChunkToModify::default();
        // Set block at extreme Y values — should clamp, not panic
        chunk.set_block(0, -1000, 0, STONE);
        assert_eq!(chunk.get_block(0, MIN_Y, 0), Some(STONE));

        chunk.set_block(0, 50000, 0, DIRT);
        assert_eq!(chunk.get_block(0, MAX_Y, 0), Some(DIRT));
    }

    #[test]
    fn chunk_set_block_with_properties() {
        let mut chunk = ChunkToModify::default();
        let bwp = BlockWithProperties::simple(STONE);
        chunk.set_block_with_properties(3, 100, 7, bwp);
        assert_eq!(chunk.get_block(3, 100, 7), Some(STONE));
    }

    #[test]
    fn chunk_sections_iterator() {
        let mut chunk = ChunkToModify::default();
        chunk.set_block(0, 0, 0, STONE);
        chunk.set_block(0, 64, 0, DIRT);
        let sections: Vec<_> = chunk.sections().collect();
        // Two different Y sections (0 is in section 0, 64 is in section 4)
        assert_eq!(sections.len(), 2);
    }

    // ── RegionToModify ──────────────────────────────────────────────

    #[test]
    fn region_get_or_create_chunk() {
        let mut region = RegionToModify::default();
        let chunk = region.get_or_create_chunk(5, 10);
        chunk.set_block(0, 0, 0, STONE);
        assert_eq!(
            region.get_chunk(5, 10).unwrap().get_block(0, 0, 0),
            Some(STONE)
        );
    }

    #[test]
    fn region_get_nonexistent_chunk() {
        let region = RegionToModify::default();
        assert!(region.get_chunk(0, 0).is_none());
    }

    // ── WorldToModify ───────────────────────────────────────────────

    #[test]
    fn world_set_and_get_block() {
        let mut world = WorldToModify::default();
        world.set_block_with_properties(100, 64, 200, BlockWithProperties::simple(STONE));
        assert_eq!(world.get_block(100, 64, 200), Some(STONE));
    }

    #[test]
    fn world_get_block_nonexistent() {
        let world = WorldToModify::default();
        assert!(world.get_block(0, 0, 0).is_none());
    }

    #[test]
    fn world_set_block_if_absent() {
        let mut world = WorldToModify::default();
        world.set_block_if_absent(10, 64, 10, STONE);
        assert_eq!(world.get_block(10, 64, 10), Some(STONE));

        // Should NOT overwrite
        world.set_block_if_absent(10, 64, 10, DIRT);
        assert_eq!(world.get_block(10, 64, 10), Some(STONE));
    }

    #[test]
    fn world_fill_column() {
        let mut world = WorldToModify::default();
        world.fill_column(5, 5, 0, 10, STONE, false);

        for y in 0..=10 {
            assert_eq!(world.get_block(5, y, 5), Some(STONE));
        }
        // Outside the column
        assert!(world.get_block(5, 11, 5).is_none());
        assert!(world.get_block(5, -1, 5).is_none());
    }

    #[test]
    fn world_fill_column_skip_existing() {
        let mut world = WorldToModify::default();
        // Pre-place a block
        world.set_block_with_properties(5, 5, 5, BlockWithProperties::simple(DIRT));
        // Fill column with skip
        world.fill_column(5, 5, 0, 10, STONE, true);

        // The pre-placed DIRT should remain
        assert_eq!(world.get_block(5, 5, 5), Some(DIRT));
        // Other positions should be STONE
        assert_eq!(world.get_block(5, 0, 5), Some(STONE));
        assert_eq!(world.get_block(5, 10, 5), Some(STONE));
    }

    #[test]
    fn world_compact_sections() {
        let mut world = WorldToModify::default();
        // Fill column to create Full storage
        world.fill_column(0, 0, 0, 15, STONE, false);
        // Now overwrite all with same block to make it compact-able
        // The entire section at y=0 is STONE where we wrote, AIR elsewhere
        // This won't compact because it's mixed. Let's force uniform:
        for x in 0..16u8 {
            for y in 0..16u8 {
                for z in 0..16u8 {
                    let chunk_x: i32 = 0 >> 4;
                    let chunk_z: i32 = 0 >> 4;
                    let region_x: i32 = chunk_x >> 5;
                    let region_z: i32 = chunk_z >> 5;
                    let region = world.get_or_create_region(region_x, region_z);
                    let chunk = region.get_or_create_chunk(chunk_x & 31, chunk_z & 31);
                    let section = chunk.sections.entry(0).or_default();
                    section.set_block(x, y, z, STONE);
                }
            }
        }
        world.compact_sections();
        // After compaction, the section should be Uniform(STONE)
        let region = world.get_region(0, 0).unwrap();
        let chunk = region.get_chunk(0, 0).unwrap();
        let section = chunk.sections.get(&0).unwrap();
        assert!(matches!(section.storage, BlockStorage::Uniform(b) if b == STONE));
    }

    #[test]
    fn world_coordinate_mapping() {
        // Verify that blocks at different world coordinates map to correct regions/chunks
        let mut world = WorldToModify::default();
        // Block at (0, 0, 0) → chunk (0,0), region (0,0)
        world.set_block_with_properties(0, 0, 0, BlockWithProperties::simple(STONE));
        // Block at (16, 0, 16) → chunk (1,1), region (0,0)
        world.set_block_with_properties(16, 0, 16, BlockWithProperties::simple(DIRT));
        // Block at (512, 0, 512) → chunk (0,0) in region (1,1)
        world.set_block_with_properties(512, 0, 512, BlockWithProperties::simple(GRASS_BLOCK));

        assert_eq!(world.get_block(0, 0, 0), Some(STONE));
        assert_eq!(world.get_block(16, 0, 16), Some(DIRT));
        assert_eq!(world.get_block(512, 0, 512), Some(GRASS_BLOCK));
    }
}
