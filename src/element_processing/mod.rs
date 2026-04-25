pub mod advertising;
pub mod amenities;
pub mod barriers;
pub mod bridges;
pub mod buildings;
pub mod doors;
pub mod emergency;
pub mod highways;
pub mod historic;
pub mod landuse;
pub mod leisure;
pub mod man_made;
pub mod natural;
pub mod power;
pub mod railways;
pub mod subprocessor;
mod surfaces;
pub mod tourisms;
pub mod tree;
pub mod water_areas;
pub mod waterways;

use crate::floodfill_cache::RoadMaskBitmap;
use crate::osm_parser::ProcessedNode;

/// Merges way segments that share endpoints into closed rings.
/// Used by water_areas.rs and boundaries.rs for assembling relation members.
pub fn merge_way_segments(rings: &mut Vec<Vec<ProcessedNode>>) {
    let mut removed: Vec<usize> = vec![];
    let mut merged: Vec<Vec<ProcessedNode>> = vec![];

    // Match nodes by ID or proximity (handles synthetic nodes from bbox clipping)
    let nodes_match = |a: &ProcessedNode, b: &ProcessedNode| -> bool {
        if a.id == b.id {
            return true;
        }
        let dx = (a.x - b.x).abs();
        let dz = (a.z - b.z).abs();
        dx <= 1 && dz <= 1
    };

    for i in 0..rings.len() {
        for j in 0..rings.len() {
            if i == j {
                continue;
            }

            if removed.contains(&i) || removed.contains(&j) {
                continue;
            }

            let x: &Vec<ProcessedNode> = &rings[i];
            let y: &Vec<ProcessedNode> = &rings[j];

            // Skip empty rings (can happen after clipping)
            if x.is_empty() || y.is_empty() {
                continue;
            }

            let x_first = &x[0];
            let x_last = x.last().unwrap();
            let y_first = &y[0];
            let y_last = y.last().unwrap();

            // Skip already-closed rings
            if nodes_match(x_first, x_last) {
                continue;
            }

            if nodes_match(y_first, y_last) {
                continue;
            }

            if nodes_match(x_first, y_first) {
                removed.push(i);
                removed.push(j);

                let mut x: Vec<ProcessedNode> = x.clone();
                x.reverse();
                x.extend(y.iter().skip(1).cloned());
                merged.push(x);
            } else if nodes_match(x_last, y_last) {
                removed.push(i);
                removed.push(j);

                let mut x: Vec<ProcessedNode> = x.clone();
                x.extend(y.iter().rev().skip(1).cloned());

                merged.push(x);
            } else if nodes_match(x_first, y_last) {
                removed.push(i);
                removed.push(j);

                let mut y: Vec<ProcessedNode> = y.clone();
                y.extend(x.iter().skip(1).cloned());

                merged.push(y);
            } else if nodes_match(x_last, y_first) {
                removed.push(i);
                removed.push(j);

                let mut x: Vec<ProcessedNode> = x.clone();
                x.extend(y.iter().skip(1).cloned());

                merged.push(x);
            }
        }
    }

    removed.sort();

    for r in removed.iter().rev() {
        rings.remove(*r);
    }

    let merged_len: usize = merged.len();
    for m in merged {
        rings.push(m);
    }

    if merged_len > 0 {
        merge_way_segments(rings);
    }
}

/// Searches outward from (x, z) in the four cardinal directions and four
/// diagonals stepping by 2 up to max_radius blocks away, and returns the
/// (x, z) position of the nearest block that satisfies predicate
///
/// Returns None if no matching block is found within range.
fn get_nearest_block_matching(
    x: i32,
    z: i32,
    max_radius: i32,
    road_mask: &RoadMaskBitmap,
    predicate: impl Fn(bool) -> bool,
) -> Option<(i32, i32)> {
    for dist in (2..=max_radius).step_by(2) {
        let candidates = [
            (x, z - dist),
            (x, z + dist),
            (x - dist, z),
            (x + dist, z),
            (x - dist, z - dist),
            (x + dist, z + dist),
            (x - dist, z + dist),
            (x + dist, z - dist),
        ];
        for (cx, cz) in candidates {
            if predicate(road_mask.contains(cx, cz)) {
                return Some((cx, cz));
            }
        }
    }
    None
}

