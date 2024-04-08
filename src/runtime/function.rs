use std::{collections::HashMap, env::Args};

use crate::{
    data::{get_from_scope, get_from_scope_mut, DataType},
    parser::Instruction,
    runtime::StatementReturn,
};

use super::{arithmetic, comparision, variable, InbuiltFunction, Scope};

pub fn run_function(
    instructions: &[Instruction],
    function_index: usize,
    arguments: Option<Scope>,
    global_scope: &mut Scope,
    inbuilt: &InbuiltFunction,
) -> Option<Vec<DataType>> {
    let mut local_scope = match arguments {
        Some(i) => i,
        None => Scope::new(),
    };
    let mut current_index = function_index + 1;

    while current_index < instructions.len() {
        match execute_line(
            instructions,
            current_index,
            global_scope,
            &mut local_scope,
            inbuilt,
        ) {
            StatementReturn::None => (),
            StatementReturn::End => return None,
            StatementReturn::GoBack => current_index = function_index,
            StatementReturn::Skip(i) => current_index += i,
            StatementReturn::Return(i) => return Some(i),
        }

        println!(
            "local_scope: {:#?}\ngloabal_scope: {:#?}\n\n",
            local_scope, global_scope
        );

        current_index += 1;
    }

    None
}

fn run_block(
    instructions: &[Instruction],
    block_index: usize,
    local_scope: &mut Scope,
    global_scope: &mut Scope,
    inbuilt: &InbuiltFunction,
) {
    let mut current_index = block_index + 1;

    while current_index < instructions.len() {
        match execute_line(
            instructions,
            current_index,
            global_scope,
            local_scope,
            inbuilt,
        ) {
            StatementReturn::None => (),
            StatementReturn::End => return,
            StatementReturn::GoBack => current_index = block_index,
            StatementReturn::Skip(i) => current_index += i,
            StatementReturn::Return(_) => panic!(),
        }

        current_index += 1;
    }
}

