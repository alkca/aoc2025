use std::collections::HashMap;

use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManifoldState {
    Empty,
    Start,
    Beam,
    Splitter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn get_manifold_state(c: char) -> ManifoldState {
    match c {
        '.' => ManifoldState::Empty,
        'S' => ManifoldState::Start,
        '|' => ManifoldState::Beam,
        '^' => ManifoldState::Splitter,
        _ => panic!("Unknown manifold state: {}", c),
    }
}

fn get_starting_point(grid: &[Vec<ManifoldState>]) -> Point {
    for (y, row) in grid.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == ManifoldState::Start {
                return Point { x, y };
            }
        }
    }
    panic!("No where to start from")
}

fn part1(input: &str) -> Result<i32> {
    let mut total = 0;
    let mut manifold: Vec<Vec<ManifoldState>> = input
        .lines()
        .map(|line| line.chars().map(get_manifold_state).collect())
        .collect();

    let y_size = manifold.len();
    let x_size = if y_size > 0 { manifold[0].len() } else { 0 };

    let starting_point = get_starting_point(&manifold);
    manifold[starting_point.y + 1][starting_point.x] = ManifoldState::Beam;

    for y in (starting_point.y + 1)..y_size {
        for x in 0..x_size {
            if manifold[y][x] == ManifoldState::Beam && y + 1 < y_size {
                if manifold[y + 1][x] == ManifoldState::Empty {
                    manifold[y + 1][x] = ManifoldState::Beam;
                } else if manifold[y + 1][x] == ManifoldState::Splitter {
                    total += 1;
                    if x > 0 && manifold[y + 1][x - 1] == ManifoldState::Empty {
                        manifold[y + 1][x - 1] = ManifoldState::Beam;
                    }
                    if x + 1 < x_size && manifold[y + 1][x + 1] == ManifoldState::Empty {
                        manifold[y + 1][x + 1] = ManifoldState::Beam;
                    }
                }
            }
        }
    }

    Ok(total)
}

fn part2_split(
    manifold: &[Vec<ManifoldState>],
    starting_point: &Point,
    total_cache: &mut HashMap<Point, i64>,
) -> i64 {
    let mut total = 0;
    let next_row = starting_point.y + 1;

    // println!(
    //     "Starting at: y={}, x={}",
    //     starting_point.y, starting_point.x
    // );
    if let Some(&v) = total_cache.get(starting_point) {
        return v;
    }

    if next_row >= manifold.len() {
        return 1; // End of grid
    }
    if manifold[starting_point.y + 1][starting_point.x] == ManifoldState::Empty {
        let next_point = Point {
            x: starting_point.x,
            y: next_row,
        };
        total += part2_split(manifold, &next_point, total_cache);
    } else {
        //Must be splitter
        if starting_point.x > 0 {
            let next_point = Point {
                x: starting_point.x - 1,
                y: next_row,
            };
            total += part2_split(manifold, &next_point, total_cache);
        }
        if starting_point.x + 1 < manifold[0].len() {
            let next_point = Point {
                x: starting_point.x + 1,
                y: next_row,
            };
            total += part2_split(manifold, &next_point, total_cache);
        }
    }

    // println!(
    //     "Finish total = {}, y={}, x={}",
    //     total, starting_point.y, starting_point.x
    // );

    total_cache.insert(*starting_point, total);
    total
}

fn part2(input: &str) -> Result<i64> {
    let manifold: Vec<Vec<ManifoldState>> = input
        .lines()
        .map(|line| line.chars().map(get_manifold_state).collect())
        .collect();
    let mut total_cache = HashMap::new();

    let starting_point = get_starting_point(&manifold);

    Ok(part2_split(
        &manifold,
        &Point {
            x: starting_point.x,
            y: starting_point.y + 1,
        },
        &mut total_cache,
    ))
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
        assert_eq!(part1(&input).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 40);
    }
}
