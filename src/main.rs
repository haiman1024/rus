use std::io::{self, BufReader};

use lex::Lexer;

mod data;
mod lex;

fn main() {
    for token in Lexer::new("<stdin>", BufReader::new(io::stdin())) {
        println!("{:?}", token);
    }
}
