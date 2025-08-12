//! rus - A Rust compiler written in Rust
//!
//! This crate provides the core functionality for the rus compiler,
//! including lexical analysis capabilities.
//!
//! ## Example
//!
//! ```
//! use rus::{lex::Lexer, data::Token};
//! use std::io::BufReader;
//! use std::io::Cursor;
//!
//! let input = "let x = 12 + 34;";
//! let reader = BufReader::new(Cursor::new(input));
//! let mut lexer = Lexer::new("test.rs", reader);
//!
//! let result = lexer.next().unwrap().data;
//! assert!(matches!(result, Ok(Token::Keyword(rus::data::Keyword::Let))));
//! ```

pub mod data;
pub mod lex;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::io::Cursor;

    #[test]
    fn test_lexer_creation() {
        let input = "";
        let reader = BufReader::new(Cursor::new(input));
        let _lexer = lex::Lexer::new("test.rs", reader);
        // We can't directly access private fields, so we'll test by trying to use the lexer
        assert!(true); // Placeholder - the lexer was created successfully
    }

    #[test]
    fn test_simple_plus_token() {
        let input = "+";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token = lexer.next().unwrap().data;
        assert!(matches!(token, Ok(data::Token::Plus)));
    }

    #[test]
    fn test_multiple_tokens() {
        let input = "12+34";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(token1, Ok(data::Token::I32(12))));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::Plus)));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::I32(34))));
    }

    #[test]
    fn test_all_operators() {
        let input = "+ - * / += -= *= /= == = && || != < > <= >= << >>";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(token1, Ok(data::Token::Plus)));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::Minus)));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::Star)));

        let token4 = lexer.next().unwrap().data;
        assert!(matches!(token4, Ok(data::Token::Divide)));

        let token5 = lexer.next().unwrap().data;
        assert!(matches!(token5, Ok(data::Token::PlusEqual)));

        let token6 = lexer.next().unwrap().data;
        assert!(matches!(token6, Ok(data::Token::MinusEqual)));

        let token7 = lexer.next().unwrap().data;
        assert!(matches!(token7, Ok(data::Token::StarEqual)));

        let token8 = lexer.next().unwrap().data;
        assert!(matches!(token8, Ok(data::Token::DivideEqual)));

        let token9 = lexer.next().unwrap().data;
        assert!(matches!(token9, Ok(data::Token::EqualEqual)));

        let token10 = lexer.next().unwrap().data;
        assert!(matches!(token10, Ok(data::Token::Equal)));

        let token11 = lexer.next().unwrap().data;
        assert!(matches!(token11, Ok(data::Token::And)));

        let token12 = lexer.next().unwrap().data;
        assert!(matches!(token12, Ok(data::Token::Or)));

        let token13 = lexer.next().unwrap().data;
        assert!(matches!(token13, Ok(data::Token::NotEqual)));

        let token14 = lexer.next().unwrap().data;
        assert!(matches!(token14, Ok(data::Token::Less)));

        let token15 = lexer.next().unwrap().data;
        assert!(matches!(token15, Ok(data::Token::Greater)));

        let token16 = lexer.next().unwrap().data;
        assert!(matches!(token16, Ok(data::Token::LessEqual)));

        let token17 = lexer.next().unwrap().data;
        assert!(matches!(token17, Ok(data::Token::GreaterEqual)));

        let token18 = lexer.next().unwrap().data;
        assert!(matches!(token18, Ok(data::Token::Shl)));

        let token19 = lexer.next().unwrap().data;
        assert!(matches!(token19, Ok(data::Token::Shr)));

        assert!(lexer.next().is_none());
    }
    #[test]
    fn test_integer_overflow() {
        let input = "9999999999999999999999999999999999999999999999999999999999999999999999999999";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let result = lexer.next().unwrap().data;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Overflow while parsing integer literal"
        );
    }

    #[test]
    fn test_char_literal() {
        let input = "'a'";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let result = lexer.next().unwrap().data;
        assert!(matches!(result, Ok(data::Token::Char('a'))));
    }

    #[test]
    fn test_empty_char_literal() {
        let input = "''";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let result = lexer.next().unwrap().data;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty character constant");
    }

    #[test]
    fn test_multichar_literal() {
        let input = "'ab'";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let result = lexer.next().unwrap().data;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Multi-character character literal not terminated"
        );
    }

    #[test]
    fn test_keywords() {
        let input = "if else while for fn";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(
            token1,
            Ok(data::Token::Keyword(data::Keyword::If))
        ));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(
            token2,
            Ok(data::Token::Keyword(data::Keyword::Else))
        ));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(
            token3,
            Ok(data::Token::Keyword(data::Keyword::While))
        ));

        let token4 = lexer.next().unwrap().data;
        assert!(matches!(
            token4,
            Ok(data::Token::Keyword(data::Keyword::For))
        ));

        let token5 = lexer.next().unwrap().data;
        assert!(matches!(
            token5,
            Ok(data::Token::Keyword(data::Keyword::Fn))
        ));

        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_identifiers() {
        let input = "my_var myFunc _private";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(token1, Ok(data::Token::Id(s)) if s == "my_var"));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::Id(s)) if s == "myFunc"));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::Id(s)) if s == "_private"));

        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_string_literal() {
        let input = "\"hello world\"";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let result = lexer.next().unwrap().data;
        assert!(matches!(result, Ok(data::Token::String(s)) if s == "hello world"));
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_float_literals() {
        let input = "3.14 1e10 2.5e-3 1.0f32 2.0f64";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(token1, Ok(data::Token::F64(v)) if v == 3.14));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::F64(v)) if v == 1e10));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::F64(v)) if v == 2.5e-3));

        let token4 = lexer.next().unwrap().data;
        assert!(matches!(token4, Ok(data::Token::F32(v)) if v == 1.0));

        let token5 = lexer.next().unwrap().data;
        assert!(matches!(token5, Ok(data::Token::F64(v)) if v == 2.0));

        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_dot_operators() {
        let input = ". .. ... ..=";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(token1, Ok(data::Token::Dot)));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::DotDot)));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::DotDotDot)));

        let token4 = lexer.next().unwrap().data;
        assert!(matches!(token4, Ok(data::Token::DotDotEq)));

        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_arrow_operators() {
        let input = "-> => :: :";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(token1, Ok(data::Token::Arrow)));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::FatArrow)));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::ColonColon)));

        let token4 = lexer.next().unwrap().data;
        assert!(matches!(token4, Ok(data::Token::Colon)));

        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_other_symbols() {
        let input = ", ; @ # $ ?";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().data;
        assert!(matches!(token1, Ok(data::Token::Comma)));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::Semi)));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::At)));

        let token4 = lexer.next().unwrap().data;
        assert!(matches!(token4, Ok(data::Token::Hash)));

        let token5 = lexer.next().unwrap().data;
        assert!(matches!(token5, Ok(data::Token::Dollar)));

        let token6 = lexer.next().unwrap().data;
        assert!(matches!(token6, Ok(data::Token::Question)));

        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_complex_expression() {
        let input = "let x = 10 + 20.0 * 3.14e2;";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        // let
        let token1 = lexer.next().unwrap().data;
        assert!(matches!(
            token1,
            Ok(data::Token::Keyword(data::Keyword::Let))
        ));

        // x
        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::Id(s)) if s == "x"));

        // =
        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::Equal)));

        // 10
        let token4 = lexer.next().unwrap().data;
        assert!(matches!(token4, Ok(data::Token::I32(10))));

        // +
        let token5 = lexer.next().unwrap().data;
        assert!(matches!(token5, Ok(data::Token::Plus)));

        // 20.0
        let token6 = lexer.next().unwrap().data;
        assert!(matches!(token6, Ok(data::Token::F64(v)) if v == 20.0));

        // *
        let token7 = lexer.next().unwrap().data;
        assert!(matches!(token7, Ok(data::Token::Star)));

        // 3.14e2
        let token8 = lexer.next().unwrap().data;
        assert!(matches!(token8, Ok(data::Token::F64(v)) if v == 3.14e2));

        // ;
        let token9 = lexer.next().unwrap().data;
        assert!(matches!(token9, Ok(data::Token::Semi)));

        assert!(lexer.next().is_none());
    }
}
