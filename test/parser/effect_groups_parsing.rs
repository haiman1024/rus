use rus::data::{Locatable, Token};
use rus::lex::Lexer;
use rus::parser::{Parser, Stmt};
use std::io::BufReader;

#[test]
fn test_effect_group_declaration() {
    let code = r#"
        effect_group FileSystemEffects = FileIO, SystemIO, DiskAccess;
    "#;

    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable| Locatable {
            location: locatable.location,
            data: locatable.data.unwrap(),
        })
        .collect();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");

    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::EffectGroup { declaration, .. } => {
            assert_eq!(declaration.name, "FileSystemEffects");
            assert_eq!(declaration.effects.len(), 3);
            assert_eq!(declaration.effects[0], "FileIO");
            assert_eq!(declaration.effects[1], "SystemIO");
            assert_eq!(declaration.effects[2], "DiskAccess");
        }
        _ => panic!("Expected effect group statement"),
    }
}

#[test]
fn test_handler_group_declaration() {
    let code = r#"
        handler_group WebServiceHandlers = ReadOnlyHandler, DefaultLogger, CacheHandler, MetricsHandler;
    "#;

    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable| Locatable {
            location: locatable.location,
            data: locatable.data.unwrap(),
        })
        .collect();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");

    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::HandlerGroup { declaration, .. } => {
            assert_eq!(declaration.name, "WebServiceHandlers");
            assert_eq!(declaration.handlers.len(), 4);
            assert_eq!(declaration.handlers[0], "ReadOnlyHandler");
            assert_eq!(declaration.handlers[1], "DefaultLogger");
            assert_eq!(declaration.handlers[2], "CacheHandler");
            assert_eq!(declaration.handlers[3], "MetricsHandler");
        }
        _ => panic!("Expected handler group statement"),
    }
}

#[test]
fn test_function_with_effects() {
    let code = r#"
        fn process_request() -> Response effects FileSystemEffects, NetworkEffects {
        }
    "#;

    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable| Locatable {
            location: locatable.location,
            data: locatable.data.unwrap(),
        })
        .collect();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");

    assert_eq!(statements.len(), 1);
    match &statements[0] {
        Stmt::Function { name, .. } => {
            assert_eq!(name, "process_request");
        }
        _ => panic!("Expected function statement"),
    }
}

#[test]
fn test_multiple_declarations() {
    let code = r#"
        effect_group Group1 = Effect1, Effect2;
        handler_group Group2 = Handler1, Handler2;

        fn test_fn() effects Group1 {
        }
    "#;

    let reader = BufReader::new(code.as_bytes());
    let lexer = Lexer::new("test.rs", reader);
    let tokens: Vec<Locatable<Token>> = lexer
        .map(|locatable| Locatable {
            location: locatable.location,
            data: locatable.data.unwrap(),
        })
        .collect();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Failed to parse");

    assert_eq!(statements.len(), 3);

    // 检查第一个声明是EffectGroup
    match &statements[0] {
        Stmt::EffectGroup { declaration, .. } => {
            assert_eq!(declaration.name, "Group1");
            assert_eq!(declaration.effects.len(), 2);
        }
        _ => panic!("Expected effect group statement as first statement"),
    }

    // 检查第二个声明是HandlerGroup
    match &statements[1] {
        Stmt::HandlerGroup { declaration, .. } => {
            assert_eq!(declaration.name, "Group2");
            assert_eq!(declaration.handlers.len(), 2);
        }
        _ => panic!("Expected handler group statement as second statement"),
    }

    // 检查第三个声明是Function
    match &statements[2] {
        Stmt::Function { name, .. } => {
            assert_eq!(name, "test_fn");
        }
        _ => panic!("Expected function statement as third statement"),
    }
}
