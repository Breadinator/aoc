use aoc22d4::*;
use std::{fs::File, io::BufReader};

fn main() {
    let (part, path) = parse_args();
    let path = path.unwrap_or_else(|| String::from("../input.txt"));
    let part = match part.as_deref() {
        Some("help") | Some("--help") => {
            println!("USAGE:\naoc22d4 [part number] [path]");
            return;
        }
        Some("1") => 1,
        _ => 2,
    };

    let file = File::open(&path).unwrap_or_else(|_| panic!("Couldn't open file at path {}", path));
    let answer = if part == 1 {
        part1::solve(BufReader::new(file))
    } else {
        part2::solve(BufReader::new(file))
    };
    println!("{answer}");
}
