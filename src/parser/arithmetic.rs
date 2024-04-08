use super::{ArithmeticOperator, Instruction};
use crate::{data::DataType, tokeniser::Token};

pub fn operator(arguments: Vec<Token>, operator: ArithmeticOperator) -> Result<Instruction, ()> {
    if arguments.len() != 3 {
        return Err(());
    }

    let lhs = DataType::token_as_type(&arguments[0]);
    let rhs = DataType::token_as_type(&arguments[1]);
    let assign_to = DataType::token_as_type(&arguments[2]);

    Ok(Instruction::Arithmetic(operator, lhs, rhs, assign_to))
}
