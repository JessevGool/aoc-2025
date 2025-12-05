use std::{cmp::max, collections::HashSet, ops::RangeInclusive};

use anyhow::{Ok, Result};
use aoc_2025::{input_path, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(5))?;
    let mut ingredient_database = IngredientDatabase::new();
    ingredient_database.parse_input(&input);
    println!(
        "Part 1: {}",
        ingredient_database.get_num_fresh_ingredients()
    );
    println!("Part 2: {}", ingredient_database.total_fresh_ingredient_ids());
    Ok(())
}

struct IngredientDatabase {
    fresh_ingredient_ranges: Vec<RangeInclusive<i64>>,
    ingredients: Vec<i64>,
}

impl IngredientDatabase {
    pub fn new() -> Self {
        Self {
            fresh_ingredient_ranges: Vec::new(),
            ingredients: Vec::new(),
        }
    }

    pub fn parse_input(&mut self, input: &str) {
        let mut passed_blank = false;

        for raw_line in input.lines() {
            let line = raw_line.trim();
            if line.is_empty() {
                passed_blank = true;
                continue;
            }

            if !passed_blank {
                let (begin_str, end_str) = line
                    .split_once('-')
                    .expect("expected a range line containing '-'");

                let begin = begin_str.parse::<i64>().unwrap();
                let end = end_str.parse::<i64>().unwrap();

                self.fresh_ingredient_ranges.push(begin..=end);
            } else {
                let id = line.parse::<i64>().unwrap();
                self.ingredients.push(id);
            }
        }
    }

    pub fn get_num_fresh_ingredients(&self) -> i32 {
        if self.ingredients.is_empty() || self.fresh_ingredient_ranges.is_empty() {
            return 0;
        } else {
            let mut fresh_ingredients = 0;

            'outer: for ingredient in &self.ingredients {
                for range in &self.fresh_ingredient_ranges {
                    if range.contains(ingredient) {
                        fresh_ingredients += 1;
                        continue 'outer;
                    }
                }
            }
            fresh_ingredients
        }
    }

    pub fn total_fresh_ingredient_ids(&self) -> i128 {
        let mut ranges: Vec<(i64, i64)> = self
            .fresh_ingredient_ranges
            .iter()
            .map(|r| (*r.start(), *r.end()))
            .collect();

        if ranges.is_empty() {
            return 0;
        }

        ranges.sort_by_key(|(start, _)| *start);

        let mut merged: Vec<(i64, i64)> = Vec::new();

        let mut current_start = ranges[0].0;
        let mut current_end = ranges[0].1;

        for (start, end) in ranges.into_iter().skip(1) {
            if start > current_end + 1 {
                merged.push((current_start, current_end));
                current_start = start;
                current_end = end;
            } else {
                current_end = max(current_end, end);
            }
        }

        merged.push((current_start, current_end));

        let mut total: i128 = 0;
        for (start, end) in merged {
            let start_i = start as i128;
            let end_i = end as i128;
            total += end_i - start_i + 1;
        }

        total
    
    }
}
