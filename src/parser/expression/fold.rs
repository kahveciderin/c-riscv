use crate::types::expression::Expression;

pub trait Fold {
    fn fold(&self) -> Option<i32>;
}

impl Fold for Expression {
    fn fold(&self) -> Option<i32> {
        match self {
            Expression::Number(num) => Some(*num),
            Expression::UnaryOp(_) => None,
            Expression::BinaryOp(_) => None,
            Expression::TernaryOp(_) => None,
            Expression::Variable(_) => None,
            Expression::FunctionSymbol(_) => None,
            Expression::Call(_) => None,
        }
    }
}
