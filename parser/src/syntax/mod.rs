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
//! Raw Function: <Function> <LeftParen> <{Name List}> <RightParen> <{Compilables}> <End>
//! Resolvable: <Name/Literal/{Function Call}/{Table Body}/{Raw Function}>
//! 
//! Special:
//! Compilables: [<{Compilable}>]*
//! 
//! Compilable:
//! Root: <{Compilables}>
//! Assignment: <Name> <Equal> <{Resolvable}>
//! Function Definition: <Function> <Name> <LeftParen> <{Name List}> <RightParen> <{Compilables}> <End>
//! Function Call: <Name> <LeftParen> <{Simple List}> <RightParen>

use std::string::ParseError;

use crate::lexic::Token;
use crate::lexic::token::Literal;
use crate::lexic::TokenStream;

enum AST {
    Assignment(String, Box<AST>), 
    // Created by <Name> <Equal> <{Raw Function}> as well
    FunctionDefinition(String, Vec<String>, Vec<AST>),
    FunctionCall(String, Vec<AST>),
    TableBody(Vec<(String, AST)>),
    Variable(String),
    Literal(Literal)
}

pub struct SyntaxTree {
    root: Vec<AST>
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

    /// Ignore EOS: [<EOS>]*
    fn ignore_eos(&mut self) {
        while let Some(token) = self.current_token() {
            if token != Token::EOS {
                break
            }
            self.to_next_token();
        }
    }
}

