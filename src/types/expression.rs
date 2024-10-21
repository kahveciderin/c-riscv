use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    UnaryOp(UnaryOp),
    BinaryOp(BinaryOp),
    TernaryOp(TernaryOp),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Plus(Arc<Expression>),
    Negation(Arc<Expression>),
    BitwiseNot(Arc<Expression>),
    LogicalNot(Arc<Expression>),
    PostfixIncrement(Arc<Expression>),
    PostfixDecrement(Arc<Expression>),
    PrefixIncrement(Arc<Expression>),
    PrefixDecrement(Arc<Expression>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Addition(Arc<Expression>, Arc<Expression>),
    Subtraction(Arc<Expression>, Arc<Expression>),
    Multiplication(Arc<Expression>, Arc<Expression>),
    Division(Arc<Expression>, Arc<Expression>),
    Modulus(Arc<Expression>, Arc<Expression>),
    BitwiseAnd(Arc<Expression>, Arc<Expression>),
    BitwiseXor(Arc<Expression>, Arc<Expression>),
    BitwiseOr(Arc<Expression>, Arc<Expression>),
    LeftShift(Arc<Expression>, Arc<Expression>),
    RightShift(Arc<Expression>, Arc<Expression>),
    LogicalAnd(Arc<Expression>, Arc<Expression>),
    LogicalOr(Arc<Expression>, Arc<Expression>),
    LessThan(Arc<Expression>, Arc<Expression>),
    GreaterThan(Arc<Expression>, Arc<Expression>),
    LessThanEquals(Arc<Expression>, Arc<Expression>),
    GreaterThanEquals(Arc<Expression>, Arc<Expression>),
    Equals(Arc<Expression>, Arc<Expression>),
    NotEquals(Arc<Expression>, Arc<Expression>),
    Assignment(Arc<Expression>, Arc<Expression>),
    AssignmentAddition(Arc<Expression>, Arc<Expression>),
    AssignmentSubtraction(Arc<Expression>, Arc<Expression>),
    AssignmentMultiplication(Arc<Expression>, Arc<Expression>),
    AssignmentDivision(Arc<Expression>, Arc<Expression>),
    AssignmentModulus(Arc<Expression>, Arc<Expression>),
    AssignmentShiftLeft(Arc<Expression>, Arc<Expression>),
    AssignmentShiftRight(Arc<Expression>, Arc<Expression>),
    AssignmentBitwiseAnd(Arc<Expression>, Arc<Expression>),
    AssignmentBitwiseXor(Arc<Expression>, Arc<Expression>),
    AssignmentBitwiseOr(Arc<Expression>, Arc<Expression>),
}

#[derive(Debug, Clone)]
pub struct TernaryOp {
    pub condition: Arc<Expression>,
    pub then_expr: Arc<Expression>,
    pub else_expr: Arc<Expression>,
}
