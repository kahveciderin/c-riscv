use super::{datatype::Datatype, scope::Scope};

#[derive(Debug)]
pub struct FunctionDefinition<'s> {
    pub name: &'s str,
    // pub arguments: Vec<&'s str>,
    pub return_type: Datatype,
    pub body: Scope,
}
