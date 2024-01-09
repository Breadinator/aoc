use super::*;

pub fn solve() -> usize {
    super::solve(INPUT, line_handler)
}

pub fn line_handler(line: InputLine) -> usize {
    let mut r: usize = 0;
    let mut g: usize = 0;
    let mut b: usize = 0;

    for set in &line.sets {
        if set.red > r {
            r = set.red;
        }
        if set.green > g {
            g = set.green;
        }
        if set.blue > b {
            b = set.blue;
        }
    }

    r * g * b
}
