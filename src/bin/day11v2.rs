use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug)]
struct PathNode {
    head: u16,
    prev: Option<Rc<PathNode>>,
}

impl PathNode {
    fn new(head: u16, prev: Option<Rc<PathNode>>) -> Rc<PathNode> {
        Rc::new(PathNode { head, prev })
    }

    /// O(length) contains check (same as Vec::contains, but no cloning)
    fn contains(node: &Rc<PathNode>, target: u16) -> bool {
        let mut cur: Option<&PathNode> = Some(node.as_ref());
        while let Some(n) = cur {
            if n.head == target {
                return true;
            }
            cur = n.prev.as_deref();
        }
        false
    }

    /// Build Vec in start->...->end order
    fn to_vec(node: &Rc<PathNode>) -> Vec<u16> {
        let mut out = Vec::new();
        let mut cur: Option<&PathNode> = Some(node.as_ref());
        while let Some(n) = cur {
            out.push(n.head);
            cur = n.prev.as_deref();
        }
        out.reverse();
        out
    }
}

//#[derive(Debug)]
#[derive(Debug, Clone)]
struct Graph {
    index_by_string: HashMap<String, u16>,
    edges: Vec<Vec<u16>>,
}

impl Graph {
    fn with_capacity(node_size: usize) -> Self {
        let mut edges: Vec<Vec<u16>> = Vec::with_capacity(node_size);
        edges.resize_with(node_size, Vec::new);

        Self {
            index_by_string: HashMap::new(),
            edges,
        }
    }

    fn add_node(&mut self, node: &str) -> u16 {
        if let Some(&index) = self.index_by_string.get(node) {
            return index;
        }

        let index = self.index_by_string.len() as u16;
        self.index_by_string.insert(node.to_string(), index);
        index
    }

    fn add_edge(&mut self, node: &str, edge: &str) {
        let from_index = self.add_node(node);
        let to_index = self.add_node(edge);
        self.edges[from_index as usize].push(to_index);
    }

    fn dfs_paths(&mut self, start: &str, end: &str) -> Vec<Vec<u16>> {
        let mut paths: Vec<Vec<u16>> = Vec::new();
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);

        // Queue for DFS of (node, path to node)
        let mut queue: Vec<(u16, Vec<u16>)> = vec![(start_index, vec![start_index])];

        while let Some((index, path)) = queue.pop() {
            if index == end_index {
                if paths.len() % 100 == 0 {
                    dbg!(paths.len(), &path);
                }

                paths.push(path.clone());
                continue;
            }
            // dbg!(start_index, end_index);
            // dbg!(&path);
            for edge in self.edges[index as usize].iter() {
                if !path.contains(edge) {
                    let mut new_path = path.clone();
                    new_path.push(*edge);
                    queue.push((*edge, new_path));
                }
            }
        }

