#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    // Identifiers + literals
    Identifier(String),
    Integer(String),

    // Operators
    Assign,
    Plus,
    Divide,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,

    // Parentheses and braces
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub fn lookup_identifier(input: &str) -> Token {
    match input {
        "let" => Token::Let,
        "fn" => Token::Function,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Identifier(input.into()),
    }
}
