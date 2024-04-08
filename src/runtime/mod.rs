use crate::{
    data::{get_from_scope, get_from_scope_mut, DataType},
    parser::Instruction,
};
use std::collections::HashMap;

mod arithmetic;
mod comparision;
mod function;
mod jump_statement;
mod variable;

pub type Scope = HashMap<String, DataType>;
pub type InbuiltFunction = HashMap<
    String,
    (
        Box<dyn Fn(&mut Scope, &mut Scope, Vec<(String, DataType)>) -> Option<Vec<DataType>>>,
        i16,
        u8,
    ),
>;
// name -> function, no of params, no of return values

enum StatementReturn {
    None,
    End,
    GoBack,
    Skip(usize),
    Return(Vec<DataType>),
}

pub fn execute(instructions: Vec<Instruction>) {
    let mut global_scope = Scope::new();
    let inbuilt = InbuiltFunction::new();

    let main_function_index = function::get_function_index(&instructions, "main").unwrap();
    function::run_function(
        &instructions,
        main_function_index,
        None,
        &mut global_scope,
        &inbuilt,
    );
}
