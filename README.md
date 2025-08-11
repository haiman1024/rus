# rus

A Rust compiler written in Rust.

## Description

This is a simple compiler project written in Rust. It demonstrates the basic principles of lexical analysis for a subset of the Rust language.

## Features

- Lexical analysis for basic arithmetic expressions
- Token recognition for numbers and operators (+, -, *, /, +=, -=, ==, =)
- Token recognition for string literals and character literals
- Identifier and keyword recognition (complete Rust keyword set)
- Improved character literal parsing with better error handling
- Error reporting with location information
- Integer overflow detection during parsing

## Building

To build the project:

```bash
cargo build
