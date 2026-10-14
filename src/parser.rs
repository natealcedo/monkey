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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        let count = program.statements.len();

        assert_eq!(
            count, 3,
            "program.statements does not contain 3 statements, received {count}"
        );

        let expected_identifiers = ["x", "y", "foobar"];

        for (i, (statement, expected)) in program
            .statements
            .iter()
            .zip(expected_identifiers)
            .enumerate()
        {
            assert_let_statement(i, statement, expected)
        }
    }

    fn assert_let_statement(i: usize, statement: &Statement, expected_name: &str) {
        assert_eq!(
            statement.token_literal(),
            "let",
            "tests[{i}] - token_literal not 'let'"
        );

        let Statement::Let(let_statement) = statement else {
            panic!("tests[{i}] - statement not a let statement, got {statement:?}");
        };

        assert_eq!(
            let_statement.name.0, expected_name,
            "tests[{i}] - name wrong"
        );
    }
}
