use std::mem::Discriminant;

use crate::lexic::{token::Keyword, Token, TokenStream, TokenType};

use std::collections::HashMap;
use std::fmt::Formatter;
use std::fmt::Debug;

use log::{debug, error, log_enabled, info, Level};

#[derive(Clone, Debug)]
pub struct Grammar {
    rules: Vec<Rule>
}

#[derive(Clone, Debug)]
struct Command {
    name: String,
    ctype: CommandType,
}

#[derive(Clone, Debug)]
pub enum CommandType {
    None(Discriminant<TokenType>),
    Keyword(Discriminant<Keyword>),
    Get(String), // Name or Literal
    Repeat(Vec<Command>), 
    Or(Vec<Command>),
}

pub struct StatementBuilder {
    grammar: Grammar,
    data: HashMap<String, Vec<StatementData>>,
    saved_tokens: Vec<Token>,
    rule_pool: Vec<Rule>,
    token_index: usize
}

pub struct Statement {
    rule: String,
    data: HashMap<String, Vec<StatementData>>
}

enum StatementData {
    Token(Token),
    Statement(Statement)
}

pub struct RuleBuilder {
    name: String,
    rule: Vec<Command>,
}

#[derive(Clone, Debug)]
struct Rule {
    name: String,
    rule: Vec<Command>,
    data: HashMap<String, Vec<Token>>,
    food_index: (usize, usize)
}

enum TokenMatch {
    Token(Discriminant<TokenType>),
    Keyword(Discriminant<Keyword>)
}

impl Command {

    fn matches_token(&self, token: &Token) -> bool {
        match &self.ctype {
            CommandType::None(discriminant) => {
                *discriminant == token.as_discriminant()
            },
            CommandType::Keyword(discriminant) => {
                match token.as_keyword() {
                    Some(keyword) => {
                        *discriminant == std::mem::discriminant(keyword)
                    },
                    None => false
                }
            },
            CommandType::Get(_) => {
                token.is_name() || token.is_literal()
            }
            CommandType::Or(v) => {
                v.iter().any(|c| c.matches_token(token))
            },
            CommandType::Repeat(_) => {
                false // Repeat is handled by the Rule struct
            }
        }
    }
}

impl Grammar {
    fn get_rule(&self, name: impl Into<String>) -> Option<&Rule> {
        let nm = name.into();
        self.rules.iter().find(|rule| &rule.name == &nm)
    }

    pub fn new() -> Self {
        Grammar {
            rules: Vec::new()
        }
    }

    pub fn add_rule(&mut self, rule: RuleBuilder) {
        self.rules.push(rule.build());
    }
}

impl RuleBuilder {

    pub fn new(name: impl Into<String>) -> Self {
        RuleBuilder {
            name: name.into(),
            rule: Vec::new(),
        }
    }

    pub fn add_name(mut self, name: impl Into<String>) -> Self{
        self.rule.push(Command {
            name: name.into(),
            ctype: CommandType::None(Token::name_discr())
        });

        self
    }

    pub fn add_keyword(mut self, name: impl Into<String>, keyword: Keyword) -> Self {
        self.rule.push(Command {
            name: name.into(),
            ctype: CommandType::Keyword(std::mem::discriminant(&keyword))
        });

        self
    }

    pub fn add_get(mut self, name: impl Into<String>) -> Self {
        let v = name.into(); 
        self.rule.push(Command {
            name: v.clone(),
            ctype: CommandType::Get(v)
        });

        self
    }

    pub fn add_repeat(mut self, name: impl Into<String>, repeat: RuleBuilder) -> Self {
        self.rule.push(Command {
            name: name.into(),
            ctype: CommandType::Repeat(repeat.rule)
        });

        self
    }

    pub fn add_or(mut self, name: impl Into<String>, rules: RuleBuilder) -> Self {
        self.rule.push(Command {
            name: name.into(),
            ctype: CommandType::Or(rules.rule)
        });

        self
    }

    fn build(self) -> Rule {
        Rule {
            name: self.name,
            rule: self.rule,
            data: HashMap::new(),
            food_index: (0, 0)
        }
    }
}

#[derive(Debug)]
enum GrammarError {

