use std::fmt::{self, Display};

pub trait TokenLiteral {
    fn token_literal(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

impl TokenLiteral for Identifier {
    fn token_literal(&self) -> &str {
        &self.0
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Integer {
        value: i64,
        literal: String,
    },
    Prefix {
        operator: String,
        right: Box<Expression>,
    },
}

impl TokenLiteral for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Self::Identifier(identifier) => &identifier.0,
            Self::Integer { literal, .. } => literal,
            Self::Prefix { operator, .. } => operator,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Integer { literal, .. } => write!(f, "{literal}"),
            Self::Prefix { operator, right } => write!(f, "({operator}{right})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

impl TokenLiteral for Node {
    fn token_literal(&self) -> &str {
        match self {
            Self::Expression(expression) => expression.token_literal(),
            Self::Statement(statement) => statement.token_literal(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return,
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let(statement) => write!(f, "{statement}"),
            Self::Return => write!(f, "return;"),
            Self::Expression(expression) => write!(f, "{expression}"),
        }
    }
}

impl TokenLiteral for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Self::Let(_) => "let",
            Self::Return => "return",
            Self::Expression(expression) => expression.token_literal(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    // TODO: Add back experession once we start parsing expressions
    pub value: Option<Expression>,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {} = ", self.name)?;
        if let Some(value) = &self.value {
            write!(f, "{value}")?;
        }
        write!(f, ";")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            write!(f, "{statement}")?
        }
        Ok(())
    }
}

impl TokenLiteral for Program {
    fn token_literal(&self) -> &str {
        self.statements.first().map_or("", |s| s.token_literal())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                name: Identifier("myVar".into()),
                value: Some(Expression::Identifier(Identifier("anotherVar".into()))),
            })],
        };
        assert_eq!(program.to_string(), "let myVar = anotherVar;")
    }
}
