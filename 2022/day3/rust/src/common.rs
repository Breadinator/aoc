use std::{collections::HashSet, env};

pub fn parse_args() -> (Option<String>, Option<String>) {
    let mut args = env::args();
    args.next(); // executable path
    (args.next(), args.next())
}

#[allow(non_upper_case_globals)]
pub fn get_char_priority(ch: i16) -> i16 {
    const A: i16 = 'A' as i16;
    const Z: i16 = 'Z' as i16;
    const a: i16 = 'a' as i16;
    ch + if ch > Z { 1i16 - a } else { 27i16 - A }
}

/// Takes an iterator of iterators of `char`s and returns the common element between them.
pub fn get_common_char<T, U>(mut strings: T) -> Option<char>
where
    T: Iterator<Item = U>,
    U: Iterator<Item = char>,
{
    let first = strings.next()?;
    let mut prev = HashSet::new();
    for ch in first {
        prev.insert(ch);
    }
    let mut curr = HashSet::new();

    for s in strings {
        curr.clear();
        for ch in s {
            if prev.contains(&ch) {
                curr.insert(ch);
            }
        }
        if curr.len() == 1 {
            return curr.into_iter().next(); // maybe a better way to do this?
        }
        prev = curr.clone();
    }

    None
}
