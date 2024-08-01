//! Grammar module
//! List of Grammars
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

pub type GrammarType = Box<dyn Grammar>;

pub trait Grammar {
    fn for_each_grammar(&mut self, f: Box<dyn FnMut(&mut GrammarType) -> bool>) ;
    fn searching_for(&self, tokens: &[Token]) -> bool; 
    fn consume(&mut self, tokens: &mut Vec<Token>);
    fn try_do(&mut self, tokens: &[Token]) -> bool {
        // Go through each sub grammar
        // If the sub grammar is searching for the top token
        // Have the sub grammar consume the top tokens
        // If the sub grammar is not searching for the top tokens
        // Return false
        let mut t = tokens.to_vec();

        self.for_each_grammar(Box::new(move |grammar: &mut GrammarType| {
            if grammar.searching_for(&t) {
                grammar.consume(&mut t);
                true
            } else {
                false
            }
        }));
        true
    }
    fn is_token(&self, token: &Token) -> bool;
}

pub struct Or {
    grammars: Vec<GrammarType>,
}

impl<'a> Grammar for Or {
    fn searching_for(&self, tokens: &[Token]) -> bool {
        self.grammars.iter().any(|g| g.searching_for(tokens))
    }

    fn consume(&mut self, tokens: &mut Vec<Token>) {
        for grammar in self.grammars.iter_mut() {
            if grammar.searching_for(tokens) {
                grammar.consume(tokens);
            }
        }
    }

    fn is_token(&self, token: &Token) -> bool {
        false 
    }
    
    fn for_each_grammar(&mut self, mut f: Box<dyn FnMut(&mut GrammarType) -> bool>) {
        for grammar in self.grammars.iter_mut() {
            if !f(grammar) { break }
        }
    }
}