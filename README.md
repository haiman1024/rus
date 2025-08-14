# rus

A compiler for the Rus language.

## Description

Rus is a new systems programming language that follows the core axiom "x(data) --- (behavior) --> y(effect)" and implements the three core concepts of "strong controllability, fine granularity, and flexibility".

This project aims to provide a platform for learning and developing compilers, helping to understand compilation principles and language implementation.

## Features

- Complete lexical analysis for the Rus language syntax
- Token recognition for various literals:
  - Integer literals (decimal, hexadecimal, octal, binary)
  - Float literals (including scientific notation)
  - String literals (with escape sequences)
  - Character literals (with escape sequences)
- Identifier and keyword recognition (complete Rus keyword set)
- Operator and symbol recognition:
  - Arithmetic operators (+, -, *, /, %, +=, -=, *=, /=, %=)
  - Bitwise operators (&, |, ^, <<, >>, &=, |=, ^=, <<=, >>=)
  - Comparison operators (==, !=, <, >, <=, >=)
  - Logical operators (&&, ||)
  - Other symbols (., .., ..=, =>, ->, ::, :, ;, ,, @, _, #, $, ?)
- Special token recognition (&mut as atomic token for linear types)
- Improved error handling with detailed lexical error types
- Error reporting with precise location information
- Comprehensive test suite covering various language features
- Strongly typed token system following language design philosophy

## Building

To build the project:

```bash
cargo build
```

## Running

To run the project:

```bash
cargo run
```

## Testing

To run the tests:

```bash
cargo test
```

## Language Design Philosophy

This compiler is built upon a core axiom and a set of rigorous concepts:

- **Core Axiom**: `x(data) --- (behavior) --> y(effect)`
- **Three Core Concepts**:
  - **Strong Controllability**: Developers must fully control program behavior and side effects.
  - **Fine Granularity**: Language elements are finely divided and defined for precise control.
  - **Flexibility**: The language's underlying architecture is solid while reserving space for upper-level syntax innovation and expansion.

The architecture is based on one main axis and three pillars:
- **Main Axis**: `x(data) --- (behavior) --> y(effect)`
- **Three Pillars**:
  - **Effect System**: Explicitly declare and manage program side effects through keywords like `with`.
  - **Contract System**: Define and verify code behavior through the `contract` keyword.
  - **Linear Types and Region System**: Ensure memory safety and deterministic resource management through strict control of ownership and mutability.

For more detailed information about the design process and implementation, see [dev.md](docs/dev.md).