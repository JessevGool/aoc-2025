use std::fmt::DebugList;

use anyhow::Result;
use aoc_2025::{input_path, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(1))?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

/**
 * Safe Dial
 * Start position is 50
 * R8 means 8 clicks right making it 58
 * L8 means 8 clicks left making it 50 again
 * 99 is max
 * 0 is min
 * 99 + 1 == 0
 * 0 - 1 == 9
 *
 * Password is how many times the dial is left at 0 after any rotation
 */

fn part1(input: &str) -> Result<i32> {
    let mut position = 50;
    let mut password = 0;

    let sequence = lines(input);

    for step in sequence {
        let (dir, rest) = step.split_at(1);
        let steps: i32 = rest.parse()?;

        let delta = match dir {
            "R" => steps,
            "L" => -steps,
            _ => panic!(),
        };

        position = (position + delta).rem_euclid(100);

        if position == 0 {
            password += 1;
        }
    }

    Ok(password)
}

/**
 * We should now count how many times the position passes 0, during or at the end of a rotation
 */
fn part2(input: &str) -> Result<i32> {
    let mut position = 50;
    let mut password = 0;

    let sequence = lines(input);

    for step in sequence {
        let (dir, rest) = step.split_at(1);
        let steps: i32 = rest.parse()?;

        if dir == "R" {
            password += (position + steps) / 100;
            position = (position + steps) % 100;
        } else {
            let reverse = (100 - position) % 100;
            password += (reverse + steps) / 100;
            position = (position - steps).rem_euclid(100);
        }
    }

    Ok(password)
}
