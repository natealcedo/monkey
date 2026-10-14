use crate::{ast::Program, lexer::Lexer, token::Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        // Read two tokens so that current_token and peek_token are set
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        let new_token = self.lexer.next_token();
        let old_peek = std::mem::replace(&mut self.peek_token, new_token);
        self.current_token = old_peek;
    }

    pub fn parse_program(&mut self) -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}
