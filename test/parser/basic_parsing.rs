//! 基本语法解析测试
//! 测试语法分析器对基本语法结构的解析能力

use rus::data::{Token, LexicalError, Locatable};
use rus::lex::Lexer;
use rus::parser::{Parser, Expr, Stmt, Literal, BinaryOperator, UnaryOperator};
use std::io::BufReader;

#[test]
fn test_basic_expressions() {
    let code = "42;";
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable_result| Locatable {
            location: locatable_result.location,
            data: locatable_result.data.unwrap(),
        })
        .collect();
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Expression { expression, .. } => {
            match expression {
                Expr::Literal { value, .. } => {
                    match value {
                        Literal::Integer(val) => assert_eq!(val, "42"),
                        _ => panic!("Expected integer literal"),
                    }
                }
                _ => panic!("Expected literal expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_binary_expressions() {
    let code = "1 + 2;";
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable_result| Locatable {
            location: locatable_result.location,
            data: locatable_result.data.unwrap(),
        })
        .collect();
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Expression { expression, .. } => {
            match expression {
                Expr::Binary { left, operator, right, .. } => {
                    match &**left {
                        Expr::Literal { value, .. } => {
                            match value {
                                Literal::Integer(val) => assert_eq!(val, "1"),
                                _ => panic!("Expected integer literal"),
                            }
                        }
                        _ => panic!("Expected literal expression"),
                    }
                    
                    assert_eq!(operator, &BinaryOperator::Add);
                    
                    match &**right {
                        Expr::Literal { value, .. } => {
                            match value {
                                Literal::Integer(val) => assert_eq!(val, "2"),
                                _ => panic!("Expected integer literal"),
                            }
                        }
                        _ => panic!("Expected literal expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_operator_precedence() {
    let code = "1 + 2 * 3;";
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable_result| Locatable {
            location: locatable_result.location,
            data: locatable_result.data.unwrap(),
        })
        .collect();
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Expression { expression, .. } => {
            // 应该解析为 1 + (2 * 3)
            match expression {
                Expr::Binary { left, operator, right, .. } => {
                    // 左操作数应该是1
                    match &**left {
                        Expr::Literal { value, .. } => {
                            match value {
                                Literal::Integer(val) => assert_eq!(val, "1"),
                                _ => panic!("Expected integer literal"),
                            }
                        }
                        _ => panic!("Expected literal expression"),
                    }
                    
                    assert_eq!(operator, &BinaryOperator::Add);
                    
                    // 右操作数应该是(2 * 3)
                    match &**right {
                        Expr::Binary { left, operator, right, .. } => {
                            assert_eq!(operator, &BinaryOperator::Multiply);
                            
                            match &**left {
                                Expr::Literal { value, .. } => {
                                    match value {
                                        Literal::Integer(val) => assert_eq!(val, "2"),
                                        _ => panic!("Expected integer literal"),
                                    }
                                }
                                _ => panic!("Expected literal expression"),
                            }
                            
                            match &**right {
                                Expr::Literal { value, .. } => {
                                    match value {
                                        Literal::Integer(val) => assert_eq!(val, "3"),
                                        _ => panic!("Expected integer literal"),
                                    }
                                }
                                _ => panic!("Expected literal expression"),
                            }
                        }
                        _ => panic!("Expected binary expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_unary_expressions() {
    let code = "-42;";
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable_result| Locatable {
            location: locatable_result.location,
            data: locatable_result.data.unwrap(),
        })
        .collect();
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Expression { expression, .. } => {
            match expression {
                Expr::Unary { operator, operand, .. } => {
                    assert_eq!(operator, &UnaryOperator::Negate);
                    
                    match &**operand {
                        Expr::Literal { value, .. } => {
                            match value {
                                Literal::Integer(val) => assert_eq!(val, "42"),
                                _ => panic!("Expected integer literal"),
                            }
                        }
                        _ => panic!("Expected literal expression"),
                    }
                }
                _ => panic!("Expected unary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_grouping_expressions() {
    let code = "(1 + 2) * 3;";
    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable_result| Locatable {
            location: locatable_result.location,
            data: locatable_result.data.unwrap(),
        })
        .collect();
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");
    
    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Expression { expression, .. } => {
            // 应该解析为 ((1 + 2) * 3)
            match expression {
                Expr::Binary { left, operator, right, .. } => {
                    assert_eq!(operator, &BinaryOperator::Multiply);
                    
                    // 左操作数应该是(1 + 2)
                    match &**left {
                        Expr::Grouping { expression, .. } => {
                            match &**expression {
                                Expr::Binary { left, operator, right, .. } => {
                                    assert_eq!(operator, &BinaryOperator::Add);
                                    
                                    match &**left {
                                        Expr::Literal { value, .. } => {
                                            match value {
                                                Literal::Integer(val) => assert_eq!(val, "1"),
                                                _ => panic!("Expected integer literal"),
                                            }
                                        }
                                        _ => panic!("Expected literal expression"),
                                    }
                                    
                                    match &**right {
                                        Expr::Literal { value, .. } => {
                                            match value {
                                                Literal::Integer(val) => assert_eq!(val, "2"),
                                                _ => panic!("Expected integer literal"),
                                            }
                                        }
                                        _ => panic!("Expected literal expression"),
                                    }
                                }
                                _ => panic!("Expected binary expression"),
                            }
                        }
                        _ => panic!("Expected grouping expression"),
                    }
                    
                    // 右操作数应该是3
                    match &**right {
                        Expr::Literal { value, .. } => {
                            match value {
                                Literal::Integer(val) => assert_eq!(val, "3"),
                                _ => panic!("Expected integer literal"),
                            }
                        }
                        _ => panic!("Expected literal expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}