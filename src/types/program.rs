use super::function_definition::{FunctionDeclaration, FunctionDefinition};

#[derive(Debug)]
pub enum ProgramStatement<'s> {
    FunctionDefinition(FunctionDefinition<'s>),
    FunctionDeclaration(FunctionDeclaration<'s>),
}

#[derive(Debug)]
pub struct Program<'s> {
    pub functions: Vec<ProgramStatement<'s>>,
}
