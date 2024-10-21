use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    create_term_parser,
    parser::{
        trivial_tokens::{parse_bang_equals, parse_double_equals},
        whitespace::parse_whitespace, Stream,
    },
    types::expression::{BinaryOp, Expression},
};

use super::level_6::parse_level_6_expression;

pub fn parse_level_7_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((parse_double_equals, parse_bang_equals)).parse_next(input)
}

pub fn level_7_operation_creator(lhs: Expression, rhs: Expression, op: String) -> Option<BinaryOp> {
    match op.as_str() {
        "==" => Some(BinaryOp::Equals(Arc::new(lhs), Arc::new(rhs))),
        "!=" => Some(BinaryOp::NotEquals(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_7_expression,
    parse_level_7_operation,
    parse_level_7_binary_operator,
    parse_level_6_expression,
    level_7_operation_creator
);
