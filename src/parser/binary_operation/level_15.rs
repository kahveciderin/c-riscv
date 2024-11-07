use std::sync::Arc;

use winnow::{PResult, Parser};

use crate::{
    create_term_parser,
    parser::{trivial_tokens::parse_comma, whitespace::parse_whitespace, Stream},
    types::expression::{BinaryOp, Expression},
};

use super::level_14::parse_level_14_expression;

pub fn parse_level_15_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    parse_comma.parse_next(input)
}

pub fn level_15_operation_creator(
    lhs: Expression,
    rhs: Expression,
    op: String,
) -> Option<BinaryOp> {
    match op.as_str() {
        "," => Some(BinaryOp::Comma(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_15_expression,
    parse_level_15_operation,
    parse_level_15_binary_operator,
    parse_level_14_expression,
    level_15_operation_creator
);
