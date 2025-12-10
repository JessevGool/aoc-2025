use std::{collections::HashMap, fmt::DebugList, hash::Hash};

use anyhow::Result;
use aoc_2025::{input_path, input_path_test, lines, read_to_string};
use minilp::{ComparisonOp, OptimizationDirection, Problem};
use regex::Regex;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = read_to_string(input_path(10))?;
    let mut machines: Vec<Machine> = Vec::new();
    for line in lines(&input) {
        machines.push(Machine::new(line));
    }
    println!("Part 1: {}", part1(&machines)?);
    println!("Part 2: {}", part2(&machines)?);
    Ok(())
}

fn part1(machines: &Vec<Machine>) -> Result<u32> {
    Ok(machines
        .iter()
        .map(|m| m.min_presses().expect("target state must be reachable"))
        .sum())
}

fn part2(machines: &Vec<Machine>) -> Result<u32> {
    Ok(machines.iter().map(|m| m.min_joltage_presses()).sum())
}

struct Machine {
    indicator_lights: Vec<IndicatorLight>,
    desired_indicator_lights: Vec<IndicatorLight>,
    buttons: Vec<Button>,
    joltage_requirements: Vec<u32>,
    desired_joltage_requirements: Vec<u32>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let re_brackets = Regex::new(r"\[(.*?)\]").unwrap();
        let re_parens = Regex::new(r"\((.*?)\)").unwrap();
        let re_braces = Regex::new(r"\{(.*?)\}").unwrap();

        let bracket = re_brackets.captures(input).unwrap()[1].to_string();

        let indicator_lights: Vec<IndicatorLight> =
            vec![IndicatorLight { enabled: false }; bracket.chars().count()];
        let mut desired_indicator_lights = indicator_lights.clone();
        for (i, c) in bracket.chars().enumerate() {
            match c {
                '.' => desired_indicator_lights[i].enabled = false,
                '#' => desired_indicator_lights[i].enabled = true,
                _ => println!("hmm"),
            }
        }

        let parens: Vec<String> = re_parens
            .captures_iter(input)
            .map(|c| c[1].to_string())
            .collect();

        let buttons: Vec<Button> = parens
            .into_iter()
            .map(|s| Button {
                light_idxs: s
                    .split(',')
                    .map(|n| n.trim().parse::<u32>().unwrap())
                    .collect(),
            })
            .collect();

        let braces = re_braces.captures(input).unwrap()[1].to_string();
        let joltage_requirements: Vec<u32> = vec![0; braces.split(',').count()];
        let desired_joltage_requirements = braces
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        Machine {
            indicator_lights,
            desired_indicator_lights,
            buttons: buttons,
            joltage_requirements: joltage_requirements,
            desired_joltage_requirements: desired_joltage_requirements,
        }
    }

    fn target_mask(&self) -> u64 {
        let mut mask = 0u64;
        for (i, light) in self.desired_indicator_lights.iter().enumerate() {
            if light.enabled {
                mask |= 1u64 << i;
            }
        }
        mask
    }

    fn button_masks(&self) -> Vec<u64> {
        let mut res = Vec::with_capacity(self.buttons.len());
        for button in &self.buttons {
            let mut mask = 0u64;
            for &idx in &button.light_idxs {
                mask |= 1u64 << (idx as u64);
            }
            res.push(mask);
        }
        res
    }

    fn min_presses(&self) -> Option<u32> {
        let n = self.desired_indicator_lights.len();

        if n > 64 {
            panic!("Too many indicator lights ({n}), bitmask won't fit in u64");
        }

        let target = self.target_mask();
        if target == 0 {
            return Some(0);
        }

        let button_masks = self.button_masks();

        let state_count = 1usize << n;
        let mut dist = vec![u32::MAX; state_count];

        let mut queue = VecDeque::new();
        dist[0] = 0;
        queue.push_back(0u64);

        while let Some(state) = queue.pop_front() {
            let d = dist[state as usize];

            for &bmask in &button_masks {
                let next = state ^ bmask;
                let idx = next as usize;
                if dist[idx] == u32::MAX {
                    dist[idx] = d + 1;
                    if next == target {
                        return Some(d + 1);
                    }
                    queue.push_back(next);
                }
            }
        }

        None
    }

    fn min_joltage_presses(&self) -> u32 {
        let buttons: Vec<Vec<usize>> = self
            .buttons
            .iter()
            .map(|b| b.light_idxs.iter().map(|&i| i as usize).collect())
            .collect();

        let delta: Vec<i64> = self
            .desired_joltage_requirements
            .iter()
            .zip(self.joltage_requirements.iter())
            .map(|(d, s)| *d as i64 - *s as i64)
            .collect();

        if delta.iter().any(|&x| x < 0) {
            panic!("Target joltage state is unreachable (target < start for some light)");
        }

        let jolts: Vec<f64> = delta.iter().map(|&x| x as f64).collect();

        let mut problem = Problem::new(OptimizationDirection::Minimize);

        let press_vars: Vec<_> = (0..buttons.len())
            .map(|_| problem.add_var(1.0, (0.0, f64::INFINITY)))
            .collect();

        for (light_idx, &required) in jolts.iter().enumerate() {
            if required == 0.0 {
                continue;
            }

            let mut terms = Vec::new();
            for (button_idx, button) in buttons.iter().enumerate() {
                if button.contains(&light_idx) {
                    terms.push((press_vars[button_idx], 1.0));
                }
            }

            problem.add_constraint(&terms, ComparisonOp::Eq, required);
        }

        let solution = problem
            .solve()
            .expect("LP solver could not find a feasible solution");

        press_vars.iter().map(|&v| solution[v].round() as u32).sum()
    }
}

#[derive(Clone)]
struct IndicatorLight {
    enabled: bool,
}

#[derive(Clone)]
struct Button {
    light_idxs: Vec<u32>,
}