fn execute_line(
    instructions: &[Instruction],
    current_index: usize,
    global_scope: &mut Scope,
    local_scope: &mut Scope,
    inbuilt: &InbuiltFunction,
) -> StatementReturn {
    let instruction = &instructions[current_index];

    match instruction {
        Instruction::CreateVariable(name, type_) => {
            variable::create_variable(local_scope, name.to_string(), type_.to_owned())
        }
        Instruction::SetVariable(name, value) => variable::set_variable(
            local_scope,
            global_scope,
            name.to_string(),
            value.to_owned(),
        ),
        Instruction::Compare(comparision_type, lhs, rhs, assign_to) => comparision::compare(
            comparision_type,
            lhs,
            rhs,
            assign_to,
            local_scope,
            global_scope,
        ),
        Instruction::Goto(block_name) => {
            let block_index = get_block_index(instructions, block_name).unwrap();
            run_block(
                instructions,
                block_index,
                local_scope,
                global_scope,
                inbuilt,
            );
        }
        Instruction::GotoIf(block_name, condition) => {
            if is_true(condition, local_scope, global_scope) {
                let block_index = get_block_index(instructions, block_name).unwrap();
                run_block(
                    instructions,
                    block_index,
                    local_scope,
                    global_scope,
                    inbuilt,
                );
            }
        }
        Instruction::GoBack => return StatementReturn::GoBack,
        Instruction::GoBackIf(condition) => {
            if is_true(condition, local_scope, global_scope) {
                return StatementReturn::GoBack;
            }
        }
        Instruction::Skip(i) => return StatementReturn::Skip(*i),
        Instruction::Arithmetic(operator, lhs, rhs, assign_to) => {
            arithmetic::operation(lhs, rhs, assign_to, operator, local_scope, global_scope)
        }
        Instruction::Call(function_name, arguments) => {
            if let Some(idx) = get_function_index(instructions, function_name) {
                let params = if let Instruction::Function(_, params) = &instructions[idx] {
                    params
                } else {
                    unreachable!()
                };

                if arguments.len() < params.len() {
                    panic!()
                }

                let function_index = get_function_index(instructions, function_name).unwrap();
                let args = if !params.is_empty() {
                    let mut args = HashMap::new();

                    for ((name, type_expected), variable_name) in params.iter().zip(arguments) {
                        let variable = if let DataType::Identifier(i) = variable_name {
                            get_from_scope(local_scope, global_scope, i).unwrap()
                        } else {
                            variable_name.clone()
                        };

                        let type_ = if let DataType::Identifier(i) = type_expected {
                            DataType::string_as_type(i)
                        } else {
                            type_expected.clone()
                        };

                        if !type_.same_type(&variable) {
                            panic!()
                        }

                        args.insert(name.clone(), variable.clone());
                    }

                    Some(args)
                } else {
                    None
                };

                if let Some(values) =
                    run_function(instructions, function_index, args, global_scope, inbuilt)
                {
                    let store_ret_vals_len = arguments.len() - params.len();

                    if store_ret_vals_len != values.len() {
                        panic!()
                    }

                    set_return_values(
                        local_scope,
                        global_scope,
                        &arguments[store_ret_vals_len + 1..],
                        &values,
                    );
                }
            }

            if let Some((parametres, function)) = inbuilt.get(function_name) {
                if arguments.len() < parametres.len() {
                    panic!()
                }

                let args = if !parametres.is_empty() {
                    let mut args = HashMap::new();

                    for ((name, type_expected), variable_name) in parametres.iter().zip(arguments) {
                        let variable = if let DataType::Identifier(i) = variable_name {
                            get_from_scope(local_scope, global_scope, i).unwrap()
                        } else {
                            variable_name.clone()
                        };

                        let type_ = if let DataType::Identifier(i) = type_expected {
                            DataType::string_as_type(i)
                        } else {
                            type_expected.clone()
                        };

                        if !type_.same_type(&variable) {
                            panic!()
                        }

                        args.insert(name.clone(), variable.clone());
                    }

                    Some(args)
                } else {
                    None
                };

                if let Some(values) = function(local_scope, global_scope, args) {
                    let store_ret_vals_len = arguments.len() - parametres.len();
                    if store_ret_vals_len != values.len() {
                        panic!()
                    }

                    set_return_values(
                        local_scope,
                        global_scope,
                        &arguments[store_ret_vals_len..],
                        &values,
                    );
                }
            }
        }
        Instruction::Return(i) => {
            return StatementReturn::Return(
                i.iter()
                    .map(|x| {
                        get_from_scope(
                            local_scope,
                            global_scope,
                            if let DataType::Identifier(i) = x {
                                i
                            } else {
                                panic!()
                            },
                        )
                        .unwrap()
                    })
                    .collect(),
            )
        }
        Instruction::End => return StatementReturn::End,
        _ => (),
    };

    StatementReturn::None
}

fn set_return_values(
    local_scope: &mut Scope,
    global_scope: &mut Scope,
    variables: &[DataType],
    values: &[DataType],
) {
    for (value, vars) in values.iter().zip(variables) {
        if let DataType::Identifier(i) = vars {
            *get_from_scope_mut(local_scope, global_scope, i).unwrap() = value.clone();
        }
    }
}

fn is_true(condition: &DataType, local_scope: &mut Scope, global_scope: &mut Scope) -> bool {
    if let DataType::Boolean(Some(true)) = condition {
        return true;
    }

    if let DataType::Identifier(i) = condition {
        if let DataType::Boolean(Some(true)) = get_from_scope(local_scope, global_scope, i).unwrap()
        {
            return true;
        }
    }

    false
}

pub fn get_function_index(instructions: &[Instruction], function_name: &str) -> Option<usize> {
    for (index, instruction) in instructions.iter().enumerate() {
        if let Instruction::Function(name, _) = instruction {
            if name == function_name {
                return Some(index);
            }
        }
    }

    None
}

fn get_block_index(instructions: &[Instruction], block_name: &str) -> Option<usize> {
    for (index, instruction) in instructions.iter().enumerate() {
        if let Instruction::Block(name) = instruction {
            if name == block_name {
                return Some(index);
            }
        }
    }

    None
}
