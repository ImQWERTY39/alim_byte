use crate::{runtime::Instruction, tokeniser::Token};
use std::collections::HashMap;

mod fucntion;
mod jump_statement;
mod operators;
mod variable;

pub fn parse(
    tokens: Vec<Vec<Token>>,
) -> (
    Vec<Instruction>,
    HashMap<Box<str>, usize>,
    HashMap<Box<str>, usize>,
) {
    let mut instruction_set = Vec::with_capacity(tokens.len());
    let mut function_indexes = HashMap::new();
    let mut block_indexes = HashMap::new();

    for (idx, val) in tokens.into_iter().enumerate() {
        let mut line = val.into_iter();

        let instruction_str = if let Token::Identifier(i) = line.next().unwrap() {
            i
        } else {
            panic!()
        };
        let arguments: Vec<Token> = line.collect();

        instruction_set.push(match instruction_str.as_ref() {
            "CREATE" => variable::instruction_create(arguments),
            "SET" => variable::instruction_set(arguments),
            "GET_IDX" => variable::instruction_get_index(arguments),
            "SET_IDX" => variable::instruction_set_index(arguments),

            "LOAD" => todo!(),
            "UNLOAD" => todo!(),
            "LOAD_PARAMS" => todo!(),
            "UNLOAD_RET" => todo!(),

            "EQ" => todo!(),
            "NE" => todo!(),
            "LT" => todo!(),
            "LE" => todo!(),
            "GT" => todo!(),
            "GE" => todo!(),

            "ADD" => todo!(),
            "SUB" => todo!(),
            "MUL" => todo!(),
            "DIV" => todo!(),
            "REM" => todo!(),

            "BAND" => todo!(),
            "BOR" => todo!(),
            "BXOR" => todo!(),
            "BNOT" => todo!(),
            "BLS" => todo!(),
            "BRS" => todo!(),

            "AND" => todo!(),
            "OR" => todo!(),
            "NOT" => todo!(),

            "GOTO" => todo!(),
            "GOTO_IF" => todo!(),
            "GO_BACK" => todo!(),
            "GO_BACK_IF" => todo!(),
            "SKIP" => todo!(),

            "FUNC" => todo!(),
            "BLOCK" => todo!(),
            "CALL" => todo!(),
            "RETURN" => todo!(),
            "END" => todo!(),

            _ => panic!(),
        });
    }

    (instruction_set, function_indexes, block_indexes)
}
