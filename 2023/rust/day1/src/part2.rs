static WRITTEN: &[&[char]] = &[
    &['z', 'e', 'r', 'o'],
    &['o', 'n', 'e'],
    &['t', 'w', 'o'],
    &['t', 'h', 'r', 'e', 'e'],
    &['f', 'o', 'u', 'r'],
    &['f', 'i', 'v', 'e'],
    &['s', 'i', 'x'],
    &['s', 'e', 'v', 'e', 'n'],
    &['e', 'i', 'g', 'h', 't'],
    &['n', 'i', 'n', 'e'],
];

pub fn solution<'a, L>(lines: L) -> u32
where
    L: Iterator<Item = &'a str>,
{
    lines.map(parse_line).sum()
}

#[allow(unused_labels)]
fn parse_line(line: &str) -> u32 {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    let chars: Vec<_> = line.chars().collect();
    let len = chars.len();

    // should still be O(n), but grotesque nonetheless
    'input: for i in 0..len {
        let ch = chars[i];
        if ch >= '0' && ch <= '9' {
            let as_num = ch as u32 - '0' as u32;
            last = Some(as_num);
            if first.is_none() {
                first = Some(as_num);
            }
            continue 'input;
        }

        let remaining = len - i;
        'word: for (word_index, word) in WRITTEN.iter().enumerate() {
            if word.len() <= remaining {
                let mut found = true;
                'comp: for (j, ch) in word.iter().enumerate() {
                    if *ch != chars[i + j] {
                        found = false;
                        break 'comp;
                    }
                }
                if found {
                    last = Some(word_index as u32);
                    if first.is_none() {
                        first = Some(word_index as u32);
                    }
                }
            }
        }
    }

    if let (Some(x), Some(y)) = (first, last) {
        x * 10 + y
    } else {
        0
    }
}
