use std::{fs::File, io::BufReader};

use aoc22d5::{part1, part2, SolutionError};

fn main() -> Result<(), SolutionError> {
    let (part, path) = aoc22d5::parse_args();
    let part = if let Some("1") = part.as_deref() {
        1
    } else {
        2
    };
    let path = path.unwrap_or_else(|| String::from("../input.txt"));

    let file = File::open(&path)
        .unwrap_or_else(|e| panic!("Couldn't open the file at path {path}\n\nERR:\n{e}"));
    let mut reader = aoc22d5::InputParser::new(
        BufReader::new(file),
        if part == 1 {
            &part1::crane
        } else {
            &part2::crane
        },
    )?;

    reader.apply_operations()?;
    let answer = reader.read_tops();
    println!("{answer}");

    Ok(())
}
