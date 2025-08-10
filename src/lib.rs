//! rus - A Rust compiler written in Rust
//!
//! This crate provides the core functionality for the rus compiler,
//! including lexical analysis capabilities.

pub mod data;
pub mod lex;

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::empty;

    #[test]
    fn test_lexer_creation() {
        let mut chars = empty::<char>();
        let lexer = lex::Lexer::new("test.rs", &mut chars);
        assert_eq!(lexer.location.line, 1);
        assert_eq!(lexer.location.column, 1);
        assert_eq!(lexer.location.file, "test.rs");
    }
}
