use std::fmt::DebugList;

use anyhow::Result;
use aoc_2025::{input_path, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(12))?;

    println!("Part 1: {}", part1(&input)?);
  
    Ok(())
}


fn part1(input: &str) -> Result<i32> {
  let result = input
    .as_bytes()
    .split(|&b| b == b'\n')
    .skip(6 * 5)
    .filter(|region| region_is_valid(region))
    .count();

    Ok(result as i32)
}


fn region_is_valid(region: &[u8]) -> bool {
    let (size_part, counts_part) = split_size_and_counts(region);

    let area = parse_dimensions(size_part);
    let required = parse_required(counts_part);

    area >= required
}

fn split_size_and_counts(region: &[u8]) -> (&[u8], &[u8]) {
    let colon_pos = region.iter().position(|&b| b == b':').unwrap();
    region.split_at(colon_pos)
}

fn parse_dimensions(size: &[u8]) -> usize {
    size.split(|&b| b == b'x')
        .map(|n| atoi::atoi::<usize>(n).unwrap())
        .product()
}

fn parse_required(counts: &[u8]) -> usize {
    counts[3..] // skip ": "
        .split(|&b| b == b' ')
        .map(|n| atoi::atoi::<usize>(n).unwrap() * 9)
        .sum()
}

