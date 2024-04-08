use crate::runtime::Scope;
use crate::tokeniser::Token;

#[derive(Debug, Clone)]
pub enum DataType {
    Integer(Option<i64>),
    Float(Option<f64>),
    Boolean(Option<bool>),
    Character(Option<char>),
    String(Option<String>),
    Identifier(String),
}

impl DataType {
    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::Integer(_), DataType::Integer(_)) => true,
            (DataType::Float(_), DataType::Float(_)) => true,
            (DataType::Boolean(_), DataType::Boolean(_)) => true,
            (DataType::Character(_), DataType::Character(_)) => true,
            (DataType::String(_), DataType::String(_)) => true,
            _ => false,
        }
    }
    pub fn string_as_type(type_str: &str) -> Self {
        match type_str {
            "int" => DataType::Integer(None),
            "float" => DataType::Float(None),
            "bool" => DataType::Boolean(None),
            "char" => DataType::Character(None),
            "String" => DataType::String(None),
            _ => unimplemented!(),
        }
    }

    pub fn token_as_type(token: &Token) -> Self {
        match token {
            Token::Identifier(i) => Self::Identifier(i.clone()),
            Token::Integer(i) => Self::Integer(Some(*i)),
            Token::Boolean(i) => Self::Boolean(Some(*i)),
            Token::Float(i) => Self::Float(Some(*i)),
            Token::Character(i) => Self::Character(Some(*i)),
            Token::String(i) => Self::String(Some(i.clone())),
        }
    }

    pub fn equal_to(&self, other: &DataType, local_scope: &Scope, global_scope: &Scope) -> bool {
        let lhs = if let Self::Identifier(i) = self {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            self.clone()
        };

        let rhs = if let Self::Identifier(i) = other {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            other.clone()
        };

        match (lhs, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => i.unwrap() == j.unwrap(),
            (DataType::Integer(i), DataType::Float(j)) => i.unwrap() as f64 == j.unwrap(),
            (DataType::Float(i), DataType::Integer(j)) => i.unwrap() == j.unwrap() as f64,
            (DataType::Float(i), DataType::Float(j)) => i.unwrap() == j.unwrap(),
            (DataType::Boolean(i), DataType::Boolean(j)) => i.unwrap() == j.unwrap(),
            (DataType::Character(i), DataType::Character(j)) => i.unwrap() == j.unwrap(),
            (DataType::String(i), DataType::String(j)) => i.unwrap() == j.unwrap(),
            _ => false,
        }
    }

    pub fn not_equal_to(
        &self,
        other: &DataType,
        local_scope: &Scope,
        global_scope: &Scope,
    ) -> bool {
        !self.equal_to(other, local_scope, global_scope)
    }

    pub fn less_than(&self, other: &DataType, local_scope: &Scope, global_scope: &Scope) -> bool {
        let lhs = if let Self::Identifier(i) = self {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            self.clone()
        };

        let rhs = if let Self::Identifier(i) = other {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            other.clone()
        };

        match (lhs, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => i.unwrap() < j.unwrap(),
            (DataType::Integer(i), DataType::Float(j)) => (i.unwrap() as f64) < j.unwrap(),
            (DataType::Float(i), DataType::Integer(j)) => i.unwrap() < j.unwrap() as f64,
            (DataType::Float(i), DataType::Float(j)) => i.unwrap() < j.unwrap(),
            (DataType::Character(i), DataType::Character(j)) => i.unwrap() < j.unwrap(),
            (DataType::String(i), DataType::String(j)) => i.unwrap() < j.unwrap(),
            _ => false,
        }
    }

    pub fn less_than_equal_to(
        &self,
        other: &DataType,
        local_scope: &Scope,
        global_scope: &Scope,
    ) -> bool {
        self.less_than(other, local_scope, global_scope)
            || self.equal_to(other, local_scope, global_scope)
    }

    pub fn greater_than(
        &self,
        other: &DataType,
        local_scope: &Scope,
        global_scope: &Scope,
    ) -> bool {
        !self.less_than_equal_to(other, local_scope, global_scope)
    }

    pub fn greater_than_equal_to(
        &self,
        other: &DataType,
        local_scope: &Scope,
        global_scope: &Scope,
    ) -> bool {
        !self.less_than(other, local_scope, global_scope)
    }

    pub fn add(&self, other: &DataType, local_scope: &Scope, global_scope: &Scope) -> Self {
        let lhs = if let Self::Identifier(i) = self {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            self.clone()
        };

        let rhs = if let Self::Identifier(i) = other {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            other.clone()
        };

        match (lhs, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() + j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 + j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() + j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() + j.unwrap()))
            }
            (DataType::String(i), DataType::String(j)) => {
                DataType::String(Some(i.unwrap() + j.unwrap().as_str()))
            }
            _ => panic!(),
        }
    }

    pub fn subtract(&self, other: &DataType, local_scope: &Scope, global_scope: &Scope) -> Self {
        let lhs = if let Self::Identifier(i) = self {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            self.clone()
        };

        let rhs = if let Self::Identifier(i) = other {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            other.clone()
        };

        match (lhs, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() - j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 - j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() - j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() - j.unwrap()))
            }
            _ => panic!(),
        }
    }

    pub fn multiply(&self, other: &DataType, local_scope: &Scope, global_scope: &Scope) -> Self {
        let lhs = if let Self::Identifier(i) = self {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            self.clone()
        };

        let rhs = if let Self::Identifier(i) = other {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            other.clone()
        };

        match (lhs, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() * j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 * j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() * j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() * j.unwrap()))
            }
            _ => panic!(),
        }
    }

    pub fn divide(&self, other: &DataType, local_scope: &Scope, global_scope: &Scope) -> Self {
        let lhs = if let Self::Identifier(i) = self {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            self.clone()
        };

        let rhs = if let Self::Identifier(i) = other {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            other.clone()
        };

        match (lhs, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() / j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 / j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() / j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() / j.unwrap()))
            }
            _ => panic!(),
        }
    }

    pub fn remainder(&self, other: &DataType, local_scope: &Scope, global_scope: &Scope) -> Self {
        let lhs = if let Self::Identifier(i) = self {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            self.clone()
        };

        let rhs = if let Self::Identifier(i) = other {
            get_from_scope(local_scope, global_scope, i).unwrap()
        } else {
            other.clone()
        };

        match (lhs, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() % j.unwrap()))
            }
            _ => panic!(),
        }
    }
}

pub fn get_from_scope(
    local_scope: &Scope,
    global_scope: &Scope,
    variable: &str,
) -> Option<DataType> {
    if local_scope.contains_key(variable) {
        return Some(local_scope.get(variable).unwrap().clone());
    }

    if global_scope.contains_key(variable) {
        return Some(global_scope.get(variable).unwrap().clone());
    }

    None
}

pub fn get_from_scope_mut<'a>(
    local_scope: &'a mut Scope,
    global_scope: &'a mut Scope,
    variable: &str,
) -> Option<&'a mut DataType> {
    if local_scope.contains_key(variable) {
        return Some(local_scope.get_mut(variable).unwrap());
    }

    if global_scope.contains_key(variable) {
        return Some(global_scope.get_mut(variable).unwrap());
    }

    None
}
