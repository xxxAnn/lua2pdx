//! Grammar module
//! List of Grammars
//! 
//! Legend:
//! <Expr>
//!   In an expr:
//!     {Complex Expr}
//!  [List]
//!  Repeat*
//! 
//! Building Blocks:
//! Ignore EOS: [<EOS>]*
//! Name List: [<Name> <Comma> <{Ignore EOS}>]*
//! Simple List: [<Name/Literal> <Comma> <{Ignore EOS}>]*
//! Assign List: [<Name> <Equal> <Name/Literal> <Comma> <{Ignore EOS}>]*
//! Function Call: <Name> <LeftParen> <{Simple List}> <RightParen>
//! Table Body: <LeftCurly> <{Assign List}> <RightCurly>
//! Resolvable: <Name/Literal/{Function Call}/{Table Body}>
//! 
//! Special:
//! Compilables: [<{Compilable}>]*
//! 
//! Compilable:
//! Root: <{Compilables}>
//! Assignment: <Name> <Equal> <{Resolvable}>
//! Function Definition: <Function> <Name> <LeftParen> <{Name List}> <RightParen> <{Compilables}> <End>

use crate::lexic::Token;
use crate::lexic::token::Literal;
use crate::lexic::TokenStream;

enum AST {
    Root(Vec<AST>),
    Assignment(String, Box<AST>),
    FunctionDefinition(String, Vec<String>, Vec<AST>),
    FunctionCall(String, Vec<AST>),
    TableBody(Vec<(String, AST)>),
    Literal(Literal)
}

#[derive(Debug)]
enum SyntaxError {
    UnexpectedToken(Token),
    UnexpectedEnd,
}

pub struct SyntaxParser {
    tokens: TokenStream,
    current: Option<Token>
}

impl SyntaxParser {

    fn current_token(&self) -> Option<Token> {
        if let Some(token) = self.current {
            Some(token)
        } else {
            self.tokens.next()
        }
    }

    fn to_next_token(&mut self) {
        self.current = None;
    }

    fn consume(&mut self, expected: Token) -> Result<(), ParseError> {
        if let Some(token) = self.current_token() {
            if token == expected {
                self.to_next_token();
            } else {
                Err(ParseError::UnexpectedToken(token.clone()))
            }
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }
    
    fn consume_token
}

