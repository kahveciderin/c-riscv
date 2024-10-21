use std::sync::Arc;

use winnow::{PResult, Parser};

use crate::{
    create_term_parser,
    parser::{trivial_tokens::parse_ampersand, whitespace::parse_whitespace, Stream},
    types::expression::{BinaryOp, Expression},
};

use super::level_7::parse_level_7_expression;

pub fn parse_level_8_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    parse_ampersand(input)
}

pub fn level_8_operation_creator(lhs: Expression, rhs: Expression, op: String) -> Option<BinaryOp> {
    match op.as_str() {
        "&" => Some(BinaryOp::BitwiseAnd(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_8_expression,
    parse_level_8_operation,
    parse_level_8_binary_operator,
    parse_level_7_expression,
    level_8_operation_creator
);
