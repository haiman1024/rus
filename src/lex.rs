use std::iter::Iterator;

use super::data::{Error, Locatable, Location, Token, TokenType};

pub type Lexer<'a> = Locatable<'a, &'a mut dyn Iterator<Item = char>>;

impl<'a> Lexer<'a> {
    pub fn new(filename: &'a str, stream: &'a mut dyn Iterator<Item = char>) -> Lexer<'a> {
        Lexer {
            location: Location {
                line: 1,
                column: 1,
                file: filename,
            },
            data: stream,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.data.next() {
            Some(_c) => TokenType::Plus,
            None => return None,
        };
        Some(Ok(Token {
            location: self.location.clone(),
            data: token,
        }))
    }
}
