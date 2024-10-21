#[derive(Debug, Clone)]
pub enum Datatype {
    Int,
}

impl Datatype {
    pub fn size(&self) -> usize {
        match self {
            Datatype::Int => 4,
        }
    }
}
