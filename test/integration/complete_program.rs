//! 集成测试
//! 测试完整的程序代码解析

use rus::data::Token;
use rus::lex::Lexer;
use std::io::BufReader;

#[test]
fn test_complete_program() {
    let code = r#"
        fn main() {
            let x = 42;
            let y = 0xFF;
            let z = 0o77;
            let s = "Hello, 世界!";
            let c = '\n';
            
            if x == 42 {
                println!("x is {}", x);
            }
        }
    "#;
    
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

    // 验证关键Token
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
    // ... 更多验证可以添加
}