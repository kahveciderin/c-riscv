use super::expression::Expression;

#[derive(Debug)]
pub enum JumpStatement {
    Return { expression: Option<Expression> },
}

#[derive(Debug)]
pub enum Statement {
    Jump { statement: JumpStatement },
    Expression { expression: Expression },
    Null,
}
