use std::fs::read_to_string;

use parse::parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);
    let s = read_to_string("$data/test.y").unwrap();
    let res = parse(&s);
    println!("{:#?}", res);
}
