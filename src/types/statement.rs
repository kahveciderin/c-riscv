use std::sync::Arc;

use super::{expression::Expression, scope::Scope};

#[derive(Debug)]
pub enum JumpStatement {
    Return { expression: Option<Expression> },
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Arc<Statement>,
    pub else_block: Option<Arc<Statement>>,
}

#[derive(Debug)]
pub enum Statement {
    Jump { statement: JumpStatement },
    Expression { expression: Expression },
    Scope { scope: Scope },
    If { statement: IfStatement },
    Null,
}
