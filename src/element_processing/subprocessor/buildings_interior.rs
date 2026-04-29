//! Building interior generation: floor plans, doors, staircases, lighting,
//! and type-appropriate furniture.
//!
//! Replaces the tiled-template approach (a fixed 23×23 layout repeated across
//! the footprint) with BSP partitioning (`super::floor_plan`) plus per-room
//! furnishing keyed off `BuildingCategory`. Determinism is preserved by
//! seeding everything from the OSM way ID.

use crate::block_definitions::*;
use crate::element_processing::buildings::{BuildingCategory, BUILDING_PASSAGE_HEIGHT};
use crate::element_processing::subprocessor::floor_plan::{
    bbox_of, partition_floor, room_interior_cells, FloorPlan, InteriorWall, Rect,
};
use crate::floodfill_cache::CoordinateBitmap;
use crate::world_editor::WorldEditor;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

/// Interior style determines the furniture set placed inside each room.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteriorStyle {
    Residential,
    Commercial,
    Public,
    Industrial,
    Farm,
    Religious,
}

impl InteriorStyle {
    pub fn from_category(category: BuildingCategory) -> Self {
        match category {
            BuildingCategory::House | BuildingCategory::Residential => InteriorStyle::Residential,
            BuildingCategory::Commercial | BuildingCategory::Hotel => InteriorStyle::Commercial,
            BuildingCategory::Office => InteriorStyle::Public,
            BuildingCategory::School | BuildingCategory::Hospital => InteriorStyle::Public,
            BuildingCategory::Religious => InteriorStyle::Religious,
            BuildingCategory::Historic | BuildingCategory::Tower => InteriorStyle::Public,
            BuildingCategory::Industrial | BuildingCategory::Warehouse => InteriorStyle::Industrial,
            BuildingCategory::Farm => InteriorStyle::Farm,
            _ => InteriorStyle::Residential,
        }
    }
}

/// Minimum side length to attempt interior partitioning. Below this the
/// building gets a single-room layout (still gets a door, light, and
/// furniture — just no interior walls).
const MIN_PARTITION_SIDE: i32 = 12;
/// Minimum number of cells in `floor_area` to bother with interior generation.
/// Below this we fall back to "skip" (the calling code already filters at 100,
/// this is belt-and-suspenders).
const MIN_INTERIOR_CELLS: usize = 36;

/// Generates interior layouts inside buildings at each floor level.
///
/// `wall_outline` is the set of exterior wall cells, used for OSM-aware door
/// placement. The interior generator never places walls outside `floor_area`,
/// so concave footprints stay correct.
#[allow(clippy::too_many_arguments)]
pub fn generate_building_interior(
    editor: &mut WorldEditor,
    floor_area: &[(i32, i32)],
    wall_outline: &[(i32, i32)],
    start_y_offset: i32,
    building_height: i32,
    wall_block: Block,
    floor_levels: &[i32],
    args: &crate::args::Args,
    element: &crate::osm_parser::ProcessedWay,
    abs_terrain_offset: i32,
    is_abandoned_building: bool,
    building_passages: &CoordinateBitmap,
    category: BuildingCategory,
) {
    if floor_area.len() < MIN_INTERIOR_CELLS {
        return;
    }
    let bbox = match bbox_of(floor_area) {
        Some(b) => b,
        None => return,
    };
    if bbox.width() < 8 || bbox.depth() < 8 {
        return;
    }

    let floor_set: HashSet<(i32, i32)> = floor_area.iter().copied().collect();
    let style = InteriorStyle::from_category(category);
    let wall_set: HashSet<(i32, i32)> = wall_outline.iter().copied().collect();

    // Pick a single staircase column shared across all floors (multi-story only).
    let staircase = if floor_levels.len() > 1 {
        pick_staircase_position(&bbox, &floor_set, element.id)
    } else {
        None
    };

    let base_seed = element.id.wrapping_mul(0xA24BAED4963EE407);

    for (floor_idx, &floor_y) in floor_levels.iter().enumerate() {
        let floor_seed = base_seed.wrapping_add(floor_idx as u64);
        let mut rng = ChaCha8Rng::seed_from_u64(floor_seed);

        let plan = if bbox.width() >= MIN_PARTITION_SIDE && bbox.depth() >= MIN_PARTITION_SIDE {
            partition_floor(bbox, &mut rng)
        } else {
            FloorPlan {
                rooms: vec![bbox],
                walls: Vec::new(),
            }
        };

        let staircase_cells: HashSet<(i32, i32)> = staircase
            .as_ref()
            .map(|s| s.reserved_cells())
            .unwrap_or_default();

        let floor_abs = floor_y + abs_terrain_offset;
        let ceiling_y = floor_ceiling_y(floor_levels, floor_idx, start_y_offset, building_height);
        let ceiling_abs = ceiling_y + abs_terrain_offset;

        place_interior_walls(
            editor,
            &plan,
            &floor_set,
            building_passages,
            &staircase_cells,
            floor_y,
            ceiling_y,
            wall_block,
            abs_terrain_offset,
            start_y_offset,
            building_height,
        );

        place_interior_doors(
            editor,
            &plan.walls,
            &floor_set,
            building_passages,
            floor_y,
            abs_terrain_offset,
            start_y_offset,
            building_height,
        );

        for room in &plan.rooms {
            furnish_room(
                editor,
                room,
                &plan.walls,
                &floor_set,
                &staircase_cells,
                floor_abs,
                ceiling_abs,
                style,
                is_abandoned_building,
                floor_idx,
                element.id,
            );
        }

        if let Some(s) = &staircase {
            place_staircase_floor(
                editor,
                s,
                floor_abs,
                ceiling_abs,
                floor_idx,
                floor_levels.len(),
                wall_block,
            );
        }
    }

    // Exterior doors only on the ground floor.
    if !floor_levels.is_empty() {
        let ground_y = floor_levels[0];
        place_exterior_doors(
            editor,
            element,
            &wall_set,
            &floor_set,
            ground_y,
            abs_terrain_offset,
            building_passages,
            args,
        );
    }
}

