use super::Instruction;
use crate::{data::DataType, tokeniser::Token};

pub fn create_variable(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 2 {
        return Err(());
    }

    let datatype = if let Token::Identifier(i) = &arguments[0] {
        DataType::string_as_type(i)
    } else {
        return Err(());
    };

    let variable_name = if let Token::Identifier(i) = &arguments[1] {
        i.clone()
    } else {
        return Err(());
    };

    Ok(Instruction::CreateVariable(variable_name, datatype))
}

pub fn set_variable(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 2 {
        return Err(());
    }

    let variable_name = if let Token::Identifier(i) = &arguments[0] {
        i.clone()
    } else {
        return Err(());
    };

    let value = DataType::token_as_type(&arguments[1]);

    Ok(Instruction::SetVariable(variable_name, value))
}
