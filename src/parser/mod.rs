use std::collections::HashMap;
use nom::{
    branch::alt, bytes::complete::{tag, take_until, take_while1}, character::complete::{char, digit1, one_of}, combinator::{opt, recognize}, multi::{many0, separated_list0}, sequence::{delimited, preceded, terminated, tuple}, IResult
};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum LuaParserValue {
    KeyValue(LuaKeyValue),
    Float(f64),
    Table(HashMap<LuaKeyValue, LuaParserValue>),
    Function(String, Vec<LuaStatement>, Vec<LuaParserValue>),
    Operation(Box<LuaParserValue>, char, Box<LuaParserValue>),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum LuaKeyValue {
    Nil,
    Boolean(bool),
    Number(u64),
    String(String),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LuaStatement {
    Assign(LuaParserValue, LuaParserValue),
    FunctionCall(LuaParserValue, Vec<LuaParserValue>),
    Return(LuaParserValue),
    // Todo: Add more control flow structures
}

impl From<u64> for LuaParserValue {
    fn from(value: u64) -> Self {
        LuaParserValue::KeyValue(LuaKeyValue::Number(value.into()))
    }
}

impl From<f64> for LuaParserValue {
    fn from(value: f64) -> Self {
        LuaParserValue::Float(value.into())
    }
}

impl LuaParserValue {
    pub fn identifier(ident: impl Into<String>) -> Self {
        LuaParserValue::KeyValue(LuaKeyValue::Identifier(ident.into()))
    }
    pub fn string(s: impl Into<String>) -> Self {
        LuaParserValue::KeyValue(LuaKeyValue::String(s.into()))
    }
    pub fn nil() -> Self {
        LuaParserValue::KeyValue(LuaKeyValue::Nil)
    }
    pub fn bool(b: impl Into<bool>) -> Self {
        LuaParserValue::KeyValue(LuaKeyValue::Boolean(b.into()))
    }
    pub fn table(t: HashMap<LuaKeyValue, LuaParserValue>) -> Self {
        LuaParserValue::Table(t)
    }
    pub fn function(name: String, statements: Vec<LuaStatement>, args: Vec<LuaParserValue>) -> Self {
        LuaParserValue::Function(name, statements, args)
    }
    pub fn as_assign(&self) -> Option<LuaStatement> {
        match self {
            LuaParserValue::Function(n, _, _) => {
                Some(LuaStatement::Assign(LuaParserValue::identifier(n), self.clone()))
            }
            _ => {
                None
            }
        }
    }
}

impl Display for LuaParserValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuaParserValue::KeyValue(simple_value) => write!(f, "{}", simple_value),
            LuaParserValue::Float(n) => write!(f, "{}", n),
            LuaParserValue::Table(t) => {
                let mut first = true;
                write!(f, "{{")?;
                for (key, value) in t {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            },
            LuaParserValue::Function(_, _, _) => write!(f, "function(...) ... end"),
            LuaParserValue::Operation(a, op, b) => write!(f, "({} {} {})", a, op, b),
        }
    }
}

impl Display for LuaKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuaKeyValue::Nil => write!(f, "nil"),
            LuaKeyValue::Boolean(b) => write!(f, "{}", b),
            LuaKeyValue::Number(n) => write!(f, "{}", n),
            LuaKeyValue::String(s) => write!(f, "\"{}\"", s),
            LuaKeyValue::Identifier(ident) => write!(f, "{}", ident),
        }
    }
}

pub fn parse_number(input: &str) -> IResult<&str, LuaParserValue> {
    let (input, num_str) = recognize(tuple((opt(char('-')), digit1, opt(tuple((char('.'), digit1))))))(input)?;
    if num_str.contains('.') {
        let num = num_str.parse::<f64>().expect("Failed to parse float number");
        Ok((input, LuaParserValue::Float(num)))
    } else {
        let num = num_str.parse::<u64>().expect("Failed to parse integer number");
        Ok((input, LuaParserValue::KeyValue(LuaKeyValue::Number(num))))
    }
}

pub fn parse_string(input: &str) -> IResult<&str, LuaParserValue> {
    let (input, string) = delimited(
        char('"'),
        take_while1(|c| c != '"'),
        char('"')
    )(input)?;
    Ok((input, LuaParserValue::KeyValue(LuaKeyValue::String(string.to_string()))))
}

pub fn parse_boolean(input: &str) -> IResult<&str, LuaParserValue> {
    let (input, bool_str) = alt((tag("true"), tag("false")))(input)?;
    let value = bool_str == "true";
    Ok((input, LuaParserValue::KeyValue(LuaKeyValue::Boolean(value))))
}

