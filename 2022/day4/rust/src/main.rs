use aoc22d4::*;
use std::{fs::File, io::BufReader};

fn main() {
    let (path, part) = parse_args();
    let path = path.unwrap_or_else(|| String::from("../input.txt"));
    let part = if let Some("1") = part.as_deref() {
        1
    } else {
        2
    };

    let file = File::open(&path).unwrap_or_else(|_| panic!("Couldn't open file at path {}", path));
    let answer = if part == 1 {
        part1::solve(BufReader::new(file))
    } else {
        part2::solve(BufReader::new(file))
    };
    println!("{answer}");
}
