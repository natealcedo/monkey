use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    char: u8,             // character under examination
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

    fn next_token(&mut self) -> Token {
        let token = match self.char {
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            0 => Token::EOF,
            _ => Token::ILLEGAL,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = [
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
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