pub fn parse_nil(input: &str) -> IResult<&str, LuaParserValue> {
    let (input, _) = tag("nil")(input)?;
    Ok((input, LuaParserValue::KeyValue(LuaKeyValue::Nil)))
}

pub fn parse_identifier(input: &str) -> IResult<&str, LuaParserValue> {
    let (input, ident) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
    Ok((input, LuaParserValue::KeyValue(LuaKeyValue::Identifier(ident.to_string()))))
}

pub fn parse_primary_expression(input: &str) -> IResult<&str, LuaParserValue> {
    alt((parse_number, parse_string, parse_boolean, parse_nil, parse_identifier))(input)
}


pub fn parse_operation_recursive(input: &str, lhs: LuaParserValue) -> IResult<&str, LuaParserValue> {
    let (input, _) = opt(parse_spaces)(input)?;

    let (input, op) = opt(alt((char('+'), char('-'), char('*'), char('/'))))(input)?;
    
    if let Some(op) = op {
        let (input, _) = opt(parse_spaces)(input)?;

        let (input, rhs) = parse_primary_expression(input)?;

        let result = LuaParserValue::Operation(Box::new(lhs), op, Box::new(rhs));
        parse_operation_recursive(input, result) // Recursive call
    } else {
        Ok((input, lhs))
    }
}

pub fn parse_expression(input: &str) -> IResult<&str, LuaParserValue> {
    // First, parse the primary expression
    let (input, lhs) = parse_primary_expression(input)?;

    // Then try to parse any following operations
    parse_operation_recursive(input, lhs)
}


pub fn parse_assign(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = parse_spaces(input)?;
    let (input, ident) = parse_identifier(input)?;
    let (input, _) = many0(char(' '))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = many0(char(' '))(input)?;
    let (input, value) = parse_expression(input)?;
    Ok((input, LuaStatement::Assign(ident, value)))
}

pub fn parse_function_call(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = parse_spaces(input)?;
    let (input, ident) = parse_identifier(input)?;
    let (input, _) = many0(char(' '))(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, args) = separated_list0(tag(","), parse_expression)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, LuaStatement::FunctionCall(ident, args)))
}

pub fn parse_return(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = parse_spaces(input)?;
    let (input, _) = tag("return")(input)?;
    let (input, _) = parse_spaces(input)?;
    println!("idkl17 {}", input);
    println!("idkl18 {}", input == " a + b");
    let (input, value) = parse_expression(input)?;
    Ok((input, LuaStatement::Return(value)))
}

pub fn parse_basic_statement(input: &str) -> IResult<&str, LuaStatement> {
    alt((parse_assign, parse_function_call, parse_return))(input)
}

fn parse_space(input: &str) -> IResult<&str, ()> {
    let (input, _) = char(' ')(input)?;
    Ok((input, ()))
}

fn parse_spaces(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(parse_space)(input)?;
    Ok((input, ()))
}

fn parse_newline(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((char('\n'), char('\r')))(input)?;
    Ok((input, ()))
}

fn parse_newlines(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(parse_newline)(input)?;
    Ok((input, ()))
}

fn clear_noise(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(alt((parse_space, parse_newline)))(input)?;
    Ok((input, ()))
}

fn parse_argument_list(input: &str) -> IResult<&str, Vec<LuaParserValue>> {
    let (input, _) = tag("(")(input)?;
    let (input, args) = separated_list0(tag(","), delimited(opt(parse_spaces), parse_expression, opt(parse_spaces)))(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, args))
}

pub fn parse_function_definition(input: &str) -> IResult<&str, LuaParserValue> {
    let (input, _) = tag("function")(input)?;
    let (input, _) = opt(parse_spaces)(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, args) = parse_argument_list(input)?;
    let (input, _) = opt(parse_spaces)(input)?;
    let (input, _) = tag("do")(input)?;
    let (input, _) = parse_newlines(input)?;
    let (input, statements) = many0(delimited(clear_noise, parse_basic_statement, clear_noise))(input)?;
    let (input, _) = parse_newlines(input)?;
    let (input, _) = tag("end")(input)?;
    Ok((input, LuaParserValue::Function(name.to_string(), statements, args)))//statements, args)))
}

pub fn parse_function_definition_as_statement(input: &str) -> IResult<&str, LuaStatement> {
    let (input, function) = parse_function_definition(input)?;
    Ok((input, function.as_assign().unwrap()))
}

pub fn parse_root_statement(input: &str) -> IResult<&str, LuaStatement> {
    alt((parse_basic_statement, parse_function_definition_as_statement))(input)
}

pub fn parse_root_statements(input: &str) -> IResult<&str, Vec<LuaStatement>> {
    many0(preceded(opt(clear_noise), parse_root_statement))(input)
}