pub fn get_nearest_road_block(
    x: i32,
    z: i32,
    max_radius: i32,
    road_mask: &RoadMaskBitmap,
) -> Option<(i32, i32)> {
    get_nearest_block_matching(x, z, max_radius, road_mask, |on_road| on_road)
}

pub fn get_nearest_non_road_block(
    x: i32,
    z: i32,
    max_radius: i32,
    road_mask: &RoadMaskBitmap,
) -> Option<(i32, i32)> {
    get_nearest_block_matching(x, z, max_radius, road_mask, |on_road| !on_road)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinate_system::cartesian::XZBBox;
    use std::collections::HashMap;

    fn make_node(id: u64, x: i32, z: i32) -> ProcessedNode {
        ProcessedNode {
            id,
            tags: HashMap::new(),
            x,
            z,
        }
    }

    // ── merge_way_segments ──────────────────────────────────────────

    #[test]
    fn merge_empty_rings() {
        let mut rings: Vec<Vec<ProcessedNode>> = vec![];
        merge_way_segments(&mut rings);
        assert!(rings.is_empty());
    }

    #[test]
    fn merge_single_closed_ring_unchanged() {
        let n1 = make_node(1, 0, 0);
        let n2 = make_node(2, 10, 0);
        let n3 = make_node(3, 10, 10);
        let ring = vec![n1.clone(), n2, n3, n1.clone()];
        let mut rings = vec![ring.clone()];
        merge_way_segments(&mut rings);
        // Already closed, should remain unchanged
        assert_eq!(rings.len(), 1);
        assert_eq!(rings[0].len(), ring.len());
    }

    #[test]
    fn merge_two_segments_sharing_endpoint() {
        // Segment A: 1 → 2 → 3
        // Segment B: 3 → 4 → 5
        // Should merge into: 1 → 2 → 3 → 4 → 5
        let seg_a = vec![make_node(1, 0, 0), make_node(2, 5, 0), make_node(3, 10, 0)];
        let seg_b = vec![
            make_node(3, 10, 0),
            make_node(4, 10, 5),
            make_node(5, 10, 10),
        ];
        let mut rings = vec![seg_a, seg_b];
        merge_way_segments(&mut rings);

        // Should be merged into one segment
        assert_eq!(rings.len(), 1);
        // Contains nodes from both segments (5 - 1 shared = 5 total)
        assert_eq!(rings[0].len(), 5);
        // First node should be id=1, last should be id=5
        assert_eq!(rings[0].first().unwrap().id, 1);
        assert_eq!(rings[0].last().unwrap().id, 5);
    }

    #[test]
    fn merge_segments_x_last_y_last() {
        // Segment A: 1 → 2 → 3
        // Segment B: 5 → 4 → 3  (shares endpoint 3 with A's last)
        let seg_a = vec![make_node(1, 0, 0), make_node(2, 5, 0), make_node(3, 10, 0)];
        let seg_b = vec![
            make_node(5, 10, 10),
            make_node(4, 10, 5),
            make_node(3, 10, 0),
        ];
        let mut rings = vec![seg_a, seg_b];
        merge_way_segments(&mut rings);
        assert_eq!(rings.len(), 1);
        assert_eq!(rings[0].len(), 5);
    }

    #[test]
    fn merge_segments_x_first_y_first() {
        // Both start at the same node → one gets reversed
        let seg_a = vec![make_node(1, 0, 0), make_node(2, 5, 0), make_node(3, 10, 0)];
        let seg_b = vec![make_node(1, 0, 0), make_node(4, 0, 5), make_node(5, 0, 10)];
        let mut rings = vec![seg_a, seg_b];
        merge_way_segments(&mut rings);
        assert_eq!(rings.len(), 1);
        assert_eq!(rings[0].len(), 5);
    }

    #[test]
    fn merge_segments_x_first_y_last() {
        // Segment A: 1 → 2 → 3
        // Segment B: 4 → 5 → 1  (B's last matches A's first)
        let seg_a = vec![make_node(1, 0, 0), make_node(2, 5, 0), make_node(3, 10, 0)];
        let seg_b = vec![make_node(4, 0, 10), make_node(5, 0, 5), make_node(1, 0, 0)];
        let mut rings = vec![seg_a, seg_b];
        merge_way_segments(&mut rings);
        assert_eq!(rings.len(), 1);
        assert_eq!(rings[0].len(), 5);
    }

    #[test]
    fn merge_skips_empty_rings() {
        let mut rings: Vec<Vec<ProcessedNode>> = vec![vec![], vec![]];
        merge_way_segments(&mut rings);
        // Empty rings remain, no panic
        assert!(rings.iter().all(|r| r.is_empty()));
    }

    #[test]
    fn merge_proximity_matching() {
        // Nodes with different IDs but within 1 block should match
        let seg_a = vec![make_node(1, 0, 0), make_node(2, 5, 0), make_node(3, 10, 0)];
        let seg_b = vec![
            make_node(99, 10, 1), // close to node 3 at (10, 0) — dx=0, dz=1
            make_node(4, 10, 5),
            make_node(5, 10, 10),
        ];
        let mut rings = vec![seg_a, seg_b];
        merge_way_segments(&mut rings);
        // Should merge via proximity
        assert_eq!(rings.len(), 1);
    }

    #[test]
    fn merge_recursive_three_segments() {
        // Three segments that form a chain
        let seg_a = vec![make_node(1, 0, 0), make_node(2, 5, 0)];
        let seg_b = vec![make_node(2, 5, 0), make_node(3, 10, 0)];
        let seg_c = vec![make_node(3, 10, 0), make_node(4, 10, 5)];
        let mut rings = vec![seg_a, seg_b, seg_c];
        merge_way_segments(&mut rings);
        // After recursive merging, should be one segment
        assert_eq!(rings.len(), 1);
        assert_eq!(rings[0].len(), 4);
    }

    // ── get_nearest_road_block / get_nearest_non_road_block ─────────

    #[test]
    fn nearest_road_block_found() {
        let bbox = XZBBox::rect_from_xz_lengths(100.0, 100.0).unwrap();
        let mut mask = RoadMaskBitmap::new(&bbox);
        // Place a road at (10, 10)
        mask.set(10, 10);

        // Search from (8, 10) — distance 2 in cardinal direction
        let result = get_nearest_road_block(8, 10, 20, &mask);
        assert!(result.is_some());
        let (rx, rz) = result.unwrap();
        assert_eq!((rx, rz), (10, 10));
    }

    #[test]
    fn nearest_road_block_not_found_within_radius() {
        let bbox = XZBBox::rect_from_xz_lengths(100.0, 100.0).unwrap();
        let mut mask = RoadMaskBitmap::new(&bbox);
        mask.set(50, 50);

        // Search from (0, 0) with small radius — won't reach (50, 50)
        let result = get_nearest_road_block(0, 0, 4, &mask);
        assert!(result.is_none());
    }

    #[test]
    fn nearest_non_road_block_found() {
        let bbox = XZBBox::rect_from_xz_lengths(100.0, 100.0).unwrap();
        let mut mask = RoadMaskBitmap::new(&bbox);
        // Fill a small area with road
        for x in 0..=20 {
            for z in 0..=20 {
                mask.set(x, z);
            }
        }
        // (22, 10) is NOT a road
        let result = get_nearest_non_road_block(10, 10, 20, &mask);
        assert!(result.is_some());
    }

    #[test]
    fn nearest_block_search_steps_by_two() {
        let bbox = XZBBox::rect_from_xz_lengths(100.0, 100.0).unwrap();
        let mut mask = RoadMaskBitmap::new(&bbox);
        // Place road at distance 1 — should NOT be found (step_by(2) starts at 2)
        mask.set(11, 10);

        let result = get_nearest_road_block(10, 10, 20, &mask);
        // dist=1 won't be found; dist=2 candidates are checked
        // (10, 8), (10, 12), (8, 10), (12, 10) etc. — none are roads
        // So unless there's a road at distance 2, result is None
        assert!(
            result.is_none() || {
                let (rx, rz) = result.unwrap();
                let dx = (rx - 10).abs();
                let dz = (rz - 10).abs();
                dx >= 2 || dz >= 2
            }
        );
    }
}
