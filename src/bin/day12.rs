use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};
use std::fmt;

#[derive(Debug, Clone, Default)]
struct Region {
    x: usize,
    y: usize,
    quantity: Vec<usize>,
}

#[derive(Clone, Default)]
struct Shape {
    x: usize,
    y: usize,
    mask: usize,
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_bits = self.x * self.y;

        writeln!(f, "Shape {{")?;
        writeln!(f, "  size: {}x{}", self.x, self.y)?;
        writeln!(
            f,
            "  mask (binary): {:0width$b}",
            self.mask,
            width = total_bits
        )?;
        writeln!(f, "  grid:")?;

        for row in 0..self.y {
            write!(f, "    ")?;
            for col in 0..self.x {
                let bit = (self.y - row - 1) * self.x + self.x - col - 1;
                let c = if (self.mask >> bit) & 1 == 1 {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        write!(f, "}}")
    }
}

fn parse_region_input(input: &Vec<&str>) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    dbg!(input);

    for region_line in input {
        dbg!(region_line);
        let mut region: Region = Region::default();
        let numbers: Vec<usize> = region_line
            .split([' ', ':', 'x'])
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        region.x = numbers[0];
        region.y = numbers[1];
        region.quantity = numbers[2..].to_vec();

        regions.push(region);
    }

    regions
}

/// Asumes shape input is rectangle and that the shape mask fits in a usize
fn parse_shape_input(input: &Vec<Vec<&str>>) -> Vec<Shape> {
    let mut shapes: Vec<Shape> = Vec::new();

    for shape_lines in input {
        let mut shape_str: String = String::new();

        for shape_line in shape_lines.iter().skip(1) {
            shape_str += shape_line.trim();
        }

        let mut shape: Shape = Shape::default();

        let mut offset = 1 << shape_str.len();
        shape.mask = 0;
        for c in shape_str.chars() {
            offset >>= 1;
            if c == '#' {
                shape.mask += offset;
            }
        }
        shape.y = shape_lines.len() - 1;
        shape.x = shape_str.len() / shape.y;
        shapes.push(shape);
    }

    shapes
}

fn shapes_fit_into_region(shapes: &Vec<Shape>, region: &Region) -> bool {
    true
}

/// Every shape is a 3x3 grid and every region size is >3 and therefore will hardcode assumptions
/// around the size for performance reasons.  The shape input will be stored as a 9 bit mask.
fn part1(input: &str) -> Result<i64> {
    const SHAPE_X_SIZE: usize = 3;
    const SHAPE_Y_SIZE: usize = 3;

    let mut parts: Vec<Vec<&str>> = Vec::new();
    let mut part: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            parts.push(part);
            part = Vec::new();
        } else {
            part.push(line);
        }
    }
    parts.push(part);

    let regions: Vec<Region> = parse_region_input(&parts.pop().unwrap());
    let shapes: Vec<Shape> = parse_shape_input(&parts);

    dbg!(&regions, &shapes);
    let mut total = 0;
    for region in regions {
        if shapes_fit_into_region(&shapes, &region) {
            total += 1;
        }
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i64> {
    let total = input.len() as i64;

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
        assert_eq!(part1(&input).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 2);
    }
}
