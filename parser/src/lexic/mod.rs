mod token;
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
            arranger: Arranger::new(&[',', '(', ')', '[', ']'])
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
            let token = Token::from_str(&arranged);
            //println!("{:?}", self.stack);
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