fn floor_ceiling_y(
    floor_levels: &[i32],
    floor_idx: usize,
    start_y_offset: i32,
    building_height: i32,
) -> i32 {
    if floor_idx + 1 < floor_levels.len() {
        floor_levels[floor_idx + 1] - 1
    } else {
        start_y_offset + building_height
    }
}

#[derive(Debug, Clone, Copy)]
struct Staircase {
    /// Open shaft cell (player walks here).
    shaft: (i32, i32),
    /// Ladder cell adjacent to `shaft` on one cardinal axis.
    ladder: (i32, i32),
}

impl Staircase {
    fn reserved_cells(&self) -> HashSet<(i32, i32)> {
        let mut s = HashSet::with_capacity(2);
        s.insert(self.shaft);
        s.insert(self.ladder);
        s
    }
}

fn pick_staircase_position(
    bbox: &Rect,
    floor_set: &HashSet<(i32, i32)>,
    element_id: u64,
) -> Option<Staircase> {
    let mut rng = ChaCha8Rng::seed_from_u64(element_id.wrapping_add(0x57A1C45E_5704C45E));
    let (cx, cz) = bbox.center();
    let mut offsets: Vec<(i32, i32)> = Vec::with_capacity(81);
    for dz in -4..=4 {
        for dx in -4..=4 {
            offsets.push((dx, dz));
        }
    }
    offsets.sort_by_key(|o| o.0.abs() + o.1.abs());
    offsets.shuffle(&mut rng);

    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for &(dx, dz) in &offsets {
        let x = cx + dx;
        let z = cz + dz;
        if !floor_set.contains(&(x, z)) {
            continue;
        }
        for &(dxx, dzz) in &dirs {
            let lx = x + dxx;
            let lz = z + dzz;
            if floor_set.contains(&(lx, lz)) {
                return Some(Staircase {
                    shaft: (x, z),
                    ladder: (lx, lz),
                });
            }
        }
    }
    None
}

