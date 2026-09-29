#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PLUS,
    DIVIDE,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    // Delimiters
    COMMA,
    SEMICOLON,

    // Parentheses and braces
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

pub fn lookup_identifier(input: &str) -> Token {
    match input {
        "let" => Token::LET,
        "fn" => Token::FUNCTION,
        _ => Token::IDENT(input.into()),
    }
}
