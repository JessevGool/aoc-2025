use std::{fmt::DebugList, num::ParseIntError};

use anyhow::Result;
use aoc_2025::{input_path, lines, read_to_string};

fn main() -> Result<()> {
    let mut input = read_to_string(input_path(3))?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> Result<i64> {
    let mut total_joltage = 0;

    let banks = lines(input);
    for bank in banks {
        let mut converted_bank = Bank {
            batteries: Vec::new(),
        };

        converted_bank.parse_batteries(bank);
        total_joltage += converted_bank.get_largest_joltage_x_batteries(2).unwrap();
    }

    Ok(total_joltage)
}
pub fn part2(input: &str) -> Result<i64> {
     let mut total_joltage = 0;

    let banks = lines(input);
    for bank in banks {
        let mut converted_bank = Bank {
            batteries: Vec::new(),
        };

        converted_bank.parse_batteries(bank);
        total_joltage += converted_bank.get_largest_joltage_x_batteries(12).unwrap();
    }

    Ok(total_joltage)
}

#[derive(Clone)]
pub struct Bank {
    pub batteries: Vec<Battery>,
}

impl Bank {
    pub fn parse_batteries(&mut self, input: &str) {
        for (i, c) in input.chars().enumerate() {
            let battery = Battery {
                idx: i as i32,
                joltage: c.to_digit(10).unwrap() as i32,
            };
            self.batteries.push(battery);
        }
    }

    pub fn get_largest_joltage_x_batteries(&self, x: usize) -> Result<i64> {
        let n = self.batteries.len();
        if x == 0 || n < x {
            return Ok(0);
        }
        let joltages: Vec<i32> = self.batteries.iter().map(|b| b.joltage).collect();
        Ok(self.best_number_x_batteries(joltages, x))
    }

    fn best_number_x_batteries(&self, digits: Vec<i32>, x: usize) -> i64 {
        let n = digits.len();
        let mut stack: Vec<i32> = Vec::with_capacity(x);
        let to_pick = x;
        for (i, &d) in digits.iter().enumerate() {
            let remaining = n - i;

            //Check if new number is higher than last number
            while !stack.is_empty()
                && stack.last().unwrap() < &d
                && stack.len() - 1 + remaining >= to_pick
            {
                stack.pop();
            }

            if stack.len() < to_pick {
                stack.push(d);
            }
        }

        stack.into_iter().fold(0_i64, |acc, d| acc * 10 + d as i64)
    }
}
#[derive(Copy, Clone)]
pub struct Battery {
    idx: i32,
    joltage: i32,
}
