use crate::data_type::{Data, Type, ValueKind};

pub enum Instruction {
    /* Variable creation, assigning */
    CreateVar(Box<str>, Type),   // variable name `Box<str>` of type `Type`
    SetVar(Box<str>, ValueKind), // Variable(Box<str>) = `ValueKind`;

    /* Indexing */
    GetIndex(ValueKind, usize, Box<str>), // Variable(Box<str>) = {Array|String}(ValueKind)[usize]
    SetIndex(Box<str>, usize, ValueKind), // Array(Box<str>)[usize] = Variable(ValueKind)

    /* Loading and unloading registers */
    Load(ValueKind),
    LoadParams(Vec<ValueKind>),
    Unload(Box<str>),
    UnloadRet(Vec<Data>),

    /* Unary and Binary */
    Relational(RelationalOperator),
    Arithmetic(ArithmeticOperator),
    Bitwise(BitwiseOperator),
    Logical(LogicalOperator),
    // Everything requires both registers filled, except for LogicalOperator::Not, needs only 1

    /* Jump Statements */
    Goto(Box<str>),
    GotoIf(Box<str>), // gets from register
    GoBack,
    GoBackIf, // gets from register
    Skip(usize),

    /* Function / Blocks */
    Function(Box<str>, Vec<(Box<str>, Type)>),
    Block(Box<str>),
    Call(Box<str>),
    Return(Vec<ValueKind>),
    End,
}

pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

pub enum RelationalOperator {
    EqualTo,
    NotEqualTo,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
}

pub enum BitwiseOperator {
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    BitwiseLeftShit,
    BitwiseRightShit,
}

pub enum LogicalOperator {
    And,
    Or,
    Not,
}
