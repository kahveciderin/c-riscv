use std::sync::Arc;

use crate::parser::expression::fold::Fold;

use super::expression::Expression;

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
    Array {
        inner: Arc<Datatype>,
        length: Expression,
    },
}

impl Datatype {
    pub fn size(&self) -> usize {
        match self {
            Datatype::Int => 4,
            Datatype::Function { .. } => 0, // Functions don't have a size
            Datatype::Pointer { .. } => 4,
            Datatype::Array { inner, length } => {
                let inner_size = inner.size();
                let length_fold = length.fold();

                if let Some(length) = length_fold {
                    inner_size * length as usize
                } else {
                    todo!("vla")
                }
            }
        }
    }
}
