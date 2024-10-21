use super::{datatype::Datatype, expression::Expression};

#[derive(Debug)]
pub struct Declaration {
    pub data_type: Datatype,
    pub name: String,
    pub value: Option<Expression>,
}
