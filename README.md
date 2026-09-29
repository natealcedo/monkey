# monkey

A Rust implementation of the Monkey programming language from Thorsten Ball's
[*Writing An Interpreter In Go*](https://interpreterbook.com/).

This is a learning project: the book's Go code is ported to Rust, using Rust
idioms where they fit (for example, tokens are an enum whose variants carry
their data, instead of a type/literal struct).

## Status

- [ ] Lexer (everything except `==` and `!=`)
- [ ] Parser
- [ ] Evaluator
- [ ] REPL

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

## Project layout

```
src/
├── lib.rs     # library crate root
├── token.rs   # Token enum and keyword lookup
├── lexer.rs   # turns source text into tokens
└── main.rs    # binary entry point
```

## Development

Requires Rust with the 2024 edition (Rust 1.85 or newer).

```sh
cargo build
cargo test
cargo run
```
