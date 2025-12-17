use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};

fn part1(input: &str) -> Result<i64> {
    let mut total = 0;
    let mut grid: Vec<Vec<&str>> = Vec::new();

    for line in input.lines() {
        grid.push(line.split_whitespace().collect());
    }

    let math_ops = grid.pop().unwrap();
    // dbg!(&math_ops);

    for column in 0..grid[0].len() {
        let mut column_total: i64 = 0;
        let mut is_first_row = true;
        for row in grid.iter() {
            let val = row[column].parse::<i64>().unwrap();
            if is_first_row {
                column_total = val;
                is_first_row = false;
            } else {
                match math_ops[column] {
                    "+" => column_total += val,
                    "*" => column_total *= val,
                    _ => panic!("Unknown math operation"),
                }
            }
        }
        total += column_total;
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i64> {
    let mut total = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut math_ops = Vec::new();
    let last_row = grid.pop().unwrap();
    for c in last_row.iter() {
        if *c != ' ' {
            math_ops.push(c);
        }
    }

    let mut num_grid: Vec<Vec<i64>> = Vec::new();
    let mut num_column: Vec<i64> = Vec::new();
    for x in 0..grid[0].len() {
        let mut num_str: String = String::new();
        for row in grid.iter() {
            if row[x] != ' ' {
                num_str += &row[x].to_string();
            }
        }
        if num_str.is_empty() {
            num_grid.push(num_column);
            num_column = Vec::new();
        } else {
            num_column.push(num_str.parse::<i64>().unwrap());
        }
    }
    num_grid.push(num_column);

    for (math_idx, x) in num_grid.iter().enumerate() {
        let mut column_total: i64 = 0;
        let mut is_first_row = true;
        for y in x.iter() {
            let val = y;
            if is_first_row {
                column_total = *val;
                is_first_row = false;
            } else {
                // println!("math={} on val={}", math_ops[math_idx], val);
                match math_ops[math_idx] {
                    '+' => column_total += val,
                    '*' => column_total *= val,
                    _ => panic!("Unknown math operation"),
                }
            }
        }
        total += column_total;
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
        assert_eq!(part1(&input).unwrap(), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 3263827);
    }
}
