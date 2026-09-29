pub trait TokenLiteral {
    fn token_literal(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
}

impl TokenLiteral for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Self::Identifier(identifier) => &identifier.0,
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
}

impl TokenLiteral for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Self::Let(_) => "let",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl TokenLiteral for Program {
    fn token_literal(&self) -> &str {
        self.statements.first().map_or("", |s| s.token_literal())
    }
}
