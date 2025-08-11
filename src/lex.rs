use std::io::{BufRead, BufReader, Read};
use std::iter::IntoIterator;
use std::vec::IntoIter;

use super::data::{Keyword, Locatable, Location, Token};
use phf::Map;

static KEYWORDS: Map<&'static str, Keyword> = phf::phf_map! {
    // Strict keywords
    "as" => Keyword::As,
    "break" => Keyword::Break,
    "const" => Keyword::Const,
    "continue" => Keyword::Continue,
    "crate" => Keyword::Crate,
    "else" => Keyword::Else,
    "enum" => Keyword::Enum,
    "extern" => Keyword::Extern,
    "false" => Keyword::False,
    "fn" => Keyword::Fn,
    "for" => Keyword::For,
    "if" => Keyword::If,
    "impl" => Keyword::Impl,
    "in" => Keyword::In,
    "let" => Keyword::Let,
    "loop" => Keyword::Loop,
    "match" => Keyword::Match,
    "mod" => Keyword::Mod,
    "move" => Keyword::Move,
    "mut" => Keyword::Mut,
    "pub" => Keyword::Pub,
    "ref" => Keyword::Ref,
    "return" => Keyword::Return,
    "self" => Keyword::SelfValue,
    "Self" => Keyword::SelfType,
    "static" => Keyword::Static,
    "struct" => Keyword::Struct,
    "super" => Keyword::Super,
    "trait" => Keyword::Trait,
    "true" => Keyword::True,
    "type" => Keyword::Type,
    "unsafe" => Keyword::Unsafe,
    "use" => Keyword::Use,
    "where" => Keyword::Where,
    "while" => Keyword::While,

    // Reserved keywords
    "abstract" => Keyword::Abstract,
    "become" => Keyword::Become,
    "box" => Keyword::Box,
    "do" => Keyword::Do,
    "final" => Keyword::Final,
    "macro" => Keyword::Macro,
    "override" => Keyword::Override,
    "priv" => Keyword::Priv,
    "typeof" => Keyword::Typeof,
    "unsized" => Keyword::Unsized,
    "virtual" => Keyword::Virtual,
    "yield" => Keyword::Yield,

    // Weak keywords (contextual)
    "async" => Keyword::Async,
    "await" => Keyword::Await,
    "dyn" => Keyword::Dyn,
    "union" => Keyword::Union,
    "try" => Keyword::Try,
    "_" => Keyword::Underscore,

    // Additional type keywords
    "i8" => Keyword::I8,
    "i16" => Keyword::I16,
    "i32" => Keyword::I32,
    "i64" => Keyword::I64,
    "i128" => Keyword::I128,
    "u8" => Keyword::U8,
    "u16" => Keyword::U16,
    "u32" => Keyword::U32,
    "u64" => Keyword::U64,
    "u128" => Keyword::U128,
    "f32" => Keyword::F32,
    "f64" => Keyword::F64,
    "isize" => Keyword::Isize,
    "usize" => Keyword::Usize,
    "bool" => Keyword::Bool,
    "char" => Keyword::CharType,
    "str" => Keyword::Str,
    "option" => Keyword::Option,
    "result" => Keyword::Result,
    "vec" => Keyword::Vec,
};

enum CharError {
    Eof,
    Newline,
    Terminator,
}

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
    fn parse_single_char(&mut self, string: bool) -> Result<char, CharError> {
        let terminator = if string { '"' } else { '\'' };
        loop {
            if let Some(c) = self.next_char() {
                if c == '\\' {
                    if let Some(c) = self.next_char() {
                        return Ok(match c {
                            '\n' => continue,
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '\\' => '\\',
                            '"' => '"',
                            '\'' => '\'',
                            '\0' => '\0',
                            'b' => '\x08',
                            'f' => '\x0c',
                            // TODO: add support for octal and hexadecimal escape sequences
                            _ => c,
                        });
                    } else {
                        return Err(CharError::Eof);
                    }
                } else if c == '\n' {
                    return Err(CharError::Newline);
                } else if c == terminator {
                    return Err(CharError::Terminator);
                } else {
                    return Ok(c);
                }
            } else {
                return Err(CharError::Eof);
            }
        }
    }

    fn parse_char(&mut self) -> Result<Token, String> {
        let c = match self.parse_single_char(false) {
            Ok(c) => c,
            Err(CharError::Terminator) => {
                return Err(String::from("Empty character constant"));
            }
            Err(CharError::Newline) => {
                return Err(String::from("Illegal newline while parsing char literal"));
            }
            Err(CharError::Eof) => {
                return Err(String::from(
                    "Missing terminating ' character in char literal",
                ));
            }
        };

        match self.next_char() {
            Some('\'') => Ok(Token::Char(c)),
            Some('\n') => Err(String::from("Illegal newline while parsing char literal")),
            Some(_) => {
                while self.next_char().is_some_and(|next_c| next_c != '\'') {}
                Err(String::from(
                    "Multi-character character literal not terminated",
                ))
            }
            None => Err(String::from(
                "Missing terminating ' character in char literal",
            )),
        }
    }
    fn parse_string(&mut self) -> Result<Token, String> {
        let mut string = String::new();
        static TERM_ERR: &str = "Missing terminating \" character in string";
        static NEWLINE_ERR: &str = "Illegal newline while parsing string literal";

        loop {
            match self.parse_single_char(true) {
                Ok(c) => string.push(c),
                Err(CharError::Eof) => {
                    return Err(String::from(TERM_ERR));
                }
                Err(CharError::Newline) => {
                    return Err(String::from(NEWLINE_ERR));
                }
                Err(CharError::Terminator) => break,
            }
        }

        Ok(Token::String(string))
    }
    fn parse_id(&mut self, start: char) -> Result<Token, String> {
        let mut id = String::from(start);
        while let Some(c) = self.next_char() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
            } else {
                self.unput(Some(c));
                break;
            }
        }
        match KEYWORDS.get::<str>(&id) {
            Some(keyword) => Ok(Token::Keyword(*keyword)),
            None => Ok(Token::Id(id)),
        }
    }
}

impl<'a, R: Read> Iterator for Lexer<'a, R> {
    // option: whether the stream is exhausted
    // result: whether the next lexeme is an error
    type Item = Locatable<'a, Result<Token, String>>;

    fn next(&mut self) -> Option<Self::Item> {
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
            'a'..='z' | 'A'..='Z' | '_' => self.parse_id(c),
            '\'' => self.parse_char(),
            '"' => self.parse_string(),
            _ => Err(String::from("unknown character")),
        };

        Some(Self::Item { location, data })
    }
}
