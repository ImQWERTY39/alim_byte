use super::Instruction;
use crate::{data::Type, tokeniser::Token};

pub fn init_function(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() % 2 == 0 {
        return Err(());
    }

    let length = arguments.len();
    let mut iter = arguments.into_iter();
    let name = if let Token::Identifier(i) = iter.next().unwrap() {
        i
    } else {
        return Err(());
    };

    if length == 1 {
        Ok(Instruction::Function(name, Vec::new()))
    } else {
        Ok(Instruction::Function(
            name,
            parse_parametres(iter.collect())?,
        ))
    }
}

fn parse_parametres(arguments: Vec<Token>) -> Result<Vec<(String, Type)>, ()> {
    let mut parametres = Vec::with_capacity(arguments.len() / 2);
    let mut param_list = arguments.into_iter();

    while let Some(Token::Identifier(type_token)) = param_list.next() {
        let name = if let Token::Identifier(i) = param_list.next().ok_or(())? {
            i
        } else {
            return Err(());
        };

        parametres.push((name, type_token.into()));
    }

    Ok(parametres)
}

pub fn init_block(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 1 {
        return Err(());
    }

    let name = if let Token::Identifier(i) = arguments.into_iter().next().unwrap() {
        i
    } else {
        return Err(());
    };

    Ok(Instruction::Block(name))
}

pub fn end(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if !arguments.is_empty() {
        return Err(());
    }

    Ok(Instruction::End)
}

pub fn return_value(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.is_empty() {
        return Err(());
    }

    Ok(Instruction::Return(
        arguments.into_iter().map(Into::into).collect(),
    ))
}

pub fn call(function_name: String, arguments: Vec<Token>) -> Result<Instruction, ()> {
    Ok(Instruction::Call(
        function_name,
        arguments.into_iter().map(Into::into).collect(),
    ))
}
