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
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.right)
    }
}

impl TokenLiteral for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Self::Identifier(identifier) => &identifier.0,
            Self::Integer(integer_literal) => &integer_literal.literal,
            Self::Prefix(prefix_expression) => &prefix_expression.operator,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Integer(intiger_literal) => write!(f, "{intiger_literal}"),
            Self::Prefix(prefix_expression) => write!(f, "({prefix_expression})",),
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
            Self::Let(_) => "let",
            Self::Return(_) => "return",
            Self::Expression(statement) => statement.expression.token_literal(),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
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
