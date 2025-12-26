use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};
use std::str::FromStr;

#[derive(Debug)]
pub enum ParsePointError {
    WrongFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]

struct Point2 {
    pub x: i64,
    pub y: i64,
}

impl Point2 {
    // Integer distance that is good for comparison
    pub fn area(self, other: Self) -> u64 {
        let dx = self.x.abs_diff(other.x) + 1;
        let dy = self.y.abs_diff(other.y) + 1;
        dx * dy
    }
}

impl FromStr for Point2 {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<i64> = s.split(",").map(|n| n.parse::<i64>().unwrap()).collect();
        Ok(Point2 {
            x: points[0],
            y: points[1],
        })
    }
}

fn find_max_area(points: &[Point2]) -> usize {
    let mut max_area = 0;
    for (x, point) in points.iter().enumerate() {
        for other_point in points.iter().skip(x) {
            let area = point.area(*other_point);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area as usize
}

fn part1(input: &str) -> Result<i64> {
    let points: Vec<Point2> = input
        .lines()
        .map(|s| s.parse::<Point2>().unwrap())
        .collect();

    Ok(find_max_area(&points) as i64)
}

fn cross(a: Point2, b: Point2, c: Point2) -> i64 {
    // cross((b-a), (c-a))
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn on_segment(a: Point2, b: Point2, p: Point2) -> bool {
    cross(a, b, p) == 0
        && p.x >= a.x.min(b.x)
        && p.x <= a.x.max(b.x)
        && p.y >= a.y.min(b.y)
        && p.y <= a.y.max(b.y)
}

fn proper_intersect(a: Point2, b: Point2, c: Point2, d: Point2) -> bool {
    let o1 = cross(a, b, c);
    let o2 = cross(a, b, d);
    let o3 = cross(c, d, a);
    let o4 = cross(c, d, b);

    // Proper intersection: each segment straddles the line through the other
    (o1 > 0 && o2 < 0 || o1 < 0 && o2 > 0) && (o3 > 0 && o4 < 0 || o3 < 0 && o4 > 0)
}

fn point_inside_polygon(points: &[Point2], p: &Point2) -> bool {
    let len = points.len();

    // If on any edge => inside
    for i in 0..len {
        let a = points[i];
        let b = points[(i + 1) % len];
        if on_segment(a, b, *p) {
            return true;
        }
    }

    let mut inside = false;

    for i in 0..len {
        let a = points[i];
        let b = points[(i + 1) % len];

        // Check if edge straddles p.y (strict to avoid double-counting vertices)
        let ay_gt = a.y > p.y;
        let by_gt = b.y > p.y;
        if ay_gt == by_gt {
            continue;
        }

        // Compute whether intersection x_int > p.x without division:
        // x_int = a.x + (p.y - a.y) * (b.x - a.x) / (b.y - a.y)
        // Compare: x_int > p.x
        // => (p.y-a.y)*(b.x-a.x) > (p.x-a.x)*(b.y-a.y)
        let lhs = (p.y - a.y) * (b.x - a.x);
        let rhs = (p.x - a.x) * (b.y - a.y);

        if b.y - a.y > 0 {
            if lhs > rhs {
                inside = !inside;
            }
        } else if lhs < rhs {
            // if denominator negative, flip inequality
            inside = !inside;
        }
    }

    inside
}

fn edge_inside_polygon(points: &[Point2], edge: &(Point2, Point2)) -> bool {
    let len = points.len();
    for i in 0..len {
        let pa = points[i];
        let pb = points[(i + 1) % len];
        if proper_intersect(edge.0, edge.1, pa, pb) {
            return false;
        }
    }

    // Also check that the midpoint of the edge is inside the polygon
    // let midpoint = Point2 {
    //     x: (edge.0.x + edge.1.x) / 2,
    //     y: (edge.0.y + edge.1.y) / 2,
    // };
    // point_inside_polygon(points, &midpoint)
    true
}

fn check_contained_inside_polygon(points: &[Point2], corner1: &Point2, corner2: &Point2) -> bool {
    let x_min = corner1.x.min(corner2.x);
    let x_max = corner1.x.max(corner2.x);
    let y_min = corner1.y.min(corner2.y);
    let y_max = corner1.y.max(corner2.y);

    let top_left = Point2 { x: x_min, y: y_min };
    let top_right = Point2 { x: x_max, y: y_min };
    let bottom_left = Point2 { x: x_min, y: y_max };
    let bottom_right = Point2 { x: x_max, y: y_max };

    let rect_corners = [top_left, top_right, bottom_right, bottom_left];
    let rect_edges = [
        (top_left, top_right),
        (top_right, bottom_right),
        (bottom_right, bottom_left),
        (bottom_left, top_left),
    ];

    for corner in rect_corners {
        if !point_inside_polygon(points, &corner) {
            return false;
        }
    }

    for edge in rect_edges {
        if !edge_inside_polygon(points, &edge) {
            return false;
        }
    }

    true
}

fn find_max_area_part2(points: &[Point2]) -> usize {
    let mut max_area = 0;
    for (i, corner1) in points.iter().enumerate() {
        for corner2 in points.iter().skip(i) {
            if check_contained_inside_polygon(points, corner1, corner2) {
                let area = corner1.area(*corner2);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area as usize
}

fn part2(input: &str) -> Result<i64> {
    let points: Vec<Point2> = input
        .lines()
        .map(|s| s.parse::<Point2>().unwrap())
        .collect();

    Ok(find_max_area_part2(&points) as i64)
}

fn main() -> Result<()> {
    let day = extract_day_from_exe();
    let input = read_input(day);

    println!("Day {:02}", day);
    println!("===>Part 1: {}", part1(&input)?);
    println!("===>Part 2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2025::read_example;

    #[test]
    fn test_part1() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part1(&input).unwrap(), 50);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 24);
    }
}
