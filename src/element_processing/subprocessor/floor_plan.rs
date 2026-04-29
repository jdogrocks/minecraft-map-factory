//! BSP-style procedural floor-plan partitioning for building interiors.
//!
//! The algorithm recursively splits a building's bounding rectangle along its
//! longer axis until each leaf room falls within a target size. Each split
//! emits an axis-aligned interior wall with a single doorway gap, giving every
//! resulting room a path back to the rest of the plan.
//!
//! BSP was chosen over slab-and-corridor or template-warp because it:
//!   * handles concave footprints — walls are clipped at write time to the
//!     building's actual `floor_set`, so interior walls never cross holes;
//!   * scales from a 6×6 single-room hut to a 60×60 apartment block with the
//!     same code path;
//!   * is cheap (O(cells × log depth)) and fully deterministic given a seed.
//!
//! Doorway placement reserves a 2-cell margin from each wall corner so doors
//! never land flush against another wall, which would otherwise wedge the door
//! shut against an adjacent room divider.

use rand::Rng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

/// Inclusive axis-aligned rectangle in (x, z) world coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x0: i32,
    pub z0: i32,
    pub x1: i32,
    pub z1: i32,
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.x1 - self.x0 + 1
    }
    pub fn depth(&self) -> i32 {
        self.z1 - self.z0 + 1
    }
    pub fn area(&self) -> i32 {
        self.width() * self.depth()
    }
    pub fn center(&self) -> (i32, i32) {
        ((self.x0 + self.x1) / 2, (self.z0 + self.z1) / 2)
    }
    pub fn contains(&self, x: i32, z: i32) -> bool {
        x >= self.x0 && x <= self.x1 && z >= self.z0 && z <= self.z1
    }
}

/// Orientation of an interior wall.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallAxis {
    /// Wall runs along the X axis at a fixed Z (separates rooms north/south).
    Horizontal,
    /// Wall runs along the Z axis at a fixed X (separates rooms east/west).
    Vertical,
}

/// A single interior wall segment with a doorway gap.
///
/// `fixed` is the constant coordinate (Z for Horizontal, X for Vertical).
/// `start..=end` is the inclusive run along the variable axis. `door_pos`
/// sits within `start+1..=end-1`, leaving the gap free of wall blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteriorWall {
    pub axis: WallAxis,
    pub fixed: i32,
    pub start: i32,
    pub end: i32,
    pub door_pos: i32,
}

impl InteriorWall {
    /// Iterate every (x, z) cell on this wall, including the door gap.
    /// Callers can filter the door gap themselves via `is_door_at`.
    pub fn iter_cells(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let (axis, fixed, start, end) = (self.axis, self.fixed, self.start, self.end);
        (start..=end).map(move |v| match axis {
            WallAxis::Horizontal => (v, fixed),
            WallAxis::Vertical => (fixed, v),
        })
    }

    pub fn door_cell(&self) -> (i32, i32) {
        match self.axis {
            WallAxis::Horizontal => (self.door_pos, self.fixed),
            WallAxis::Vertical => (self.fixed, self.door_pos),
        }
    }

    pub fn is_door_at(&self, x: i32, z: i32) -> bool {
        self.door_cell() == (x, z)
    }
}

/// Result of partitioning a building footprint into rooms.
#[derive(Debug, Clone)]
pub struct FloorPlan {
    pub rooms: Vec<Rect>,
    pub walls: Vec<InteriorWall>,
}

/// Minimum room dimension (along either axis) before BSP stops splitting.
/// 5 leaves room for furniture against each wall plus a 1-block walking aisle.
const MIN_ROOM_DIM: i32 = 5;
/// A room is left whole when both dimensions are <= this. Above it, the BSP
/// keeps splitting (probabilistically — see `split_chance`).
const TARGET_ROOM_DIM: i32 = 10;

/// Probability of further splitting a room based on its larger dimension.
/// Big rooms always split; rooms that are already near target size split
/// occasionally so we get some open-plan layouts mixed with smaller rooms.
fn split_chance(longer: i32) -> f64 {
    if longer >= 18 {
        1.0
    } else if longer >= 14 {
        0.8
    } else if longer > TARGET_ROOM_DIM {
        0.5
    } else {
        0.0
    }
}

