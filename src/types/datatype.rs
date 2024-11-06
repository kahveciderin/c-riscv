use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub datatype: Arc<Datatype>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Datatype {
    Int,
    Function {
        return_type: Arc<Datatype>,
        arguments: Vec<Argument>,
    },
    Pointer {
        inner: Arc<Datatype>,
    },
}

impl Datatype {
    pub fn size(&self) -> usize {
        match self {
            Datatype::Int => 4,
            Datatype::Function { .. } => 0, // Functions don't have a size
            Datatype::Pointer { .. } => 4,
        }
    }
}
