use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::process;

use lex::Lexer;

mod data;
mod lex;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // 没有参数，从标准输入读取
        1 => {
            for token in Lexer::new("<stdin>", BufReader::new(io::stdin())) {
                match token.data {
                    Ok(t) => println!("{:?}: {:?}", token.location, t),
                    Err(e) => {
                        eprintln!(
                            "Error at {}:{}:{}: {}",
                            token.location.file, token.location.line, token.location.column, e
                        );
                        process::exit(1);
                    }
                }
            }
        }
        // 一个参数，作为文件名处理
        2 => {
            let filename = &args[1];
            match File::open(filename) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    for token in Lexer::new(filename, reader) {
                        match token.data {
                            Ok(t) => println!("{:?}: {:?}", token.location, t),
                            Err(e) => {
                                eprintln!(
                                    "Error at {}:{}:{}: {}",
                                    token.location.file,
                                    token.location.line,
                                    token.location.column,
                                    e
                                );
                                process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to open file '{}': {}", filename, e);
                    process::exit(1);
                }
            }
        }
        // 参数过多
        _ => {
            eprintln!("Usage: {} [filename]", args[0]);
            process::exit(1);
        }
    }
}