/// Partition the building bounding box into rooms separated by interior walls.
///
/// The returned plan's walls reference the *world* coordinates of the original
/// `bbox`. Walls span the full bounding box even when the building is concave;
/// callers are expected to clip wall placement against the building's
/// `floor_set` so that walls never appear outside the actual floor area.
pub fn partition_floor(bbox: Rect, rng: &mut ChaCha8Rng) -> FloorPlan {
    let mut rooms = Vec::new();
    let mut walls = Vec::new();
    let mut queue: Vec<Rect> = vec![bbox];

    while let Some(rect) = queue.pop() {
        let w = rect.width();
        let d = rect.depth();
        let longer = w.max(d);

        let chance = split_chance(longer);
        let want_split = chance > 0.0 && rng.random_bool(chance);
        let split_horizontal = w >= d;
        let max_along_split = if split_horizontal { w } else { d };

        if !want_split || max_along_split < 2 * MIN_ROOM_DIM + 1 {
            rooms.push(rect);
            continue;
        }

        let split_min_offset = MIN_ROOM_DIM;
        let split_max_offset = max_along_split - MIN_ROOM_DIM - 1;
        if split_max_offset < split_min_offset {
            rooms.push(rect);
            continue;
        }
        let split_offset = rng.random_range(split_min_offset..=split_max_offset);

        if split_horizontal {
            let wall_x = rect.x0 + split_offset;
            let door_min = rect.z0 + 1;
            let door_max = rect.z1 - 1;
            let door_pos = if door_max >= door_min {
                rng.random_range(door_min..=door_max)
            } else {
                (rect.z0 + rect.z1) / 2
            };
            walls.push(InteriorWall {
                axis: WallAxis::Vertical,
                fixed: wall_x,
                start: rect.z0,
                end: rect.z1,
                door_pos,
            });
            queue.push(Rect {
                x0: rect.x0,
                z0: rect.z0,
                x1: wall_x - 1,
                z1: rect.z1,
            });
            queue.push(Rect {
                x0: wall_x + 1,
                z0: rect.z0,
                x1: rect.x1,
                z1: rect.z1,
            });
        } else {
            let wall_z = rect.z0 + split_offset;
            let door_min = rect.x0 + 1;
            let door_max = rect.x1 - 1;
            let door_pos = if door_max >= door_min {
                rng.random_range(door_min..=door_max)
            } else {
                (rect.x0 + rect.x1) / 2
            };
            walls.push(InteriorWall {
                axis: WallAxis::Horizontal,
                fixed: wall_z,
                start: rect.x0,
                end: rect.x1,
                door_pos,
            });
            queue.push(Rect {
                x0: rect.x0,
                z0: rect.z0,
                x1: rect.x1,
                z1: wall_z - 1,
            });
            queue.push(Rect {
                x0: rect.x0,
                z0: wall_z + 1,
                x1: rect.x1,
                z1: rect.z1,
            });
        }
    }

    FloorPlan { rooms, walls }
}

/// Compute the bounding rectangle of a floor area cell list. Returns `None`
/// when the slice is empty.
pub fn bbox_of(cells: &[(i32, i32)]) -> Option<Rect> {
    let first = cells.first()?;
    let mut x0 = first.0;
    let mut x1 = first.0;
    let mut z0 = first.1;
    let mut z1 = first.1;
    for &(x, z) in cells.iter().skip(1) {
        if x < x0 {
            x0 = x;
        }
        if x > x1 {
            x1 = x;
        }
        if z < z0 {
            z0 = z;
        }
        if z > z1 {
            z1 = z;
        }
    }
    Some(Rect { x0, z0, x1, z1 })
}

