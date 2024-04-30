use super::types::Type;
use std::collections::HashMap;

pub enum Data {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
    Array(Type, Vec<Data>),
    Struct(Box<str>, HashMap<Box<str>, Data>),
}

impl Data {
    pub fn equal_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => i == j,
            (Data::Integer(i), Data::Float(j)) => *i as f64 == *j,
            (Data::Float(i), Data::Integer(j)) => *i == *j as f64,
            (Data::Float(i), Data::Float(j)) => i == j,
            (Data::Boolean(i), Data::Boolean(j)) => i == j,
            (Data::Character(i), Data::Character(j)) => i == j,
            (Data::String(i), Data::String(j)) => i == j,
            (Data::Array(type1, val1), Data::Array(type2, val2)) => {
                type1 == type2 && val1.iter().zip(val2).all(|(v1, v2)| v1.equal_to(v2))
            }
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
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 + *j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(*i + *j as f64)),
            (Data::Float(i), Data::Float(j)) => Some(Data::Float(i + j)),
            (Data::String(i), Data::String(j)) => Some(Data::String(i.clone() + j.as_str())),
            _ => None,
        }
    }

    pub fn subtract(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i - j)),
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 - *j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(*i - *j as f64)),
            (Data::Float(i), Data::Float(j)) => Some(Data::Float(i - j)),
            _ => None,
        }
    }

    pub fn multiply(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i * j)),
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 * *j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(*i * *j as f64)),
            (Data::Float(i), Data::Float(j)) => Some(Data::Float(i * j)),
            _ => None,
        }
    }

    pub fn divide(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Data::Integer(i), Data::Integer(j)) => Some(Data::Integer(i / j)),
            (Data::Integer(i), Data::Float(j)) => Some(Data::Float(*i as f64 / *j)),
            (Data::Float(i), Data::Integer(j)) => Some(Data::Float(*i / *j as f64)),
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
}
