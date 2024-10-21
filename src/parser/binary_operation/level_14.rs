use std::sync::Arc;

use winnow::{combinator, error, PResult, Parser};

use crate::{
    parser::{
        trivial_tokens::{
            parse_ampersand_equals, parse_caret_equals, parse_double_greater_than_equals,
            parse_double_less_than_equals, parse_equals, parse_minus_equals, parse_percent_equals,
            parse_pipe_equals, parse_plus_equals, parse_slash_equals, parse_star_equals,
        },
        whitespace::parse_whitespace, Stream,
    },
    types::expression::{BinaryOp, Expression},
};

use super::{level_12::parse_level_12_expression, HalfBinaryOp};

pub fn parse_level_14_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_equals,
        parse_plus_equals,
        parse_minus_equals,
        parse_star_equals,
        parse_slash_equals,
        parse_percent_equals,
        parse_double_greater_than_equals,
        parse_double_less_than_equals,
        parse_ampersand_equals,
        parse_caret_equals,
        parse_pipe_equals,
    ))
    .parse_next(input)
}

pub fn level_14_operation_creator(
    lhs: Expression,
    rhs: Expression,
    op: String,
) -> Option<BinaryOp> {
    match op.as_str() {
        "=" => Some(BinaryOp::Assignment(Arc::new(lhs), Arc::new(rhs))),
        "+=" => Some(BinaryOp::AssignmentAddition(Arc::new(lhs), Arc::new(rhs))),
        "-=" => Some(BinaryOp::AssignmentSubtraction(
            Arc::new(lhs),
            Arc::new(rhs),
        )),
        "*=" => Some(BinaryOp::AssignmentMultiplication(
            Arc::new(lhs),
            Arc::new(rhs),
        )),
        "/=" => Some(BinaryOp::AssignmentDivision(Arc::new(lhs), Arc::new(rhs))),
        "%=" => Some(BinaryOp::AssignmentModulus(Arc::new(lhs), Arc::new(rhs))),
        ">>=" => Some(BinaryOp::AssignmentShiftRight(Arc::new(lhs), Arc::new(rhs))),
        "<<=" => Some(BinaryOp::AssignmentShiftLeft(Arc::new(lhs), Arc::new(rhs))),
        "&=" => Some(BinaryOp::AssignmentBitwiseAnd(Arc::new(lhs), Arc::new(rhs))),
        "^=" => Some(BinaryOp::AssignmentBitwiseXor(Arc::new(lhs), Arc::new(rhs))),
        "|=" => Some(BinaryOp::AssignmentBitwiseOr(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

fn parse_half_level_14_operation<'s>(input: &mut Stream<'s>) -> PResult<HalfBinaryOp> {
    parse_whitespace(input)?;

    let op = parse_level_14_binary_operator(input)?.to_string();
    let rhs = parse_level_14_expression(input)?;
    Ok(HalfBinaryOp { rhs, op })
}

// these operators are right-associative so we handle them in a different way
pub fn parse_level_14_expression<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    let lhs = parse_level_12_expression(input)?;
    let half_operation = parse_half_level_14_operation(input);

    if let Ok(half_operation) = half_operation {
        let op = level_14_operation_creator(lhs, half_operation.rhs, half_operation.op);

        if let Some(op) = op {
            return Ok(Expression::BinaryOp(op));
        } else {
            return Err(error::ErrMode::Backtrack(error::ContextError::new()));
        }
    } else {
        return Ok(lhs);
    }
}
