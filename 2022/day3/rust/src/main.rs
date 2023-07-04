use aoc22d3::*;
use std::{fs::File, io::BufReader};

fn main() {
    let (path, part) = parse_args();
    let path = path.expect("Couldn't get input file path");
    let part = match part.as_deref() {
        Some("1") => 1,
        _ => 2,
    };

    let file = File::open(path).expect("Couldn't open given file");
    let answer = if part == 1 {
        part1::solve(BufReader::new(file))
    } else {
        part2::solve(BufReader::new(file))
    };
    println!("{answer}");
}
