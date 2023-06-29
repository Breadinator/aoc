use std::{io::BufRead, num::ParseIntError, ops::Index, str::FromStr};

type FmtError = std::fmt::Error;

#[derive(Debug, Clone, Default)]
pub struct Top3 {
    items: [isize; 3],
}

impl Index<usize> for Top3 {
    type Output = isize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl Top3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(reader: impl BufRead) -> Result<Self, ParseIntError> {
        let lines = reader.lines();
        let mut top3 = Self::new();

        let mut cur = 0;

        for line in lines {
            let l = line.as_deref();
            match l {
                Ok("") => {
                    top3.insert(cur);
                    cur = 0;
                }
                Ok(l) => cur += isize::from_str(l)?,
                Err(err) => println!("{err}"),
            }
        }

        // leftover
        if cur != 0 {
            top3.insert(cur);
        }

        Ok(top3)
    }

    pub fn insert(&mut self, item: isize) {
        if item > self.items[0] {
            self.items.rotate_left(1);
            self.items[0] = item;
        } else if item > self.items[1] {
            self.items[2] = self.items[1];
            self.items[1] = item;
        } else if item > self.items[2] {
            self.items[2] = item;
        }
    }

    pub fn sum(&self) -> isize {
        self[0] + self[1] + self[2]
    }

    pub fn print(&self) {
        println!(
            "Top 3:\n 1: {}\n 2: {}\n 3: {}\n\nSum of top 3: {}",
            self[0],
            self[1],
            self[2],
            self.sum()
        );
    }
}

/// Just puting it in a module here because I can't be bothered to make a test dir for this lol
#[cfg(test)]
mod tests {
    #[test]
    fn top3_insert() {
        let mut top3 = super::Top3::new();
        top3.insert(10);
        top3.insert(3);
        top3.insert(5);
        top3.insert(7);
        top3.insert(-5);

        assert_eq!(top3[0], 10);
        assert_eq!(top3[1], 7);
        assert_eq!(top3[2], 5);
    }
}
