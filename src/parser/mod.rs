use crate::{data::DataType, tokeniser::Token};

mod arithmetic;
mod comparision;
mod function;
mod jump_statement;
mod variable;

#[derive(Debug)]
pub enum Instruction {
    Function(String, Vec<(String, DataType)>),
    Block(String),
    CreateVariable(String, DataType),
    SetVariable(String, DataType),
    Compare(ComparisionType, DataType, DataType, DataType),
    Goto(String),
    GotoIf(String, DataType),
    GoBack,
    GoBackIf(DataType),
    Skip(usize),
    Arithmetic(ArithmeticOperator, DataType, DataType, DataType),
    Call(String, Vec<DataType>),
    Return(Vec<DataType>),
    End,
}

#[derive(Debug)]
pub enum ComparisionType {
    EqualTo,
    NotEqualTo,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
}

#[derive(Debug)]
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
