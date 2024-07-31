use std::io::Lines;

mod token;

use token::Token;


pub struct Lexicalizer {
    input: Vec<String>,
    current_line: usize,
    stack: Vec<String>
}
impl Lexicalizer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexicalizer {
            input: input.lines().map(|s| s.to_string()).collect(),
            current_line: 0,
            stack: Vec::new()
        };

        lex.update_stack();

        lex
    }

    fn update_stack(&mut self) {
        self.stack.extend(self.input[self.current_line].split_whitespace().map(|s| s.to_string()));
        self.stack.reverse();
        //println!("{:?}", self.input[self.current_line].split_whitespace().map(|s| s.to_string()));
    }

    fn sanitize(&mut self, txt: &str) -> String {
        //println!("{:?}", self.stack);
        if txt.starts_with('(') && txt.len() > 1 {
            self.stack.push(txt[1..].to_string());
            return "(".to_string()
        } else if txt.ends_with(')') && txt.len() > 1 {
            self.stack.push(")".to_string());
            return self.sanitize(&txt[..txt.len()-1].to_string())
        } else if (!txt.starts_with('"') || !txt.ends_with('"')) && txt.contains('(') && txt.len() > 1{
            let split = txt.split('(').collect::<Vec<&str>>();
            self.stack.push(split[1].to_string());
            self.stack.push("(".to_string());
            return split[0].to_string()
        }
        else {
            return txt.to_string()
        }
    }

    pub fn lexicalize(&mut self) -> Option<Token> {
        if self.current_line >= self.input.len() || (self.current_line == self.input.len()-1 && self.stack.is_empty()) {
            return None
        }
        if self.stack.is_empty() {
            self.current_line += 1;
            //println!("Line: {}", self.current_line);
            self.update_stack();
            return Some(Token::eos())
        } else {
            //println!("{:?}", self.stack);
            let text = self.stack.pop().unwrap_or(String::from("."));
            let token = Token::from_str(&self.sanitize(&text));
            return Some(token)
        }
    }
}

impl Iterator for Lexicalizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexicalize()
    }
}