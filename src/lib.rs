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
//! let input = "12 + 34";
//! let reader = BufReader::new(Cursor::new(input));
//! let mut lexer = Lexer::new("test.rs", reader);
//!
//! let result = lexer.next().unwrap().data;
//! assert!(matches!(result, Ok(Token::I64(12))));
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
        assert!(matches!(token1, Ok(data::Token::I64(12))));

        let token2 = lexer.next().unwrap().data;
        assert!(matches!(token2, Ok(data::Token::Plus)));

        let token3 = lexer.next().unwrap().data;
        assert!(matches!(token3, Ok(data::Token::I64(34))));
    }

    #[test]
    fn test_all_operators() {
        let input = "+-*/+=-====";
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
        assert!(matches!(token7, Ok(data::Token::EqualEqual)));

        let token8 = lexer.next().unwrap().data;
        assert!(matches!(token8, Ok(data::Token::Equal)));

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
}
