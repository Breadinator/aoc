use std::{env, str::FromStr};

pub fn parse_args() -> (Option<String>, Option<String>) {
    let mut args = env::args();
    args.next();
    (args.next(), args.next())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedPair {
    pub a: ParsedSingle,
    pub b: ParsedSingle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedSingle {
    pub min: usize,
    pub max: usize,
}

impl ParsedPair {
    pub fn has_subset(&self) -> bool {
        (self.a.min >= self.b.min && self.a.max <= self.b.max)
            || (self.a.min <= self.b.min && self.a.max >= self.b.max)
    }

    pub fn has_any_overlap(&self) -> bool {
        (self.a.min <= self.b.min && self.a.max >= self.b.min)
            || (self.b.min <= self.a.min && self.b.max >= self.a.min)
            || (self.a.max >= self.b.max && self.a.min <= self.b.max)
            || (self.b.max >= self.a.max && self.b.min <= self.a.max)
    }
}

// using `From` because for some reason `TryFrom` was conflicting and i'm only going to unwrap it anyways
impl<T: AsRef<str>> From<T> for ParsedPair {
    fn from(value: T) -> Self {
        let mut s = value.as_ref().split(',');
        Self {
            a: s.next().unwrap().into(),
            b: s.next().unwrap().into(),
        }
    }
}

impl<T: AsRef<str>> From<T> for ParsedSingle {
    fn from(value: T) -> Self {
        let mut s = value.as_ref().split('-');
        Self {
            min: usize::from_str(s.next().unwrap()).unwrap(),
            max: usize::from_str(s.next().unwrap()).unwrap(),
        }
    }
}
