use super::{get, get_mut, Scope, StoredValue};
use crate::parser::{ArithmeticOperator, ValueKind};

pub fn operation(
    lvalue: &ValueKind,
    rvalue: &ValueKind,
    avalue: &ValueKind,
    operator: &ArithmeticOperator,
    local_scope: &mut Scope,
    global_scope: &mut Scope,
) {
    let lhs = match lvalue {
        ValueKind::Variable(i) => get(i, local_scope, global_scope).get_value(),
        ValueKind::Literal(i) => &i,
    };

    let rhs = match rvalue {
        ValueKind::Variable(i) => get(i, local_scope, global_scope).get_value(),
        ValueKind::Literal(i) => &i,
    };

    let result = match operator {
        ArithmeticOperator::Add => lhs.add(rhs),
        ArithmeticOperator::Subtract => lhs.subtract(rhs),
        ArithmeticOperator::Muliply => lhs.multiply(rhs),
        ArithmeticOperator::Divide => lhs.divide(rhs),
        ArithmeticOperator::Remainder => lhs.remainder(rhs),
    };

    if let ValueKind::Variable(i) = avalue {
        *get_mut(i, local_scope, global_scope) = StoredValue::Value(result.unwrap());
    } else {
        panic!()
    }
}
