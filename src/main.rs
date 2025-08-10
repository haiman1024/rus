use std::io::{self, BufRead, BufReader, Read};
use std::iter::Iterator;

use lex::Lexer;

mod data;
mod lex;

fn main() {
    let reader = BufReader::new(io::stdin());
    let mut buf_char_reader = BufCharReader::new(reader);
    for token in Lexer::new("<stdin>", &mut buf_char_reader) {
        println!("{:?}", token);
    }
}

struct BufCharReader<R: Read> {
    lines: std::io::Lines<BufReader<R>>,
    current_line: Option<std::str::Chars<'static>>,
}

impl<R: Read> BufCharReader<R> {
    fn new(reader: BufReader<R>) -> Self {
        BufCharReader {
            lines: reader.lines(),
            current_line: None,
        }
    }
}

impl<R: Read> Iterator for BufCharReader<R> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut chars) = self.current_line {
                if let Some(ch) = chars.next() {
                    return Some(ch);
                }
            }

            match self.lines.next() {
                Some(Ok(line)) => {
                    // 注意：这里使用了 unsafe，因为在实际项目中应该使用更安全的方法
                    // 这里为了简化实现，使用 unsafe 将 String 转换为 &'static str
                    let line_str: &'static str = unsafe { std::mem::transmute(line.as_str()) };
                    self.current_line = Some(line_str.chars());
                }
                Some(Err(_)) => return None,
                None => return None,
            }
        }
    }
}
