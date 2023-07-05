use crate::*;
use std::io::BufRead;

pub fn solve(reader: impl BufRead) -> usize {
    reader
        .lines()
        .map(Result::unwrap)
        .map(ParsedPair::from)
        .map(|p| p.has_subset())
        .fold(0, |acc, item| acc + if item { 1 } else { 0 })
}
