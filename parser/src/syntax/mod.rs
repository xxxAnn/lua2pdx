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
//! Root: <{Compilables}>
//! 
//! Compilable:
//! Assignment: <Name> <Equal> <{Resolvable}>
//! Function Definition: <Function> <Name> <LeftParen> <{Name List}> <RightParen> <{Compilables}> <End>
//! Function Call: <Name> <LeftParen> <{Simple List}> <RightParen>

use crate::lexic::Token;
use crate::lexic::token::{Keyword, Literal};
use crate::lexic::TokenStream;

mod grammar;

use grammar::{StatementBuilder, Statement};

pub use grammar::{Grammar, RuleBuilder};

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken(Token),
    UnexpectedEnd,
}

pub struct SyntaxParser {
    tokens: TokenStream,
    current: Option<Token>,
    grammar: Grammar,
}

pub struct SyntaxTree {
    root: Vec<Statement>,
}

impl SyntaxParser {


    pub fn new(tokens: TokenStream, grammar: Grammar) -> Self {
        Self {
            tokens,
            current: None,
            grammar,
        }
    }

    fn current_token(&mut self) -> Option<Token> {
        self.tokens.current()
    }

    fn to_next_token(&mut self) {
        self.current = None;
    }

    /// Ignore EOS: [<EOS>]*
    fn ignore_eos(&mut self) -> Result<(), SyntaxError> {
        match self.current_token() {
            Some(t) => {
                self.tokens.ignore_eos(t).map_err(|_| SyntaxError::UnexpectedEnd)
            }
            _ => Ok(()),
        }
    }

    fn parse_root(&mut self) -> Result<SyntaxTree, SyntaxError> {
        let mut root = Vec::new();
        while let Some(token) = self.current_token() {
            self.ignore_eos()?;
            root.push(StatementBuilder::new(self.grammar.clone()).build(&mut self.tokens, token));
        }
        Ok(SyntaxTree { root })
    }

    pub fn parse(&mut self) -> Result<SyntaxTree, SyntaxError> {
        self.parse_root()
    }
}
