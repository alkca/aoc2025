use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};
use std::cmp::Ordering;
use std::{collections::BinaryHeap, str::FromStr};

#[derive(Debug)]
pub enum ParsePoint3Error {
    WrongFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]

struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    // Integer distance that is good for comparison
    pub fn distance_int(self, other: Self) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

impl FromStr for Point3 {
    type Err = ParsePoint3Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<i32> = s.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        Ok(Point3 {
            x: points[0],
            y: points[1],
            z: points[2],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

struct DistBetweenPoint3 {
    dist: i64,
    origin: Point3,
    dest: Point3,
}

impl Ord for DistBetweenPoint3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist
            .cmp(&other.dist) // larger dist = greater (max-heap)
            .then_with(|| self.origin.cmp(&other.origin))
            .then_with(|| self.dest.cmp(&other.dest))
    }
}

impl PartialOrd for DistBetweenPoint3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc_shortest_points_list(points: &[Point3], size: usize) -> BinaryHeap<DistBetweenPoint3> {
    let mut list: BinaryHeap<DistBetweenPoint3> = BinaryHeap::with_capacity(size + 1);

    for (i, point) in points.iter().enumerate() {
        //Optimize by not starting at the beginning of points for second point as already calced
        for other in points.iter().skip(i + 1) {
            let dist = DistBetweenPoint3 {
                dist: point.distance_int(*other),
                origin: *point,
                dest: *other,
            };
            list.push(dist);
            if list.len() > size {
                list.pop();
            }
        }
    }

    list
}

fn calc_top_circuits(
    points_list: BinaryHeap<DistBetweenPoint3>,
    shortest_count: usize,
    top_count: usize,
) -> usize {
    let mut total = 1;
    let mut circuits: Vec<Vec<Point3>> = Vec::new();
    let mut connection_count = shortest_count;

    for p in points_list.into_sorted_vec().iter() {
        // dbg!(p);
        let mut merge_circuit_idx = None;
        let mut found_circuit_idx = None;
        for (i, circuit) in circuits.iter_mut().enumerate() {
            //        for circuit in circuits.iter_mut() {
            let origin_exists = circuit.contains(&p.origin);
            let dest_exists = circuit.contains(&p.dest);
            if origin_exists && dest_exists {
                break;
            } else if origin_exists || dest_exists {
                if found_circuit_idx.is_some() {
                    //If we had already found the circuit we need to merge into it
                    merge_circuit_idx = Some(i);
                    break;
                }
                connection_count -= 1;
                found_circuit_idx = Some(i);
                if !origin_exists {
                    circuit.push(p.origin);
                }
                if !dest_exists {
                    circuit.push(p.dest);
                }
            }
        }
        if let Some(merge_into_idx) = found_circuit_idx {
            if let Some(idx) = merge_circuit_idx {
                let mut to_merge = circuits.remove(idx);
                circuits[merge_into_idx].append(&mut to_merge);
                circuits[merge_into_idx].sort();
                circuits[merge_into_idx].dedup();
            }
        } else {
            circuits.push(vec![p.origin, p.dest]);
            connection_count -= 1;
        }
        if connection_count == 0 {
            break;
        }
        // dbg!(&circuits);
    }
    // dbg!(&circuits);

    circuits.sort_by_key(|v| v.len());
    for _ in 0..top_count {
        let box_count = circuits.pop().unwrap().len();
        dbg!(box_count);
        total *= box_count;
        // dbg!(total);
    }

    total
}

// shortest_count: is what number of shortest points between two points to collect
// circuit_count: is the number of top largest circuits to factor in the calculation
fn part1(input: &str, shortest_count: usize, circuit_count: usize) -> Result<i64> {
    let boxes: Vec<Point3> = input
        .lines()
        .map(|s| s.parse::<Point3>().unwrap())
        .collect();

    let points_list: BinaryHeap<DistBetweenPoint3> =
        calc_shortest_points_list(&boxes, shortest_count);

    let total = calc_top_circuits(points_list, shortest_count, circuit_count) as i64;

    Ok(total)
}

fn calc_last_two_boxes(points_list: BinaryHeap<DistBetweenPoint3>, total_boxes: usize) -> i32 {
    let mut total: i32 = 1;
    let mut circuits: Vec<Vec<Point3>> = Vec::new();

    for p in points_list.into_sorted_vec().iter() {
        // dbg!(p);
        let mut merge_circuit_idx = None;
        let mut found_circuit_idx = None;
        for (i, circuit) in circuits.iter_mut().enumerate() {
            //        for circuit in circuits.iter_mut() {
            let origin_exists = circuit.contains(&p.origin);
            let dest_exists = circuit.contains(&p.dest);
            if origin_exists && dest_exists {
                break;
            } else if origin_exists || dest_exists {
                if found_circuit_idx.is_some() {
                    //If we had already found the circuit we need to merge into it
                    merge_circuit_idx = Some(i);
                    break;
                }
                found_circuit_idx = Some(i);
                if !origin_exists {
                    circuit.push(p.origin);
                }
                if !dest_exists {
                    circuit.push(p.dest);
                }
            }
        }
        if let Some(merge_into_idx) = found_circuit_idx {
            if let Some(idx) = merge_circuit_idx {
                let mut to_merge = circuits.remove(idx);
                circuits[merge_into_idx].append(&mut to_merge);
                circuits[merge_into_idx].sort();
                circuits[merge_into_idx].dedup();
            }
        } else {
            circuits.push(vec![p.origin, p.dest]);
        }
        if circuits[0].len() == total_boxes {
            total = p.origin.x * p.dest.x;
            break;
        }
        // dbg!(&circuits);
    }
    // dbg!(&circuits);

    total
}

fn part2(input: &str) -> Result<i64> {
    let boxes: Vec<Point3> = input
        .lines()
        .map(|s| s.parse::<Point3>().unwrap())
        .collect();

    let points_list: BinaryHeap<DistBetweenPoint3> =
        calc_shortest_points_list(&boxes, boxes.len() * 10); // UGLY Hack with 10 constant to throw processing power instead of elegant solution

    let total = calc_last_two_boxes(points_list, boxes.len()) as i64;

    Ok(total)
}

fn main() -> Result<()> {
    let day = extract_day_from_exe();
    let input = read_input(day);

    println!("Day {:02}", day);
    println!("===>Part 1: {}", part1(&input, 1000, 3)?);
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
        assert_eq!(part1(&input, 10, 3).unwrap(), 40);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 25272);
    }
}
