use std::str::FromStr;

pub mod part1;
pub mod part2;

pub fn solve(input: &str, f: impl FnMut(InputLine) -> usize) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(InputLine::from_str)
        .map(Result::unwrap)
        .map(f)
        .sum()
}

pub static INPUT: &str = include_str!("../../../inputs/day2.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputLine {
    pub id: usize,
    pub sets: Vec<Triplet>,
}

/// Set of red, green, blue representing number of cubes
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Triplet {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl FromStr for InputLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id_sets_split = s.split(':');
        let id = id_sets_split
            .next()
            .ok_or(())?
            .get(5..)
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;
        let sets = id_sets_split
            .next()
            .ok_or(())?
            .split(';')
            .map(|set| set.parse::<Triplet>())
            .collect::<Result<_, _>>()?;

        Ok(Self { id, sets })
    }
}

impl FromStr for Triplet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for mut part in s.split(',').map(|part| part.trim().split(' ')) {
            let num: usize = part.next().ok_or(())?.parse().map_err(|_| ())?;
            match part.next().ok_or(())? {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                _ => return Err(()),
            }
        }

        Ok(Self { red, green, blue })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn parse_line() {
        let line = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"#;
        let parsed: InputLine = line.parse().unwrap();
        assert_eq!(
            parsed,
            InputLine {
                id: 1,
                sets: vec![
                    Triplet {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Triplet {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Triplet {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            }
        );
    }

    #[test]
    fn part1_test_case() {
        let answer = solve(TEST_CASE, part1::line_handler);
        assert_eq!(answer, 8);
    }

    #[test]
    fn part2_test_case() {
        let answer = solve(TEST_CASE, part2::line_handler);
        assert_eq!(answer, 2286);
    }
}
