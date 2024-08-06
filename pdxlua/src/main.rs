use std::env;
use std::fs;

use parser::Lexicalizer;
use parser::SyntaxParser;
use parser::Grammar;
use parser::RuleBuilder;
use parser::Keyword;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: pdxlua <filename>");
        return;
    }

    let mut grammar = Grammar::new();

    grammar.add_rule(
        RuleBuilder::new("_FUNCTION_CALL")
            .add_name("_FUNCTION_NAME")
            .add_keyword("K_LEFTPAREN", Keyword::LeftParen)
            // Should be a resolable not just name but TEMPORARY
            // Will be replaced with a more complex rule
            .add_repeat("args", RuleBuilder::new("args").add_name("arg").add_keyword(",", Keyword::Comma))
            .add_keyword("K_RIGHTPAREN", Keyword::RightParen)
    );

    grammar.add_rule(
        RuleBuilder::new("_RED_HERRING")
            .add_name("_FUNCTION_NAME")
            .add_keyword("K_LEFTBRACKET", Keyword::LeftBracket)
    );
     
    let token_stream = Lexicalizer::new(fs::read_to_string(&args[1]).unwrap()).into_iter();

    let mut parser = SyntaxParser::new(token_stream, grammar);

    parser.parse();

}
