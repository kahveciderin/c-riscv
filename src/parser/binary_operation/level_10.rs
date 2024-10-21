use std::sync::Arc;

use winnow::{PResult, Parser};

use crate::{
    create_term_parser,
    parser::{trivial_tokens::parse_pipe, whitespace::parse_whitespace, Stream},
    types::expression::{BinaryOp, Expression},
};

use super::level_9::parse_level_9_expression;

pub fn parse_level_10_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    parse_pipe(input)
}

pub fn level_10_operation_creator(
    lhs: Expression,
    rhs: Expression,
    op: String,
) -> Option<BinaryOp> {
    match op.as_str() {
        "|" => Some(BinaryOp::BitwiseOr(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_10_expression,
    parse_level_10_operation,
    parse_level_10_binary_operator,
    parse_level_9_expression,
    level_10_operation_creator
);
