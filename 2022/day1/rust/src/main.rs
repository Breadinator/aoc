use aoc2022day1::*;
use std::{env, fs::File, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Expected input file as a command-line argument.");
        return;
    }

    let file = File::open(args[1].clone()).expect("Couldn't open file.");
    let top3 = Top3::get(BufReader::new(file)).expect("Couldn't parse file.");
    top3.print();
}
