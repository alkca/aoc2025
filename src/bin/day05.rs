use std::ops::RangeInclusive;

use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

fn compress_range(mut ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    let mut compressed_ranges: Vec<RangeInclusive<i64>> = Vec::new();
    let mut last_range: Option<RangeInclusive<i64>> = None;

    ranges.sort_by_key(|r| *r.start());

    for range in ranges.iter() {
        if let Some(r) = last_range {
            if range.start() <= r.end() {
                last_range = Some(*r.start()..=*range.end().max(r.end()));
            } else {
                compressed_ranges.push(r);
                last_range = Some(range.clone());
            }
        } else {
            last_range = Some(range.clone());
        }
    }
    compressed_ranges.push(last_range.unwrap());

    compressed_ranges
}

fn part1(input: &str) -> Result<i64> {
    let mut total = 0;
    let mut is_ingrediant = false;
    let mut fresh_range: Vec<RangeInclusive<i64>> = Vec::new();
    let mut ingrediants: Vec<i64> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            is_ingrediant = true;
        } else if is_ingrediant {
            ingrediants.push(line.parse::<i64>().unwrap());
        } else {
            let nums: Vec<&str> = line.split("-").collect();
            let range = RangeInclusive::new(
                nums[0].parse::<i64>().unwrap(),
                nums[1].parse::<i64>().unwrap(),
            );
            fresh_range.push(range);
        }
    }

    let compressed_fresh_range = compress_range(fresh_range);

    for i in ingrediants.iter() {
        for r in compressed_fresh_range.iter() {
            if r.end() >= i {
                if r.contains(i) {
                    total += 1;
                }
                break;
            }
        }
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i64> {
    let mut total = 0;
    let mut is_ingrediant = false;
    let mut fresh_range: Vec<RangeInclusive<i64>> = Vec::new();
    let mut ingrediants: Vec<i64> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            is_ingrediant = true;
        } else if is_ingrediant {
            ingrediants.push(line.parse::<i64>().unwrap());
        } else {
            let nums: Vec<&str> = line.split("-").collect();
            let range = RangeInclusive::new(
                nums[0].parse::<i64>().unwrap(),
                nums[1].parse::<i64>().unwrap(),
            );
            fresh_range.push(range);
        }
    }

    let compressed_fresh_range = compress_range(fresh_range);

    for r in compressed_fresh_range.iter() {
        total += r.end() - r.start() + 1;
    }

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
        assert_eq!(part1(&input).unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 14);
    }
}
