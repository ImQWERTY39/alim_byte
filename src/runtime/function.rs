use super::{arithmetic, comparision, get, variable, InbuiltFunction, Scope};
use crate::{
    data::Data,
    parser::{Instruction, ValueKind},
    runtime::StatementReturn,
};

pub fn run_function(
    instructions: &[Instruction],
    function_index: usize,
    arguments: Option<Scope>,
    global_scope: &mut Scope,
    inbuilt: &InbuiltFunction,
) -> Option<Vec<Data>> {
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
            variable::create_variable(local_scope, name.to_string(), type_.clone())
        }
        Instruction::SetVariable(variable, set_to) => {
            variable::set_variable(local_scope, global_scope, variable, set_to.clone())
        }
        Instruction::Compare(comparision_type, lhs, rhs, assign_to) => comparision::compare(
            comparision_type,
            lhs,
            rhs,
            assign_to,
            local_scope,
            global_scope,
        ),
        Instruction::Goto(i) => run_block(
            instructions,
            get_block_index(instructions, i).unwrap(),
            local_scope,
            global_scope,
            inbuilt,
        ),
        Instruction::GotoIf(block, condition) => {
            let should_go = match condition {
                ValueKind::Variable(i) => {
                    if let Data::Boolean(j) = get(i, local_scope, global_scope).get_value() {
                        j
                    } else {
                        panic!()
                    }
                }
                ValueKind::Literal(i) => {
                    if let Data::Boolean(j) = i {
                        j
                    } else {
                        panic!()
                    }
                }
            };

            if *should_go {
                run_block(
                    instructions,
                    get_block_index(instructions, block).unwrap(),
                    local_scope,
                    global_scope,
                    inbuilt,
                );
            }
        }
        Instruction::GoBack => return StatementReturn::GoBack,
        Instruction::GoBackIf(condition) => {
            let should_go = match condition {
                ValueKind::Variable(i) => {
                    if let Data::Boolean(j) = get(i, local_scope, global_scope).get_value() {
                        j
                    } else {
                        panic!()
                    }
                }
                ValueKind::Literal(i) => {
                    if let Data::Boolean(j) = i {
                        j
                    } else {
                        panic!()
                    }
                }
            };

            if *should_go {
                return StatementReturn::GoBack;
            }
        }
        Instruction::Skip(i) => return StatementReturn::Skip(*i),
        Instruction::Arithmetic(operator, lhs, rhs, assign_to) => {
            arithmetic::operation(lhs, rhs, assign_to, operator, local_scope, global_scope)
        }
        Instruction::Call(_, _) => unimplemented!(),
        Instruction::Return(_) => unimplemented!(),
        Instruction::End => return StatementReturn::End,
        _ => unreachable!(),
    }

    StatementReturn::None
}

pub fn get_function_index(instructions: &[Instruction], function_name: &str) -> Option<usize> {
    for (idx, val) in instructions.iter().enumerate() {
        if let Instruction::Function(i, _) = val {
            if i == function_name {
                return Some(idx);
            }
        }
    }

    None
}

pub fn get_block_index(instructions: &[Instruction], block_name: &str) -> Option<usize> {
    for (idx, val) in instructions.iter().enumerate() {
        if let Instruction::Block(i) = val {
            if i == block_name {
                return Some(idx);
            }
        }
    }

    None
}
