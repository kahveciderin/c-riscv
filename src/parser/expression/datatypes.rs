use crate::types::datatype::Datatype;

pub trait GetType {
    fn get_type(&self) -> Datatype;
}
