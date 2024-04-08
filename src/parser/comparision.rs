use crate::{data::DataType, tokeniser::Token};

use super::{ComparisionType, Instruction};

pub fn compare(arguments: Vec<Token>, comparision: ComparisionType) -> Result<Instruction, ()> {
    if arguments.len() != 3 {
        return Err(());
    }

    let lhs = DataType::token_as_type(&arguments[0]);
    let rhs = DataType::token_as_type(&arguments[1]);
    let assign_to = DataType::token_as_type(&arguments[2]);

    Ok(Instruction::Compare(comparision, lhs, rhs, assign_to))
}
