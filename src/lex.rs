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

    /// 解析数字字面量的主要函数
    fn parse_number(&mut self) -> Result<Token, String> {
        let mut number_str = String::new();
        let mut has_decimal_point = false;
        let mut has_exponent = false;

        // Collect the integer part
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.next_char();
                number_str.push(c);
            } else {
                break;
            }
        }

        // Check for decimal point
        if let Some('.') = self.peek() {
            // Check if the character after '.' is a digit
            self.next_char(); // consume '.'
            if let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    number_str.push('.');
                    has_decimal_point = true;
                    self.next_char(); // consume the digit
                    number_str.push(c);

                    // Collect remaining fractional digits
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            self.next_char();
                            number_str.push(c);
                        } else {
                            break;
                        }
                    }
                } else {
                    // Put back the '.' if it's not followed by a digit
                    self.unput(Some('.'));
                }
            } else {
                // Put back the '.' if it's at the end
                self.unput(Some('.'));
            }
        }

        // Check for exponent
        if !has_exponent && let Some(c) = self.peek().filter(|&c| c == 'e' || c == 'E') {
            self.next_char();
            number_str.push(c);
            has_exponent = true;

            // Check for exponent sign
            if let Some(sign) = self.peek().filter(|&sign| sign == '+' || sign == '-') {
                self.next_char();
                number_str.push(sign);
            }

            // Must have at least one digit in exponent
            let mut has_exp_digits = false;
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    self.next_char();
                    number_str.push(c);
                    has_exp_digits = true;
                } else {
                    break;
                }
            }

            if !has_exp_digits {
                return Err(String::from("Expected at least one digit in exponent"));
            }
        }

        // Parse suffix if any
        let suffix = self.parse_integer_suffix();

        // Determine if this is a float or integer
        if has_decimal_point || has_exponent {
            // This is a float literal
            match suffix.as_str() {
                "f32" => match number_str.parse::<f32>() {
                    Ok(value) => Ok(Token::F32(value)),
                    Err(_) => Err(String::from("Invalid f32 literal")),
                },
                "f64" | "" => match number_str.parse::<f64>() {
                    Ok(value) => Ok(Token::F64(value)),
                    Err(_) => Err(String::from("Invalid f64 literal")),
                },
                _ => {
                    // Try to parse as the specified float type
                    if suffix.starts_with('f') {
                        match number_str.parse::<f64>() {
                            Ok(value) => Ok(Token::F64(value)),
                            Err(_) => Err(String::from("Invalid float literal")),
                        }
                    } else {
                        Err(format!("Invalid suffix for float literal: {}", suffix))
                    }
                }
            }
        } else {
            // This is an integer literal
            // Parse as i64 first for range checking
            match number_str.parse::<i64>() {
                Ok(value) => self.convert_integer(value, &suffix),
                Err(_) => Err(String::from("Overflow while parsing integer literal")),
            }
        }
    }

    fn convert_integer(&mut self, value: i64, suffix: &str) -> Result<Token, String> {
        match suffix {
            "i8" => {
                if value > i8::MAX as i64 || value < i8::MIN as i64 {
                    Err(String::from("Integer literal out of range for i8"))
                } else {
                    Ok(Token::I8(value as i8))
                }
            }
            "i16" => {
                if value > i16::MAX as i64 || value < i16::MIN as i64 {
                    Err(String::from("Integer literal out of range for i16"))
                } else {
                    Ok(Token::I16(value as i16))
                }
            }
            "i32" => {
                if value > i32::MAX as i64 || value < i32::MIN as i64 {
                    Err(String::from("Integer literal out of range for i32"))
                } else {
                    Ok(Token::I32(value as i32))
                }
            }
            "i64" => Ok(Token::I64(value)),
            "i128" => Ok(Token::I128(value as i128)),
            "u8" => {
                if value > u8::MAX as i64 || value < 0 {
                    Err(String::from("Integer literal out of range for u8"))
                } else {
                    Ok(Token::U8(value as u8))
                }
            }
            "u16" => {
                if value > u16::MAX as i64 || value < 0 {
                    Err(String::from("Integer literal out of range for u16"))
                } else {
                    Ok(Token::U16(value as u16))
                }
            }
            "u32" => {
                if value > u32::MAX as i64 || value < 0 {
                    Err(String::from("Integer literal out of range for u32"))
                } else {
                    Ok(Token::U32(value as u32))
                }
            }
            "u64" => {
                if value < 0 {
                    Err(String::from("Integer literal out of range for u64"))
                } else {
                    Ok(Token::U64(value as u64))
                }
            }
            "u128" => {
                if value < 0 {
                    Err(String::from("Integer literal out of range for u128"))
                } else {
                    Ok(Token::U128(value as u128))
                }
            }
            "isize" => Ok(Token::Isize(value as isize)),
            "usize" => {
                if value < 0 {
                    Err(String::from("Integer literal out of range for usize"))
                } else {
                    Ok(Token::Usize(value as usize))
                }
            }
            "" => {
                // Default to i32 if within range
                if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                    Ok(Token::I32(value as i32))
                } else {
                    Err(String::from("Integer literal out of default i32 range"))
                }
            }
            _ => Err(format!("Invalid integer suffix: {}", suffix)),
        }
    }

    fn parse_integer_suffix(&mut self) -> String {
        let mut suffix = String::new();

        // Check if next char starts a valid suffix
        if let Some(c) = self.peek().filter(|c| c.is_ascii_alphabetic()) {
            self.next_char();
            suffix.push(c);

            // Continue collecting alphanumeric characters
            while let Some(next_c) = self.peek() {
                if next_c.is_ascii_alphanumeric() {
                    self.next_char();
                    suffix.push(next_c);
                } else {
                    break;
                }
            }
        }

        // Check if the suffix is valid
        match suffix.as_str() {
            "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128"
            | "isize" | "usize" | "f32" | "f64" => suffix,
            _ => {
                // Put back characters if not a valid suffix
                for c in suffix.chars().rev() {
                    self.unput(Some(c));
                }
                String::new()
            }
        }
    }

    fn parse_plus(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::PlusEqual)
            }
            _ => Ok(Token::Plus),
        }
    }

    fn parse_minus(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::MinusEqual)
            }
            Some('>') => {
                self.next_char();
                Ok(Token::Arrow)
            }
            _ => Ok(Token::Minus),
        }
    }

    fn parse_equal(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::EqualEqual)
            }
            Some('>') => {
                self.next_char();
                Ok(Token::FatArrow)
            }
            _ => Ok(Token::Equal),
        }
    }

    fn parse_star(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::StarEqual)
            }
            _ => Ok(Token::Star),
        }
    }

    fn parse_divide(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::DivideEqual)
            }
            _ => Ok(Token::Divide),
        }
    }

    fn parse_percent(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::PercentEqual)
            }
            _ => Ok(Token::Percent),
        }
    }

    fn parse_bitand(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::BitAndEqual)
            }
            Some('&') => {
                self.next_char();
                Ok(Token::And)
            }
            _ => Ok(Token::BitAnd),
        }
    }

    fn parse_bitor(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::BitOrEqual)
            }
            Some('|') => {
                self.next_char();
                Ok(Token::Or)
            }
            _ => Ok(Token::BitOr),
        }
    }

    fn parse_bitxor(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::BitXorEqual)
            }
            _ => Ok(Token::BitXor),
        }
    }

    fn parse_not(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::NotEqual)
            }
            _ => Ok(Token::Not),
        }
    }

    fn parse_greater(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::GreaterEqual)
            }
            Some('>') => {
                self.next_char();
                match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Ok(Token::ShrEqual)
                    }
                    _ => Ok(Token::Shr),
                }
            }
            _ => Ok(Token::Greater),
        }
    }

    fn parse_less(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('=') => {
                self.next_char();
                Ok(Token::LessEqual)
            }
            Some('<') => {
                self.next_char();
                match self.peek() {
                    Some('=') => {
                        self.next_char();
                        Ok(Token::ShlEqual)
                    }
                    _ => Ok(Token::Shl),
                }
            }
            _ => Ok(Token::Less),
        }
    }

    fn parse_dot(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some('.') => {
                self.next_char();
                match self.peek() {
                    Some('.') => {
                        self.next_char();
                        Ok(Token::DotDotDot)
                    }
                    Some('=') => {
                        self.next_char();
                        Ok(Token::DotDotEq)
                    }
                    _ => Ok(Token::DotDot),
                }
            }
            _ => Ok(Token::Dot),
        }
    }

    fn parse_colon(&mut self) -> Result<Token, String> {
        match self.peek() {
            Some(':') => {
                self.next_char();
                Ok(Token::ColonColon)
            }
            _ => Ok(Token::Colon),
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
            '+' => self.parse_plus(),
            '-' => self.parse_minus(),
            '=' => self.parse_equal(),
            '*' => self.parse_star(),
            '/' => self.parse_divide(),
            '%' => self.parse_percent(),
            '&' => self.parse_bitand(),
            '|' => self.parse_bitor(),
            '^' => self.parse_bitxor(),
            '!' => self.parse_not(),
            '>' => self.parse_greater(),
            '<' => self.parse_less(),
            '.' => {
                // Check if this is a float starting with a decimal point
                match self.peek() {
                    Some(c) if c.is_ascii_digit() => {
                        self.unput(Some('.'));
                        self.parse_number()
                    }
                    _ => self.parse_dot(),
                }
            }
            ':' => self.parse_colon(),
            '0'..='9' => {
                self.unput(Some(c));
                self.parse_number()
            }
            'a'..='z' | 'A'..='Z' | '_' => self.parse_id(c),
            '\'' => self.parse_char(),
            '"' => self.parse_string(),
            ',' => Ok(Token::Comma),
            ';' => Ok(Token::Semi),
            '@' => Ok(Token::At),
            '#' => Ok(Token::Hash),
            '$' => Ok(Token::Dollar),
            '?' => Ok(Token::Question),
            _ => Err(String::from("unknown character")),
        };

        Some(Self::Item { location, data })
    }
}
