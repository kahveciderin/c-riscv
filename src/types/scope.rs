use super::{declaration::Declaration, statement::Statement};

#[derive(Debug, Clone)]
pub enum ScopeItem {
    Statement(Statement),
    Declaration(Declaration),
    Label(Label),
}

#[derive(Debug, Clone)]
pub enum Label {
    Named(String),
    Case { id: String, value: i32 },
    Default { id: String },
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub items: Vec<ScopeItem>,
}