/// Cells of a room's interior available for furniture: rectangle interior
/// minus any walls that pass through it minus cells outside `floor_set`.
pub fn room_interior_cells(
    room: &Rect,
    walls: &[InteriorWall],
    floor_set: &HashSet<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut out = Vec::with_capacity(room.area() as usize);
    for z in room.z0..=room.z1 {
        for x in room.x0..=room.x1 {
            if !floor_set.contains(&(x, z)) {
                continue;
            }
            let on_wall = walls.iter().any(|w| match w.axis {
                WallAxis::Horizontal => {
                    z == w.fixed && x >= w.start && x <= w.end && !w.is_door_at(x, z)
                }
                WallAxis::Vertical => {
                    x == w.fixed && z >= w.start && z <= w.end && !w.is_door_at(x, z)
                }
            });
            if on_wall {
                continue;
            }
            out.push((x, z));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn full_floor(rect: Rect) -> HashSet<(i32, i32)> {
        let mut s = HashSet::new();
        for z in rect.z0..=rect.z1 {
            for x in rect.x0..=rect.x1 {
                s.insert((x, z));
            }
        }
        s
    }

    #[test]
    fn small_footprint_yields_single_room() {
        let bbox = Rect {
            x0: 0,
            z0: 0,
            x1: 8,
            z1: 8,
        };
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let plan = partition_floor(bbox, &mut rng);
        assert_eq!(plan.rooms.len(), 1);
        assert!(plan.walls.is_empty());
    }

    #[test]
    fn medium_footprint_partitions_into_multiple_rooms() {
        let bbox = Rect {
            x0: 0,
            z0: 0,
            x1: 19,
            z1: 19,
        };
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let plan = partition_floor(bbox, &mut rng);
        assert!(plan.rooms.len() >= 2, "expected splits, got {plan:?}");
        let total_cells = bbox.area();
        let room_area: i32 = plan.rooms.iter().map(|r| r.area()).sum();
        let wall_cells: i32 = plan
            .walls
            .iter()
            .map(|w| (w.end - w.start + 1) as i32)
            .sum();
        assert_eq!(room_area + wall_cells, total_cells);
    }

    #[test]
    fn determinism_same_seed_same_plan() {
        let bbox = Rect {
            x0: 0,
            z0: 0,
            x1: 30,
            z1: 25,
        };
        let mut a = ChaCha8Rng::seed_from_u64(123);
        let mut b = ChaCha8Rng::seed_from_u64(123);
        let p1 = partition_floor(bbox, &mut a);
        let p2 = partition_floor(bbox, &mut b);
        assert_eq!(p1.rooms, p2.rooms);
        assert_eq!(p1.walls, p2.walls);
    }

    #[test]
    fn doors_lie_within_walls_and_not_at_endpoints() {
        let bbox = Rect {
            x0: -10,
            z0: -10,
            x1: 30,
            z1: 30,
        };
        let mut rng = ChaCha8Rng::seed_from_u64(7);
        let plan = partition_floor(bbox, &mut rng);
        for w in &plan.walls {
            assert!(w.door_pos > w.start, "door at start corner: {w:?}");
            assert!(w.door_pos < w.end, "door at end corner: {w:?}");
        }
    }

    #[test]
    fn rooms_meet_min_dimension() {
        let bbox = Rect {
            x0: 0,
            z0: 0,
            x1: 50,
            z1: 50,
        };
        let mut rng = ChaCha8Rng::seed_from_u64(9999);
        let plan = partition_floor(bbox, &mut rng);
        for r in &plan.rooms {
            assert!(r.width() >= MIN_ROOM_DIM, "room too narrow on X: {r:?}");
            assert!(r.depth() >= MIN_ROOM_DIM, "room too narrow on Z: {r:?}");
        }
    }

    #[test]
    fn room_interior_excludes_walls() {
        let bbox = Rect {
            x0: 0,
            z0: 0,
            x1: 19,
            z1: 19,
        };
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let plan = partition_floor(bbox, &mut rng);
        let floor = full_floor(bbox);
        for room in &plan.rooms {
            let interior = room_interior_cells(room, &plan.walls, &floor);
            for &(x, z) in &interior {
                for w in &plan.walls {
                    let on_wall_proper = match w.axis {
                        WallAxis::Horizontal => {
                            z == w.fixed && x >= w.start && x <= w.end && !w.is_door_at(x, z)
                        }
                        WallAxis::Vertical => {
                            x == w.fixed && z >= w.start && z <= w.end && !w.is_door_at(x, z)
                        }
                    };
                    assert!(
                        !on_wall_proper,
                        "interior cell ({x},{z}) is on wall {w:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn bbox_of_handles_negatives() {
        let cells = vec![(-3, 5), (-7, 2), (4, -1), (1, 9)];
        let r = bbox_of(&cells).unwrap();
        assert_eq!(
            r,
            Rect {
                x0: -7,
                z0: -1,
                x1: 4,
                z1: 9
            }
        );
    }
}
