#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    Character,
    String,
    Array(Box<Type>),
    Struct(Box<str>),
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        if value.starts_with('[') && value.ends_with(']') {
            return Type::Array(Box::new(value[1..value.len() - 1].into()));
        }

        match value {
            "int" => Type::Integer,
            "float" => Type::Float,
            "bool" => Type::Boolean,
            "char" => Type::Character,
            "string" => Type::String,
            other => Type::Struct(other.into()),
        }
    }
}
