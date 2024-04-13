use super::{get, get_mut, Scope, StoredValue};
use crate::{data::Type, parser::ValueKind};

pub fn create_variable(local_scope: &mut Scope, name: String, type_: Type) {
    local_scope.insert(name, StoredValue::Null(type_));
}

pub fn set_variable(
    local_scope: &mut Scope,
    global_scope: &mut Scope,
    variable: &str,
    set_to: ValueKind,
) {
    let set_value = match set_to {
        ValueKind::Variable(i) => get(&i, local_scope, global_scope).clone(),
        ValueKind::Literal(i) => StoredValue::Value(i),
    };

    *get_mut(variable, local_scope, global_scope) = set_value;
}
