mod data;
mod registers;
mod types;

pub use data::Data;
pub use registers::Registers;
pub use types::Type;

pub enum StoredValue {
    Null(Type),
    Value(Data),
}

pub enum ValueKind {
    Literal(Data),
    Variable(Box<str>),
}
