use super::{
    declaration::Declaration, function_definition::FunctionDeclaration, statement::Statement,
};

#[derive(Debug, Clone)]
pub enum ScopeItem {
    Statement(Statement),
    Declaration(Declaration),
    FunctionDeclaration(FunctionDeclaration),
    Label(Label),
}

#[allow(dead_code)]
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
