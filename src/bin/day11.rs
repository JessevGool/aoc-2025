use std::{
    collections::{HashMap, HashSet},
    fmt::DebugList,
};

use anyhow::Result;
use aoc_2025::{input_path, input_path_test, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(11))?;
    let devices = create_devices(&input);
    println!("Part 1: {}", part1(&devices)?);
    println!("Part 2: {}", part2(&devices)?);
    Ok(())
}

fn create_devices(input: &str) -> HashMap<String, Vec<String>> {
    let mut devices = HashMap::new();

    for raw in lines(input) {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once(':').unwrap();
        let name = left.trim().to_string();

        let outputs = right
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        devices.insert(name, outputs);
    }

    devices
}

fn part1(input: &HashMap<String, Vec<String>>) -> Result<u64> {
    let start = "you";
    let end = "out";

    let mut visited = HashSet::new();
    let paths = dfs_count(start, end, input, &mut visited);

    Ok(paths)
}

fn part2(graph: &HashMap<String, Vec<String>>) -> Result<u64> {
    let start = "svr";
    let end = "out";

    // required nodes in a fixed order (bit 0 = dac, bit 1 = fft)
    let required = ["dac", "fft"];

    let mut memo: HashMap<(String, u8), u64> = HashMap::new();
    let paths = count_paths_required_memo(start, end, graph, 0, &required, &mut memo);

    Ok(paths)
}

fn dfs_count(
    current: &str,
    end: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> u64 {
    if current == end {
        return 1;
    }

    if visited.contains(current) {
        return 0;
    }

    visited.insert(current.to_string());

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            total_paths += dfs_count(next, end, graph, visited)
        }
    }

    visited.remove(current);

    total_paths
}

fn count_paths_required_memo(
    current: &str,
    end: &str,
    graph: &HashMap<String, Vec<String>>,
    mut mask: u8,
    required: &[&str; 2],
    memo: &mut HashMap<(String, u8), u64>,
) -> u64 {
    for (i, &req) in required.iter().enumerate() {
        if current == req {
            mask |= 1 << i;
        }
    }

    if current == end {
        return if mask == 0b11 { 1 } else { 0 };
    }
    if let Some(&cached) = memo.get(&(current.to_string(), mask)) {
        return cached;
    }

    let mut total = 0u64;
    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            total += count_paths_required_memo(next, end, graph, mask, required, memo);
        }
    }

    memo.insert((current.to_string(), mask), total);
    total
}

