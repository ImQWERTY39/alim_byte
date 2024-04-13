use super::{ArithmeticOperator, Instruction};
use crate::tokeniser::Token;

pub fn operator(arguments: Vec<Token>, operator: ArithmeticOperator) -> Result<Instruction, ()> {
    if arguments.len() != 3 {
        return Err(());
    }

    let mut iter = arguments.into_iter();
    let lhs = iter.next().unwrap().into();
    let rhs = iter.next().unwrap().into();
    let assign_to = iter.next().unwrap().into();

    Ok(Instruction::Arithmetic(operator, lhs, rhs, assign_to))
}
