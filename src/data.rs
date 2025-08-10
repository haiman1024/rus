#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    Divide,
    I64(i64),
}

// holds where a piece of code came from
// should almost always be a immutable reference
#[derive(Clone, Debug)]
pub struct Location<'a> {
    pub line: usize,
    pub column: usize,
    pub file: &'a str,
}

#[derive(Debug)]
pub struct Locatable<'a, T> {
    pub location: Location<'a>,
    pub data: T,
}

pub type Token<'a> = Locatable<'a, TokenType>;
pub type Error<'a> = Locatable<'a, String>;
