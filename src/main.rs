pub mod ast;
pub mod token;
pub mod temp;
pub mod types;

use ast::funcs::*;
use token::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let token_stream = Tokenizer::new(args[1].to_string()).tokenize();

    println!("{}", Tokenizer::print(&token_stream));
}
