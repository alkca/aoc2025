use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

fn chunk_string(s: &str, size: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn split_string(s: &str, size: usize) -> (String, String) {
    let mut chars = s.chars();
    let start: String = chars.by_ref().take(size).collect();
    let rest: String = chars.collect();

    (start, rest)
}

fn valid_id_part1(id: i64) -> bool {
    let str_id = id.to_string();

    let (start, rest) = split_string(&str_id, str_id.len() / 2);
    !start.eq(&rest)
}

fn valid_id_part2(id: i64) -> bool {
    let str_id = id.to_string();

    // dbg!(&str_id, &str_id.len() / 2);

    for size in 1..(str_id.len() / 2 + 1) {
        let chunks = chunk_string(&str_id, size);
        let mut is_valid = false;
        for chunk in &chunks[1..] {
            if !chunks[0].eq(chunk) {
                is_valid = true;
                break;
            }
        }
        if !is_valid {
            return false;
        }
    }
    true
}

fn process_id_range(min_id: i64, max_id: i64, valid_id: fn(i64) -> bool) -> i64 {
    let mut total = 0i64;
    // dbg!(("id range ", min_id, max_id));

    for id in min_id..=max_id {
        if valid_id(id) {
            // dbg!(("valid: ", id));
        } else {
            // dbg!(("invalid: ", id));
            total += id;
        }
    }

    total
}

fn part1(input: &str) -> Result<i64> {
    let mut total = 0i64;
    let id_ranges: Vec<Vec<i64>> = input
        .split(",")
        .map(|ids| ids.split("-").map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();

    // dbg!(&id_ranges);

    for ids in id_ranges.iter() {
        total += process_id_range(ids[0], ids[1], valid_id_part1);
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i64> {
    let mut total = 0i64;
    let id_ranges: Vec<Vec<i64>> = input
        .split(",")
        .map(|ids| ids.split("-").map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();

    // dbg!(&id_ranges);

    for ids in id_ranges.iter() {
        total += process_id_range(ids[0], ids[1], valid_id_part2);
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
        assert_eq!(part1(&input).unwrap(), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 4174379265);
    }
}
