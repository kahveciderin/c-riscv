use crate::parser::ParserScopeState;

use super::{declaration::Declaration, statement::Statement};

#[derive(Debug, Clone)]
pub enum ScopeItem {
    Statement(Statement),
    Declaration(Declaration),
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub items: Vec<ScopeItem>,
}
