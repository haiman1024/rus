//! rus - A Rust compiler written in Rust
//!
//! This crate provides the core functionality for the rus compiler,
//! including lexical analysis capabilities.
//!
//! ## Example
//!
//! ```
//! use rus::{lex::Lexer, data::TokenType};
//! use std::io::BufReader;
//! use std::io::Cursor;
//!
//! let input = "12 + 34";
//! let reader = BufReader::new(Cursor::new(input));
//! let mut lexer = Lexer::new("test.rs", reader);
//!
//! let token1 = lexer.next().unwrap().unwrap();
//! assert!(matches!(token1.data, TokenType::I64(12)));
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

        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token.data, data::TokenType::Plus));
    }

    #[test]
    fn test_multiple_tokens() {
        let input = "12+34";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().unwrap();
        assert!(matches!(token1.data, data::TokenType::I64(12)));

        let token2 = lexer.next().unwrap().unwrap();
        assert!(matches!(token2.data, data::TokenType::Plus));

        let token3 = lexer.next().unwrap().unwrap();
        assert!(matches!(token3.data, data::TokenType::I64(34)));
    }

    #[test]
    fn test_all_operators() {
        let input = "+-*/";
        let reader = BufReader::new(Cursor::new(input));
        let mut lexer = lex::Lexer::new("test.rs", reader);

        let token1 = lexer.next().unwrap().unwrap();
        assert!(matches!(token1.data, data::TokenType::Plus));

        let token2 = lexer.next().unwrap().unwrap();
        assert!(matches!(token2.data, data::TokenType::Minus));

        let token3 = lexer.next().unwrap().unwrap();
        assert!(matches!(token3.data, data::TokenType::Star));

        let token4 = lexer.next().unwrap().unwrap();
        assert!(matches!(token4.data, data::TokenType::Divide));
    }
}
