use std::collections::HashSet;

use anyhow::Result;
use aoc_2025::{input_path, input_path_test, lines, read_to_string};
use itertools::Itertools;

fn main() -> Result<()> {
    // swap to `input_path(9)` when you want to run on real input
    let input = read_to_string(input_path(9))?;
    let grid = Grid::new(&input);

    println!("Part 1: {}", part1(&grid)?);
    println!("Part 2: {}", part2(grid)?);

    Ok(())
}

fn part1(input: &Grid) -> Result<i128> {
    let mut largest_area: i128 = 0;

    // all pairs of red squares
    for combo in input.red_squares.iter().combinations(2) {
        let a = combo[0];
        let b = combo[1];

        let area = a.calculate_area(b);
        if area > largest_area {
            largest_area = area;
        }
    }

    Ok(largest_area)
}

fn part2(input: Grid) -> Result<i128> {
    let points = &input.red_squares;
    let poly = &input.polygon_path;

    // build all rectangles from pairs of red points
    let mut rectangles: Vec<(Coordinate, Coordinate, i128)> = points
        .iter()
        .combinations(2)
        .map(|combo| {
            let a = *combo[0];
            let b = *combo[1];
            let area = a.calculate_area(&b);
            (a, b, area)
        })
        .collect();

    // largest area first
    rectangles.sort_by(|x, y| y.2.cmp(&x.2));

    // return the first rectangle (largest area) that is valid
    for (a, b, area) in rectangles {
        if rectangle_valid(&a, &b, poly) {
            return Ok(area);
        }
    }

    Ok(0)
}

/// Check that the open interior of the rectangle between a and b
/// does NOT contain any point from the polygon path.
fn rectangle_valid(a: &Coordinate, b: &Coordinate, poly: &HashSet<Coordinate>) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    for point in poly {
        let x = point.x;
        let y = point.y;

        // strictly inside (not on the border)
        if x > min_x && x < max_x && y > min_y && y < max_y {
            return false;
        }
    }

    true
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: i128,
    y: i128,
}

impl Coordinate {
    fn calculate_area(&self, other: &Coordinate) -> i128 {
        ((other.x - self.x).abs() + 1) * ((other.y - self.y).abs() + 1)
    }
}

struct Grid {
    red_squares: Vec<Coordinate>,
    polygon_path: HashSet<Coordinate>,
}

impl Grid {
    fn new(input: &str) -> Self {
        // read all red squares
        let mut red_squares: Vec<Coordinate> = Vec::new();
        for line in lines(input) {
            let coords: Vec<i128> = line
                .split(',')
                .map(|x| x.parse::<i128>().unwrap())
                .collect();
            red_squares.push(Coordinate {
                x: coords[0],
                y: coords[1],
            });
        }

        // build polygon_path: all tiles on straight segments between consecutive reds
        let mut polygon_path: HashSet<Coordinate> = HashSet::new();
        let n = red_squares.len();

        for i in 0..n {
            let start = red_squares[i];
            let end = red_squares[(i + 1) % n];

            if start.x == end.x {
                // vertical segment
                for y in create_range_boxed(start.y, end.y) {
                    polygon_path.insert(Coordinate { x: start.x, y });
                }
            } else if start.y == end.y {
                // horizontal segment
                for x in create_range_boxed(start.x, end.x) {
                    polygon_path.insert(Coordinate { x, y: start.y });
                }
            } else {
                panic!("Input assumption violated: consecutive reds not on same row or column");
            }
        }

        Grid {
            red_squares,
            polygon_path,
        }
    }
}

/// Inclusive range from a to b, forwards or backwards.
fn create_range_boxed(a: i128, b: i128) -> Box<dyn Iterator<Item = i128>> {
    if a <= b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}
