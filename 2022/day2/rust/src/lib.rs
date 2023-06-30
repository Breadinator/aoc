use std::{env, io::BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryInto<Move> for u8 {
    type Error = ();
    fn try_into(self) -> Result<Move, Self::Error> {
        Ok(match self {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => return Err(()),
        })
    }
}

pub fn get_path() -> Option<String> {
    let mut args = env::args();
    args.next();
    args.next()
}

pub mod part1;
pub mod part2;
