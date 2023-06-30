use crate::*;

pub fn calculate_move(enemy_move: Move, friendly_move: Move) -> isize {
    friendly_move as isize
        + if friendly_move == enemy_move {
            // draw
            3
        } else if (friendly_move == Move::Rock && enemy_move == Move::Scissors)
            || (friendly_move == Move::Scissors && enemy_move == Move::Paper)
            || (friendly_move == Move::Paper && enemy_move == Move::Rock)
        {
            // win
            // probably a better way to check this but the hard-coded conditions will have to do
            6
        } else {
            // loss
            0
        }
}

fn parse_line(line: &str) -> Option<(Move, Move)> {
    if line.len() != 3 {
        None
    } else {
        let mut chars = line.chars();
        Some((
            match chars.next()? {
                'A' => Move::Rock,
                'B' => Move::Paper,
                'C' => Move::Scissors,
                _ => return None,
            },
            match chars.nth(1)? {
                'X' => Move::Rock,
                'Y' => Move::Paper,
                'Z' => Move::Scissors,
                _ => return None,
            },
        ))
    }
}

pub fn calculate(reader: impl BufRead) -> isize {
    // the gigaunwrap
    reader
        .lines()
        .map(|l| parse_line(l.as_ref().unwrap()))
        .map(|moves| calculate_move(moves.unwrap().0, moves.unwrap().1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_ay() {
        let line = "A Y";
        let (en, fr) = parse_line(line).unwrap();
        assert_eq!(en, Move::Rock);
        assert_eq!(fr, Move::Paper);
    }

    #[test]
    fn parse_line_bx() {
        let line = "B X";
        let (en, fr) = parse_line(line).unwrap();
        assert_eq!(en, Move::Paper);
        assert_eq!(fr, Move::Rock);
    }

    #[test]
    fn parse_line_cz() {
        let line = "C Z";
        let (en, fr) = parse_line(line).unwrap();
        assert_eq!(en, Move::Scissors);
        assert_eq!(fr, Move::Scissors);
    }
}
