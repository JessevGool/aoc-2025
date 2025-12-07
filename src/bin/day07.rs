use std::collections::HashMap;

use anyhow::Result;
use aoc_2025::{input_path, lines, read_to_string};
use std::sync::{ Mutex};

fn main() -> Result<()> {
    let input = read_to_string(input_path(7))?;

    
    let mut christmas_tree = ChristmasTree::new(&input);
    println!("Part 1: {}", part1(&mut christmas_tree)?);
    christmas_tree.reset();
    println!("Part 2: {}", part2(&mut christmas_tree)?);
    Ok(())
}

fn part1(christmas_tree: &mut ChristmasTree) -> Result<i128> {
    let mut start: Option<(usize, usize)> = None;

    'outer: for x in 0..christmas_tree.columns.len() {
        for y in 0..christmas_tree.columns[x].characters.len() {
            if christmas_tree.columns[x].characters[y] == 'S' {
                start = Some((x, y + 1)); // start below S
                break 'outer;
            }
        }
    }

    let answer = if let Some((sx, sy)) = start {
        draw_lines(christmas_tree, sx, sy)
    } else {
        0
    };

    Ok(answer)
}

fn draw_lines(christmas_tree: &mut ChristmasTree, start_x: usize, start_y: usize) -> i128 {
    let mut splits: i128 = 0;
    let height = christmas_tree.columns[start_x].characters.len();

    for y in start_y..height {
        let c = christmas_tree.columns[start_x].characters[y];

        match c {
            '.' => {
                christmas_tree.columns[start_x].characters[y] = '|';
            }
            '^' => {
                splits += 1;

                if start_x > 0 {
                   
                    splits += draw_lines(christmas_tree, start_x - 1, y);
                }
                if start_x + 1 < christmas_tree.columns.len() {
                    splits += draw_lines(christmas_tree, start_x + 1, y);
                }

                break;
            }
            _ => {
                break;
            }
        }
    }

    splits
}

fn part2(christmas_tree: &ChristmasTree) -> Result<i128> {
    let mut start: Option<(usize, usize)> = None;

    'outer: for x in 0..christmas_tree.columns.len() {
        for y in 0..christmas_tree.columns[x].characters.len() {
            if christmas_tree.columns[x].characters[y] == 'S' {
                start = Some((x, y + 1));
                break 'outer;
            }
        }
    }

    let memo: Memo = Mutex::new(HashMap::new());

    let answer = if let Some((sx, sy)) = start {
        count_timelines(christmas_tree, &memo, sx, sy)
    } else {
        0
    };

    Ok(answer)
}


type Memo = Mutex<HashMap<(usize, usize), i128>>;

use rayon::join;

fn count_timelines(
    christmas_tree: &ChristmasTree,
    memo: &Memo,
    start_x: usize,
    start_y: usize,
) -> i128 {
    {
        let memo_guard = memo.lock().unwrap();
        if let Some(&cached) = memo_guard.get(&(start_x, start_y)) {
            return cached;
        }
    }

    let height = christmas_tree.columns[start_x].characters.len();
    let mut result = 1; 

    for y in start_y..height {
        let c = christmas_tree.columns[start_x].characters[y];

        if c == '^' {
            let left = if start_x > 0 {
                Some((start_x - 1, y))
            } else {
                None
            };

            let right = if start_x + 1 < christmas_tree.columns.len() {
                Some((start_x + 1, y))
            } else {
                None
            };

            result = match (left, right) {
                (Some((lx, ly)), Some((rx, ry))) => {
                    let (l, r) = join(
                        || count_timelines(christmas_tree, memo, lx, ly),
                        || count_timelines(christmas_tree, memo, rx, ry),
                    );
                    l + r
                }
                (Some((lx, ly)), None) => count_timelines(christmas_tree, memo, lx, ly),
                (None, Some((rx, ry))) => count_timelines(christmas_tree, memo, rx, ry),
                (None, None) => 0,
            };

            break;
        }
    }
    {
        let mut memo_guard = memo.lock().unwrap();
        memo_guard.insert((start_x, start_y), result);
    }

    result
}


struct ChristmasTree {
    columns: Vec<Column>,
}

impl ChristmasTree {
    fn new(input: &String) -> Self {
        let input_lines = lines(&input);
        let mut columns: Vec<Column> = Vec::new();
        for line in input_lines {
            for (i, char) in line.chars().enumerate() {
                if columns.len() <= i {
                    columns.push(Column::new());
                };

                columns[i].characters.push(char);
            }
        }
        ChristmasTree { columns }
    }

    fn reset(&mut self) {
        for column in self.columns.iter_mut() {
            for ch in column.characters.iter_mut() {
                if *ch == '|' {
                    *ch = '.';
                }
            }
        }
    }
}

#[derive(Clone)]
struct Column {
    characters: Vec<char>,
}

impl Column {
    fn new() -> Self {
        Column {
            characters: Vec::new(),
        }
    }
}
