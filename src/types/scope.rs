use crate::parser::ParserScopeState;

use super::{declaration::Declaration, statement::Statement};

#[derive(Debug)]
pub enum ScopeItem {
    Statement(Statement),
    Declaration(Declaration),
}

#[derive(Debug)]
pub struct Scope {
    pub items: Vec<ScopeItem>,
    pub scope_state: ParserScopeState,
}
