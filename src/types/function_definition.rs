use crate::parser::ParserScopeState;

use super::{datatype::Datatype, scope::Scope};

#[derive(Debug)]
pub struct FunctionArgument {
    pub name: String,
    pub unique_name: String,
    pub datatype: Datatype,
}

#[derive(Debug, Clone)]
pub struct FunctionArgumentOptionalName {
    pub name: Option<String>,
    pub datatype: Datatype,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Datatype,
    pub body: Scope,
    pub scope_state: ParserScopeState,
}
