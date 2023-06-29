use aoc2022day2::*;
use std::{fs::File, io::BufReader};

fn main() {
    let path = get_path().expect("Expected input file path as command-line argument");
    let file = File::open(path).expect("Couldn't open given file");
    let score = part1::calculate(BufReader::new(file));

    println!("Part 1: {score}");
}