    // Overlapping Signature mean that two rules' first few tokens
    // Before any Repeats, Ors or Gets are the same
    //
    // We can't determine which rule to use without going through the entire rule

    OverlappingSignature
}

impl Rule {

    // Only consider the signature of the rule

    // Signature is defined as:
    // The tokens before any Repeats or Ors
    fn get_possible_tokens_at(&self, index: usize) -> Result<Vec<TokenMatch>, GrammarError> {
        let mut possible_tokens = Vec::new();

        for (i, command) in self.rule.iter().enumerate() {
            if i > index {
                break;
            }
            match command.ctype {
                CommandType::Get(_) | CommandType::Repeat(_) | CommandType::Or(_)  => { 
                    return Err(GrammarError::OverlappingSignature);
                },
                _ => {
                    if i == index {
                        match command.ctype {
                            CommandType::None(token) => {
                                possible_tokens.push(TokenMatch::Token(token));
                            },
                            CommandType::Keyword(keyword) => {
                                possible_tokens.push(TokenMatch::Keyword(keyword));
                            },
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(possible_tokens)
    }

    fn get_current_command(&self) -> Option<&Command> {
        if let Some(c) = self.rule.get(self.food_index.0) {
            match &c.ctype {
                CommandType::Repeat(v) => {
                    info!("Repeating: {:?}, current at index: {:?}", v, self.food_index.1);
                    if let Some(c) = v.get(self.food_index.1 % v.len()) {
                        info!("Got {:?}", c);
                        return Some(c);
                    } else {
                        return None;
                    }
                }
                _ => {
                    return Some(c);
                }
            }
        }
        None
    }

    fn current_command_is_repeat(&self) -> bool {
        // We can't just call get_current_command and check if it's a repeat
        // Because get_current_command will look inside the repeat and return a subcommand
        // We need to check the current command directly
        if let Some(c) = self.rule.get(self.food_index.0) {
            match &c.ctype {
                CommandType::Repeat(_) => {
                    return true;
                },
                _ => {
                    return false;
                }
            }
        }
        false
    }

    fn go_to_next_command(&mut self) {
        //if let Some(c) = self.get_current_command() {
            if self.current_command_is_repeat() {
                self.food_index.1 += 1;
            } else {
                self.food_index.0 += 1;
                self.food_index.1 = 0;
            }
        //}
    }

    /// This takes the token and gets the current command
    /// Then it checks if the token matches the current command
    /// If it doesn't, it increments the food_index.0 and resets food_index.1
    /// This signifies that the repeat has ended and we are moving to the next command
    /// If it does, it does nothing, signifying that the repeat is still going on
    /// Technically food_index.1 should always be 0 when food_index.0 is incremented
    /// Otherwise we are ending a repeat in the middle of it
    fn check_if_repeat_ended(&mut self, token: &Token) {        
        if let Some(c) = self.get_current_command() {
            if c.matches_token(token) {
                return;
            }
        }
        self.food_index = (self.food_index.0 + 1, 0);
    }

    fn eat_token(&mut self, token: &Token) {
        self.check_if_repeat_ended(&token);
        if let Some(c) = self.get_current_command() {
            if c.matches_token(&token) {
                match &c.ctype {
                    CommandType::Get(name) => { self.data.entry(name.clone()).or_insert(Vec::new()).push(token.clone()); },
                    CommandType::None(n) => {
                        if n == &Token::name_discr() {
                            self.data.entry(c.name.clone()).or_insert(Vec::new()).push(token.clone());
                        }
                    }
                    _ => { }
                }
                self.go_to_next_command();
            }
        }
    }

    fn is_satiated(&mut self, token: &Token) -> bool {
        // Either we have reached the end of the rule
        // Or this rule ends with a repeat and we have reached the end of the repeat
        info!("Checking if satiated: {:?} {:?} {:?}", token, self.get_current_command(), self.food_index);
        self.check_if_repeat_ended(token);
        self.get_current_command().is_none()
    }
}

impl TokenMatch {

    fn matches(&self, token: &Token) -> bool {
        match self {
            TokenMatch::Token(discriminant) => {
                *discriminant == token.as_discriminant()
            },
            TokenMatch::Keyword(discriminant) => {
                match token.as_keyword() {
                    Some(keyword) => {
                        *discriminant == std::mem::discriminant(keyword)
                    },
                    None => false
                }
            }
        }
    }
    
}

impl Debug for TokenMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenMatch::Token(discriminant) => {
                write!(f, "Token({:?})", Token::from_discriminant(discriminant))
            },
            TokenMatch::Keyword(discriminant) => {
                write!(f, "Keyword({:?})", Keyword::from_discriminant(discriminant))
            }
        }
    }
}



impl StatementBuilder {
    pub fn new(grammar: Grammar) -> Self {
        let rule_pool = grammar.rules.clone();
        StatementBuilder {
            grammar,
            data: HashMap::new(),
            rule_pool,
            saved_tokens: Vec::new(),
            token_index: 0,
        }
    }

    fn save_token(&mut self, token: Token) {
        self.saved_tokens.push(token);
    }

    fn fits_rule_at(&self, token: Token, rule: &Rule, at_index: usize) -> bool {
        debug!("{} expects {:?} at position {} in signature.", rule.name, rule.get_possible_tokens_at(at_index), at_index);
        debug!("Got {:?}", &token);
        let r = rule.get_possible_tokens_at(at_index).unwrap().iter().any(|t| t.matches(&token));

        if !r {
            debug!("{} is eliminated.", rule.name);
        }

        r
    }   

    fn get_rule(&self) -> Option<&Rule> {
        if self.rule_pool.len() == 1 {
            self.rule_pool.first()
        } else {
            None
        }
    }

    fn trim_rules(&mut self, token: Token) {
        self.rule_pool = self.rule_pool.clone().into_iter().filter(|r| {
            self.fits_rule_at(token.clone(), r, self.token_index)
        }).collect();
    }

    fn select_rule(&mut self, stream: &mut TokenStream, token: Token) {
        while self.rule_pool.len() > 1 {
            debug!("Current token: {:?}", stream.current().unwrap());
            self.save_token(stream.current().unwrap());
            self.trim_rules(stream.current().unwrap());
            self.token_index += 1;
            stream.advance();
        }
    }

    pub fn build(&mut self, token_stream: &mut TokenStream, token: Token) -> Statement {
        self.select_rule(token_stream, token.clone());

        info!("Found rule: {:?}", self.get_rule().map(|r| r.name.clone()));
        info!("Saved tokens: {:?}", &self.saved_tokens.clone()[..(3.min(self.saved_tokens.len()-1))]);
        info!("Current token: {:?}", token_stream.current().unwrap());

        let mut selected_rule = self.get_rule().unwrap().clone();

        for n in self.saved_tokens.iter() {
            selected_rule.eat_token(n);
        }

        while token_stream.current().is_some() && !selected_rule.is_satiated(&token_stream.current().unwrap()) {
            info!("Not satiated yet: {:?}", &token_stream.current().unwrap());
            selected_rule.eat_token(&token_stream.current().unwrap());
            token_stream.advance();
        }
        info!("Satiated: {:?}", &token_stream.current().unwrap());

        dbg!(selected_rule.data.clone());
        //dbg!(selected_rule);

        todo!()
    }

}


// Statements:
//     Assignment: <Name> <Equal> <{Resolvable}>
//     Function Definition: <Function> <Name> <LeftParen> <{Name List}> <RightParen>
//     Function Definition2: <Name> <Equal> <Function> <LeftParen> <{Name List}> <RightParen>
//     Function Call: <Name> <LeftParen> <{Simple List}> <RightParen>
//     Block End: <End>
// Building Blocks:
//     Name List: [<Name> <Comma> <{Ignore EOS}>]*
//     Simple List: [<Name/Literal> <Comma> <{Ignore EOS}>]*
//     Assign List: [<Name> <Equal> <Name/Literal> <Comma> <{Ignore EOS}>]*
//     Function Call: <Name> <LeftParen> <{Simple List}> <RightParen>
//     Table Body: <LeftCurly> <{Assign List}> <RightCurly>
//     Resolvable: <Name/Literal/{Function Call}/{Table Body}/{Raw Function}>
// 
