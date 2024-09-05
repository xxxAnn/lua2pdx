use std::{collections::HashMap, fmt::Display};

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum LuaParserValue {
    KeyValue(LuaKeyValue),
    Float(f64),
    Table(HashMap<LuaKeyValue, LuaParserValue>),
    Function(String, Vec<LuaStatement>, Vec<LuaParserValue>),
    Operation(Box<LuaParserValue>, char, Box<LuaParserValue>),

    // var=1,10,(1)
    Conditional(Box<LuaParserValue>, String, Box<LuaParserValue>),
    NumericFor(LuaKeyValue, Box<LuaParserValue>, Box<LuaParserValue>, Option<Box<LuaParserValue>>),
    GenericFor(LuaKeyValue, Option<LuaKeyValue>)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize)]
pub enum LuaKeyValue {
    Nil,
    Boolean(bool),
    Number(u64),
    String(String),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum LuaStatement {
    Assign(LuaParserValue, LuaParserValue),
    FunctionCall(LuaParserValue, Vec<LuaParserValue>),
    Return(LuaParserValue),
    // Todo: Add more control flow structures
    Break,
    // Contains the condition: LuaParserValue
    // The statements: Vec<LuaStatement>
    // The else if statements: Vec<LuaStatement>
    // The else statements: Option<LuaStatement>
    If(LuaParserValue, Vec<LuaStatement>, Vec<LuaStatement>, Option<Box<LuaStatement>>),
    Else(Vec<LuaStatement>),
    While(LuaParserValue, Vec<LuaStatement>),
    // In a Numeric For the LuaParserValue
    // Contains the NumericFor Variant
    // In a Generic For the LuaParserValue
    // Contains the GenericFor Variant
    // And its first LuaStatement contains the object to iterate over
    For(LuaParserValue, Vec<LuaStatement>)
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
    pub fn expect_key_value(&self) -> LuaKeyValue {
        match self {
            LuaParserValue::KeyValue(kv) => kv.clone(),
            _ => panic!("Expected key value, found {:?}", self),
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
            LuaParserValue::Conditional(a, op, b) => write!(f, "({} {} {})", a, op, b),
            LuaParserValue::NumericFor(var, start, end, step) => {
                write!(f, "for {} = {}, {}, {}", var, start, end, step.as_ref().map(|s| s.to_string()).unwrap_or("1".to_string()))
            },
            LuaParserValue::GenericFor(var, iter) => {
                write!(f, "for {} in {}", var, iter.as_ref().map(|i| i.to_string()).unwrap_or("".to_string()))
            }
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