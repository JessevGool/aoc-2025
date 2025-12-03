use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2025::{input_path, read_to_string};

// Reuse src/bin/day03.rs as a module
#[path = "../src/bin/day03.rs"]
mod day03;

fn bench_part1(c: &mut Criterion) {
    // Load your real AoC input once
    let input = read_to_string(input_path(3)).unwrap();

    c.bench_function("day03_part1", |b| {
        b.iter(|| {
            let res = day03::part1(black_box(&input)).unwrap();
            black_box(res);
        })
    });
}

// Optional: benchmark just get_largest_joltage
fn bench_get_largest(c: &mut Criterion) {
    let line = "987654321111111";

    let mut bank = day03::Bank { batteries: Vec::new() };
    bank.parse_batteries(line);

    c.bench_function("day03_get_largest_joltage", |b| {
        b.iter(|| {
            let res = bank.get_largest_joltage_x_batteries(2).unwrap();
            black_box(res);
        })
    });
}

criterion_group!(benches, bench_part1, bench_get_largest);
criterion_main!(benches);
