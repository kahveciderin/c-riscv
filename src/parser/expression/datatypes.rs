use std::sync::Arc;

use crate::{
    parser::{ParserState, ParserSymbol},
    types::{
        datatype::Datatype,
        expression::{BinaryOp, Call, Expression, TernaryOp, UnaryOp},
    },
};

pub trait GetType {
    fn get_type(&self, state: &ParserState) -> Datatype;
}

impl GetType for ParserSymbol {
    fn get_type(&self, _state: &ParserState) -> Datatype {
        match self {
            ParserSymbol::Variable(var) => var.datatype.clone(),
            ParserSymbol::Argument(var) => var.datatype.clone(),
            ParserSymbol::Function(var) => var.datatype.clone(),
        }
    }
}

impl GetType for Expression {
    fn get_type(&self, state: &ParserState) -> Datatype {
        match self {
            Expression::Number(_) => Datatype::Int,
            Expression::UnaryOp(op) => op.get_type(state),
            Expression::BinaryOp(op) => op.get_type(state),
            Expression::TernaryOp(op) => op.get_type(state),
            Expression::Variable(name) => state
                .get_by_unique_name(name)
                .unwrap_or_else(|| panic!("Variable not found"))
                .get_type(state),
            Expression::FunctionSymbol(name) => state
                .get_by_unique_name(name)
                .unwrap_or_else(|| panic!("Variable not found"))
                .get_type(state),
            Expression::Call(call) => call.get_type(state),
        }
    }
}

impl GetType for UnaryOp {
    fn get_type(&self, state: &ParserState) -> Datatype {
        match self {
            UnaryOp::Nothing(expr) => expr.get_type(state),
            UnaryOp::Plus(expr) => expr.get_type(state),
            UnaryOp::Negation(expr) => expr.get_type(state),
            UnaryOp::BitwiseNot(expr) => expr.get_type(state),
            UnaryOp::LogicalNot(expr) => expr.get_type(state),
            UnaryOp::PostfixIncrement(expr) => expr.get_type(state),
            UnaryOp::PostfixDecrement(expr) => expr.get_type(state),
            UnaryOp::PrefixIncrement(expr) => expr.get_type(state),
            UnaryOp::PrefixDecrement(expr) => expr.get_type(state),
            UnaryOp::Ref(expr) => {
                println!("Ref {:?}", expr.get_type(state));
                return Datatype::Pointer {
                    inner: Arc::new(expr.get_type(state)),
                };
            }
            UnaryOp::Deref(expr) => {
                let expression_type = expr.get_type(state);

                if let Datatype::Pointer { inner } = expression_type {
                    inner.as_ref().clone()
                } else if let Datatype::Function {
                    arguments,
                    return_type,
                } = expression_type
                {
                    Datatype::Function {
                        arguments,
                        return_type,
                    }
                } else {
                    panic!("Trying to dereference non-pointer value")
                }
            }
        }
    }
}

// todo: boolean type
impl GetType for BinaryOp {
    fn get_type(&self, state: &ParserState) -> Datatype {
        match self {
            BinaryOp::Addition(left, _) => left.get_type(state),
            BinaryOp::Subtraction(left, _) => left.get_type(state),
            BinaryOp::Multiplication(left, _) => left.get_type(state),
            BinaryOp::Division(left, _) => left.get_type(state),
            BinaryOp::Modulus(left, _) => left.get_type(state),
            BinaryOp::BitwiseAnd(left, _) => left.get_type(state),
            BinaryOp::BitwiseXor(left, _) => left.get_type(state),
            BinaryOp::BitwiseOr(left, _) => left.get_type(state),
            BinaryOp::LeftShift(left, _) => left.get_type(state),
            BinaryOp::RightShift(left, _) => left.get_type(state),
            BinaryOp::LogicalAnd(left, _) => left.get_type(state),
            BinaryOp::LogicalOr(left, _) => left.get_type(state),
            BinaryOp::LessThan(left, _) => left.get_type(state),
            BinaryOp::GreaterThan(left, _) => left.get_type(state),
            BinaryOp::LessThanEquals(left, _) => left.get_type(state),
            BinaryOp::GreaterThanEquals(left, _) => left.get_type(state),
            BinaryOp::Equals(left, _) => left.get_type(state),
            BinaryOp::NotEquals(left, _) => left.get_type(state),
            BinaryOp::Assignment(left, _) => left.get_type(state),
            BinaryOp::AssignmentAddition(left, _) => left.get_type(state),
            BinaryOp::AssignmentSubtraction(left, _) => left.get_type(state),
            BinaryOp::AssignmentMultiplication(left, _) => left.get_type(state),
            BinaryOp::AssignmentDivision(left, _) => left.get_type(state),
            BinaryOp::AssignmentModulus(left, _) => left.get_type(state),
            BinaryOp::AssignmentShiftLeft(left, _) => left.get_type(state),
            BinaryOp::AssignmentShiftRight(left, _) => left.get_type(state),
            BinaryOp::AssignmentBitwiseAnd(left, _) => left.get_type(state),
            BinaryOp::AssignmentBitwiseOr(left, _) => left.get_type(state),
            BinaryOp::AssignmentBitwiseXor(left, _) => left.get_type(state),
            BinaryOp::Comma(_, right) => right.get_type(state),
        }
    }
}

impl GetType for TernaryOp {
    fn get_type(&self, state: &ParserState) -> Datatype {
        let then_expr = self.then_expr.get_type(state);
        let else_expr = self.else_expr.get_type(state);

        if else_expr == then_expr {
            then_expr
        } else {
            panic!("TernaryOp types do not match");
        }
    }
}

impl GetType for Call {
    fn get_type(&self, state: &ParserState) -> Datatype {
        let function = self.expression.get_type(state);

        println!("function {function:?}");

        let (return_type, arguments) = if let Datatype::Function {
            return_type,
            arguments,
        } = function
        {
            (return_type, arguments)
        } else if let Datatype::Pointer { inner } = function {
            if let Datatype::Function {
                return_type,
                arguments,
            } = inner.as_ref().clone()
            {
                (return_type, arguments)
            } else {
                panic!("Call expression is not a function");
            }
        } else {
            panic!("Call expression is not a function");
        };

        if self.arguments.len() != arguments.len() {
            panic!("Incorrect number of arguments in function call");
        }

        for (i, arg) in self.arguments.iter().enumerate() {
            let arg_type = arg.get_type(state);

            if arguments.len() <= i {
                panic!("Too many arguments in function call");
            }

            let expected_arg = &arguments[i];

            if arg_type != *expected_arg.datatype {
                panic!("Argument type does not match expected type");
            }
        }

        return_type.as_ref().clone()
    }
}
