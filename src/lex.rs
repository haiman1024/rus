use std::io::{BufRead, BufReader, Read};
use std::iter::IntoIterator;
use std::vec::IntoIter;

use super::data::{Locatable, Location, Token};

pub struct Lexer<'a, R: Read> {
    location: Location<'a>,
    reader: BufReader<R>,
    iterator: IntoIter<char>,
    current: Option<char>,
}

impl<'a, R: Read> Lexer<'a, R> {
    pub fn new(filename: &'a str, stream: BufReader<R>) -> Lexer<'a, R> {
        Lexer {
            location: Location {
                line: 0,
                column: 0,
                file: filename,
            },
            reader: stream,
            iterator: Vec::new().into_iter(),
            current: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.current {
            self.current = None;
            Some(c)
        } else {
            match self.iterator.next() {
                Some(c) => {
                    self.location.column += 1;
                    Some(c)
                }
                None => {
                    let mut buf: String = String::new();
                    if let Err(msg) = self.reader.read_line(&mut buf) {
                        eprintln!("FATAL: Error reading line: {}", msg);
                        return None;
                    }
                    self.location.line += 1;
                    self.location.column = 1;
                    self.iterator = buf.chars().collect::<Vec<_>>().into_iter();
                    self.iterator.next()
                }
            }
        }
    }
    fn unput(&mut self, c: Option<char>) {
        self.current = c;
    }
    fn peek(&mut self) -> Option<char> {
        self.current = self.next_char();
        self.current
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }
    fn parse_i64(&mut self) -> Result<Token, String> {
        let mut current: i64 = 0;
        let mut digits = std::iter::from_fn(|| match self.peek() {
            Some(c) if c.is_ascii_digit() => {
                self.next_char();
                Some(c as i64 - '0' as i64)
            }
            _ => None,
        });

        while let Some(digit) = digits.next() {
            match current.checked_mul(10).and_then(|c| c.checked_add(digit)) {
                Some(c) => {
                    current = c;
                }
                None => {
                    while digits.next().is_some() {}
                    return Err(String::from("Overflow while parsing integer literal"));
                }
            }
        }

        Ok(Token::I64(current))
    }
    fn parse_plus_family(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::PlusEqual)
            }
            _ => Ok(Token::Plus),
        }
    }
    fn parse_minus_family(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::MinusEqual)
            }
            _ => Ok(Token::Minus),
        }
    }
    fn parse_equal_family(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::EqualEqual)
            }
            _ => Ok(Token::Equal),
        }
    }
    fn parse_star_family(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::StarEqual)
            }
            _ => Ok(Token::Star),
        }
    }
    fn parse_divide_family(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::DivideEqual)
            }
            _ => Ok(Token::Divide),
        }
    }
}

impl<'a, R: Read> Iterator for Lexer<'a, R> {
    // option: whether the stream is exhausted
    // result: whether the next lexeme is an error
    type Item = Locatable<'a, Result<Token, String>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.skip_whitespace();

            let c = self.next_char()?;
            let location = self.location.clone();

            let data = match c {
                '+' => self.parse_plus_family(),
                '-' => self.parse_minus_family(),
                '=' => self.parse_equal_family(),
                '*' => self.parse_star_family(),
                '/' => self.parse_divide_family(),
                '0'..='9' => {
                    self.unput(Some(c));
                    self.parse_i64()
                }
                _ => Err(String::from("unknown character")),
            };

            return Some(Self::Item { location, data });
        }
    }
}
