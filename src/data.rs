use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // 核心关键字，直接体现语言哲学
    Fn,       // fn
    Let,      // let
    Var,      // var
    With,     // with
    Contract, // contract
    Impl,     // impl
    Mut,      // mut (作为一个独立的关键字)
    Effect,   // effect (代数效应关键字)
    Handle,   // handle (处理器关键字)
    EffectGroup,   // effect_group (效果组关键字)
    HandlerGroup,  // handler_group (处理器组关键字)

    // 控制流与其他通用关键字
    If,
    Else,
    For,
    In,
    Loop,
    While,
    Match,
    Break,
    Continue,
    Return,
    As,
    Use,
    Pub,
    Enum,
    Struct,
    Trait,

    // 布尔字面量
    True,
    False,

    // 补充 Async, Await, Try 关键字
    Async,
    Await,
    Try,

    // 字面量：细粒度类型，但识别形态
    IntegerLiteral(String),
    FloatLiteral(String),
    StringLiteral(String),
    CharLiteral(char),

    // 标识符：代表细粒度的命名
    Identifier(String),

    // 单字符操作符与分隔符，语义的区分留给语法分析器
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Ampersand,  // & (统一为 Ampersand)
    Pipe,       // |
    Caret,      // ^
    Bang,       // !
    Equal,      // =
    Less,       // <
    Greater,    // >
    Dot,        // .
    Comma,      // ,
    Semicolon,  // ;
    Colon,      // :
    Question,   // ?
    At,         // @
    Hash,       // #
    Dollar,     // $
    Underscore, // _

    // 多字符操作符
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    PercentEqual,   // %=
    AmpersandEqual, // &=
    PipeEqual,      // |=
    CaretEqual,     // ^=
    ShlEqual,       // <<=
    ShrEqual,       // >>=
    MutRef,         // &mut (作为原子记号，体现线性类型)
    PathSep,        // ::
    Arrow,          // ->
    FatArrow,       // =>
    EqualEqual,     // ==
    BangEqual,      // !=
    LessEqual,      // <=
    GreaterEqual,   // >=
    And,            // &&
    Or,             // ||
    Shl,            // <<
    Shr,            // >>
    Range,          // ..
    RangeInclusive, // ..=

    // 括号与分隔符
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // 文件结束和错误
    Eof,
    Error(LexicalError),
}

/// 词法分析错误类型，提供更精确的错误信息
#[derive(Debug, PartialEq, Clone)]
pub enum LexicalError {
    /// 未知字符
    UnknownCharacter(char),
    /// 字符串字面量未正确终止
    UnterminatedString,
    /// 字符字面量未正确终止
    UnterminatedChar,
    /// 字符字面量为空
    EmptyCharLiteral,
    /// 字符字面量包含多个字符
    MultipleCharactersInCharLiteral,
    /// 未知的转义序列
    UnknownEscapeSequence(char),
    /// 文件结束时遇到未完成的字面量
    UnexpectedEofInLiteral,
    /// 数字格式错误
    InvalidNumberFormat,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::UnknownCharacter(c) => write!(f, "Unknown character: '{}'", c),
            LexicalError::UnterminatedString => write!(f, "Unterminated string literal"),
            LexicalError::UnterminatedChar => write!(f, "Unterminated character literal"),
            LexicalError::EmptyCharLiteral => write!(f, "Empty character literal"),
            LexicalError::MultipleCharactersInCharLiteral => {
                write!(f, "Multiple characters in character literal")
            }
            LexicalError::UnknownEscapeSequence(c) => write!(f, "Unknown escape sequence: \\{}", c),
            LexicalError::UnexpectedEofInLiteral => write!(f, "Unexpected end of file in literal"),
            LexicalError::InvalidNumberFormat => write!(f, "Invalid number format"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location<'a> {
    pub line: usize,
    pub column: usize,
    pub file: &'a str,
}

#[derive(Debug, Clone)]
pub struct Locatable<'a, T> {
    pub location: Location<'a>,
    pub data: T,
}

// 为Token实现From trait，方便创建Locatable<Token>
impl<'a> From<(Location<'a>, Token)> for Locatable<'a, Token> {
    fn from((location, data): (Location<'a>, Token)) -> Self {
        Locatable { location, data }
    }
}

// 为LexicalError实现From trait，方便创建Locatable<LexicalError>
impl<'a> From<(Location<'a>, LexicalError)> for Locatable<'a, LexicalError> {
    fn from((location, data): (Location<'a>, LexicalError)) -> Self {
        Locatable { location, data }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Fn,
    Let,
    Var,
    With,
    Contract,
    Impl,
    Mut,
    Effect,
    Handle,
    EffectGroup,   // effect_group
    HandlerGroup,  // handler_group
    If,
    Else,
    For,
    In,
    Loop,
    While,
    Match,
    Break,
    Continue,
    Return,
    As,
    Use,
    Pub,
    Enum,
    Struct,
    Trait,
    True,
    False,
    Async,
    Await,
    Try,
}
