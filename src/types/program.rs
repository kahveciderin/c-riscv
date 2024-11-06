use super::function_definition::FunctionDefinition;

#[derive(Debug)]
pub enum ProgramStatement {
    FunctionDefinition(FunctionDefinition),
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<ProgramStatement>,
}
