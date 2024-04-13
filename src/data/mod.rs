use std::collections::HashMap;

type Members = HashMap<String, Data>;

#[derive(Debug, Clone)]
pub enum Data {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
    Struct(String, Members), // name, fields
}

#[derive(Clone)]
pub enum Type {
    Any,
    Integer,
    Float,
    Boolean,
    Character,
    String,
    Struct(String),
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        match value.as_str() {
            "any" => Type::Any,
            "int" => Type::Integer,
            "float" => Type::Float,
            "bool" => Type::Boolean,
            "char" => Type::Character,
            "String" => Type::String,
            _ => Type::Struct(value),
        }
    }
}

impl Data {
    pub fn is_type(&self, type_: Type) -> bool {
        if matches!(type_, Type::Any) {
            return true;
        }

        match (self, type_) {
            (Data::Integer(_), Type::Integer) => true,
            (Data::Float(_), Type::Float) => true,
            (Data::Boolean(_), Type::Boolean) => true,
            (Data::Character(_), Type::Character) => true,
            (Data::String(_), Type::String) => true,
            (Data::Struct(i, _), Type::Struct(j)) => *i == j,
            _ => false,
        }
    }

    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Integer(_), Data::Integer(_)) => true,
            (Data::Float(_), Data::Float(_)) => true,
            (Data::Boolean(_), Data::Boolean(_)) => true,
            (Data::Character(_), Data::Character(_)) => true,
            (Data::String(_), Data::String(_)) => true,
            (Data::Struct(i, _), Data::Struct(j, _)) => i == j,
            _ => false,
        }
    }

    pub fn equal_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => i == j,
            (Data::Integer(i), Data::Float(j)) => *i as f64 == *j,
            (Data::Float(i), Data::Integer(j)) => *i == *j as f64,
            (Data::Float(i), Data::Float(j)) => i == j,
            (Data::Boolean(i), Data::Boolean(j)) => i == j,
            (Data::Character(i), Data::Character(j)) => i == j,
            (Data::String(i), Data::String(j)) => i == j,
            (Data::Struct(i, j), Data::Struct(k, l)) => i == k && equal_fields(j, l),
            _ => false,
        }
    }

    pub fn not_equal_to(&self, other: &Self) -> bool {
        !self.equal_to(other)
    }

    pub fn less_than(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(i < j),
            (Data::Integer(i), Data::Float(j)) => Some((*i as f64) < *j),
            (Data::Float(i), Data::Integer(j)) => Some(*i < *j as f64),
            (Data::Float(i), Data::Float(j)) => Some(i < j),
            (Data::Boolean(i), Data::Boolean(j)) => Some(i < j),
            (Data::Character(i), Data::Character(j)) => Some(i < j),
            (Data::String(i), Data::String(j)) => Some(i < j),
            _ => None,
        }
    }

    pub fn less_than_equal_to(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(i <= j),
            (Data::Integer(i), Data::Float(j)) => Some((*i as f64) <= *j),
            (Data::Float(i), Data::Integer(j)) => Some(*i <= *j as f64),
            (Data::Float(i), Data::Float(j)) => Some(i <= j),
            (Data::Boolean(i), Data::Boolean(j)) => Some(i <= j),
            (Data::Character(i), Data::Character(j)) => Some(i <= j),
            (Data::String(i), Data::String(j)) => Some(i <= j),
            _ => None,
        }
    }

    pub fn greater_than(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(i > j),
            (Data::Integer(i), Data::Float(j)) => Some((*i as f64) > *j),
            (Data::Float(i), Data::Integer(j)) => Some(*i > *j as f64),
            (Data::Float(i), Data::Float(j)) => Some(i > j),
            (Data::Boolean(i), Data::Boolean(j)) => Some(i > j),
            (Data::Character(i), Data::Character(j)) => Some(i > j),
            (Data::String(i), Data::String(j)) => Some(i > j),
            _ => None,
        }
    }

    pub fn greater_than_equal_to(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(i >= j),
            (Data::Integer(i), Data::Float(j)) => Some((*i as f64) >= *j),
            (Data::Float(i), Data::Integer(j)) => Some(*i >= *j as f64),
            (Data::Float(i), Data::Float(j)) => Some(i >= j),
            (Data::Boolean(i), Data::Boolean(j)) => Some(i >= j),
            (Data::Character(i), Data::Character(j)) => Some(i >= j),
            (Data::String(i), Data::String(j)) => Some(i >= j),
            _ => None,
        }
    }

    pub fn add(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i + j)),
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 + j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(i + *j as f64)),
            (Data::Float(i), Data::Float(j)) => Some(Data::Float(i + j)),
            (Data::String(i), Data::String(j)) => Some(Data::String(format!("{}{}", i, j))),
            _ => None,
        }
    }

    pub fn subtract(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i - j)),
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 - j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(i - *j as f64)),
            (Data::Float(i), Data::Float(j)) => Some(Data::Float(i - j)),
            _ => None,
        }
    }

    pub fn multiply(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i * j)),
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 * j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(i * *j as f64)),
            (Data::Float(i), Data::Float(j)) => Some(Data::Float(i * j)),
            _ => None,
        }
    }

    pub fn divide(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i / j)),
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 / j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(i / *j as f64)),
            (Data::Float(i), Data::Float(j)) => Some(Data::Float(i / j)),
            _ => None,
        }
    }

    pub fn remainder(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i % j)),
            _ => None,
        }
    }

    pub fn printable_string(&self) -> String {
        match self {
            Data::Integer(i) => i.to_string(),
            Data::Float(i) => i.to_string(),
            Data::Boolean(i) => i.to_string(),
            Data::Character(i) => i.to_string(),
            Data::String(i) => i.clone(),
            Data::Struct(_, _) => panic!(),
        }
    }
}

fn equal_fields(f1: &Members, f2: &Members) -> bool {
    for (k, v) in f1.iter() {
        if f2.get(k).unwrap().not_equal_to(v) {
            return false;
        }
    }

    true
}
