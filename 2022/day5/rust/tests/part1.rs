use aoc22d5::*;
use std::io::BufReader;

#[test]
fn example_step_by_step() -> Result<(), SolutionError> {
    let input = String::from(
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
    );
    let mut reader = InputParser::new(BufReader::new(input.as_bytes()), &part1::crane)?;
    assert_eq!("NDP", reader.read_tops().as_str());

    let next_op = (&mut reader).next().unwrap()?;
    assert_eq!(
        next_op,
        CraneOperation {
            n: 1,
            from: 1,
            to: 0,
        }
    );
    reader.apply_operation(next_op)?;
    assert_eq!("DCP", reader.read_tops().as_str());

    let next_op = (&mut reader).next().unwrap()?;
    assert_eq!(
        next_op,
        CraneOperation {
            n: 3,
            from: 0,
            to: 2,
        }
    );
    reader.apply_operation(next_op)?;
    assert_eq!(" CZ", reader.read_tops().as_str());

    let next_op = (&mut reader).next().unwrap()?;
    assert_eq!(
        next_op,
        CraneOperation {
            n: 2,
            from: 1,
            to: 0,
        }
    );
    reader.apply_operation(next_op)?;
    assert_eq!("M Z", reader.read_tops().as_str());

    let next_op = (&mut reader).next().unwrap()?;
    assert_eq!(
        next_op,
        CraneOperation {
            n: 1,
            from: 0,
            to: 1,
        }
    );
    reader.apply_operation(next_op)?;
    assert_eq!("CMZ", reader.read_tops().as_str());

    Ok(())
}
