use std::{collections::HashMap, mem::Discriminant};
use log::{debug, error, log_enabled, info, Level};

/// Token represents a single token in the language.
/// It contains the line and character position of the token in the source code
/// and the type of the token.
#[derive(Debug, Clone)]
pub struct Token {
    line: usize,
    char: usize,
    ttype: TokenType
}

#[derive(Debug, Clone)]
pub  enum TokenType {
    Keyword(Keyword),
    Name(String),
    Literal(Literal),
    EOS
}

impl Token {
    pub fn from_str(s: &str, line: usize, char: usize) -> Token {
        if let Some(keyword) = Keyword::from_str(s) {
            Token {
                line,
                char,
                ttype: TokenType::Keyword(keyword)
            }
        }
        else if let Some(literal) = Literal::from_str(s) {
            Token {
                line,
                char,
                ttype: TokenType::Literal(literal)
            }
        }
        else {
            Token {
                line,
                char,
                ttype: TokenType::Name(s.to_string())
            }
        }
    }

    pub fn as_discriminant(&self) -> Discriminant<TokenType> {
        std::mem::discriminant(&self.ttype)
    }

    pub fn keyword_discr() -> Discriminant<TokenType> {
        std::mem::discriminant(&TokenType::Keyword(Keyword::Equal))
    }

    pub fn name_discr() -> Discriminant<TokenType> {
        std::mem::discriminant(&TokenType::Name("".to_string()))
    }

    pub fn literal_discr() -> Discriminant<TokenType> {
        std::mem::discriminant(&TokenType::Literal(Literal::String("".to_string())))
    }

    pub fn eos_discr() -> Discriminant<TokenType> {
        std::mem::discriminant(&TokenType::EOS)
    }

    pub fn eos(line: usize, char: usize) -> Token {
        Token {
            line,
            char,
            ttype: TokenType::EOS
        }
    }

    pub fn as_keyword(&self) -> Option<&Keyword> {
        match &self.ttype {
            TokenType::Keyword(k) => Some(k),
            _ => None
        }
    }
}

/// TokenLiteral represents the different types of literals that can be
/// represented in the language.
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil
}

impl Literal {
    fn from_str(s: &str) -> Option<Literal> {
        if s.starts_with("\"") && s.ends_with("\"") {
            if s.len() == 1 {
                return None
            }
            if s.len() == 2 {
                return None
            }
            Some(Literal::String(s[1..s.len()-1].to_string()))
        }
        else if s == "true" {
            Some(Literal::Boolean(true))
        }
        else if s == "false" {
            Some(Literal::Boolean(false))
        }
        else if s == "nil" {
            Some(Literal::Nil)
        }
        else {
            match s.parse::<f64>() {
                Ok(n) => Some(Literal::Number(n)),
                Err(_) => None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Equal,
    Require,
    Execute,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Comma,
    End,
    Do,
    Then,
    Pdx, // Pdxscript
    Function, // Regular function
    // Control Flow
    If,
    Else,
    While,
    For,
    Return,
    // Operator
    Plus,
    Minus,
    Multiply,
    Divide
}

impl Token {
    pub fn from_discriminant(discr: &Discriminant<TokenType>) -> String {
        let mut discr_to_token = HashMap::new();

        discr_to_token.insert(Token::keyword_discr(), "Keyword".to_string());
        discr_to_token.insert(Token::name_discr(), "Name".to_string());
        discr_to_token.insert(Token::literal_discr(), "Literal".to_string());
        discr_to_token.insert(Token::literal_discr(), "EOS".to_string());
        
        discr_to_token.get(discr).unwrap().clone()
    }
}




impl Keyword {
    pub fn from_discriminant(discr: &Discriminant<Keyword>) -> Keyword {
        let mut discr_to_keyword = HashMap::new();

        for keyword in vec![
            Keyword::Equal,
            Keyword::Require,
            Keyword::Execute,
            Keyword::LeftParen,
            Keyword::RightParen,
            Keyword::LeftCurly,
            Keyword::RightCurly,
            Keyword::LeftBracket,
            Keyword::RightBracket,
            Keyword::Comma,
            Keyword::End,
            Keyword::Do,
            Keyword::Then,
            Keyword::Pdx,
            Keyword::Function,
            Keyword::If,
            Keyword::Else,
            Keyword::While,
            Keyword::For,
            Keyword::Return,
            Keyword::Plus,
            Keyword::Minus,
            Keyword::Multiply,
            Keyword::Divide
        ] {
            discr_to_keyword.insert(std::mem::discriminant(&keyword), keyword);
        }

        discr_to_keyword.get(discr).unwrap().clone()
    }
}

impl Keyword {
    
    fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "=" => Some(Keyword::Equal),
            "require" => Some(Keyword::Require),
            "execute" => Some(Keyword::Execute),
            "do" => Some(Keyword::Do),
            "then" => Some(Keyword::Then),
            "pdx" => Some(Keyword::Pdx),
            "end" => Some(Keyword::End),
            "function" => Some(Keyword::Function),
            "," => Some(Keyword::Comma),
            "+" => Some(Keyword::Plus),
            "-" => Some(Keyword::Minus),
            "*" => Some(Keyword::Multiply),
            "/" => Some(Keyword::Divide),
            "(" => Some(Keyword::LeftParen),
            ")" => Some(Keyword::RightParen),
            "{" => Some(Keyword::LeftCurly),
            "}" => Some(Keyword::RightCurly),
            "[" => Some(Keyword::LeftBracket),
            "]" => Some(Keyword::RightBracket),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "return" => Some(Keyword::Return),
            _ => None
        }
    }

}