fn place_staircase_floor(
    editor: &mut WorldEditor,
    s: &Staircase,
    floor_abs: i32,
    ceiling_abs: i32,
    floor_idx: usize,
    floor_count: usize,
    wall_block: Block,
) {
    let (lx, lz) = s.ladder;
    let (sx, sz) = s.shaft;

    // Shaft column: clear air from floor+1 to ceiling-1 so the player can climb.
    let max_climb_y = ceiling_abs.saturating_sub(1).max(floor_abs + 1);
    for y in (floor_abs + 1)..=max_climb_y {
        editor.set_block_absolute(AIR, sx, y, sz, None, Some(&[]));
    }

    // Ladder column right next to the shaft.
    for y in (floor_abs + 1)..=max_climb_y {
        editor.set_block_absolute(LADDER, lx, y, lz, None, Some(&[]));
    }

    // Top transition: hole through the ceiling for non-top floors, cap for the
    // top floor so the column doesn't poke through the roof.
    if floor_idx + 1 < floor_count {
        editor.set_block_absolute(AIR, sx, ceiling_abs, sz, None, Some(&[]));
        editor.set_block_absolute(AIR, lx, ceiling_abs, lz, None, Some(&[]));
    } else {
        editor.set_block_absolute(wall_block, sx, ceiling_abs, sz, None, Some(&[]));
        editor.set_block_absolute(wall_block, lx, ceiling_abs, lz, None, Some(&[]));
    }
}

