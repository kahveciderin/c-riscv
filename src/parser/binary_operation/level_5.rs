use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    create_term_parser,
    parser::{
        trivial_tokens::{parse_double_greater_than, parse_double_less_than},
        whitespace::parse_whitespace, Stream,
    },
    types::expression::{BinaryOp, Expression},
};

use super::level_4::parse_level_4_expression;

pub fn parse_level_5_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((parse_double_greater_than, parse_double_less_than)).parse_next(input)
}

pub fn level_5_operation_creator(lhs: Expression, rhs: Expression, op: String) -> Option<BinaryOp> {
    match op.as_str() {
        "<<" => Some(BinaryOp::LeftShift(Arc::new(lhs), Arc::new(rhs))),
        ">>" => Some(BinaryOp::RightShift(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_5_expression,
    parse_level_5_operation,
    parse_level_5_binary_operator,
    parse_level_4_expression,
    level_5_operation_creator
);
