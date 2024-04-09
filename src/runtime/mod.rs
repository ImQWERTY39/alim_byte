use crate::{data::DataType, parser::Instruction};
use std::{collections::HashMap, io::Write};

mod arithmetic;
mod comparision;
mod function;
mod jump_statement;
mod variable;

pub type Scope = HashMap<String, DataType>;
pub type InbuiltFunction = HashMap<
    String, // function name
    (
        Vec<(String, DataType)>, // arguments expected
        Box<dyn Fn(&mut Scope, &mut Scope, Option<Scope>) -> Option<Vec<DataType>>>, // local scope, global scope, arguments
    ),
>;

enum StatementReturn {
    None,
    End,
    GoBack,
    Skip(usize),
    Return(Vec<DataType>),
}

pub fn execute(instructions: Vec<Instruction>) {
    let mut global_scope = Scope::new();
    let inbuilt = inbuilts();

    let main_function_index = function::get_function_index(&instructions, "main").unwrap();
    function::run_function(
        &instructions,
        main_function_index,
        None,
        &mut global_scope,
        &inbuilt,
    );
}

fn inbuilts() -> InbuiltFunction {
    let mut inbuilt_functions = InbuiltFunction::new();

    inbuilt_functions.insert(
        String::from("math_sin"),
        (
            vec![(String::from("x"), DataType::Float(None))],
            Box::new(|_, _, arguments| {
                if let Some(DataType::Float(Some(i))) = arguments.unwrap().get("x") {
                    Some(vec![DataType::Float(Some(i.sin()))])
                } else {
                    panic!()
                }
            }),
        ),
    );

    inbuilt_functions.insert(
        String::from("std_print"),
        (
            vec![(String::from("value"), DataType::Identifier(String::new()))],
            Box::new(|local_scope, global_scope, arguments| {
                print!(
                    "{}",
                    arguments
                        .unwrap()
                        .get("value")
                        .unwrap()
                        .printable_string(&local_scope, &global_scope)
                );
                std::io::stdout().flush().unwrap();

                None
            }),
        ),
    );

    inbuilt_functions.insert(
        String::from("std_println"),
        (
            vec![(String::from("value"), DataType::Identifier(String::new()))],
            Box::new(|local_scope, global_scope, arguments| {
                println!(
                    "{}",
                    arguments
                        .unwrap()
                        .get("value")
                        .unwrap()
                        .printable_string(&local_scope, &global_scope)
                );
                std::io::stdout().flush().unwrap();

                None
            }),
        ),
    );

    inbuilt_functions
}
