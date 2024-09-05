use super::{IResult, LuaParserValue, LuaStatement, alt, char, clear_noise, delimited, many0, opt, parse_break, parse_conditional, parse_expression, parse_identifier, parse_newlines, parse_root_statement, parse_spaces, preceded, separated_list0, tag};

pub fn parse_if(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = tag("if")(input)?;
    let (input, ()) = parse_spaces(input)?;
    let (input, condition) = parse_conditional(input)?;
    let (input, _) = tag("then")(input)?;
    let (input, ()) = parse_newlines(input)?;
    let (input, if_statements) = many0(delimited(clear_noise, parse_root_statement, clear_noise))(input)?;
    
    // Optionally parse elseif blocks
    let (input, else_if_statements) = many0(parse_elseif)(input)?;

    // Optionally parse else block
    let (input, else_statements) = opt(parse_else)(input)?;

    let (input, _) = tag("end")(input)?;

    Ok((
        input,
        LuaStatement::If(
            condition,
            if_statements,
            else_if_statements.concat(), // Flatten elseif blocks into the same vector as regular if-statements
            else_statements.map(|s| Box::new(LuaStatement::Else(s))),
        ),
    ))
}

pub fn parse_while(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = tag("while")(input)?;
    let (input, ()) = parse_spaces(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = tag("do")(input)?;
    let (input, ()) = parse_newlines(input)?;
    let (input, body) = many0(delimited(clear_noise, parse_root_statement, clear_noise))(input)?;
    let (input, _) = tag("end")(input)?;
    Ok((input, LuaStatement::While(condition, body)))
}

pub fn parse_numeric_for(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = tag("for")(input)?;
    let (input, ()) = parse_spaces(input)?;
    let (input, var) = parse_identifier(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, start) = parse_expression(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, end) = parse_expression(input)?;

    // Optional step
    let (input, step) = opt(preceded(tag(","), parse_expression))(input)?;

    let (input, _) = tag("do")(input)?;
    let (input, ()) = parse_newlines(input)?;
    let (input, body) = many0(delimited(clear_noise, parse_root_statement, clear_noise))(input)?;
    let (input, _) = tag("end")(input)?;
    
    Ok((
        input,
        LuaStatement::For(
            LuaParserValue::NumericFor(
                var.expect_key_value(), 
                Box::new(start), 
                Box::new(end), 
                step.map(Box::new),
            ),
            body,
        ),
    ))
}

pub fn parse_generic_for(input: &str) -> IResult<&str, LuaStatement> {
    let (input, _) = tag("for")(input)?;
    let (input, ()) = parse_spaces(input)?;
    let (input, var) = parse_identifier(input)?;
    let (input, _) = tag("in")(input)?;
    let (input, iterable) = parse_expression(input)?;
    let (input, _) = tag("do")(input)?;
    let (input, ()) = parse_newlines(input)?;
    let (input, body) = many0(delimited(clear_noise, parse_root_statement, clear_noise))(input)?;
    let (input, _) = tag("end")(input)?;
    
    Ok((
        input,
        LuaStatement::For(
            LuaParserValue::GenericFor(var.expect_key_value(), None),
            body,
        ),
    ))
}

fn parse_elseif(input: &str) -> IResult<&str, Vec<LuaStatement>> {
    let (input, _) = tag("elseif")(input)?;
    let (input, ()) = parse_spaces(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = tag("then")(input)?;
    let (input, ()) = parse_newlines(input)?;
    let (input, statements) = many0(delimited(clear_noise, parse_root_statement, clear_noise))(input)?;
    Ok((input, vec![LuaStatement::If(condition, statements, vec![], None)]))
}

fn parse_else(input: &str) -> IResult<&str, Vec<LuaStatement>> {
    let (input, _) = tag("else")(input)?;
    let (input, ()) = parse_newlines(input)?;
    let (input, statements) = many0(delimited(clear_noise, parse_root_statement, clear_noise))(input)?;
    Ok((input, statements))
}

pub fn parse_assign(input: &str) -> IResult<&str, LuaStatement> {
    let (input, ()) = parse_spaces(input)?;
    let (input, ident) = parse_identifier(input)?;
    let (input, _) = many0(char(' '))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = many0(char(' '))(input)?;
    let (input, value) = parse_expression(input)?;
    Ok((input, LuaStatement::Assign(ident, value)))
}

pub fn parse_function_call(input: &str) -> IResult<&str, LuaStatement> {
    let (input, ()) = parse_spaces(input)?;
    let (input, ident) = parse_identifier(input)?;
    let (input, _) = many0(char(' '))(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, args) = separated_list0(tag(","), parse_expression)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, LuaStatement::FunctionCall(ident, args)))
}

pub fn parse_return(input: &str) -> IResult<&str, LuaStatement> {
    let (input, ()) = parse_spaces(input)?;
    let (input, _) = tag("return")(input)?;
    let (input, ()) = parse_spaces(input)?;
    let (input, value) = parse_expression(input)?;
    Ok((input, LuaStatement::Return(value)))
}

pub fn parse_basic_statement(input: &str) -> IResult<&str, LuaStatement> {
    alt((
        parse_assign,               // Assignment statement
        parse_function_call,        // Function call statement
        parse_return,               // Return statement
        parse_break,                // Break statement
        parse_if,                   // If statement
        parse_while,                // While statement
        parse_numeric_for,          // Numeric for loop
        parse_generic_for,          // Generic for loop
    ))(input)
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
    let (input, ()) = parse_newlines(input)?;
    let (input, statements) = many0(delimited(clear_noise, parse_basic_statement, clear_noise))(input)?;
    let (input, ()) = parse_newlines(input)?;
    let (input, _) = tag("end")(input)?;
    Ok((input, LuaParserValue::Function(name.to_string(), statements, args)))//statements, args)))
}

pub fn parse_function_definition_as_statement(input: &str) -> IResult<&str, LuaStatement> {
    let (input, function) = parse_function_definition(input)?;
    Ok((input, function.as_assign().unwrap()))
}
