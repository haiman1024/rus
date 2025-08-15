use std::io::{BufRead, BufReader, Read};
use std::iter::IntoIterator;
use std::vec::IntoIter;

use super::data::{Keyword, LexicalError, Locatable, Location, Token};
use phf::Map;

static KEYWORDS: Map<&'static str, Keyword> = phf::phf_map! {
    "fn" => Keyword::Fn,
    "let" => Keyword::Let,
    "var" => Keyword::Var,
    "with" => Keyword::With,
    "contract" => Keyword::Contract,
    "impl" => Keyword::Impl,
    "mut" => Keyword::Mut,
    "effect" => Keyword::Effect,
    "handle" => Keyword::Handle,
    "effect_group" => Keyword::EffectGroup,
    "handler_group" => Keyword::HandlerGroup,
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "for" => Keyword::For,
    "in" => Keyword::In,
    "loop" => Keyword::Loop,
    "while" => Keyword::While,
    "match" => Keyword::Match,
    "break" => Keyword::Break,
    "continue" => Keyword::Continue,
    "return" => Keyword::Return,
    "as" => Keyword::As,
    "use" => Keyword::Use,
    "pub" => Keyword::Pub,
    "enum" => Keyword::Enum,
    "struct" => Keyword::Struct,
    "trait" => Keyword::Trait,
    "true" => Keyword::True,
    "false" => Keyword::False,
    "async" => Keyword::Async,
    "await" => Keyword::Await,
    "try" => Keyword::Try,
};

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
                line: 1,
                column: 0,
                file: filename,
            },
            reader: stream,
            iterator: Vec::new().into_iter(),
            current: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.current.take() {
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
                    if buf.is_empty() {
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
        if self.location.column > 0 && c.is_some() {
            self.location.column -= 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.current.is_none() {
            self.current = self.next_char();
        }
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

    fn parse_number(&mut self) -> Result<Token, LexicalError> {
        let mut number_str = String::new();
        let mut is_float = false;

        // 检查数字前缀以支持不同进制
        let (radix, has_prefix) = if self.peek() == Some('0') {
            self.next_char(); // 消费 '0'
            match self.peek() {
                Some('x') | Some('X') => {
                    self.next_char(); // 消费 'x' 或 'X'
                    (16, true) // 十六进制
                }
                Some('o') | Some('O') => {
                    self.next_char(); // 消费 'o' 或 'O'
                    (8, true) // 八进制
                }
                Some('b') | Some('B') => {
                    self.next_char(); // 消费 'b' 或 'B'
                    (2, true) // 二进制
                }
                Some('.') | Some('e') | Some('E') | None => {
                    // 这是十进制数，以0开头
                    number_str.push('0');
                    (10, false)
                }
                Some(c) if c.is_ascii_digit() => {
                    // 以0开头的八进制数（传统表示法）
                    number_str.push('0');
                    (8, false)
                }
                _ => {
                    number_str.push('0');
                    (10, false)
                }
            }
        } else {
            (10, false) // 默认十进制
        };

        // 解析整数部分
        match radix {
            16 => {
                // 对于十六进制，我们需要包含前缀在字符串中
                if has_prefix {
                    number_str.push_str("0x");
                }
                while let Some(c) = self.peek() {
                    if c.is_ascii_hexdigit() {
                        self.next_char();
                        number_str.push(c);
                    } else {
                        break;
                    }
                }
            }
            8 => {
                // 对于八进制，我们需要包含前缀在字符串中
                if has_prefix {
                    number_str.push_str("0o");
                }
                while let Some(c) = self.peek() {
                    if ('0'..='7').contains(&c) {
                        self.next_char();
                        number_str.push(c);
                    } else {
                        break;
                    }
                }
            }
            2 => {
                // 对于二进制，我们需要包含前缀在字符串中
                if has_prefix {
                    number_str.push_str("0b");
                }
                while let Some(c) = self.peek() {
                    if c == '0' || c == '1' {
                        self.next_char();
                        number_str.push(c);
                    } else {
                        break;
                    }
                }
            }
            10 => {
                // 原有的十进制解析逻辑
                if radix == 10 && number_str.is_empty() {
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            self.next_char();
                            number_str.push(c);
                        } else {
                            break;
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        // 处理小数部分（仅适用于十进制）
        if radix == 10 && !is_float {
            if let Some('.') = self.peek() {
                self.next_char();
                if let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        is_float = true;
                        number_str.push('.');
                        self.next_char();
                        number_str.push(c);
                        while let Some(c) = self.peek() {
                            if c.is_ascii_digit() {
                                self.next_char();
                                number_str.push(c);
                            } else {
                                break;
                            }
                        }
                    } else {
                        self.unput(Some('.'));
                    }
                } else {
                    self.unput(Some('.'));
                }
            }

            // 处理科学计数法（仅适用于十进制浮点数）
            if let Some(c) = self.peek().filter(|&c| c == 'e' || c == 'E') {
                self.next_char();
                number_str.push(c);
                is_float = true;
                if let Some(sign) = self.peek().filter(|&sign| sign == '+' || sign == '-') {
                    self.next_char();
                    number_str.push(sign);
                }
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        self.next_char();
                        number_str.push(c);
                    } else {
                        break;
                    }
                }
            }
        }

        // 解析类型后缀（如 u32, i64, f64 等）
        let suffix = self.parse_identifier();
        if !suffix.is_empty() {
            number_str.push('_'); // 使用下划线分隔数字和后缀
            number_str.push_str(&suffix);
        }

        // 根据进制和是否为浮点数返回相应类型的Token
        if is_float && radix == 10 {
            Ok(Token::FloatLiteral(number_str))
        } else {
            Ok(Token::IntegerLiteral(number_str))
        }
    }

    fn parse_string(&mut self) -> Result<Token, LexicalError> {
        let mut string_content = String::new();
        while let Some(c) = self.next_char() {
            match c {
                '"' => return Ok(Token::StringLiteral(string_content)),
                '\\' => {
                    if let Some(escaped) = self.next_char() {
                        match escaped {
                            'n' => string_content.push('\n'),
                            'r' => string_content.push('\r'),
                            't' => string_content.push('\t'),
                            '0' => string_content.push('\0'),
                            '\'' => string_content.push('\''),
                            '"' => string_content.push('"'),
                            '\\' => string_content.push('\\'),
                            _ => return Err(LexicalError::UnknownEscapeSequence(escaped)),
                        }
                    } else {
                        return Err(LexicalError::UnexpectedEofInLiteral);
                    }
                }
                _ => string_content.push(c),
            }
        }
        Err(LexicalError::UnterminatedString)
    }

    fn parse_char(&mut self) -> Result<Token, LexicalError> {
        let c = match self.next_char() {
            Some(c) => c,
            None => return Err(LexicalError::UnexpectedEofInLiteral),
        };

        let result = match c {
            '\\' => match self.next_char() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some('0') => '\0',
                Some('\'') => '\'',
                Some('"') => '"',
                Some('\\') => '\\',
                Some(c) => {
                    return Err(LexicalError::UnknownEscapeSequence(c));
                }
                None => return Err(LexicalError::UnexpectedEofInLiteral),
            },
            '\'' => {
                return Err(LexicalError::EmptyCharLiteral);
            }
            c => c,
        };

        match self.next_char() {
            Some('\'') => Ok(Token::CharLiteral(result)),
            Some(_) => Err(LexicalError::MultipleCharactersInCharLiteral),
            None => Err(LexicalError::UnterminatedChar),
        }
    }

    fn parse_identifier(&mut self) -> String {
        let mut id = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.next_char();
                id.push(c);
            } else {
                break;
            }
        }
        id
    }
}

