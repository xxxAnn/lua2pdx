use super::*;

pub fn parse_break(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = tag("break")(input)?;
    Ok((input, LuaStatement::Break))
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

pub fn parse_conditional_recursive(input: &str, lhs: LuaParserValue) -> IResult<&str, LuaParserValue> {
    let (input, _) = opt(parse_spaces)(input)?;

    let (input, op) = opt(alt((
        tag("=="),
        tag("~="),    // Not equal
        tag("<="),
        tag(">="),
        tag("<"),
        tag(">"),
        tag("and"),
        tag("or"),
    )))(input)?;

    if let Some(op) = op {
        let (input, _) = opt(parse_spaces)(input)?;

        let (input, rhs) = parse_primary_expression(input)?;

        let result = LuaParserValue::Conditional(Box::new(lhs), op.to_string(), Box::new(rhs));
        parse_conditional_recursive(input, result) // Recursively handle chained conditionals
    } else {
        Ok((input, lhs))
    }
}

pub fn parse_conditional(input: &str) -> IResult<&str, LuaParserValue> {
    let (input, lhs) = parse_primary_expression(input)?;
    parse_conditional_recursive(input, lhs)
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
