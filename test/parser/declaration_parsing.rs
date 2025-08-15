//! 声明语句解析测试
//! 测试语法分析器对变量声明和函数声明的解析能力

use rus::data::{Token, Locatable};
use rus::lex::Lexer;
use rus::parser::{Parser, Stmt, Expr, Literal};
use std::io::BufReader;

#[test]
fn test_let_declaration() {
    let code = "let x = 42;";
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
        Stmt::Let { identifier, initializer, .. } => {
            assert_eq!(identifier, "x");
            
            if let Some(expr) = initializer {
                match expr {
                    Expr::Literal { value, .. } => {
                        match value {
                            Literal::Integer(val) => assert_eq!(val, "42"),
                            _ => panic!("Expected integer literal"),
                        }
                    }
                    _ => panic!("Expected literal expression"),
                }
            } else {
                panic!("Expected initializer");
            }
        }
        _ => panic!("Expected let statement"),
    }
}

#[test]
fn test_let_declaration_without_initializer() {
    let code = "let x;";
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
        Stmt::Let { identifier, initializer, .. } => {
            assert_eq!(identifier, "x");
            assert!(initializer.is_none());
        }
        _ => panic!("Expected let statement"),
    }
}

#[test]
fn test_var_declaration() {
    let code = "var x = 42;";
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
        Stmt::Var { identifier, initializer, .. } => {
            assert_eq!(identifier, "x");
            
            if let Some(expr) = initializer {
                match expr {
                    Expr::Literal { value, .. } => {
                        match value {
                            Literal::Integer(val) => assert_eq!(val, "42"),
                            _ => panic!("Expected integer literal"),
                        }
                    }
                    _ => panic!("Expected literal expression"),
                }
            } else {
                panic!("Expected initializer");
            }
        }
        _ => panic!("Expected var statement"),
    }
}

#[test]
fn test_function_declaration() {
    let code = "fn foo() { let x = 42; }";
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
        Stmt::Function { name, parameters, body, .. } => {
            assert_eq!(name, "foo");
            assert!(parameters.is_empty());
            assert_eq!(body.len(), 1);
            
            match &body[0] {
                Stmt::Let { identifier, initializer, .. } => {
                    assert_eq!(identifier, "x");
                    
                    if let Some(expr) = initializer {
                        match expr {
                            Expr::Literal { value, .. } => {
                                match value {
                                    Literal::Integer(val) => assert_eq!(val, "42"),
                                    _ => panic!("Expected integer literal"),
                                }
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    } else {
                        panic!("Expected initializer");
                    }
                }
                _ => panic!("Expected let statement in function body"),
            }
        }
        _ => panic!("Expected function statement"),
    }
}

#[test]
fn test_function_declaration_with_parameters() {
    let code = "fn add(a, b) { }";
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
        Stmt::Function { name, parameters, body, .. } => {
            assert_eq!(name, "add");
            assert_eq!(parameters, &["a", "b"]);
            assert_eq!(body.len(), 0);
        }
        _ => panic!("Expected function statement"),
    }
}

#[test]
fn test_block_statement() {
    let code = "{ let x = 1; let y = 2; }";
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
        Stmt::Block { statements, .. } => {
            assert_eq!(statements.len(), 2);
            
            match &statements[0] {
                Stmt::Let { identifier, .. } => {
                    assert_eq!(identifier, "x");
                }
                _ => panic!("Expected let statement"),
            }
            
            match &statements[1] {
                Stmt::Let { identifier, .. } => {
                    assert_eq!(identifier, "y");
                }
                _ => panic!("Expected let statement"),
            }
        }
        _ => panic!("Expected block statement"),
    }
}