impl<'a, R: Read> Iterator for Lexer<'a, R> {
    type Item = Locatable<'a, Result<Token, LexicalError>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let c = self.next_char()?;
        let location = self.location.clone();

        let data = match c {
            '0'..='9' => {
                self.unput(Some(c));
                self.parse_number()
            }
            '"' => self.parse_string(),
            '\'' => self.parse_char(),

            'a'..='z' | 'A'..='Z' | '_' => {
                self.unput(Some(c));
                let identifier_str = self.parse_identifier();
                if let Some(keyword) = KEYWORDS.get(&identifier_str) {
                    match keyword {
                        Keyword::Fn => Ok(Token::Fn),
                        Keyword::Let => Ok(Token::Let),
                        Keyword::Var => Ok(Token::Var),
                        Keyword::With => Ok(Token::With),
                        Keyword::Contract => Ok(Token::Contract),
                        Keyword::Impl => Ok(Token::Impl),
                        Keyword::Mut => Ok(Token::Mut),
                        Keyword::Effect => Ok(Token::Effect),
                        Keyword::Handle => Ok(Token::Handle),
                        Keyword::EffectGroup => Ok(Token::EffectGroup),
                        Keyword::HandlerGroup => Ok(Token::HandlerGroup),
                        Keyword::If => Ok(Token::If),
                        Keyword::Else => Ok(Token::Else),
                        Keyword::For => Ok(Token::For),
                        Keyword::In => Ok(Token::In),
                        Keyword::Loop => Ok(Token::Loop),
                        Keyword::While => Ok(Token::While),
                        Keyword::Match => Ok(Token::Match),
                        Keyword::Break => Ok(Token::Break),
                        Keyword::Continue => Ok(Token::Continue),
                        Keyword::Return => Ok(Token::Return),
                        Keyword::As => Ok(Token::As),
                        Keyword::Use => Ok(Token::Use),
                        Keyword::Pub => Ok(Token::Pub),
                        Keyword::Enum => Ok(Token::Enum),
                        Keyword::Struct => Ok(Token::Struct),
                        Keyword::Trait => Ok(Token::Trait),
                        Keyword::True => Ok(Token::True),
                        Keyword::False => Ok(Token::False),
                        Keyword::Async => Ok(Token::Async),
                        Keyword::Await => Ok(Token::Await),
                        Keyword::Try => Ok(Token::Try),
                    }
                } else {
                    Ok(Token::Identifier(identifier_str))
                }
            }

            '+' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    Ok(Token::PlusEqual)
                } else {
                    Ok(Token::Plus)
                }
            }
            '-' => match self.peek() {
                Some('=') => {
                    self.next_char();
                    Ok(Token::MinusEqual)
                }
                Some('>') => {
                    self.next_char();
                    Ok(Token::Arrow)
                }
                _ => Ok(Token::Minus),
            },
            '=' => match self.peek() {
                Some('=') => {
                    self.next_char();
                    Ok(Token::EqualEqual)
                }
                Some('>') => {
                    self.next_char();
                    Ok(Token::FatArrow)
                }
                _ => Ok(Token::Equal),
            },
            '*' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    Ok(Token::StarEqual)
                } else {
                    Ok(Token::Star)
                }
            }
            '/' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    Ok(Token::SlashEqual)
                } else {
                    Ok(Token::Slash)
                }
            }
            '%' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    Ok(Token::PercentEqual)
                } else {
                    Ok(Token::Percent)
                }
            }
            '&' => {
                // 检查是否是 &mut
                let peek_location = self.location.clone();
                if self.peek() == Some('m') {
                    self.next_char(); // 消费 'm'
                    if self.peek() == Some('u') {
                        self.next_char(); // 消费 'u'
                        if self.peek() == Some('t') {
                            self.next_char(); // 消费 't'
                            Ok(Token::MutRef)
                        } else {
                            // 不是 &mut，回退并检查是否是 &&
                            self.unput(Some('t')); // 回退 't'
                            self.unput(Some('u')); // 回退 'u'
                            self.unput(Some('m')); // 回退 'm'
                            self.location = peek_location;
                            if self.peek() == Some('&') {
                                self.next_char(); // 消费第二个 '&'
                                Ok(Token::And)
                            } else {
                                Ok(Token::Ampersand)
                            }
                        }
                    } else {
                        // 不是 &mut，回退并检查是否是 &&
                        self.unput(Some('u')); // 回退 'u'
                        self.unput(Some('m')); // 回退 'm'
                        self.location = peek_location;
                        if self.peek() == Some('&') {
                            self.next_char(); // 消费第二个 '&'
                            Ok(Token::And)
                        } else {
                            Ok(Token::Ampersand)
                        }
                    }
                } else if self.peek() == Some('&') {
                    // 是 &&
                    self.next_char(); // 消费第二个 '&'
                    Ok(Token::And)
                } else {
                    // 单独的 &
                    Ok(Token::Ampersand)
                }
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.next_char();
                    Ok(Token::Or)
                } else {
                    Ok(Token::Pipe)
                }
            }
            '^' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    Ok(Token::CaretEqual)
                } else {
                    Ok(Token::Caret)
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.next_char();
                    Ok(Token::BangEqual)
                } else {
                    Ok(Token::Bang)
                }
            }
            '<' => match self.peek() {
                Some('=') => {
                    self.next_char();
                    Ok(Token::LessEqual)
                }
                Some('<') => {
                    self.next_char();
                    if self.peek() == Some('=') {
                        self.next_char();
                        Ok(Token::ShlEqual)
                    } else {
                        Ok(Token::Shl)
                    }
                }
                _ => Ok(Token::Less),
            },
            '>' => match self.peek() {
                Some('=') => {
                    self.next_char();
                    Ok(Token::GreaterEqual)
                }
                Some('>') => {
                    self.next_char();
                    if self.peek() == Some('=') {
                        self.next_char();
                        Ok(Token::ShrEqual)
                    } else {
                        Ok(Token::Shr)
                    }
                }
                _ => Ok(Token::Greater),
            },
            '.' => {
                match self.peek() {
                    Some('.') => {
                        self.next_char(); // 消费第二个点
                        if self.peek() == Some('=') {
                            self.next_char(); // 消费等号
                            Ok(Token::RangeInclusive)
                        } else {
                            Ok(Token::Range)
                        }
                    }
                    _ => Ok(Token::Dot),
                }
            }
            ',' => Ok(Token::Comma),
            ';' => Ok(Token::Semicolon),
            ':' => {
                if self.peek() == Some(':') {
                    self.next_char();
                    Ok(Token::PathSep)
                } else {
                    Ok(Token::Colon)
                }
            }
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            '?' => Ok(Token::Question),
            '@' => Ok(Token::At),
            '#' => Ok(Token::Hash),
            '$' => Ok(Token::Dollar),
            _ => Err(LexicalError::UnknownCharacter(c)),
        };

        Some(Self::Item { location, data })
    }
}
