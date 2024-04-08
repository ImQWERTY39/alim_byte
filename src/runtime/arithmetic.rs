use super::Scope;
use crate::{
    data::{get_from_scope_mut, DataType},
    parser::ArithmeticOperator,
};

pub fn operation(
    lhs: &DataType,
    rhs: &DataType,
    assign_to: &DataType,
    operator: &ArithmeticOperator,
    local_scope: &mut Scope,
    global_scope: &mut Scope,
) {
    let result = match operator {
        ArithmeticOperator::Add => lhs.add(rhs, local_scope, global_scope),
        ArithmeticOperator::Subtract => lhs.subtract(rhs, local_scope, global_scope),
        ArithmeticOperator::Muliply => lhs.multiply(rhs, local_scope, global_scope),
        ArithmeticOperator::Divide => lhs.divide(rhs, local_scope, global_scope),
        ArithmeticOperator::Remainder => lhs.remainder(rhs, local_scope, global_scope),
    };

    if let DataType::Identifier(i) = assign_to {
        *get_from_scope_mut(local_scope, global_scope, i).unwrap() = result;
    } else {
        panic!()
    }
}
