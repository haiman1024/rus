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

    /// 测试字符串解析功能
    #[test]
    fn test_string_parsing() {
        let code = r#""hello" "world\n" "with \"quotes\"""#;
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

        assert_eq!(token_values[0], Token::StringLiteral("hello".to_string()));
        assert_eq!(token_values[1], Token::StringLiteral("world\n".to_string()));
        assert_eq!(
            token_values[2],
            Token::StringLiteral("with \"quotes\"".to_string())
        );
    }

    /// 测试字符解析功能
    #[test]
    fn test_char_parsing() {
        let code = r"'a' '\n' '\''";
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

        assert_eq!(token_values[0], Token::CharLiteral('a'));
        assert_eq!(token_values[1], Token::CharLiteral('\n'));
        assert_eq!(token_values[2], Token::CharLiteral('\''));
    }

    /// 测试操作符解析功能
    #[test]
    fn test_operator_parsing() {
        let code = "+ - * / += -= == != <= >=";
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
        assert_eq!(token_values[4], Token::PlusEqual);
        assert_eq!(token_values[5], Token::MinusEqual);
        assert_eq!(token_values[6], Token::EqualEqual);
        assert_eq!(token_values[7], Token::BangEqual);
        assert_eq!(token_values[8], Token::LessEqual);
        assert_eq!(token_values[9], Token::GreaterEqual);
    }

    /// 测试关键字解析功能
    #[test]
    fn test_keyword_parsing() {
        let code = "fn let var with contract impl mut";
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
        assert_eq!(token_values[1], Token::Let);
        assert_eq!(token_values[2], Token::Var);
        assert_eq!(token_values[3], Token::With);
        assert_eq!(token_values[4], Token::Contract);
        assert_eq!(token_values[5], Token::Impl);
        assert_eq!(token_values[6], Token::Mut);
    }

    /// 测试错误处理功能
    #[test]
    fn test_error_handling() {
        let code = "let x = 42`;"; // ` 是未知字符
        let reader = BufReader::new(code.as_bytes());
        let lexer = Lexer::new("test.rs", reader);

        let tokens: Vec<Locatable<Result<Token, LexicalError>>> = lexer.collect();

        // 查找错误token
        let mut error_found = false;
        for token in &tokens {
            if let Err(e) = &token.data {
                match e {
                    LexicalError::UnknownCharacter(c) => {
                        assert_eq!(*c, '`');
                        error_found = true;
                        break;
                    }
                    _ => panic!("Expected UnknownCharacter error"),
                }
            }
        }

        assert!(error_found, "Expected an error token but none was found");
    }
}
