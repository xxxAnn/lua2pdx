use super::{IResult, alt, char, many0};

pub fn parse_space(input: &str) -> IResult<&str, ()> {
    let (input, _) = char(' ')(input)?;
    Ok((input, ()))
}

pub fn parse_spaces(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(parse_space)(input)?;
    Ok((input, ()))
}

pub fn parse_newline(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((char('\n'), char('\r')))(input)?;
    Ok((input, ()))
}

pub fn parse_newlines(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(parse_newline)(input)?;
    Ok((input, ()))
}

pub fn clear_noise(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(alt((parse_space, parse_newline)))(input)?;
    Ok((input, ()))
}