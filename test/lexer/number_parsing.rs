//! 数字解析测试
//! 测试词法分析器对各种数字格式的解析能力

use rus::data::Token;
use rus::lex::Lexer;
use std::io::BufReader;

#[test]
fn test_integer_formats() {
    let code = "42 0xFF 0o77 0b1010";
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
    let token_values: Vec<Token> = tokens
        .into_iter()
        .map(|t| t.data.unwrap())
        .collect();

    assert_eq!(token_values[0], Token::IntegerLiteral("42".to_string()));
    assert_eq!(token_values[1], Token::IntegerLiteral("0xFF".to_string()));
    assert_eq!(token_values[2], Token::IntegerLiteral("0o77".to_string()));
    assert_eq!(token_values[3], Token::IntegerLiteral("0b1010".to_string()));
}

#[test]
fn test_float_formats() {
    let code = "3.14 1.23e10 1.23E-5";
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
    let token_values: Vec<Token> = tokens
        .into_iter()
        .map(|t| t.data.unwrap())
        .collect();

    assert_eq!(token_values[0], Token::FloatLiteral("3.14".to_string()));
    assert_eq!(token_values[1], Token::FloatLiteral("1.23e10".to_string()));
    assert_eq!(token_values[2], Token::FloatLiteral("1.23E-5".to_string()));
}