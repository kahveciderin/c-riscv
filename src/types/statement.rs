use std::sync::Arc;

use crate::parser::Case;

use super::{declaration::Declaration, expression::Expression, scope::Scope};

#[derive(Debug, Clone)]
pub enum JumpStatement {
    Return { expression: Option<Expression> },
    Break { id: String },
    Continue { id: String },
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Arc<Statement>,
    pub else_block: Option<Arc<Statement>>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Arc<Statement>,
    pub id: String,
}

#[derive(Debug, Clone)]
pub enum ForInit {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub init: Option<ForInit>,
    pub condition: Option<Expression>,
    pub increment: Option<Expression>,
    pub block: Arc<Statement>,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct SwitchStatement {
    pub expression: Expression,
    pub body: Arc<Statement>,
    pub id: String,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Jump { statement: JumpStatement },
    Expression { expression: Expression },
    Scope { scope: Scope },
    If { statement: IfStatement },
    While { statement: WhileStatement },
    For { statement: ForStatement },
    Switch { statement: SwitchStatement },
    Null,
}
