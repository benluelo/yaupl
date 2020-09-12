#![feature(or_patterns)]

pub mod ast;
pub mod token;
pub mod types;
pub mod constants;

use token::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let token_stream = Tokenizer::new(args[1].to_string()).tokenize();

    println!("{:#?}\n{}", token_stream, Tokenizer::print(&token_stream));
}
