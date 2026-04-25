/// Generates the coordinates for a line between two points using the Bresenham algorithm.
/// The result is a vector of 3D coordinates (x, y, z).
pub fn bresenham_line(
    x1: i32,
    y1: i32,
    z1: i32,
    x2: i32,
    y2: i32,
    z2: i32,
) -> Vec<(i32, i32, i32)> {
    // Calculate max possible points needed
    let dx = if x2 > x1 { x2 - x1 } else { x1 - x2 };
    let dy = if y2 > y1 { y2 - y1 } else { y1 - y2 };
    let dz = if z2 > z1 { z2 - z1 } else { z1 - z2 };

    // Pre-allocate vector with exact size needed
    let capacity = dx.max(dy).max(dz) + 1;
    let mut points = Vec::with_capacity(capacity as usize);
    points.reserve_exact(capacity as usize);

    let xs = if x1 < x2 { 1 } else { -1 };
    let ys = if y1 < y2 { 1 } else { -1 };
    let zs = if z1 < z2 { 1 } else { -1 };

    let mut x = x1;
    let mut y = y1;
    let mut z = z1;

    // Determine dominant axis once, outside the loop
    if dx >= dy && dx >= dz {
        let mut p1 = 2 * dy - dx;
        let mut p2 = 2 * dz - dx;

        while x != x2 {
            points.push((x, y, z));

            if p1 >= 0 {
                y += ys;
                p1 -= 2 * dx;
            }
            if p2 >= 0 {
                z += zs;
                p2 -= 2 * dx;
            }
            p1 += 2 * dy;
            p2 += 2 * dz;
            x += xs;
        }
    } else if dy >= dx && dy >= dz {
        let mut p1 = 2 * dx - dy;
        let mut p2 = 2 * dz - dy;

        while y != y2 {
            points.push((x, y, z));

            if p1 >= 0 {
                x += xs;
                p1 -= 2 * dy;
            }
            if p2 >= 0 {
                z += zs;
                p2 -= 2 * dy;
            }
            p1 += 2 * dx;
            p2 += 2 * dz;
            y += ys;
        }
    } else {
        let mut p1 = 2 * dy - dz;
        let mut p2 = 2 * dx - dz;

        while z != z2 {
            points.push((x, y, z));

            if p1 >= 0 {
                y += ys;
                p1 -= 2 * dz;
            }
            if p2 >= 0 {
                x += xs;
                p2 -= 2 * dz;
            }
            p1 += 2 * dy;
            p2 += 2 * dx;
            z += zs;
        }
    }

    points.push((x2, y2, z2));
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_point() {
        let points = bresenham_line(5, 5, 5, 5, 5, 5);
        assert_eq!(points, vec![(5, 5, 5)]);
    }

    #[test]
    fn horizontal_x_line() {
        let points = bresenham_line(0, 0, 0, 5, 0, 0);
        assert_eq!(points.len(), 6);
        assert_eq!(points[0], (0, 0, 0));
        assert_eq!(points[5], (5, 0, 0));
        // All y and z should be 0
        assert!(points.iter().all(|&(_, y, z)| y == 0 && z == 0));
    }

    #[test]
    fn horizontal_z_line() {
        let points = bresenham_line(0, 0, 0, 0, 0, 5);
        assert_eq!(points.len(), 6);
        assert_eq!(points[0], (0, 0, 0));
        assert_eq!(points[5], (0, 0, 5));
    }

    #[test]
    fn vertical_y_line() {
        let points = bresenham_line(0, 0, 0, 0, 5, 0);
        assert_eq!(points.len(), 6);
        assert_eq!(points[0], (0, 0, 0));
        assert_eq!(points[5], (0, 5, 0));
    }

    #[test]
    fn diagonal_3d() {
        let points = bresenham_line(0, 0, 0, 3, 3, 3);
        // Should start at origin and end at (3,3,3)
        assert_eq!(*points.first().unwrap(), (0, 0, 0));
        assert_eq!(*points.last().unwrap(), (3, 3, 3));
        // Should have 4 points (0,1,2,3)
        assert_eq!(points.len(), 4);
    }

    #[test]
    fn negative_direction() {
        let points = bresenham_line(5, 5, 5, 0, 0, 0);
        assert_eq!(*points.first().unwrap(), (5, 5, 5));
        assert_eq!(*points.last().unwrap(), (0, 0, 0));
    }

    #[test]
    fn endpoints_always_included() {
        let points = bresenham_line(1, 2, 3, 7, 11, 4);
        assert_eq!(*points.first().unwrap(), (1, 2, 3));
        assert_eq!(*points.last().unwrap(), (7, 11, 4));
    }

    #[test]
    fn adjacent_points_are_connected() {
        let points = bresenham_line(0, 0, 0, 10, 7, 3);
        for i in 1..points.len() {
            let (x1, y1, z1) = points[i - 1];
            let (x2, y2, z2) = points[i];
            let max_step = (x2 - x1).abs().max((y2 - y1).abs()).max((z2 - z1).abs());
            assert!(
                max_step <= 1,
                "Gap between points {} and {}: step={}",
                i - 1,
                i,
                max_step
            );
        }
    }

    #[test]
    fn flat_2d_line() {
        // Common use case: 2D line at y=0
        let points = bresenham_line(0, 0, 0, 10, 0, 5);
        assert!(points.iter().all(|&(_, y, _)| y == 0));
        assert_eq!(*points.last().unwrap(), (10, 0, 5));
    }
}
