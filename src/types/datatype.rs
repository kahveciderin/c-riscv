use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Datatype {
    Int,
    FunctionPointer {
        return_type: Arc<Datatype>,
        arguments: Vec<Datatype>,
    },
}

impl Datatype {
    pub fn size(&self) -> usize {
        match self {
            Datatype::Int => 4,
            Datatype::FunctionPointer { .. } => 4,
        }
    }
}
