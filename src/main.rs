#![feature(or_patterns)]

pub mod ast;
pub mod token;
pub mod types;

use token::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let token_stream = Tokenizer::new(args[1].to_string()).tokenize();

    println!("{:#?}, {}", token_stream, Tokenizer::print(&token_stream));
}
