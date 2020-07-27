pub mod ast;
pub mod token;

use ast::funcs::*;
use token::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut token_stream = Tokenizer::new(args[1].to_string()).tokenize();

    println!("{}", Tokenizer::print(&token_stream));

    println!("{:#?}", match_with(&mut token_stream));
}
