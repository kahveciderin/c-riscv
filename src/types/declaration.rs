use super::{datatype::Datatype, expression::Expression};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Declaration {
    pub data_type: Datatype,
    pub name: String,
    pub value: Option<Expression>,
}
