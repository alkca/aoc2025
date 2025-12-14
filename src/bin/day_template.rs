use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

fn part1(input: &str) -> Result<i64> {
    let total = input.len() as i64;

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
        assert_eq!(part1(&input).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 40);
    }
}
