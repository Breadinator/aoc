use super::*;

pub fn solve() -> usize {
    super::solve(INPUT, line_handler)
}

pub fn line_handler(line: InputLine) -> usize {
    for set in &line.sets {
        if set.red > 12 || set.green > 13 || set.blue > 14 {
            return 0;
        }
    }
    line.id
}
