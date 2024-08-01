use std::env;
use std::fs;

use parser::Lexicalizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: pdxlua <filename>");
        return;
    }
     
    for x in Lexicalizer::new(fs::read_to_string(&args[1]).unwrap()) {
        println!("{:?}", x);
    }
}
