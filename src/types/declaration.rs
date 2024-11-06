use super::{datatype::Datatype, expression::Expression};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Declaration {
    pub datatype: Datatype,
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct Argument {
    pub name: String,
    pub datatype: Datatype,
}
#[derive(Debug)]
pub struct Declarator {
    pub name: String,
    pub datatype: Datatype,
}
