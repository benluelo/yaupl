#![feature(or_patterns)]

pub mod ast;
pub mod token;
pub mod types;
pub mod constants;
pub mod fuzzer;

use token::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // let token_stream = Tokenizer::new(args[1].to_string()).tokenize();
    let random_tokens = fuzzer::fuzz(200);
    println!("{}", random_tokens.0);
    let token_stream = Tokenizer::from_string(random_tokens.0.as_str()).tokenize();

    println!("{:#?}\n{}", token_stream, Tokenizer::print(&token_stream));
}
