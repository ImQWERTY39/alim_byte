use crate::{
    data::{Data, Type},
    parser::Instruction,
};
use std::{collections::HashMap, io::Write};

mod arithmetic;
mod comparision;
mod function;
mod jump_statement;
mod variable;

#[derive(Clone)]
pub enum StoredValue {
    Null(Type),
    Value(Data),
}

impl StoredValue {
    pub fn get_value(&self) -> &Data {
        if let Self::Value(i) = self {
            i
        } else {
            panic!()
        }
    }
}

pub type Scope = HashMap<String, StoredValue>;
pub type InbuiltFunction = HashMap<
    String, // function name
    (
        Vec<(String, Type)>,                             // arguments expected
        Box<dyn Fn(Option<Scope>) -> Option<Vec<Data>>>, // local scope, global scope, arguments
    ),
>;

enum StatementReturn {
    None,
    End,
    GoBack,
    Skip(usize),
    Return(Vec<Data>),
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
        String::from("std_print"),
        (
            vec![(String::from("value"), Type::Any)],
            Box::new(|arguments| {
                print!(
                    "{}",
                    match arguments.unwrap().get("value").unwrap() {
                        StoredValue::Null(_) => panic!(),
                        StoredValue::Value(i) => i.printable_string(),
                    }
                );
                std::io::stdout().flush().unwrap();

                None
            }),
        ),
    );

    inbuilt_functions.insert(
        String::from("std_println"),
        (
            vec![(String::from("value"), Type::Any)],
            Box::new(|arguments| {
                println!(
                    "{}",
                    match arguments.unwrap().get("value").unwrap() {
                        StoredValue::Null(_) => panic!(),
                        StoredValue::Value(i) => i.printable_string(),
                    }
                );
                std::io::stdout().flush().unwrap();

                None
            }),
        ),
    );

    inbuilt_functions
}

fn get<'a>(varible: &'a str, local_scope: &'a Scope, global_scope: &'a Scope) -> &'a StoredValue {
    if let Some(i) = local_scope.get(varible) {
        i
    } else if let Some(i) = global_scope.get(varible) {
        i
    } else {
        panic!()
    }
}

fn get_mut<'a>(
    varible: &'a str,
    local_scope: &'a mut Scope,
    global_scope: &'a mut Scope,
) -> &'a mut StoredValue {
    if let Some(i) = local_scope.get_mut(varible) {
        i
    } else if let Some(i) = global_scope.get_mut(varible) {
        i
    } else {
        panic!()
    }
}
