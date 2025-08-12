#[derive(Debug)]
pub enum Token {
    // arithmetic operators
    Plus,
    Minus,
    Star,
    Divide,
    Percent, // %

    // bitwise operators
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    Shl,    // <<
    Shr,    // >>
    BitNot, // !

    // comparison operators
    EqualEqual,   // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // assignment operators
    Equal,        // =
    PlusEqual,    // +=
    MinusEqual,   // -=
    StarEqual,    // *=
    DivideEqual,  // /=
    PercentEqual, // %=
    BitAndEqual,  // &=
    BitOrEqual,   // |=
    BitXorEqual,  // ^=
    ShlEqual,     // <<=
    ShrEqual,     // >>=

    // logical operators
    And, // &&
    Or,  // ||

    // other operators
    Not,        // !
    Dot,        // .
    DotDot,     // ..
    DotDotDot,  // ...
    DotDotEq,   // ..=
    Comma,      // ,
    Semi,       // ;
    Colon,      // :
    ColonColon, // ::
    Arrow,      // ->
    FatArrow,   // =>
    At,         // @
    Underscore, // _
    Hash,       // #
    Dollar,     // $
    Question,   // ?

    // literals
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Isize(isize),
    Usize(usize),
    String(String),
    Char(char),

    // identifiers and keywords
    Id(String),
    Keyword(Keyword),
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

#[derive(Clone, Copy, Debug)]
pub enum Keyword {
    // Strict keywords
    As,
    Break,
    Const,
    Continue,
    Crate,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    SelfValue, // self
    SelfType,  // Self
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Unsafe,
    Use,
    Where,
    While,

    // Reserved keywords
    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,

    // Weak keywords (contextual)
    Async,
    Await,
    Dyn,
    Union,
    Try,
    Underscore, // _

    // Additional type keywords
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Isize,
    Usize,
    Bool,
    CharType, // Char is already used for character literals, so using CharType
    Str,      // String type
    Option,
    Result,
    Vec,
    Slice,
    Array,
    Tuple,
    Unit,       // ()
    Never,      // !
    Reference,  // &
    RawPointer, // *const, *mut
    FnPointer,  // fn()
    Closure,    // || {}
}
