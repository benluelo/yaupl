pub mod token;
pub mod ast;

use token::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("{}", Tokenizer::print(&Tokenizer::new(args[1].to_string()).tokenize()));
}