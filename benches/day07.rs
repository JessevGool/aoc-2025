use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2025::{input_path, read_to_string};

use crate::day07::ChristmasTree;

// Reuse src/bin/day03.rs as a module
#[path = "../src/bin/day07.rs"]
mod day07;

fn bench_part2(c: &mut Criterion) {
    // Load your real AoC input once
    let input = read_to_string(input_path(7)).unwrap();
 let mut christmas_tree = ChristmasTree::new(&input);
    c.bench_function("day07_part2", |b| {
        b.iter(|| {
            let res = day07::part2(black_box(&christmas_tree)).unwrap();
            black_box(res);
        })
    });
}


criterion_group!(benches, bench_part2);
criterion_main!(benches);
