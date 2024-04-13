use super::Instruction;
use crate::tokeniser::Token;

pub fn goto(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 1 {
        return Err(());
    }

    let mut iter = arguments.into_iter();
    let block_name = if let Token::Identifier(i) = iter.next().unwrap() {
        i
    } else {
        return Err(());
    };

    Ok(Instruction::Goto(block_name))
}

pub fn goto_if(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 2 {
        return Err(());
    }

    let mut iter = arguments.into_iter();
    let block_name = if let Token::Identifier(i) = iter.next().unwrap() {
        i
    } else {
        return Err(());
    };
    let condition = iter.next().unwrap().into();

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

    let mut iter = arguments.into_iter();
    let condition = iter.next().unwrap().into();

    Ok(Instruction::GoBackIf(condition))
}

pub fn skip(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 1 {
        return Err(());
    }

    let skip_instructions = if let Token::Integer(i) = arguments.into_iter().next().unwrap() {
        if i.is_negative() {
            return Err(());
        }

        i as usize
    } else {
        return Err(());
    };

    Ok(Instruction::Skip(skip_instructions))
}
