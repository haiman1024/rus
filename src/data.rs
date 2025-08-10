#[derive(Debug)]
pub enum Token {
    PlusEqual,
    MinusEqual,
    StarEqual,
    DivideEqual,
    EqualEqual,
    Plus,
    Minus,
    Star,
    Divide,
    Equal,
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
