use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, take_while1},
    character::complete::{char, digit1, one_of},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
pub enum LuaParserValue {
    // Any simple value (nil, boolean, number, string, identifier)
    // That can be used as a key in a table
    KeyValue(LuaKeyValue),
    // Complex value (table, function)
    // They can only be used as values in a table
    Float(f64),
    Table(HashMap<LuaKeyValue, LuaParserValue>),
    Function(Vec<LuaStatement>),
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
    Assign(String, LuaParserValue),
    FunctionCall(String, Vec<LuaParserValue>),
    If(Vec<(LuaParserValue, Vec<LuaStatement>)>, Option<Vec<LuaStatement>>),
    While(LuaParserValue, Vec<LuaStatement>),
    Do(Vec<LuaStatement>),
    Return(Vec<LuaParserValue>),
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
    pub fn function(statements: Vec<LuaStatement>) -> Self {
        LuaParserValue::Function(statements)
    }
}

// PartialEq implementation for LuaParserValue already done via `#[derive(PartialEq)]`

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
            LuaParserValue::Function(_) => write!(f, "function(...) ... end"),
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
        escaped_transform(take_while1(|c| c != '"' && c != '\\'), '\\', one_of("\"n\\")),
        char('"')
    )(input)?;
    Ok((input, LuaParserValue::KeyValue(LuaKeyValue::String(string))))
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
