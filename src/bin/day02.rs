use core::num;
use std::string;

use anyhow::Result;
use aoc_2025::{input_path, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(2))?;
    let items: Vec<&str> = input.split(",").collect();
    println!("Part 1: {}", part1(&items)?);
    println!("Part 2: {}", part2(&items)?);
    Ok(())
}

fn part1(input: &Vec<&str>) -> Result<i64> {
    let mut sum = 0;
    for item in input {
        let mut product_id_range = ProductIdRange {
            first_id: 0,
            last_id: 0,
        };

        product_id_range.parse_input(item);
        let invalid_ids = product_id_range.return_invalid_ids();
        for invalid_id in invalid_ids {
            sum += invalid_id;
        }
    }
    Ok(sum)
}

fn part2(input: &Vec<&str>) -> Result<i64> {
    Ok(0)
}

struct ProductIdRange {
    first_id: i64,
    last_id: i64,
}

impl ProductIdRange {
    fn parse_input(&mut self, input: &str) {
        let split_input: Vec<i64> = input
            .trim()
            .split('-')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        self.first_id = split_input[0];
        self.last_id = split_input[1];
    }

    fn return_invalid_ids(&self) -> Vec<i64> {
        let mut invalid_ids: Vec<i64> = Vec::new();
        for id in self.first_id..=self.last_id {
            let string_id = id.to_string();
            let length = string_id.len();


            if length % 2 != 0 {
                continue;
            }
           
           let half = length / 2;
           if &string_id[..half] == &string_id[half..] {
            invalid_ids.push(id);
           }
        }
        invalid_ids
    }
}
