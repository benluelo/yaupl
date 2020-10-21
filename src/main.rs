#![feature(or_patterns, trait_alias, associated_type_bounds)]

use std::fs::read_to_string;

use crate::parse::r#type;

mod ast;
mod fuzzer;
mod parse;
mod types;

// use yaupl::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);
    let s = read_to_string("test.y").unwrap();
    let res = r#type(&s, (0usize, 0usize).into());
    println!("{:#?}", res);
}

// fn main() {
//     let args: Vec<String> = std::env::args().collect();

//     if args.len() == 2 {
//         let token_stream = Tokenizer::new(args[1].to_string()).tokenize();
//         println!("{:#?}\n{}", token_stream, Tokenizer::print(&token_stream));
//     } else {
//         let random_tokens = fuzzer::fuzz(20);
//         println!("{}", random_tokens.0);
//         let token_stream = Tokenizer::from_string(random_tokens.0.as_str()).tokenize();

//         println!("{:#?}\n{}", token_stream, Tokenizer::print(&token_stream));
//     }
// }
