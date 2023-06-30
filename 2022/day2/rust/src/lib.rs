use std::{env, io::BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn get_path() -> Option<String> {
    let mut args = env::args();
    args.next();
    args.next()
}

pub mod part1;
