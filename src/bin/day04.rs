use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PosState {
    Paper,
    Empty,
    Pending,
}

type Offset = (i32, i32);

const NEIGHBORS: [Offset; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_grid_state(c: char) -> PosState {
    match c {
        '@' => PosState::Paper,
        '.' => PosState::Empty,
        'x' => PosState::Pending,
        _ => panic!("Unknown state: {}", c),
    }
}

fn grid_get(grid: &[Vec<PosState>], p: Point) -> Option<PosState> {
    let y = usize::try_from(p.y).ok()?;
    let x = usize::try_from(p.x).ok()?;
    grid.get(y)?.get(x).copied()
}

fn has_access(grid: &mut [Vec<PosState>], pos: Point) -> bool {
    let mut count_paper = 0;
    for neighbor in NEIGHBORS.iter() {
        let new_pos = Point {
            x: pos.x + neighbor.0,
            y: pos.y + neighbor.1,
        };
        if let Some(state) = grid_get(grid, new_pos)
            && state != PosState::Empty
        {
            count_paper += 1;
        }
    }
    if count_paper < 4 {
        grid[pos.y as usize][pos.x as usize] = PosState::Pending;
        true
    } else {
        false
    }
}

fn clean_grid(grid: &mut [Vec<PosState>]) {
    let y_size = &grid.len();
    let x_size = &grid[0].len();

    for y in 0..*y_size {
        for x in 0..*x_size {
            if grid[y][x] == PosState::Pending {
                grid[y][x] = PosState::Empty;
            }
        }
    }
}

fn part1(input: &str) -> Result<i64> {
    let mut total = 0;
    let mut grid: Vec<Vec<PosState>> = input
        .lines()
        .map(|line| line.chars().map(get_grid_state).collect())
        .collect();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == PosState::Paper
                && has_access(
                    &mut grid,
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                )
            {
                // dbg!(x, y, total);
                total += 1;
            }
        }
    }
    // dbg!(&grid, total);

    Ok(total)
}

fn part2(input: &str) -> Result<i64> {
    let mut total = 0;
    let mut grid: Vec<Vec<PosState>> = input
        .lines()
        .map(|line| line.chars().map(get_grid_state).collect())
        .collect();

    let y_size = &grid.len();
    let x_size = &grid[0].len();

    let mut keep_repeating = true;
    while keep_repeating {
        keep_repeating = false;
        for y in 0..*y_size {
            for x in 0..*x_size {
                if grid[y][x] == PosState::Paper
                    && has_access(
                        &mut grid,
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                    )
                {
                    // dbg!(x, y, total);
                    total += 1;
                    keep_repeating = true;
                }
            }
        }
        if keep_repeating {
            clean_grid(&mut grid);
        }
    }
    // dbg!(&grid, total);

    Ok(total)
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
        assert_eq!(part1(&input).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 43);
    }
}
