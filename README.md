# rus

A compiler for the Rus language.

## Description

Rus is a new systems programming language that follows the core axiom "x(data) --- (behavior) --> y(effect)" and implements the three core concepts of "strong controllability, fine granularity, and flexibility".

This project aims to provide a platform for learning and developing compilers, helping to understand compilation principles and language implementation.

The language is built upon one main axis and three pillars:

- **Main Axis**: **Effect System** - Explicitly declare and manage program side effects
- **Three Pillars**:
  - **Algebraic Effects and Handlers**: Decouple side effects from core business logic
  - **Contract System**: Define and verify code behavior through contracts
  - **Linear Types and Region System**: Ensure memory safety and deterministic resource management

## Features

### Lexical Analysis

- Complete lexical analysis for the Rus language syntax
- Token recognition for various literals:
  - Integer literals (decimal, hexadecimal, octal, binary)
  - Float literals (including scientific notation)
  - String literals (with escape sequences)
  - Character literals (with escape sequences)
- Identifier and keyword recognition (complete Rus keyword set)
- Operator and symbol recognition:
  - Arithmetic operators (+, -, *, /, %, +=, -=,*=, /=, %=)
  - Bitwise operators (&, |, ^, <<, >>, &=, |=, ^=, <<=, >>=)
  - Comparison operators (==, !=, <, >, <=, >=)
  - Logical operators (&&, ||)
  - Other symbols (., .., ..=, =>, ->, ::, :, ;, ,, @, _, #, $, ?)
- Special token recognition (&mut as atomic token for linear types)
- Improved error handling with detailed lexical error types
- Error reporting with precise location information
- Comprehensive test suite covering various language features
- Strongly typed token system following language design philosophy

### Parsing

- Recursive descent parser implementation
- Pratt parsing for operator precedence and associativity
- Support for core language constructs:
  - Effect declarations (`effect`)
  - Handler declarations (`handle`)
  - Effect group declarations (`effect_group`)
  - Handler group declarations (`handler_group`)
  - Function declarations (`fn`)
  - Variable declarations (`let`, `var`)
- Expression parsing:
  - Literals (integer, float, string, character, boolean)
  - Identifiers
  - Function calls
  - Binary operations (arithmetic, comparison, logical)
  - Unary operations (negation, logical NOT)
  - Grouping expressions
  - Effect operations (`effect.operation(...)`)
- Statement parsing:
  - Expression statements
  - Block statements
- Panic mode error recovery for better error reporting

### Language Constructs (Planned)

- Struct declarations (`struct`)
- Enum declarations (`enum`)
- Trait declarations (`trait`)
- Implementation blocks (`impl`)
- With statements (`with`)
- Resume expressions (`resume!`)
- Conditional expressions (`if`)
- Pattern matching expressions (`match`)
- Loop statements (`loop`, `while`, `for`)
- Control flow statements (`break`, `continue`, `return`)

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

For release builds:

```bash
cargo build --release
```

## Testing

To run all tests:

```bash
cargo test
```

To run specific test suites:

```bash
# Run lexer tests
cargo test --test lexer_tests
cargo test --test number_parsing_tests
cargo test --test string_parsing_tests

# Run parser tests
cargo test --test parser_basic_tests
cargo test --test parser_declaration_tests
cargo test --test parser_algebraic_effects_tests
cargo test --test parser_effect_groups_tests

# Run integration tests
cargo test --test integration_tests
```

## Documentation

For detailed information about the language design, implementation progress, and development notes, see [dev.md](docs/dev.md).

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
