use crate::{data::Data, data::Type, tokeniser::Token};

mod arithmetic;
mod comparision;
mod function;
mod jump_statement;
mod variable;

#[derive(Clone)]
pub enum ValueKind {
    Variable(String),
    Literal(Data),
}

impl From<Token> for ValueKind {
    fn from(value: Token) -> Self {
        match value {
            Token::Identifier(i) => ValueKind::Variable(i),
            Token::Integer(i) => ValueKind::Literal(Data::Integer(i)),
            Token::Boolean(i) => ValueKind::Literal(Data::Boolean(i)),
            Token::Float(i) => ValueKind::Literal(Data::Float(i)),
            Token::Character(i) => ValueKind::Literal(Data::Character(i)),
            Token::String(i) => ValueKind::Literal(Data::String(i)),
        }
    }
}

pub enum Instruction {
    Function(String, Vec<(String, Type)>), // name, (param_name, type)
    Block(String),                         // name
    CreateVariable(String, Type),          // name type
    SetVariable(String, ValueKind),        // name data
    Compare(ComparisionType, ValueKind, ValueKind, ValueKind), // operator lhs rhs set
    Goto(String),                          // block_name
    GotoIf(String, ValueKind),             // block_name condition
    GoBack,                                // go_back
    GoBackIf(ValueKind),                   // go_back condition
    Skip(usize),                           // number_of_instructions
    Arithmetic(ArithmeticOperator, ValueKind, ValueKind, ValueKind), // operator lhs rhs
    Call(String, Vec<ValueKind>),          // function_name arguments
    Return(Vec<ValueKind>),                // return_values
    End,
}

pub enum ComparisionType {
    EqualTo,
    NotEqualTo,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
}

pub enum ArithmeticOperator {
    Add,
    Subtract,
    Muliply,
    Divide,
    Remainder,
}

pub fn parse(tokens: Vec<Vec<Token>>) -> Vec<Instruction> {
    tokens.into_iter().map(|x| parse_line(x).unwrap()).collect()
}

fn parse_line(line: Vec<Token>) -> Result<Instruction, ()> {
    let mut line_iter = line.clone().into_iter();

    let instruction = match line_iter.next().unwrap() {
        Token::Identifier(i) => i,
        _ => panic!(),
    };
    let arguments: Vec<Token> = line_iter.collect();

    match instruction.as_str() {
        "CREATE_VAR" => variable::create_variable(arguments),
        "SET_VAR" => variable::set_variable(arguments),

        "EQ" => comparision::compare(arguments, ComparisionType::EqualTo),
        "NE" => comparision::compare(arguments, ComparisionType::NotEqualTo),
        "LT" => comparision::compare(arguments, ComparisionType::LessThan),
        "LE" => comparision::compare(arguments, ComparisionType::LessThanEqualTo),
        "GT" => comparision::compare(arguments, ComparisionType::GreaterThan),
        "GE" => comparision::compare(arguments, ComparisionType::GreaterThanEqualTo),

        "GOTO" => jump_statement::goto(arguments),
        "GOTO_IF" => jump_statement::goto_if(arguments),
        "GO_BACK" => jump_statement::go_back(arguments),
        "GO_BACK_IF" => jump_statement::go_back_if(arguments),
        "SKIP" => jump_statement::skip(arguments),

        "ADD" => arithmetic::operator(arguments, ArithmeticOperator::Add),
        "SUB" => arithmetic::operator(arguments, ArithmeticOperator::Subtract),
        "MUL" => arithmetic::operator(arguments, ArithmeticOperator::Muliply),
        "DIV" => arithmetic::operator(arguments, ArithmeticOperator::Divide),
        "REM" => arithmetic::operator(arguments, ArithmeticOperator::Remainder),

        "FUNC" => function::init_function(arguments),
        "BLOCK" => function::init_block(arguments),
        "END" => function::end(arguments),
        "RETURN" => function::return_value(arguments),
        function_name => function::call(function_name.to_string(), arguments),
    }
}
