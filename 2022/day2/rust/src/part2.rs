use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseError {
    InvalidLength,
    InvalidCharacter { position: usize },
}

fn parse_line(line: &str) -> Result<(Move, TurnResult), ParseError> {
    if line.len() != 3 {
        Err(ParseError::InvalidLength)
    } else {
        let mut chars = line.chars();
        Ok((
            match chars.next().unwrap() {
                'A' => Move::Rock,
                'B' => Move::Paper,
                'C' => Move::Scissors,
                _ => return Err(ParseError::InvalidCharacter { position: 0 }),
            },
            match chars.nth(1).unwrap() {
                'X' => TurnResult::Loss,
                'Y' => TurnResult::Draw,
                'Z' => TurnResult::Win,
                _ => return Err(ParseError::InvalidCharacter { position: 2 }),
            },
        ))
    }
}

fn what_move_do(op_move: Move, desired_result: TurnResult) -> Move {
    if desired_result == TurnResult::Draw {
        return op_move;
    }

    let off = if desired_result == TurnResult::Loss {
        1
    } else {
        0
    };

    (((op_move as u8) + off) % 3).try_into().unwrap()
}

fn calculate_move(op_move: Move, desired_result: TurnResult) -> isize {
    part1::calculate_move(op_move, what_move_do(op_move, desired_result))
}

pub fn calculate(reader: impl BufRead) -> isize {
    reader
        .lines()
        .map(|l| parse_line(l.as_ref().unwrap()))
        .map(|turn| calculate_move(turn.unwrap().0, turn.unwrap().1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_ay() {
        let line = "A Y";
        let (en, ex) = parse_line(line).unwrap();
        assert_eq!(en, Move::Rock);
        assert_eq!(ex, TurnResult::Draw);
    }

    #[test]
    fn parse_line_bx() {
        let line = "B X";
        let (en, ex) = parse_line(line).unwrap();
        assert_eq!(en, Move::Paper);
        assert_eq!(ex, TurnResult::Loss);
    }

    #[test]
    fn parse_line_cz() {
        let line = "C Z";
        let (en, ex) = parse_line(line).unwrap();
        assert_eq!(en, Move::Scissors);
        assert_eq!(ex, TurnResult::Win);
    }
}
