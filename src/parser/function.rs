use super::Instruction;
use crate::{data::DataType, tokeniser::Token};

pub fn init_function(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() % 2 == 0 {
        return Err(());
    }

    let name = if let Token::Identifier(i) = &arguments[0] {
        i.clone()
    } else {
        return Err(());
    };

    if arguments.len() == 1 {
        Ok(Instruction::Function(name, Vec::new()))
    } else {
        Ok(Instruction::Function(
            name,
            parse_parametres(&arguments[1..])?,
        ))
    }
}

fn parse_parametres(arguments: &[Token]) -> Result<Vec<(String, DataType)>, ()> {
    let mut parametres = Vec::with_capacity(arguments.len() / 2);
    let mut param_list = arguments.iter();

    while let Some(type_token) = param_list.next() {
        let name = if let Token::Identifier(i) = param_list.next().ok_or(())? {
            i.clone()
        } else {
            return Err(());
        };

        parametres.push((name, DataType::token_as_type(type_token)));
    }

    Ok(parametres)
}

pub fn init_block(arguments: Vec<Token>) -> Result<Instruction, ()> {
    if arguments.len() != 1 {
        return Err(());
    }

    let name = if let Token::Identifier(i) = &arguments[0] {
        i.clone()
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
        arguments.iter().map(DataType::token_as_type).collect(),
    ))
}

pub fn call(function_name: String, arguments: Vec<Token>) -> Result<Instruction, ()> {
    Ok(Instruction::Call(
        function_name,
        arguments.iter().map(DataType::token_as_type).collect(),
    ))
}
