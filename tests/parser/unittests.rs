use std::collections::HashMap;

use pdxlua::parser::{parse_boolean, parse_identifier, parse_nil, parse_number, parse_string, LuaKeyValue, LuaParserValue};


#[test]
fn test_parse_number_integer() {
    let result = parse_number("12345");
    assert_eq!(
        result.unwrap(),
        ("", LuaParserValue::KeyValue(LuaKeyValue::Number(12345)))
    );
}

#[test]
fn test_parse_number_float() {
    let result = parse_number("123.45");
    assert_eq!(
        result.unwrap(),
        ("", LuaParserValue::Float(123.45))
    );
}

#[test]
fn test_parse_string() {
    let result = parse_string("\"Hello, world!\"");
    assert_eq!(
        result.unwrap(),
        ("", LuaParserValue::KeyValue(LuaKeyValue::String(String::from("Hello, world!"))))
    );
}

#[test]
fn test_parse_boolean_true() {
    let result = parse_boolean("true");
    assert_eq!(
        result.unwrap(),
        ("", LuaParserValue::KeyValue(LuaKeyValue::Boolean(true)))
    );
}

#[test]
fn test_parse_boolean_false() {
    let result = parse_boolean("false");
    assert_eq!(
        result.unwrap(),
        ("", LuaParserValue::KeyValue(LuaKeyValue::Boolean(false)))
    );
}

#[test]
fn test_parse_nil() {
    let result = parse_nil("nil");
    assert_eq!(
        result.unwrap(),
        ("", LuaParserValue::KeyValue(LuaKeyValue::Nil))
    );
}

#[test]
fn test_parse_identifier() {
    let result = parse_identifier("my_variable123");
    assert_eq!(
        result.unwrap(),
        ("", LuaParserValue::KeyValue(LuaKeyValue::Identifier(String::from("my_variable123"))))
    );
}

#[test]
fn test_parse_table() {
    let mut table = HashMap::new();
    table.insert(
        LuaKeyValue::String(String::from("key1")),
        LuaParserValue::KeyValue(LuaKeyValue::Number(42)),
    );
    table.insert(
        LuaKeyValue::String(String::from("key2")),
        LuaParserValue::Float(3.14),
    );
    let value = LuaParserValue::Table(table);

    let mut formatted_table = HashMap::new();
    formatted_table.insert(
        LuaKeyValue::String(String::from("key1")),
        LuaParserValue::KeyValue(LuaKeyValue::Number(42)),
    );
    formatted_table.insert(
        LuaKeyValue::String(String::from("key2")),
        LuaParserValue::Float(3.14),
    );

    assert_eq!(value, LuaParserValue::Table(formatted_table));
}
