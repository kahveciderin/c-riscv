use crate::parser::ParserScopeState;

use super::{datatype::Datatype, scope::Scope};

#[derive(Debug)]
pub struct FunctionArgument {
    pub name: String,
    pub unique_name: String,
    pub datatype: Datatype,
}

#[derive(Debug)]
pub struct FunctionDefinition<'s> {
    pub name: &'s str,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Datatype,
    pub body: Scope,
    pub scope_state: ParserScopeState,
}

#[derive(Debug)]
pub struct FunctionDeclaration<'s> {
    pub name: &'s str,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Datatype,
}
