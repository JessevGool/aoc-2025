use anyhow::Result;
use aoc_2025::{input_path, lines, read_to_string};

fn main() -> Result<()> {
    let input = read_to_string(input_path(4))?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let lines = lines(input);
    let mut grid = ToiletRollGrid::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let toilet_roll_pos = ToiletRollPosition {
                x: x,
                y: y,
                contains_roll: if c == '@' { true } else { false },
            };
            grid.rolls.push(toilet_roll_pos);
        }
    }
    Ok(check_forklift_access_part_one(grid))
}

fn part2(input: &str) -> Result<i32> {
    let lines = lines(input);
    let mut grid = ToiletRollGrid::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let toilet_roll_pos = ToiletRollPosition {
                x: x,
                y: y,
                contains_roll: if c == '@' { true } else { false },
            };
            grid.rolls.push(toilet_roll_pos);
        }
    }
    let mut total_removed = 0;

    loop {
        let removable = check_forklift_access_part_two(&grid);
        println!("Removable rolls: {}", removable.len());
        if removable.is_empty() {
            break;
        }

        total_removed += removable.len();

        grid.update_grid(removable);
    }
    Ok(total_removed as i32)
}

//Check 8 positions surrounding the roll
//Make sure we check for edges
fn check_forklift_access_part_one(grid: ToiletRollGrid) -> i32 {
    let mut number_of_rolls = 0;
    for toilet_roll in &grid.rolls {
        if toilet_roll.contains_roll
            && grid.check_surrounding_positions(toilet_roll.x, toilet_roll.y)
        {
            number_of_rolls += 1;
        }
    }
    number_of_rolls
}

fn check_forklift_access_part_two(grid: &ToiletRollGrid) -> Vec<ToiletRollPosition> {
    let mut removable_rolls: Vec<ToiletRollPosition> = Vec::new();

    for toilet_roll in &grid.rolls {
        if toilet_roll.contains_roll
            && grid.check_surrounding_positions(toilet_roll.x, toilet_roll.y)
        {
            removable_rolls.push(toilet_roll.clone());
        }
    }
    removable_rolls
}

struct PointToCheck(i32, i32);
struct ToiletRollGrid {
    pub rolls: Vec<ToiletRollPosition>,
    points_to_check: Vec<PointToCheck>,
}

impl ToiletRollGrid {
    pub fn new() -> Self {
        Self {
            rolls: Vec::new(),
            points_to_check: vec![
                PointToCheck(-1, -1),
                PointToCheck(0, -1),
                PointToCheck(1, -1),
                PointToCheck(-1, 0),
                PointToCheck(1, 0),
                PointToCheck(-1, 1),
                PointToCheck(0, 1),
                PointToCheck(1, 1),
            ],
        }
    }
    pub fn check_surrounding_positions(&self, x: usize, y: usize) -> bool {
        let mut number_of_rolls_surrounding = 0;
        for point_to_check in &self.points_to_check {
            if let Some(p) =
                self.find_roll(point_to_check.0 + x as i32, point_to_check.1 + y as i32)
            {
                if p.contains_roll {
                    number_of_rolls_surrounding += 1
                }
            }
        }
        if number_of_rolls_surrounding < 4 {
            true
        } else {
            false
        }
    }

    fn find_roll(&self, x: i32, y: i32) -> Option<&ToiletRollPosition> {
        self.rolls
            .iter()
            .find(|p| p.x as i32 == x && p.y as i32 == y)
    }

    fn update_grid(&mut self, rolls_to_update: Vec<ToiletRollPosition>) {
        for roll in rolls_to_update {
            if let Some(existing) = self
                .rolls
                .iter_mut()
                .find(|p| p.x == roll.x && p.y == roll.y)
            {
                existing.contains_roll = false;
            }
        }
    }
}

#[derive(Clone)]
struct ToiletRollPosition {
    x: usize,
    y: usize,
    contains_roll: bool,
}
