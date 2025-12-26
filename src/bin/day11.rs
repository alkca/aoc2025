use anyhow::Result;
use aoc2025::{extract_day_from_exe, read_input};
use std::rc::Rc;
use std::time::Instant;
use std::{collections::HashMap, thread::panicking};

#[derive(Debug)]
struct PathNode {
    head: usize,
    prev: Option<Rc<PathNode>>,
}

impl PathNode {
    fn new(head: usize, prev: Option<Rc<PathNode>>) -> Rc<PathNode> {
        Rc::new(PathNode { head, prev })
    }

    /// O(length) contains check (same as Vec::contains, but no cloning)
    fn contains(node: &Rc<PathNode>, target: usize) -> bool {
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
    fn to_vec(node: &Rc<PathNode>) -> Vec<usize> {
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
    index_by_string: HashMap<String, usize>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn with_capacity(node_size: usize) -> Self {
        let mut edges: Vec<Vec<usize>> = Vec::with_capacity(node_size);
        edges.resize_with(node_size, Vec::new);

        Self {
            index_by_string: HashMap::new(),
            edges,
        }
    }

    fn add_node(&mut self, node: &str) -> usize {
        if let Some(&index) = self.index_by_string.get(node) {
            return index;
        }

        let index = self.index_by_string.len();
        self.index_by_string.insert(node.to_string(), index);
        index
    }

    fn add_edge(&mut self, node: &str, edge: &str) {
        let from_index = self.add_node(node);
        let to_index = self.add_node(edge);
        self.edges[from_index].push(to_index);
    }

    fn dfs_paths(&mut self, start: &str, end: &str) -> Vec<Vec<usize>> {
        let mut paths: Vec<Vec<usize>> = Vec::new();
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);

        // Queue for DFS of (node, path to node)
        let mut queue: Vec<(usize, Vec<usize>)> = vec![(start_index, vec![start_index])];

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
            for edge in self.edges[index].iter() {
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
        required_ids: &[usize],
    ) -> Vec<Vec<usize>> {
        let mut paths: Vec<Vec<usize>> = Vec::new();
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);

        // Queue for DFS of (node, path to node)
        let mut queue: Vec<(usize, Vec<usize>)> = vec![(start_index, vec![start_index])];

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
            for edge in self.edges[index].iter() {
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
        required_ids: &[usize],
    ) -> Vec<Vec<usize>> {
        let mut paths: Vec<Vec<usize>> = Vec::new();
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);

        // Queue for DFS of (node, path_to_node_as_rc)
        let start_path = PathNode::new(start_index, None);
        let mut queue: Vec<(usize, Rc<PathNode>)> = vec![(start_index, start_path)];

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

            for &edge in self.edges[index].iter() {
                if !PathNode::contains(&path, edge) {
                    // NO cloning of the whole path: just one new PathNode
                    let new_path = PathNode::new(edge, Some(Rc::clone(&path)));
                    queue.push((edge, new_path));
                }
            }
        }

        paths
    }

    fn convert_node_str_to_index(&mut self, node_str: Vec<&str>) -> Vec<usize> {
        let mut node_index: Vec<usize> = Vec::new();
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

    let paths = graph.dfs_paths_with_required_ids2(START, END, &required_node_idx_on_path);
    // dbg!(&paths);
    dbg!(&required_node_idx_on_path);
    dbg!(&graph);
    let mut total = 0;
    for path in paths.iter() {
        let mut valid_path = true;
        for node in required_node_idx_on_path.iter() {
            if !path.contains(node) {
                valid_path = false;
                break;
            }
        }
        if valid_path {
            total += 1;
        }
        dbg!(&valid_path, &path);
    }
    // dbg!(&devices);

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
