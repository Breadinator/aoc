use crate::{ApplyOperationError, CraneOperation};
use std::collections::LinkedList;

pub fn crane(
    crates: &mut Vec<LinkedList<char>>,
    operation: CraneOperation,
) -> Result<(), ApplyOperationError> {
    if crates.len() <= operation.from || crates.len() <= operation.to {
        return Err(ApplyOperationError::StackOutOfBounds);
    }
    let at = crates[operation.from].len() - operation.n;
    let split = crates[operation.from].split_off(at);
    crates[operation.to].extend(split.into_iter().rev());
    Ok(())
}
