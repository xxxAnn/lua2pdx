pub struct Arranger {
    stack: Vec<String>,
    special_chars: Vec<char>
}

impl Arranger {

    pub fn new(special_chars: &[char]) -> Self {
        Arranger {
            stack: Vec::new(),
            special_chars: special_chars.to_owned()
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
        if txt.len() > 1 {
            for chr in self.special_chars.clone() {
                if txt.contains(chr) {
                    let split = txt.splitn(2, chr).collect::<Vec<&str>>();
                    //println!("\n{:?}", txt);
                    //println!("{:?}", split);
                    let after = split[1..].join("");
                    if after.len() > 0 {
                        self.stack.push(after);
                    }
                    //println!("");
                    self.stack.push(chr.to_string());
                    println!("{:?}", split[0]);
                    return self.separate_special_chars(&split[0].to_string())
                }
            }
        }

        return txt.to_string()
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

