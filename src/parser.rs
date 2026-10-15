use crate::{
    ast::{Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
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
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::EndOfFile && !self.current_token_is(Token::EndOfFile) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement)
            }
            self.next_token();
        }

        program
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn peek_error(&mut self, expected: &str) {
        let message = format!(
            "expected next token to be {expected}, got {:?} instead",
            self.peek_token
        );
        self.errors.push(message);
    }

    fn next_token(&mut self) {
        let new_token = self.lexer.next_token();
        let old_peek = std::mem::replace(&mut self.peek_token, new_token);
        self.current_token = old_peek;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn current_token_is(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        self.peek_token == *token
    }

    fn expect_peek(&mut self, token: &Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            self.peek_error(&format!("{token:?}"));
            false
        }
    }

    fn expect_peek_identifier(&mut self) -> Option<Identifier> {
        let Token::Identifier(name) = &self.peek_token else {
            return None;
        };
        let identifier = Identifier(name.clone());
        self.next_token();
        Some(identifier)
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let name = self.expect_peek_identifier()?;

        if !self.expect_peek(&Token::Assign) {
            return None;
        }
        // TODO: We're skipping the expressions until we encounter a semicolon
        while !self.current_token_is(Token::Semicolon) && !self.current_token_is(Token::EndOfFile) {
            self.next_token();
        }

        // TODO: Add value once we start parsing expressions
        Some(Statement::Let(LetStatement { name, value: None }))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        // TODO: We're skipping the expressions until we encounter a semicolon
        while !self.current_token_is(Token::Semicolon) && !self.current_token_is(Token::EndOfFile) {
            self.next_token();
        }

        Some(Statement::Return)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.is_empty() {
            return;
        }
        let mut message = format!("parser has {} errors", parser.errors().len());
        for error in errors {
            message.push_str(&format!("\nparser error: {error}"));
        }
        panic!("{message}");
    }

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
        check_parser_errors(&parser);
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

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        let count = program.statements.len();

        assert_eq!(
            count, 3,
            "program.statements does not contain 3 statements, received {count}"
        );

        for (i, statement) in program.statements.iter().enumerate() {
            assert_eq!(
                statement.token_literal(),
                "return",
                "tests[{i}] - token literal not 'return'"
            );
            assert!(
                matches!(statement, Statement::Return),
                "tests[{i}] - statement not a return statement, got {statement:?}"
            )
        }
    }

    #[test]
    fn test_let_statement_without_semicolon() {
        let lexer = Lexer::new("let x = 5");
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        assert_let_statement(0, &program.statements[0], "x");
    }

    #[test]
    fn test_return_statement_without_semicolon() {
        let lexer = Lexer::new("return 5");
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(program.statements, [Statement::Return]);
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

        assert_eq!(
            let_statement.name.token_literal(),
            expected_name,
            "tests[{i}] - name.token_literal() wrong"
        );
    }
}
