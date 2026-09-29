use crate::token::{self, Token};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    char: u8,             // character under examination
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token != Token::Eof {
            Some(token)
        } else {
            None
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            char: 0,
        };
        lexer.read_char(); // load the first character
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();

        if self.is_letter() {
            return self.read_identifier();
        }

        if self.char.is_ascii_digit() {
            return self.read_number();
        }

        let token = match self.char {
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'!' => Token::Bang,
            b'-' => Token::Minus,
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            0 => Token::Eof,
            _ => Token::Illegal,
        };
        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        self.char = if self.read_position >= self.input.len() {
            // 0 means "end of input"
            0
        } else {
            self.input.as_bytes()[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_white_space(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn is_letter(&self) -> bool {
        self.char.is_ascii_alphabetic() || self.char == b'_'
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.is_letter() {
            self.read_char();
        }
        token::lookup_identifier(self.input[start..self.position].into())
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.char.is_ascii_digit() {
            self.read_char();
        }

        Token::Int(self.input[start..self.position].into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = [
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for (i, expected) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                &tok, expected,
                "tests[{i}] - token wrong. expected={expected:?}, got={tok:?}"
            );
        }
    }

    #[test]
    fn test_next_token_with_literal() {
        let input = "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
    ";
        let tests = [
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::LParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for (i, expected) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                &tok, expected,
                "tests[{i}] - token wrong. expected={expected:?}, got={tok:?}"
            );
        }
    }

    #[test]
    fn test_next_token_with_literal_and_operators() {
        let input = "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
            return true;
            } else {
            return false;
            }
    ";
        let tests = [
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::LParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::Gt,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for (i, expected) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                &tok, expected,
                "tests[{i}] - token wrong. expected={expected:?}, got={tok:?}"
            );
        }
    }
}
