use super::Instruction;
use crate::tokeniser::Token;

pub fn create_variable(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 2 {
        return Err(());
    }

    let mut iter = arguments.into_iter();
    let type_ = iter.next().unwrap();
    let name = iter.next().unwrap();

    let variable_name = if let Token::Identifier(i) = name {
        i
    } else {
        return Err(());
    };

    let variable_type = match type_ {
        Token::Identifier(i) => i.into(),
        _ => return Err(()),
    };

    Ok(Instruction::CreateVariable(variable_name, variable_type))
}

pub fn set_variable(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 2 {
        return Err(());
    }

    let mut iter = arguments.into_iter();
    let name = iter.next().unwrap();
    let value = iter.next().unwrap();

    let variable_name = match name {
        Token::Identifier(i) => i,
        _ => return Err(()),
    };

    Ok(Instruction::SetVariable(variable_name, value.into()))
}
