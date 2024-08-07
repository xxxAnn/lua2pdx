use log::{debug, error, log_enabled, info, Level};

pub struct Arranger {
    stack: Vec<String>,
    special_chars: Vec<String>
}

impl Arranger {

    pub fn new(special_chars: Vec<impl Into<String>>) -> Self {
        Arranger {
            stack: Vec::new(),
            special_chars: special_chars.into_iter().map(|s| s.into()).collect::<Vec<_>>()
        } 
    }

    pub fn set_stack(&mut self, v: Vec<String>) {
        self.stack.extend(v);
        self.stack.reverse();
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn separate_special_chars(&mut self, txt: &str) -> String {
        debug!("{} in {:?}? {}", txt, &self.special_chars, self.special_chars.iter().any(|s| txt == s));
        if txt.len() > 1 && !self.special_chars.iter().any(|s| txt == s) {
            for chr in self.special_chars.clone() {
                if txt.contains(&chr) {
                    let split = txt.splitn(2, &chr).collect::<Vec<&str>>();
                    debug!("Separating special characters in {:?}", txt);
                    debug!("Got {:?}", split);
                    let after = split[1..].join("");
                    if after.len() > 0 {
                        self.stack.push(after);
                    }
                    self.stack.push(chr.to_string());
                    debug!("Now parsing {:?}", split[0]);
                    return self.separate_special_chars(&split[0].to_string())
                }
            }
        }

        return txt.to_string()
    }

    pub fn get_stack(&self) -> Vec<String> {
        self.stack.clone()
    }

    fn generate_multi_token_literal(&mut self, txt: &str) -> String {
        let mut buf = txt.to_string();
        while !buf.ends_with('"') || txt.len() == 1 {
            buf.push_str(&format!(" {}", self.stack.pop().unwrap_or(String::new())));
        }
        return buf
    }

    pub fn arrange(&mut self, txt: &str) -> String {

        let mut modif = txt.to_string();
        if txt.len() > 1 {
            modif = self.separate_special_chars(txt)
        }
        if modif.starts_with('"') && (!modif.ends_with('"') || modif.len() == 1) {
            return self.generate_multi_token_literal(txt)
        }


        return modif.to_string()
    }

    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }
}

