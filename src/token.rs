use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    EndOfFile,

    // Identifiers + literals
    Identifier(String),
    Integer(String),

    // Operators
    Assign,
    Plus,
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Illegal => "ILLEGAL",
            Self::EndOfFile => "EOF",
            Self::Identifier(name) => name,
            Self::Integer(digits) => digits,
            Self::Assign => "=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Bang => "!",
            Self::Asterisk => "*",
            Self::Slash => "/",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::Function => "fn",
            Self::Let => "let",
            Self::True => "true",
            Self::False => "false",
            Self::If => "if",
            Self::Else => "else",
            Self::Return => "return",
        };
        write!(f, "{text}")
    }
}

#[must_use]
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
