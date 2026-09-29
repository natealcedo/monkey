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
        if token != Token::EndOfFile {
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
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'+' => Token::Plus,
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'-' => Token::Minus,
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            0 => Token::EndOfFile,
            _ => Token::Illegal,
        };
        self.read_char();
        return token;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
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

        Token::Integer(self.input[start..self.position].into())
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
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EndOfFile,
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
            Token::Identifier("five".into()),
            Token::Assign,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::Assign,
            Token::Integer("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".into()),
            Token::Plus,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("add".into()),
            Token::LeftParen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::RightParen,
            Token::Semicolon,
            Token::EndOfFile,
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
            10 == 10;
            10 != 9;
    ";
        let tests = [
            Token::Let,
            Token::Identifier("five".into()),
            Token::Assign,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::Assign,
            Token::Integer("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".into()),
            Token::Plus,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("add".into()),
            Token::LeftParen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::RightParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::Integer("5".into()),
            Token::LessThan,
            Token::Integer("10".into()),
            Token::GreaterThan,
            Token::Integer("5".into()),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Integer("5".into()),
            Token::LessThan,
            Token::Integer("10".into()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RightBrace,
            Token::Integer("10".into()),
            Token::Equal,
            Token::Integer("10".into()),
            Token::Semicolon,
            Token::Integer("10".into()),
            Token::NotEqual,
            Token::Integer("9".into()),
            Token::Semicolon,
            Token::EndOfFile,
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
