use std::fmt::{self, Display};

pub trait TokenLiteral {
    fn token_literal(&self) -> &str;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}

impl TokenLiteral for Program {
    fn token_literal(&self) -> &str {
        self.statements.first().map_or("", |s| s.token_literal())
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
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let(statement) => write!(f, "{statement}"),
            Self::Return(statement) => write!(f, "{statement}"),
            Self::Expression(statement) => write!(f, "{statement}"),
        }
    }
}

impl TokenLiteral for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Self::Let(statement) => statement.token_literal(),
            Self::Return(statement) => statement.token_literal(),
            Self::Expression(statement) => statement.token_literal(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    // TODO: Make this a plain expression once we start parsing expressions
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

impl TokenLiteral for LetStatement {
    fn token_literal(&self) -> &'static str {
        "let"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement {
    // TODO: Make this a plain expression once we start parsing expressions
    pub value: Option<Expression>,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return ")?;
        if let Some(value) = &self.value {
            write!(f, "{value}")?;
        }
        write!(f, ";")
    }
}

impl TokenLiteral for ReturnStatement {
    fn token_literal(&self) -> &'static str {
        "return"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl TokenLiteral for ExpressionStatement {
    fn token_literal(&self) -> &str {
        self.expression.token_literal()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Integer(integer_literal) => write!(f, "{integer_literal}"),
            Self::Prefix(prefix_expression) => write!(f, "{prefix_expression}"),
        }
    }
}

impl TokenLiteral for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Self::Identifier(identifier) => identifier.token_literal(),
            Self::Integer(integer_literal) => integer_literal.token_literal(),
            Self::Prefix(prefix_expression) => prefix_expression.token_literal(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TokenLiteral for Identifier {
    fn token_literal(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub value: i64,
    pub literal: String,
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

impl TokenLiteral for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.literal
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl TokenLiteral for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.operator
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    name: Identifier("myVar".into()),
                    value: Some(Expression::Identifier(Identifier("anotherVar".into()))),
                }),
                Statement::Return(ReturnStatement {
                    value: Some(Expression::Integer(IntegerLiteral {
                        value: 5,
                        literal: "5".into(),
                    })),
                }),
                Statement::Expression(ExpressionStatement {
                    expression: Expression::Prefix(PrefixExpression {
                        operator: "-".into(),
                        right: Box::new(Expression::Identifier(Identifier("x".into()))),
                    }),
                }),
            ],
        };
        assert_eq!(program.to_string(), "let myVar = anotherVar;return 5;(-x)");
    }
}
