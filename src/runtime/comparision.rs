use crate::{
    data::{get_from_scope_mut, DataType},
    parser::ComparisionType,
};

use super::Scope;

pub fn compare(
    comparision_type: &ComparisionType,
    lhs: &DataType,
    rhs: &DataType,
    assign_to: &DataType,
    local_scope: &mut Scope,
    global_scope: &mut Scope,
) {
    let result = match comparision_type {
        ComparisionType::EqualTo => lhs.equal_to(rhs, local_scope, global_scope),
        ComparisionType::NotEqualTo => lhs.not_equal_to(rhs, local_scope, global_scope),
        ComparisionType::LessThan => lhs.less_than(rhs, local_scope, global_scope),
        ComparisionType::LessThanEqualTo => lhs.less_than_equal_to(rhs, local_scope, global_scope),
        ComparisionType::GreaterThan => lhs.greater_than(rhs, local_scope, global_scope),
        ComparisionType::GreaterThanEqualTo => {
            lhs.greater_than_equal_to(rhs, local_scope, global_scope)
        }
    };

    if let DataType::Identifier(i) = assign_to {
        *get_from_scope_mut(local_scope, global_scope, i).unwrap() =
            DataType::Boolean(Some(result));
    } else {
        panic!()
    }
}
