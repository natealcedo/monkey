# monkey

[![CI](https://github.com/natealcedo/monkey/actions/workflows/ci.yml/badge.svg)](https://github.com/natealcedo/monkey/actions/workflows/ci.yml)

A Rust implementation of the Monkey programming language from Thorsten Ball's
[*Writing An Interpreter In Go*](https://interpreterbook.com/).

This is a learning project: the book's Go code is ported to Rust, using Rust
idioms where they fit (for example, tokens are an enum whose variants carry
their data, instead of a type/literal struct).

## Status

- [x] Lexer
- [ ] Parser
  - [x] `let` and `return` statements (values are skipped for now)
  - [x] Expression statements
  - [x] Identifiers and integer literals
  - [x] Prefix operators (`!`, `-`)
  - [ ] Infix operators (`+ - * / < > == !=`) — test written, parsing in progress
  - [ ] Booleans, grouped expressions, `if`, functions and calls
- [ ] Evaluator
- [ ] REPL (currently prints the tokens for each line)

## Monkey at a glance

```monkey
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

if (5 < 10) {
    return true;
} else {
    return false;
}
```

## Design notes

- **Tokens** are an enum; `Identifier` and `Integer` carry their text.
- **The AST** uses enums for `Statement` and `Expression`. Every variant
  wraps a named struct (`Expression::Prefix(PrefixExpression)`), and each
  struct implements `Display` and `TokenLiteral`.
- **The Pratt parser** picks a prefix parse function with a `match` on the
  current token instead of the book's map of registered functions.
- **Parse functions** return `Option`; failures are recorded in
  `Parser::errors()`.

## Project layout

```
src/
├── lib.rs     # library crate root
├── token.rs   # Token enum and keyword lookup
├── lexer.rs   # turns source text into tokens
├── ast.rs     # AST nodes, Display and TokenLiteral
├── parser.rs  # Pratt parser from tokens to the AST
├── repl.rs    # read-eval-print loop
└── main.rs    # binary entry point
```

## Development

Requires Rust with the 2024 edition (Rust 1.85 or newer).

```sh
cargo build
cargo test
cargo run              # start the REPL
cargo fmt
cargo clippy           # pedantic lints are enabled in Cargo.toml
```

CI runs `cargo fmt --check`, `cargo clippy` and `cargo test` as separate
jobs on every push to `main` and on pull requests.
