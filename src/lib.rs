//! rus - A compiler for the Rus language
//!
//! This project aims to provide a platform for learning and developing compilers,
//! helping to understand compilation principles and language implementation.
//!
//! # Language Design Philosophy
//!
//! The Rus language follows the core axiom: `x(data) --- (behavior) --> y(effect)`
//! and implements three core concepts: strong controllability, fine granularity, and flexibility.
//!
//! # Features
//!
//! - Complete lexical analysis for the Rus language syntax
//! - Detailed error handling mechanism
//! - Precise location tracking
//!
//! # Example
//!
//! ```
//! use rus::data::{Token, Keyword};
//! use rus::lex::Lexer;
//! use std::io::BufReader;
//!
//! let code = "let x = 42;";
//! let reader = BufReader::new(code.as_bytes());
//! let mut lexer = Lexer::new("example.rs", reader);
//!
//! let tokens: Vec<_> = lexer.collect();
//! assert!(tokens[0].data.as_ref().unwrap() == &Token::Let);
//! assert!(tokens[1].data.as_ref().unwrap() == &Token::Identifier("x".to_string()));
//! assert!(tokens[2].data.as_ref().unwrap() == &Token::Equal);
//! assert!(tokens[3].data.as_ref().unwrap() == &Token::IntegerLiteral("42".to_string()));
//! assert!(tokens[4].data.as_ref().unwrap() == &Token::Semicolon);
//! ```

pub mod data;
pub mod lex;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::data::{LexicalError, Locatable, Token};
    use super::lex::Lexer;
    use std::io::BufReader;

    /// 测试基本的词法分析功能
    #[test]
    fn test_basic_lexing() {
        let code = "fn main() { let x = 42; }";
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 检查是否有错误
        for token in &tokens {
            if let Err(e) = &token.data {
                panic!("Lexing error: {}", e);
            }
        }

        // 获取实际的token值
        let token_values: Vec<Token> = tokens.into_iter().map(|t| t.data.unwrap()).collect();

        assert_eq!(token_values[0], Token::Fn);
        assert_eq!(token_values[1], Token::Identifier("main".to_string()));
        assert_eq!(token_values[2], Token::LParen);
        assert_eq!(token_values[3], Token::RParen);
        assert_eq!(token_values[4], Token::LBrace);
        assert_eq!(token_values[5], Token::Let);
        assert_eq!(token_values[6], Token::Identifier("x".to_string()));
        assert_eq!(token_values[7], Token::Equal);
        assert_eq!(token_values[8], Token::IntegerLiteral("42".to_string()));
        assert_eq!(token_values[9], Token::Semicolon);
        assert_eq!(token_values[10], Token::RBrace);
    }

    /// 测试数字解析功能
    #[test]
    fn test_number_parsing() {
        let code = "42 3.14 0xFF 0o77 0b1010";
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 检查是否有错误
        for token in &tokens {
            if let Err(e) = &token.data {
                panic!("Lexing error: {}", e);
            }
        }

        // 获取实际的token值
        let token_values: Vec<Token> = tokens.into_iter().map(|t| t.data.unwrap()).collect();

        assert_eq!(token_values[0], Token::IntegerLiteral("42".to_string()));
        assert_eq!(token_values[1], Token::FloatLiteral("3.14".to_string()));
        assert_eq!(token_values[2], Token::IntegerLiteral("0xFF".to_string()));
        assert_eq!(token_values[3], Token::IntegerLiteral("0o77".to_string()));
        assert_eq!(token_values[4], Token::IntegerLiteral("0b1010".to_string()));
    }

    /// 测试字符串和字符解析功能
    #[test]
    fn test_string_parsing() {
        let code = r#""Hello, World!" '\n'"#;
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 检查是否有错误
        for token in &tokens {
            if let Err(e) = &token.data {
                panic!("Lexing error: {}", e);
            }
        }

        // 获取实际的token值
        let token_values: Vec<Token> = tokens.into_iter().map(|t| t.data.unwrap()).collect();

        assert_eq!(
            token_values[0],
            Token::StringLiteral("Hello, World!".to_string())
        );
        assert_eq!(token_values[1], Token::CharLiteral('\n'));
    }

    /// 测试关键字解析功能
    #[test]
    fn test_keyword_parsing() {
        let code = "let var fn with contract impl mut effect handle";
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 检查是否有错误
        for token in &tokens {
            if let Err(e) = &token.data {
                panic!("Lexing error: {}", e);
            }
        }

        // 获取实际的token值
        let token_values: Vec<Token> = tokens.into_iter().map(|t| t.data.unwrap()).collect();

        assert_eq!(token_values[0], Token::Let);
        assert_eq!(token_values[1], Token::Var);
        assert_eq!(token_values[2], Token::Fn);
        assert_eq!(token_values[3], Token::With);
        assert_eq!(token_values[4], Token::Contract);
        assert_eq!(token_values[5], Token::Impl);
        assert_eq!(token_values[6], Token::Mut);
        assert_eq!(token_values[7], Token::Effect);
        assert_eq!(token_values[8], Token::Handle);
    }

    /// 测试操作符解析功能
    #[test]
    fn test_operator_parsing() {
        let code = "+ - * / % & | ^ ! = < > . , ; : ? @ # $ _";
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 检查是否有错误
        for token in &tokens {
            if let Err(e) = &token.data {
                panic!("Lexing error: {}", e);
            }
        }

        // 获取实际的token值
        let token_values: Vec<Token> = tokens.into_iter().map(|t| t.data.unwrap()).collect();

        assert_eq!(token_values[0], Token::Plus);
        assert_eq!(token_values[1], Token::Minus);
        assert_eq!(token_values[2], Token::Star);
        assert_eq!(token_values[3], Token::Slash);
        assert_eq!(token_values[4], Token::Percent);
        assert_eq!(token_values[5], Token::Ampersand);
        assert_eq!(token_values[6], Token::Pipe);
        assert_eq!(token_values[7], Token::Caret);
        assert_eq!(token_values[8], Token::Bang);
        assert_eq!(token_values[9], Token::Equal);
        assert_eq!(token_values[10], Token::Less);
        assert_eq!(token_values[11], Token::Greater);
        assert_eq!(token_values[12], Token::Dot);
        assert_eq!(token_values[13], Token::Comma);
        assert_eq!(token_values[14], Token::Semicolon);
        assert_eq!(token_values[15], Token::Colon);
        assert_eq!(token_values[16], Token::Question);
        assert_eq!(token_values[17], Token::At);
        assert_eq!(token_values[18], Token::Hash);
        assert_eq!(token_values[19], Token::Dollar);
        assert_eq!(token_values[20], Token::Identifier("_".to_string()));
    }

    /// 测试错误处理功能
    #[test]
    fn test_error_handling() {
        // 使用一个非法的转义序列来触发错误
        let code = r#"let x = '\z';"#;
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 检查是否有错误token
        let has_error = tokens.iter().any(|token| token.data.is_err());
        assert!(has_error);
    }

    /// 测试字符解析功能
    #[test]
    fn test_char_parsing() {
        let code = r#"'\n' '\'' '\\'"#;
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 检查是否有错误
        for token in &tokens {
            if let Err(e) = &token.data {
                panic!("Lexing error: {}", e);
            }
        }

        // 获取实际的token值
        let token_values: Vec<Token> = tokens.into_iter().map(|t| t.data.unwrap()).collect();

        assert_eq!(token_values[0], Token::CharLiteral('\n'));
        assert_eq!(token_values[1], Token::CharLiteral('\''));
        assert_eq!(token_values[2], Token::CharLiteral('\\'));
    }
}
