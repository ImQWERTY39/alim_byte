use super::Instruction;
use crate::{data::DataType, tokeniser::Token};

pub fn goto(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 1 {
        return Err(());
    }

    let block_name = if let Token::Identifier(i) = &arguments[0] {
        i.clone()
    } else {
        return Err(());
    };

    Ok(Instruction::Goto(block_name))
}

pub fn goto_if(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 2 {
        return Err(());
    }

    let block_name = if let Token::Identifier(i) = &arguments[0] {
        i.clone()
    } else {
        return Err(());
    };

    let condition = DataType::token_as_type(&arguments[1]);

    Ok(Instruction::GotoIf(block_name, condition))
}

pub fn go_back(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if !arguments.is_empty() {
        return Err(());
    }

    Ok(Instruction::GoBack)
}

pub fn go_back_if(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 1 {
        return Err(());
    }

    let condition = DataType::token_as_type(&arguments[0]);

    Ok(Instruction::GoBackIf(condition))
}

pub fn skip(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 1 {
        return Err(());
    }

    let skip_instructions = if let Token::Integer(i) = &arguments[0] {
        if i.is_negative() {
            return Err(());
        }

        *i as usize
    } else {
        return Err(());
    };

    Ok(Instruction::Skip(skip_instructions))
}
