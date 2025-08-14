//! 基本Token测试
//! 测试词法分析器对基本Token的识别能力

use rus::data::Token;
use rus::lex::Lexer;
use std::io::BufReader;

#[test]
fn test_keywords() {
    let code = "fn let var with contract impl mut if else for in loop while match break continue return as use pub enum struct trait true false async await try";
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

    assert_eq!(token_values[0], Token::Fn);
    assert_eq!(token_values[1], Token::Let);
    assert_eq!(token_values[2], Token::Var);
    assert_eq!(token_values[3], Token::With);
    assert_eq!(token_values[4], Token::Contract);
    assert_eq!(token_values[5], Token::Impl);
    assert_eq!(token_values[6], Token::Mut);
    assert_eq!(token_values[7], Token::If);
    assert_eq!(token_values[8], Token::Else);
    assert_eq!(token_values[9], Token::For);
    assert_eq!(token_values[10], Token::In);
    assert_eq!(token_values[11], Token::Loop);
    assert_eq!(token_values[12], Token::While);
    assert_eq!(token_values[13], Token::Match);
    assert_eq!(token_values[14], Token::Break);
    assert_eq!(token_values[15], Token::Continue);
    assert_eq!(token_values[16], Token::Return);
    assert_eq!(token_values[17], Token::As);
    assert_eq!(token_values[18], Token::Use);
    assert_eq!(token_values[19], Token::Pub);
    assert_eq!(token_values[20], Token::Enum);
    assert_eq!(token_values[21], Token::Struct);
    assert_eq!(token_values[22], Token::Trait);
    assert_eq!(token_values[23], Token::True);
    assert_eq!(token_values[24], Token::False);
    assert_eq!(token_values[25], Token::Async);
    assert_eq!(token_values[26], Token::Await);
    assert_eq!(token_values[27], Token::Try);
}

#[test]
fn test_operators() {
    let code = "+ - * / % += -= *= /= %= == != < > <= >= && || & | ^ ! << >> = -> => :: .. ..= ";
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

    assert_eq!(token_values[0], Token::Plus);
    assert_eq!(token_values[1], Token::Minus);
    assert_eq!(token_values[2], Token::Star);
    assert_eq!(token_values[3], Token::Slash);
    assert_eq!(token_values[4], Token::Percent);
    assert_eq!(token_values[5], Token::PlusEqual);
    assert_eq!(token_values[6], Token::MinusEqual);
    assert_eq!(token_values[7], Token::StarEqual);
    assert_eq!(token_values[8], Token::SlashEqual);
    assert_eq!(token_values[9], Token::PercentEqual);
    assert_eq!(token_values[10], Token::EqualEqual);
    assert_eq!(token_values[11], Token::BangEqual);
    assert_eq!(token_values[12], Token::Less);
    assert_eq!(token_values[13], Token::Greater);
    assert_eq!(token_values[14], Token::LessEqual);
    assert_eq!(token_values[15], Token::GreaterEqual);
    assert_eq!(token_values[16], Token::And);
    assert_eq!(token_values[17], Token::Or);
    assert_eq!(token_values[18], Token::Ampersand);
    assert_eq!(token_values[19], Token::Pipe);
    assert_eq!(token_values[20], Token::Caret);
    assert_eq!(token_values[21], Token::Bang);
    assert_eq!(token_values[22], Token::Shl);
    assert_eq!(token_values[23], Token::Shr);
    assert_eq!(token_values[24], Token::Equal);
    assert_eq!(token_values[25], Token::Arrow);
    assert_eq!(token_values[26], Token::FatArrow);
    assert_eq!(token_values[27], Token::PathSep);
    assert_eq!(token_values[28], Token::Range);
    assert_eq!(token_values[29], Token::RangeInclusive);
    
    // 确保没有更多token
    assert_eq!(token_values.len(), 30);
}
