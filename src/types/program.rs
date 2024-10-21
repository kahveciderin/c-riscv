use super::function_definition::FunctionDefinition;

#[derive(Debug)]
pub struct Program<'s> {
    pub functions: Vec<FunctionDefinition<'s>>,
}