        paths
    }

    fn dfs_paths_with_required_ids(
        &mut self,
        start: &str,
        end: &str,
        required_ids: &[u16],
    ) -> Vec<Vec<u16>> {
        let mut paths: Vec<Vec<u16>> = Vec::new();
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);

        // Queue for DFS of (node, path to node)
        let mut queue: Vec<(u16, Vec<u16>)> = vec![(start_index, vec![start_index])];

        let mut count: i64 = 0;
        let start_time = Instant::now();
        let mut last_debug_time = start_time;

        while let Some((index, path)) = queue.pop() {
            count += 1;

            if count % 10000000 == 0 {
                let current_time = Instant::now();
                let elapsed_since_last = current_time.duration_since(last_debug_time);
                dbg!(count, &queue.len(), elapsed_since_last);
                last_debug_time = current_time;
            }
            if index == end_index {
                // if paths.len() % 1000 == 0 {
                //     dbg!(required_ids, paths.len(), &path);
                // }

                let mut valid_path = true;
                for node in required_ids.iter() {
                    if !path.contains(node) {
                        valid_path = false;
                        break;
                    }
                }
                if valid_path {
                    dbg!(required_ids, paths.len(), &path);
                    paths.push(path.clone());
                }
                continue;
            }
            // dbg!(start_index, end_index);
            // dbg!(&path);
            for edge in self.edges[index as usize].iter() {
                if !path.contains(edge) {
                    let mut new_path = Vec::with_capacity(path.len() + 1);
                    new_path.extend_from_slice(&path);
                    new_path.push(*edge);
                    queue.push((*edge, new_path));
                }
            }
        }

        paths
    }

    fn dfs_paths_with_required_ids2(
        &mut self,
        start: &str,
        end: &str,
        required_ids: &[u16],
    ) -> Vec<Vec<u16>> {
        let mut paths: Vec<Vec<u16>> = Vec::new();
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);

        // Queue for DFS of (node, path_to_node_as_rc)
        let start_path = PathNode::new(start_index, None);
        let mut queue: Vec<(u16, Rc<PathNode>)> = vec![(start_index, start_path)];

        let mut count: i64 = 0;
        let start_time = Instant::now();
        let mut last_debug_time = start_time;

        while let Some((index, path)) = queue.pop() {
            count += 1;

            if count % 10_000_000 == 0 {
                let current_time = Instant::now();
                let elapsed_since_last = current_time.duration_since(last_debug_time);
                dbg!(count, queue.len(), elapsed_since_last);
                last_debug_time = current_time;
            }

            if index == end_index {
                // validate required_ids against the Rc-path
                let mut valid_path = true;
                for &req in required_ids {
                    if !PathNode::contains(&path, req) {
                        valid_path = false;
                        break;
                    }
                }

                if valid_path {
                    // materialize only at the end
                    let vec_path = PathNode::to_vec(&path);
                    dbg!(required_ids, paths.len(), &vec_path);
                    paths.push(vec_path);
                }
                continue;
            }

            for &edge in self.edges[index as usize].iter() {
                if !PathNode::contains(&path, edge) {
                    // NO cloning of the whole path: just one new PathNode
                    let new_path = PathNode::new(edge, Some(Rc::clone(&path)));
                    queue.push((edge, new_path));
                }
            }
        }

        paths
    }

    // Optimized version with early pruning and memoization
    fn count_paths_with_required_nodes(
        &mut self,
        start: &str,
        end: &str,
        required_nodes: &[u16],
    ) -> i64 {
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);

        // Create a bitmask for required nodes for fast checking
        let required_set: HashSet<u16> = required_nodes.iter().copied().collect();

        // Memoization: (current_node, visited_required_mask) -> path_count
        let mut memo: HashMap<(u16, u64), i64> = HashMap::new();

        fn dfs(
            graph: &Graph,
            current: u16,
            end: u16,
            required_set: &HashSet<u16>,
            visited: &mut HashSet<u16>,
            visited_required_mask: u64,
            memo: &mut HashMap<(u16, u64), i64>,
        ) -> i64 {
            if current == end {
                // Check if all required nodes have been visited
                return if visited_required_mask == (1u64 << required_set.len()) - 1 {
                    1
                } else {
                    0
                };
            }

            let memo_key = (current, visited_required_mask);
            if let Some(&cached) = memo.get(&memo_key) {
                return cached;
            }

            let mut total_paths = 0;

            for &next_node in &graph.edges[current as usize] {
                if !visited.contains(&next_node) {
                    visited.insert(next_node);

                    let mut new_mask = visited_required_mask;
                    if required_set.contains(&next_node) {
                        // Find which required node this is
                        let req_index = required_set.iter().position(|&x| x == next_node).unwrap();
                        new_mask |= 1u64 << req_index;
                    }

                    total_paths +=
                        dfs(graph, next_node, end, required_set, visited, new_mask, memo);
                    visited.remove(&next_node);
                }
            }

            memo.insert(memo_key, total_paths);
            total_paths
        }

        let mut visited = HashSet::new();
        visited.insert(start_index);

        let mut initial_mask = 0u64;
        if required_set.contains(&start_index) {
            let req_index = required_set.iter().position(|&x| x == start_index).unwrap();
            initial_mask |= 1u64 << req_index;
        }

        dfs(
            self,
            start_index,
            end_index,
            &required_set,
            &mut visited,
            initial_mask,
            &mut memo,
        )
    }

    fn convert_node_str_to_index(&mut self, node_str: Vec<&str>) -> Vec<u16> {
        let mut node_index: Vec<u16> = Vec::new();
        for node in node_str.iter() {
            node_index.push(*self.index_by_string.get(*node).unwrap());
        }
        node_index
    }
}

fn build_graph_from_list(list: &HashMap<&str, Vec<&str>>) -> Graph {
    let node_size = list.len() + 1;
    let mut graph: Graph = Graph::with_capacity(node_size);

    for (node, edges) in list.iter() {
        for edge in edges.iter() {
            graph.add_edge(node, edge);
        }
    }

    graph
}

fn part1(input: &str) -> Result<usize> {
    const START: &str = "you";
    const END: &str = "out";

    let mut devices: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let device_str: Vec<&str> = line
            .split([' ', ':'])
            // .split(|c| c == ' ' || c == ':')
            .filter(|p| !p.is_empty())
            .collect();
        devices.insert(device_str[0], device_str[1..].to_vec());
    }

    let mut graph: Graph = build_graph_from_list(&devices);

    let total = graph.dfs_paths(START, END).len();
    // dbg!(&devices);

    Ok(total)
}

fn part2(input: &str) -> Result<i64> {
    const START: &str = "svr";
    const END: &str = "out";
    let required_node_str_on_path: Vec<&str> = vec!["dac", "fft"];

    let mut devices: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let device_str: Vec<&str> = line
            .split([' ', ':'])
            // .split(|c| c == ' ' || c == ':')
            .filter(|p| !p.is_empty())
            .collect();
        devices.insert(device_str[0], device_str[1..].to_vec());
    }

    let mut graph: Graph = build_graph_from_list(&devices);
    let required_node_idx_on_path = graph.convert_node_str_to_index(required_node_str_on_path);

    let total = graph.count_paths_with_required_nodes(START, END, &required_node_idx_on_path);
    dbg!(&required_node_idx_on_path);
    println!("Found {} valid paths", total);

    Ok(total)
}

fn main() -> Result<()> {
    let day = extract_day_from_exe();
    let input = read_input(day);

    println!("Day {:02}", day);
    // println!("===>Part 1: {}", part1(&input)?);
    println!("===>Part 2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2025::{read_example, read_example_part2};

    #[test]
    fn test_part1() {
        let input = read_example(extract_day_from_exe());
        assert_eq!(part1(&input).unwrap(), 5);
    }

    #[test]
    fn test_part2() {
        let input = read_example_part2(extract_day_from_exe());
        assert_eq!(part2(&input).unwrap(), 2);
    }
}
