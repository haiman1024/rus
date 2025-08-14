//! 字符串和字符解析测试
//! 测试词法分析器对字符串和字符字面量的解析能力

use rus::data::{LexicalError, Token};
use rus::lex::Lexer;
use std::io::BufReader;

#[test]
fn test_string_literals() {
    let code = r#""hello" "world\n" "with \"quotes\"""#;
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);

    let tokens: Vec<_> = lexer.collect();

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

#[test]
fn test_char_literals() {
    let code = r"'a' '\n' '\''";
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);

    let tokens: Vec<_> = lexer.collect();

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

#[test]
fn test_error_handling() {
    // 测试未知字符错误
    let code = "let x = 42`;";
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);

    let tokens: Vec<_> = lexer.collect();

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
