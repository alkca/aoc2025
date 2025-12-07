use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

const SAFE_DIAL_POS: i32 = 100;

fn split_at_first_letter(s: &str) -> Option<(String, String)> {
    let mut chars = s.chars();
    let first = chars.next()?.to_string();
    let rest = chars.as_str().to_string();
    Some((first, rest))
}

fn part1(input: &str) -> Result<i32> {
    let mut times_at_zero = 0;
    let mut dial_pos = 50;

    for line in input.lines() {
        let (dir, amount) = split_at_first_letter(line)
            .map(|(first, rest)| (first.to_string(), rest.parse::<i32>().ok()))
            .and_then(|(s, num)| num.map(|n| (s, n)))
            .expect("Invalid dial rotation format");

        if dir == "L" {
            dial_pos -= amount % SAFE_DIAL_POS;
            if dial_pos < 0 {
                dial_pos += SAFE_DIAL_POS;
            }
        } else {
            dial_pos += amount % SAFE_DIAL_POS;
            if dial_pos >= SAFE_DIAL_POS {
                dial_pos -= SAFE_DIAL_POS;
            }
        }
        if dial_pos == 0 {
            times_at_zero += 1
        }
    }

    Ok(times_at_zero)
}

fn part2(input: &str) -> Result<i32> {
    let mut times_past_zero = 0;
    let mut dial_pos = 50;

    for line in input.lines() {
        let (dir, amount) = split_at_first_letter(line)
            .map(|(first, rest)| (first.to_string(), rest.parse::<i32>().ok()))
            .and_then(|(s, num)| num.map(|n| (s, n)))
            .expect("Invalid dial rotation format");

        if dir == "L" {
            if dial_pos == 0 {
                times_past_zero -= 1 //Don't count if already on zero
            }
            dial_pos -= amount;
            while dial_pos < 0 {
                dial_pos += SAFE_DIAL_POS;
                times_past_zero += 1;
            }
            if dial_pos == 0 {
                times_past_zero += 1
            }
        } else {
            dial_pos += amount;
            while dial_pos >= SAFE_DIAL_POS {
                times_past_zero += 1;
                dial_pos -= SAFE_DIAL_POS;
            }
        }
    }

    Ok(times_past_zero)
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
        assert_eq!(part2(&input).unwrap(), 6);
    }
}
