use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    create_term_parser,
    parser::{
        trivial_tokens::{parse_minus, parse_plus},
        whitespace::parse_whitespace, Stream,
    },
    types::expression::{BinaryOp, Expression},
};

use super::level_3::parse_level_3_expression;

pub fn parse_level_4_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((parse_plus, parse_minus)).parse_next(input)
}

pub fn level_4_operation_creator(lhs: Expression, rhs: Expression, op: String) -> Option<BinaryOp> {
    match op.as_str() {
        "+" => Some(BinaryOp::Addition(Arc::new(lhs), Arc::new(rhs))),
        "-" => Some(BinaryOp::Subtraction(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_4_expression,
    parse_level_4_operation,
    parse_level_4_binary_operator,
    parse_level_3_expression,
    level_4_operation_creator
);
