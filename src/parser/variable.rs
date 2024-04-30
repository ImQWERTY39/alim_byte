use crate::{runtime::Instruction, tokeniser::Token};

pub fn instruction_create(arguments: Vec<Token>) -> Instruction {
    if arguments.len() != 2 {
        panic!()
    }

    let var_type = if let Token::Identifier(i) = arguments[0] {
        i.as_ref().into()
    } else {
        panic!()
    };

    let var_name = if let Token::Identifier(i) = arguments[1] {
        i
    } else {
        panic!()
    };

    Instruction::CreateVar(var_name, var_type)
}

pub fn instruction_set(arguments: Vec<Token>) -> Instruction {
    todo!()
}

pub fn instruction_get_index(arguments: Vec<Token>) -> Instruction {
    todo!()
}

pub fn instruction_set_index(arguments: Vec<Token>) -> Instruction {
    todo!()
}
