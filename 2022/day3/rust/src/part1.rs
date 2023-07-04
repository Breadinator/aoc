use crate::common;
use std::{convert::AsRef, io::BufRead};

struct Bag {
    pub comp1: Vec<char>,
    pub comp2: Vec<char>,
}

impl Bag {
    pub fn parse(s: impl AsRef<str>) -> Self {
        let (comp1, comp2) = s.as_ref().split_at(s.as_ref().len() / 2);
        let comp1 = comp1.chars().collect();
        let comp2 = comp2.chars().collect();
        Self { comp1, comp2 }
    }

    pub fn calculate(self) -> i32 {
        common::get_char_priority(
            common::get_common_char([self.comp1.into_iter(), self.comp2.into_iter()].into_iter())
                .unwrap() as i16,
        ) as i32
    }
}

pub fn solve(reader: impl BufRead) -> i32 {
    reader
        .lines()
        .map(Result::unwrap)
        .map(Bag::parse)
        .map(Bag::calculate)
        .sum()
}
