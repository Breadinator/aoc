pub fn solution<'a, L>(lines: L) -> u32
where
    L: Iterator<Item = &'a str>,
{
    lines.map(parse_line).sum()
}

fn parse_line(line: &str) -> u32 {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for ch in line.chars() {
        if ch >= '0' && ch <= '9' {
            let as_num = ch as u32 - '0' as u32;
            last = Some(as_num);
            if first.is_none() {
                first = Some(as_num);
            }
        }
    }

    if let (Some(x), Some(y)) = (first, last) {
        x * 10 + y
    } else {
        0
    }
}
