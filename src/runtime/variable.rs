use super::Scope;
use crate::data::{get_from_scope_mut, DataType};

pub fn create_variable(local_scope: &mut Scope, name: String, type_: DataType) {
    local_scope.insert(name, type_);
}

pub fn set_variable(
    local_scope: &mut Scope,
    global_scope: &mut Scope,
    name: String,
    value: DataType,
) {
    *get_from_scope_mut(local_scope, global_scope, &name).unwrap() = value;
}
