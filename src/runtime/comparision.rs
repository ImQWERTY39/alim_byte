use super::{get, get_mut, Scope, StoredValue};
use crate::{
    data::Data,
    parser::{ComparisionType, ValueKind},
};

pub fn compare(
    comparision_type: &ComparisionType,
    lvalue: &ValueKind,
    rvalue: &ValueKind,
    avalue: &ValueKind,
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

    let result = match comparision_type {
        ComparisionType::EqualTo => lhs.equal_to(rhs),
        ComparisionType::NotEqualTo => lhs.not_equal_to(rhs),
        ComparisionType::LessThan => lhs.less_than(rhs).unwrap(),
        ComparisionType::LessThanEqualTo => lhs.less_than_equal_to(rhs).unwrap(),
        ComparisionType::GreaterThan => lhs.greater_than(rhs).unwrap(),
        ComparisionType::GreaterThanEqualTo => lhs.greater_than_equal_to(rhs).unwrap(),
    };

    if let ValueKind::Variable(i) = avalue {
        *get_mut(i, local_scope, global_scope) = StoredValue::Value(Data::Boolean(result));
    } else {
        panic!()
    }
}
