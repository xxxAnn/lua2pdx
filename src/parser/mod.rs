use nom::{
    branch::alt, bytes::complete::{tag, take_while1}, character::complete::{char, digit1}, combinator::{opt, recognize}, multi::{many0, separated_list0}, sequence::{delimited, preceded, tuple}, IResult
};

mod ast;
mod expressions;
mod utils;
mod statements;
pub use ast::*;
pub use expressions::*;
pub use utils::*;
pub use statements::*;


pub fn parse_root_statement(input: &str) -> IResult<&str, LuaStatement> {
    alt((parse_basic_statement, parse_function_definition_as_statement))(input)
}

pub fn parse_root_statements(input: &str) -> IResult<&str, Vec<LuaStatement>> {
    many0(preceded(opt(clear_noise), parse_root_statement))(input)
}