#[allow(clippy::too_many_arguments)]
fn place_interior_walls(
    editor: &mut WorldEditor,
    plan: &FloorPlan,
    floor_set: &HashSet<(i32, i32)>,
    building_passages: &CoordinateBitmap,
    staircase_cells: &HashSet<(i32, i32)>,
    floor_y: i32,
    ceiling_y: i32,
    wall_block: Block,
    abs_terrain_offset: i32,
    start_y_offset: i32,
    building_height: i32,
) {
    let passage_top = start_y_offset + BUILDING_PASSAGE_HEIGHT.min(building_height);
    let in_passage_height = floor_y < passage_top;

    for wall in &plan.walls {
        for (x, z) in wall.iter_cells() {
            if !floor_set.contains(&(x, z)) {
                continue;
            }
            if in_passage_height && building_passages.contains(x, z) {
                continue;
            }
            if wall.is_door_at(x, z) {
                continue;
            }
            if staircase_cells.contains(&(x, z)) {
                // Don't wall through the staircase column.
                continue;
            }
            let from = floor_y + 1 + abs_terrain_offset;
            let to = ceiling_y + abs_terrain_offset;
            for y in from..=to {
                editor.set_block_absolute(wall_block, x, y, z, None, Some(&[]));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn place_interior_doors(
    editor: &mut WorldEditor,
    walls: &[InteriorWall],
    floor_set: &HashSet<(i32, i32)>,
    building_passages: &CoordinateBitmap,
    floor_y: i32,
    abs_terrain_offset: i32,
    start_y_offset: i32,
    building_height: i32,
) {
    let passage_top = start_y_offset + BUILDING_PASSAGE_HEIGHT.min(building_height);
    let in_passage_height = floor_y < passage_top;

    for wall in walls {
        let (dx, dz) = wall.door_cell();
        if !floor_set.contains(&(dx, dz)) {
            continue;
        }
        if in_passage_height && building_passages.contains(dx, dz) {
            continue;
        }
        let lower_y = floor_y + 1 + abs_terrain_offset;
        editor.set_block_absolute(DARK_OAK_DOOR_LOWER, dx, lower_y, dz, None, Some(&[]));
        editor.set_block_absolute(DARK_OAK_DOOR_UPPER, dx, lower_y + 1, dz, None, Some(&[]));
    }
}

#[allow(clippy::too_many_arguments)]
fn place_exterior_doors(
    editor: &mut WorldEditor,
    element: &crate::osm_parser::ProcessedWay,
    wall_set: &HashSet<(i32, i32)>,
    floor_set: &HashSet<(i32, i32)>,
    ground_y: i32,
    abs_terrain_offset: i32,
    building_passages: &CoordinateBitmap,
    _args: &crate::args::Args,
) {
    let lower_y = ground_y + 1 + abs_terrain_offset;

    // Pass 1: building-way vertices tagged entrance=*/door=*.
    let mut placed_any = false;
    for node in &element.nodes {
        let has_entrance = node.tags.contains_key("entrance");
        let has_door = node.tags.contains_key("door");
        if !has_entrance && !has_door {
            continue;
        }
        if let Some(level_str) = node.tags.get("level") {
            if let Ok(level) = level_str.parse::<i32>() {
                if level != 0 {
                    continue;
                }
            }
        }
        let pos = (node.x, node.z);
        if building_passages.contains(pos.0, pos.1) {
            continue;
        }
        let wall_cell = if wall_set.contains(&pos) {
            pos
        } else if let Some(snap) = nearest_wall_cell(pos, wall_set, 2) {
            snap
        } else {
            continue;
        };
        place_oak_door_pair(editor, wall_cell.0, wall_cell.1, lower_y);
        placed_any = true;
    }

    // Pass 2: procedural front door — longest wall run with interior on one side.
    if !placed_any {
        if let Some(door_cell) = pick_procedural_front_door(wall_set, floor_set, building_passages)
        {
            place_oak_door_pair(editor, door_cell.0, door_cell.1, lower_y);
        }
    }
}

fn nearest_wall_cell(p: (i32, i32), wall_set: &HashSet<(i32, i32)>, r: i32) -> Option<(i32, i32)> {
    let mut best: Option<((i32, i32), i32)> = None;
    for dz in -r..=r {
        for dx in -r..=r {
            let q = (p.0 + dx, p.1 + dz);
            if wall_set.contains(&q) {
                let d = dx.abs() + dz.abs();
                if best.map(|(_, bd)| d < bd).unwrap_or(true) {
                    best = Some((q, d));
                }
            }
        }
    }
    best.map(|(q, _)| q)
}

fn pick_procedural_front_door(
    wall_set: &HashSet<(i32, i32)>,
    floor_set: &HashSet<(i32, i32)>,
    building_passages: &CoordinateBitmap,
) -> Option<(i32, i32)> {
    if wall_set.is_empty() {
        return None;
    }
    let mut by_z: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    let mut by_x: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    for &(x, z) in wall_set {
        by_z.entry(z).or_default().push(x);
        by_x.entry(x).or_default().push(z);
    }

    let mut best: Option<((i32, i32), i32)> = None;

    for (&z, xs) in &mut by_z {
        xs.sort_unstable();
        for run in consecutive_runs(xs) {
            let len = run.1 - run.0 + 1;
            let mid = (run.0 + run.1) / 2;
            if building_passages.contains(mid, z) {
                continue;
            }
            let interior_side =
                floor_set.contains(&(mid, z + 1)) || floor_set.contains(&(mid, z - 1));
            if !interior_side {
                continue;
            }
            if best.map(|(_, bl)| len > bl).unwrap_or(true) {
                best = Some(((mid, z), len));
            }
        }
    }
    for (&x, zs) in &mut by_x {
        zs.sort_unstable();
        for run in consecutive_runs(zs) {
            let len = run.1 - run.0 + 1;
            let mid = (run.0 + run.1) / 2;
            if building_passages.contains(x, mid) {
                continue;
            }
            let interior_side =
                floor_set.contains(&(x + 1, mid)) || floor_set.contains(&(x - 1, mid));
            if !interior_side {
                continue;
            }
            if best.map(|(_, bl)| len > bl).unwrap_or(true) {
                best = Some(((x, mid), len));
            }
        }
    }
    best.map(|(c, _)| c)
}

fn consecutive_runs(sorted: &[i32]) -> Vec<(i32, i32)> {
    let mut runs = Vec::new();
    if sorted.is_empty() {
        return runs;
    }
    let mut start = sorted[0];
    let mut prev = sorted[0];
    for &v in &sorted[1..] {
        if v == prev + 1 {
            prev = v;
        } else {
            runs.push((start, prev));
            start = v;
            prev = v;
        }
    }
    runs.push((start, prev));
    runs
}

fn place_oak_door_pair(editor: &mut WorldEditor, x: i32, z: i32, y_lower_abs: i32) {
    editor.set_block_absolute(OAK_DOOR, x, y_lower_abs, z, None, Some(&[]));
    editor.set_block_absolute(OAK_DOOR_UPPER, x, y_lower_abs + 1, z, None, Some(&[]));
}

// ===========================================================================
// Furniture
// ===========================================================================

#[allow(clippy::too_many_arguments)]
fn furnish_room(
    editor: &mut WorldEditor,
    room: &Rect,
    walls: &[InteriorWall],
    floor_set: &HashSet<(i32, i32)>,
    staircase_cells: &HashSet<(i32, i32)>,
    floor_abs: i32,
    ceiling_abs: i32,
    style: InteriorStyle,
    is_abandoned: bool,
    floor_idx: usize,
    element_id: u64,
) {
    let interior = room_interior_cells(room, walls, floor_set);
    if interior.is_empty() {
        return;
    }

    let mut available: HashSet<(i32, i32)> = interior.iter().copied().collect();
    for s in staircase_cells {
        available.remove(s);
    }

    place_room_light(editor, room, &available, ceiling_abs);

    let mut rng = ChaCha8Rng::seed_from_u64(
        element_id
            .wrapping_mul(0x9E3779B97F4A7C15)
            .wrapping_add((room.x0 as u64) ^ ((room.z0 as u64) << 16))
            .wrapping_add(floor_idx as u64),
    );

    if is_abandoned {
        place_abandoned_dressing(editor, &available, floor_abs, &mut rng);
        return;
    }

    match style {
        InteriorStyle::Residential => {
            furnish_residential(editor, room, &available, floor_abs, &mut rng, floor_idx)
        }
        InteriorStyle::Commercial => furnish_commercial(editor, &available, floor_abs, &mut rng),
        InteriorStyle::Public => furnish_public(editor, &available, floor_abs, &mut rng),
        InteriorStyle::Industrial => furnish_industrial(editor, &available, floor_abs, &mut rng),
        InteriorStyle::Farm => furnish_farm(editor, &available, floor_abs, &mut rng),
        InteriorStyle::Religious => {
            furnish_religious(editor, room, &available, floor_abs, &mut rng)
        }
    }
}

fn place_room_light(
    editor: &mut WorldEditor,
    room: &Rect,
    available: &HashSet<(i32, i32)>,
    ceiling_abs: i32,
) {
    let center = room.center();
    let pick = if available.contains(&center) {
        Some(center)
    } else {
        let mut best: Option<((i32, i32), i32)> = None;
        for &c in available {
            let d = (c.0 - center.0).abs() + (c.1 - center.1).abs();
            if best.map(|(_, bd)| d < bd).unwrap_or(true) {
                best = Some((c, d));
            }
        }
        best.map(|(c, _)| c)
    };
    if let Some((x, z)) = pick {
        editor.set_block_absolute(GLOWSTONE, x, ceiling_abs - 1, z, None, Some(&[]));
    }
}

/// Place blocks from `plan` onto random unused cells in `available`.
fn place_furniture_set(
    editor: &mut WorldEditor,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
    plan: &[Block],
) {
    let cells = sorted_cells(available);
    if cells.is_empty() {
        return;
    }
    let mut taken: HashSet<(i32, i32)> = HashSet::new();
    for &block in plan {
        let candidates: Vec<(i32, i32)> = cells
            .iter()
            .copied()
            .filter(|c| !taken.contains(c))
            .collect();
        if candidates.is_empty() {
            break;
        }
        let pick = candidates[rng.random_range(0..candidates.len())];
        editor.set_block_absolute(block, pick.0, floor_abs + 1, pick.1, None, Some(&[]));
        taken.insert(pick);
    }
}

fn furnish_residential(
    editor: &mut WorldEditor,
    room: &Rect,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
    floor_idx: usize,
) {
    let _placed_bed = place_bed_in_corner(editor, room, available, floor_abs, rng);

    let mut plan: Vec<Block> = vec![CRAFTING_TABLE, FURNACE, CHEST];
    if floor_idx == 0 {
        plan.push(BOOKSHELF);
    }
    plan.extend_from_slice(&[RED_CARPET, RED_CARPET, RED_CARPET]);
    place_furniture_set(editor, available, floor_abs, rng, &plan);
}

fn furnish_commercial(
    editor: &mut WorldEditor,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
) {
    let plan: &[Block] = &[
        BARREL,
        BARREL,
        BARREL,
        CHEST,
        CHEST,
        OAK_STAIRS,
        OAK_STAIRS,
        WHITE_CARPET,
        WHITE_CARPET,
    ];
    place_furniture_set(editor, available, floor_abs, rng, plan);
}

fn furnish_public(
    editor: &mut WorldEditor,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
) {
    let plan: &[Block] = &[
        BOOKSHELF,
        BOOKSHELF,
        BOOKSHELF,
        CRAFTING_TABLE,
        CRAFTING_TABLE,
        OAK_STAIRS,
        OAK_STAIRS,
        NOTE_BLOCK,
        WHITE_CARPET,
        WHITE_CARPET,
        WHITE_CARPET,
        CHEST,
    ];
    place_furniture_set(editor, available, floor_abs, rng, plan);
}

fn furnish_industrial(
    editor: &mut WorldEditor,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
) {
    let plan: &[Block] = &[
        FURNACE,
        FURNACE,
        BARREL,
        BARREL,
        BARREL,
        ANVIL,
        ANVIL,
        IRON_BLOCK,
        CRAFTING_TABLE,
        CHEST,
    ];
    place_furniture_set(editor, available, floor_abs, rng, plan);
}

fn furnish_farm(
    editor: &mut WorldEditor,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
) {
    let plan: &[Block] = &[
        HAY_BALE,
        HAY_BALE,
        HAY_BALE,
        HAY_BALE,
        BARREL,
        BARREL,
        CAULDRON,
        CRAFTING_TABLE,
        CHEST,
        FURNACE,
    ];
    place_furniture_set(editor, available, floor_abs, rng, plan);
}

fn furnish_religious(
    editor: &mut WorldEditor,
    room: &Rect,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
) {
    // Altar: brewing stand at the cell furthest from room center.
    let cells = sorted_cells(available);
    if cells.is_empty() {
        return;
    }
    let center = room.center();
    let altar = cells
        .iter()
        .max_by_key(|(x, z)| (x - center.0).abs() + (z - center.1).abs())
        .copied();
    let mut taken: HashSet<(i32, i32)> = HashSet::new();
    if let Some(a) = altar {
        editor.set_block_absolute(BREWING_STAND, a.0, floor_abs + 1, a.1, None, Some(&[]));
        taken.insert(a);
    }
    let plan: &[Block] = &[
        OAK_STAIRS,
        OAK_STAIRS,
        OAK_STAIRS,
        OAK_STAIRS,
        BOOKSHELF,
        BOOKSHELF,
        WHITE_CARPET,
        WHITE_CARPET,
        WHITE_CARPET,
        WHITE_CARPET,
    ];
    for &block in plan {
        let candidates: Vec<(i32, i32)> = cells
            .iter()
            .copied()
            .filter(|c| !taken.contains(c))
            .collect();
        if candidates.is_empty() {
            break;
        }
        let pick = candidates[rng.random_range(0..candidates.len())];
        editor.set_block_absolute(block, pick.0, floor_abs + 1, pick.1, None, Some(&[]));
        taken.insert(pick);
    }
}

fn place_abandoned_dressing(
    editor: &mut WorldEditor,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
) {
    let plan: &[Block] = &[COBWEB, COBWEB, COBWEB, DAMAGED_ANVIL, BARREL];
    place_furniture_set(editor, available, floor_abs, rng, plan);
}

fn place_bed_in_corner(
    editor: &mut WorldEditor,
    room: &Rect,
    available: &HashSet<(i32, i32)>,
    floor_abs: i32,
    rng: &mut ChaCha8Rng,
) -> bool {
    if room.width() < 3 || room.depth() < 3 {
        return false;
    }
    /// Bed candidate: (head_cell, head→foot offset, (head_block, foot_block)).
    type BedCandidate = ((i32, i32), (i32, i32), (Block, Block));
    let candidates: [BedCandidate; 4] = [
        (
            (room.x0 + 1, room.z0 + 1),
            (1, 0),
            (RED_BED_EAST_HEAD, RED_BED_EAST_FOOT),
        ),
        (
            (room.x1 - 1, room.z0 + 1),
            (-1, 0),
            (RED_BED_WEST_HEAD, RED_BED_WEST_FOOT),
        ),
        (
            (room.x0 + 1, room.z1 - 1),
            (0, -1),
            (RED_BED_NORTH_HEAD, RED_BED_NORTH_FOOT),
        ),
        (
            (room.x1 - 1, room.z1 - 1),
            (0, 1),
            (RED_BED_SOUTH_HEAD, RED_BED_SOUTH_FOOT),
        ),
    ];
    let mut order: Vec<usize> = (0..candidates.len()).collect();
    order.shuffle(rng);
    for i in order {
        let (head, (dx, dz), (head_block, foot_block)) = candidates[i];
        let foot = (head.0 + dx, head.1 + dz);
        if available.contains(&head) && available.contains(&foot) {
            editor.set_block_absolute(head_block, head.0, floor_abs + 1, head.1, None, Some(&[]));
            editor.set_block_absolute(foot_block, foot.0, floor_abs + 1, foot.1, None, Some(&[]));
            return true;
        }
    }
    false
}

fn sorted_cells(set: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32, i32)> = set.iter().copied().collect();
    v.sort_unstable();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interior_style_maps_categories() {
        assert_eq!(
            InteriorStyle::from_category(BuildingCategory::House),
            InteriorStyle::Residential
        );
        assert_eq!(
            InteriorStyle::from_category(BuildingCategory::Commercial),
            InteriorStyle::Commercial
        );
        assert_eq!(
            InteriorStyle::from_category(BuildingCategory::School),
            InteriorStyle::Public
        );
        assert_eq!(
            InteriorStyle::from_category(BuildingCategory::Religious),
            InteriorStyle::Religious
        );
        assert_eq!(
            InteriorStyle::from_category(BuildingCategory::Industrial),
            InteriorStyle::Industrial
        );
        assert_eq!(
            InteriorStyle::from_category(BuildingCategory::Farm),
            InteriorStyle::Farm
        );
    }

    #[test]
    fn consecutive_runs_groups_adjacent_values() {
        assert_eq!(
            consecutive_runs(&[1, 2, 3, 5, 6, 9]),
            vec![(1, 3), (5, 6), (9, 9)]
        );
        assert_eq!(consecutive_runs(&[]), Vec::<(i32, i32)>::new());
        assert_eq!(consecutive_runs(&[7]), vec![(7, 7)]);
    }

    #[test]
    fn nearest_wall_cell_finds_within_radius() {
        let mut wall = HashSet::new();
        wall.insert((10, 10));
        wall.insert((12, 12));
        assert_eq!(nearest_wall_cell((10, 10), &wall, 0), Some((10, 10)));
        assert_eq!(nearest_wall_cell((11, 11), &wall, 1), Some((10, 10)));
        assert_eq!(nearest_wall_cell((20, 20), &wall, 1), None);
    }

    #[test]
    fn pick_staircase_returns_adjacent_pair() {
        let bbox = Rect {
            x0: 0,
            z0: 0,
            x1: 9,
            z1: 9,
        };
        let mut floor: HashSet<(i32, i32)> = HashSet::new();
        for z in 0..=9 {
            for x in 0..=9 {
                floor.insert((x, z));
            }
        }
        let s = pick_staircase_position(&bbox, &floor, 12345).expect("staircase");
        let (sx, sz) = s.shaft;
        let (lx, lz) = s.ladder;
        let dist = (sx - lx).abs() + (sz - lz).abs();
        assert_eq!(dist, 1, "shaft and ladder must be orthogonal neighbours");
        assert!(floor.contains(&s.shaft));
        assert!(floor.contains(&s.ladder));
    }

    #[test]
    fn pick_staircase_returns_none_for_empty_floor() {
        let bbox = Rect {
            x0: 0,
            z0: 0,
            x1: 4,
            z1: 4,
        };
        let floor: HashSet<(i32, i32)> = HashSet::new();
        assert!(pick_staircase_position(&bbox, &floor, 1).is_none());
    }

    #[test]
    fn pick_procedural_front_door_picks_long_run_with_interior_side() {
        let mut wall = HashSet::new();
        let mut floor = HashSet::new();
        for x in 0..=5 {
            wall.insert((x, 0));
            wall.insert((x, 5));
        }
        for z in 0..=5 {
            wall.insert((0, z));
            wall.insert((5, z));
        }
        for z in 1..=4 {
            for x in 1..=4 {
                floor.insert((x, z));
            }
        }
        let passages = CoordinateBitmap::new_empty();
        let pick = pick_procedural_front_door(&wall, &floor, &passages).expect("door");
        assert!(wall.contains(&pick));
        let interior_neighbour = floor.contains(&(pick.0, pick.1 + 1))
            || floor.contains(&(pick.0, pick.1 - 1))
            || floor.contains(&(pick.0 + 1, pick.1))
            || floor.contains(&(pick.0 - 1, pick.1));
        assert!(interior_neighbour);
    }
}
