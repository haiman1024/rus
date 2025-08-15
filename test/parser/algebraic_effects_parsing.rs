// 代数效应和处理器语法解析测试
// 测试语法分析器对effect和handle关键字的解析能力

use rus::data::{Token, Locatable};
use rus::lex::Lexer;
use rus::parser::{Parser, Stmt, Expr};
use std::io::BufReader;

#[test]
fn test_effect_declaration() {
    let code = r#"
        effect FileSystem {
            fn read_file(path: string) -> string;
            fn write_file(path: string, content: string) -> unit;
        }
    "#;
    
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
        Stmt::Effect { declaration, .. } => {
            assert_eq!(declaration.name, "FileSystem");
            assert_eq!(declaration.operations.len(), 2);
            
            let read_op = &declaration.operations[0];
            assert_eq!(read_op.name, "read_file");
            assert_eq!(read_op.parameters.len(), 1);
            assert_eq!(read_op.parameters[0].0, "path");
            assert_eq!(read_op.parameters[0].1, "string");
            assert_eq!(read_op.return_type.as_ref().unwrap(), "string");
            
            let write_op = &declaration.operations[1];
            assert_eq!(write_op.name, "write_file");
            assert_eq!(write_op.parameters.len(), 2);
            assert_eq!(write_op.parameters[0].0, "path");
            assert_eq!(write_op.parameters[0].1, "string");
            assert_eq!(write_op.parameters[1].0, "content");
            assert_eq!(write_op.parameters[1].1, "string");
            assert_eq!(write_op.return_type.as_ref().unwrap(), "unit");
        }
        _ => panic!("Expected effect statement"),
    }
}

#[test]
fn test_handler_declaration() {
    // 使用一个非常简单的处理器声明来测试
    let code = r#"
        handle FileSystem {
            read_file(path) {
                content;
            }
        }
    "#;
    
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
        Stmt::Handler { declaration, .. } => {
            assert_eq!(declaration.effect, "FileSystem");
            assert_eq!(declaration.clauses.len(), 1);
            
            let read_clause = &declaration.clauses[0];
            assert_eq!(read_clause.operation, "read_file");
            assert_eq!(read_clause.parameters, vec!["path"]);
            // 简化断言，只检查语句数量
            assert_eq!(read_clause.body.len(), 1);
        }
        _ => panic!("Expected handler statement"),
    }
}

#[test]
fn test_complex_handler_declaration() {
    let code = r#"
        handle FileSystem {
            read_file(path) {
                let x = 42;
                x;
            }
        }
    "#;
    
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
        Stmt::Handler { declaration, .. } => {
            assert_eq!(declaration.effect, "FileSystem");
            assert_eq!(declaration.clauses.len(), 1);
            
            let read_clause = &declaration.clauses[0];
            assert_eq!(read_clause.operation, "read_file");
            assert_eq!(read_clause.parameters, vec!["path"]);
            assert_eq!(read_clause.body.len(), 2);
        }
        _ => panic!("Expected handler statement"),
    }
}

#[test]
fn test_effect_operation_call() {
    let code = r#"FileSystem.read_file("test.txt");"#;
    
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
                Expr::EffectOperation { effect, operation, arguments, .. } => {
                    assert_eq!(effect, "FileSystem");
                    assert_eq!(operation, "read_file");
                    assert_eq!(arguments.len(), 1);
                }
                _ => panic!("Expected effect operation expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}