use std::fs;
use std::path::Path;

pub fn read_to_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    fs::read_to_string(path)
}

pub fn input_path(day: u8) -> String {
    format!("inputs/day{:02}.txt", day)
}

pub fn input_path_test(day: u8) -> String {
    format!("inputs/day{:02}test.txt", day)
}


pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().filter(|l| !l.is_empty())
}