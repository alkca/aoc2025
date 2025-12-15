use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

fn calc_line_voltage(line: &str, num_of_batteries: usize) -> i64 {
    let mut total = 0i64;

    let digits: Vec<i8> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    let mut battery_list: Vec<i8> = vec![-1; num_of_batteries];
    let mut starting_pos = 0;
    for i in 0..battery_list.len() {
        for x in starting_pos..digits.len() - num_of_batteries + i + 1 {
            if digits[x] > battery_list[i] {
                starting_pos = x + 1;
                battery_list[i] = digits[x];
            }
        }
    }

    let mut multiplier = 10i64.pow(num_of_batteries as u32 - 1);
    for digit in battery_list.iter() {
        total += *digit as i64 * multiplier;
        multiplier /= 10;
    }
    total
}

fn part1(input: &str) -> Result<i64> {
    let mut total = 0i64;

    for line in input.lines() {
        total += calc_line_voltage(line, 2);
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i64> {
    let mut total = 0i64;

    for line in input.lines() {
        total += calc_line_voltage(line, 12);
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
        assert_eq!(part1(&input).unwrap(), 357);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 3121910778619);
    }
}
