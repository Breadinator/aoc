use std::{
    collections::LinkedList,
    fmt,
    io::{self, BufRead, Lines},
    num,
};
use thiserror::Error;

pub type Crane<'a> =
    &'a dyn Fn(&mut Vec<LinkedList<char>>, CraneOperation) -> Result<(), ApplyOperationError>;

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("parse error")]
    ParseError(#[from] InputParserParseError),
    #[error("op error")]
    OperationError(#[from] ApplyOperationError),
}

#[derive(Debug, Error)]
pub enum InputParserParseError {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Unexpected end of file")]
    UnexpectedEof,
    #[error("Unexpected character")]
    UnexpectedCharacter,
    #[error("Parse int error")]
    ParseIntError(#[from] num::ParseIntError),
}

#[derive(Debug, Error)]
pub enum ApplyOperationError {
    #[error("Crate stack out of bounds")]
    StackOutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CraneOperation {
    pub n: usize,
    pub from: usize,
    pub to: usize,
}

pub struct InputParser<'a, T: io::Read> {
    lines: Lines<T>,
    crates: Vec<LinkedList<char>>,
    /// Disgusting dependency injection? What is this, Java?
    crane: Crane<'a>,
}

impl<T: io::Read + fmt::Debug> fmt::Debug for InputParser<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        self.lines.fmt(f)?;
        self.crates.fmt(f)
    }
}

impl<T: io::BufRead> Iterator for &mut InputParser<'_, T> {
    type Item = Result<CraneOperation, InputParserParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|r| {
            r.map_err(InputParserParseError::from)
                .and_then(CraneOperation::try_from)
        })
    }
}

impl<'a, T: io::BufRead + Sized> InputParser<'a, T> {
    pub fn new(reader: T, crane: Crane<'a>) -> Result<Self, InputParserParseError> {
        let mut lines = reader.lines();
        let crates = parse_crates(&mut lines)?;
        Ok(Self {
            lines,
            crates,
            crane,
        })
    }

    pub fn apply_operation(
        &mut self,
        operation: CraneOperation,
    ) -> Result<(), ApplyOperationError> {
        (self.crane)(&mut self.crates, operation)
    }

    pub fn apply_operations(&mut self) -> Result<(), SolutionError> {
        let ops = (&mut self.lines).map(|r| {
            r.map_err(InputParserParseError::from)
                .and_then(CraneOperation::try_from)
        });
        for op in ops {
            (self.crane)(&mut self.crates, op?)?;
        }
        Ok(())
    }

    pub fn read_tops(&self) -> String {
        self.crates
            .iter()
            .map(LinkedList::back)
            .map(|x| x.unwrap_or(&' '))
            .map(Clone::clone)
            .collect()
    }
}

impl TryFrom<String> for CraneOperation {
    type Error = InputParserParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (n, from, to): (usize, usize, usize);
        let mut tokens = value.split(' ');

        tokens.next(); // "move"
        n = tokens
            .next()
            .ok_or(InputParserParseError::UnexpectedCharacter)?
            .parse::<usize>()?;
        tokens.next(); // "from"
        from = tokens
            .next()
            .ok_or(InputParserParseError::UnexpectedCharacter)?
            .parse::<usize>()?
            - 1;
        tokens.next(); // "to"
        to = tokens
            .next()
            .ok_or(InputParserParseError::UnexpectedCharacter)?
            .parse::<usize>()?
            - 1;

        Ok(Self { n, from, to })
    }
}

pub fn parse_args() -> (Option<String>, Option<String>) {
    let mut args = std::env::args();
    args.next();
    (args.next(), args.next())
}

fn parse_crates(
    lines: &mut Lines<impl BufRead>,
) -> Result<Vec<LinkedList<char>>, InputParserParseError> {
    let mut line_buf = Vec::<String>::with_capacity(8);
    loop {
        let line = lines.next().ok_or(InputParserParseError::UnexpectedEof)??;
        if line.len() > 2 && line.chars().nth(1) == Some('1') {
            // this line is the final line of the crates
            let num_stacks = (line.len() + 1) / 4; // easily broken, but w/ consistent input should be fine

            // init all linked lists
            let mut crates = Vec::with_capacity(num_stacks);
            for _ in 0..num_stacks {
                crates.push(LinkedList::new());
            }

            // populate linked lists
            #[allow(clippy::needless_range_loop)] // so I can return early
            for stack_index in 0..num_stacks {
                for row in (0..line_buf.len()).rev() {
                    let char_index = 1 + (stack_index * 4);
                    match line_buf[row].as_bytes()[char_index] {
                        ch if ch.is_ascii_uppercase() => crates[stack_index].push_back(ch as char),
                        b' ' => break,
                        _ => return Err(InputParserParseError::UnexpectedCharacter),
                    }
                }
            }

            // remove empty line
            lines.next();

            return Ok(crates);
        } else {
            line_buf.push(line);
        }
    }
}
