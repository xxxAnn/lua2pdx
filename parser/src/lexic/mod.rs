pub mod token;
mod arranger;

pub use token::Token;
use arranger::Arranger;


pub struct Lexicalizer {
    input: Vec<String>,
    current_line: usize,
    arranger: Arranger
}

impl Lexicalizer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexicalizer {
            input: input.lines().map(|s| s.to_string()).collect(),
            current_line: 0,
            arranger: Arranger::new(&[',', '(', ')', '[', ']', '#'])
        };

        lex.update_stack();

        lex
    }

    fn update_stack(&mut self) {
        self.arranger.set_stack(self.input[self.current_line].split_whitespace().map(|s| s.to_string()).collect());
        //println!("{:?}", self.input[self.current_line].split_whitespace().map(|s| s.to_string()));
    }

    fn arrange(&mut self, txt: &str) -> String {
        self.arranger.arrange(txt)
    }

    fn empty_stack(&mut self) {
        self.arranger.set_stack(Vec::new());
    }

    pub fn lexicalize(&mut self) -> Option<Token> {
        if self.current_line >= self.input.len() || (self.current_line == self.input.len()-1 && self.arranger.is_empty()) {
            return None
        }
        if self.arranger.is_empty() {
            self.current_line += 1;
            //println!("Line: {}", self.current_line);
            self.update_stack();
            return Some(Token::eos())
        } else {
            //println!("{:?}", self.stack);
            let text = self.arranger.pop().unwrap_or(String::from("."));
            let arranged = self.arrange(&text);
            if arranged == "" {
                return None
            }
            if arranged == "#" {
                self.empty_stack();
                return None
            }
            let token = Token::from_str(&arranged);
            //println!("{:?}", self.stack);
            return Some(token)
        }
    }
}

pub struct TokenStream {
    lex: Lexicalizer,
    ctoken: Option<Token>
}

impl IntoIterator for Lexicalizer {
    type Item = Token;
    type IntoIter = TokenStream;

    fn into_iter(self) -> Self::IntoIter {
        TokenStream {
            lex: self,
            ctoken: None
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex.lexicalize()
    }
}

impl TokenStream {
    pub fn ignore_eos(&mut self, current_token: Token) -> Result<(), &'static str> {
        let mut ctoken = Some(current_token);
        while let Some(token) = ctoken.clone() {
            if std::mem::discriminant(&token) != std::mem::discriminant(&Token::EOS) {
                break
            }
            ctoken = self.next()
        }
        if ctoken.is_none() {
            Err("No more tokens")
        } else {
            Ok(())
        }
    }

    pub fn current(&mut self) -> Option<Token> {
        if let Some(token) = &self.ctoken {
            Some(token.clone())
        } else {
            self.ctoken = self.next();
            self.ctoken.clone().map(|t| t.clone())
        }
    }

    pub fn advance(&mut self) {
        self.ctoken = None;
    }
}

