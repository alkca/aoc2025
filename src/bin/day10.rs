use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]

struct Machine {
    button_count: usize,
    target: u16,
    toggles: Vec<u16>,
    voltages: Vec<u16>,
}

#[derive(Debug)]
pub enum ParseMachineError {
    WrongFormat,
}

impl FromStr for Machine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list: Vec<&str> = s.split_whitespace().collect();

        let target_str = &list[0][1..list[0].len() - 1];
        let button_count = target_str.len();
        let mut power = 1 << (button_count - 1);
        let mut target = 0;
        for c in target_str.chars() {
            if c == '#' {
                target += power;
            }
            power >>= 1;
        }

        let voltages_str = list.pop().unwrap();
        let voltages: Vec<u16> = voltages_str[1..voltages_str.len() - 1]
            .split(",")
            .map(|n| n.parse::<u16>().unwrap())
            .collect();

        let mut toggles: Vec<u16> = Vec::new();
        for toggle in list.iter().skip(1) {
            let mut toggle_num = 0;
            for button in toggle[1..toggle.len() - 1]
                .split(",")
                .map(|n| n.parse::<u16>().unwrap())
            {
                toggle_num += 1 << (button_count as u16 - button - 1);
            }
            toggles.push(toggle_num);
        }

        Ok(Machine {
            button_count,
            target,
            toggles,
            voltages,
        })
    }
}

/// Leverage BFS algorithm to find shortest path from 0 to the target value.  Every possible button
/// value is a node, every toggle represents an edge from a node value to another node value.
fn process_machine(machine: Machine) -> Option<i64> {
    const START: u16 = 0;
    let node_size = 1 << machine.button_count;
    let mut graph: Vec<Vec<u16>> = Vec::with_capacity(node_size);
    graph.resize_with(node_size, Vec::new);

    // Build graph edges
    for (i, node) in graph.iter_mut().enumerate() {
        for toggle in &machine.toggles {
            let edge = i as u16 ^ toggle;
            if !node.contains(&edge) {
                node.push(edge);
            }
        }
    }

    // BFS to find shortest path from 0 to target
    let mut queue: VecDeque<u16> = VecDeque::new();
    let mut parent: Vec<Option<u16>> = vec![None; node_size];

    parent[START as usize] = Some(START);
    queue.push_front(START);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        for edge in graph[node as usize].iter() {
            if edge == &machine.target {
                let mut n = node;
                let mut depth: i64 = 1;
                while n != START {
                    n = parent[n as usize].unwrap();
                    depth += 1;
                }
                dbg!(&depth);
                return Some(depth);
            }
            if parent[*edge as usize].is_none() {
                parent[*edge as usize] = Some(node);
                queue.push_back(*edge);
            }
        }
    }

    None
}

/// Process part 1 of the Day 10 puzzle.  With the input format looking like a binary switch and
/// applying binary XOR's to the switch to get the target I will convert all of these to binary
/// input.  It doesn't make sense to press the same button more than once as the second press
/// cancels out the first.
fn part1(input: &str) -> Result<i64> {
    let mut total = 0;

    for line in input.lines() {
        let machine = line.parse::<Machine>().unwrap();
        // dbg!(&machine);
        if let Some(num) = process_machine(machine) {
            total += num;
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
        assert_eq!(part1(&input).unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 33);
    }
}
