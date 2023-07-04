use crate::common::*;
use std::io::BufRead;

pub fn solve(file: impl BufRead) -> i32 {
    let mut lines = file.lines();
    let mut acc = 0;
    loop {
        match (lines.next(), lines.next(), lines.next()) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => acc += handle_block(a, b, c),
            _ => return acc,
        }
    }
}

fn handle_block(a: String, b: String, c: String) -> i32 {
    get_char_priority(
        get_common_char([a.chars(), b.chars(), c.chars()].into_iter()).unwrap() as i16,
    ) as i32
}
