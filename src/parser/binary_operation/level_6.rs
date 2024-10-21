use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    create_term_parser,
    parser::{
        trivial_tokens::{
            parse_greater_than, parse_greater_than_equals, parse_less_than, parse_less_than_equals,
        },
        whitespace::parse_whitespace, Stream,
    },
    types::expression::{BinaryOp, Expression},
};

use super::level_5::parse_level_5_expression;

pub fn parse_level_6_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_greater_than,
        parse_less_than,
        parse_greater_than_equals,
        parse_less_than_equals,
    ))
    .parse_next(input)
}

pub fn level_6_operation_creator(lhs: Expression, rhs: Expression, op: String) -> Option<BinaryOp> {
    match op.as_str() {
        "<" => Some(BinaryOp::LessThan(Arc::new(lhs), Arc::new(rhs))),
        ">" => Some(BinaryOp::GreaterThan(Arc::new(lhs), Arc::new(rhs))),
        "<=" => Some(BinaryOp::LessThanEquals(Arc::new(lhs), Arc::new(rhs))),
        ">=" => Some(BinaryOp::GreaterThanEquals(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_6_expression,
    parse_level_6_operation,
    parse_level_6_binary_operator,
    parse_level_5_expression,
    level_6_operation_creator
);
