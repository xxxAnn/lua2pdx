#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Name(String),
    Literal(Literal),
    EOS
}

impl Token {
    pub fn from_str(s: &str) -> Token {
        if let Some(keyword) = Keyword::from_str(s) {
            Token::Keyword(keyword)
        }
        else if let Some(literal) = Literal::from_str(s) {
            Token::Literal(literal)
        }
        else {
            Token::Name(s.to_string())
        }
    }

    pub fn eos() -> Token {
        Token::EOS
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

#[derive(Debug, Clone)]
pub enum Operator {
    
}

#[derive(Debug, Clone)]
pub enum ControlFlow {
x
}