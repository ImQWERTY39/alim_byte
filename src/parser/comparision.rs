use super::{ComparisionType, Instruction};
use crate::tokeniser::Token;

pub fn compare(arguments: Vec<Token>, comparision: ComparisionType) -> Result<Instruction, ()> {
    if arguments.len() != 3 {
        return Err(());
    }

    let mut iter = arguments.into_iter();
    let lhs = iter.next().unwrap().into();
    let rhs = iter.next().unwrap().into();
    let assign_to = iter.next().unwrap().into();

    Ok(Instruction::Compare(comparision, lhs, rhs, assign_to))
}
