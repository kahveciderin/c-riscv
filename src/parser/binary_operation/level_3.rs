use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    create_term_parser, parser::{
        expression::parse_factor,
        trivial_tokens::{parse_percent, parse_slash, parse_star},
        whitespace::parse_whitespace, Stream,
    }, types::expression::{BinaryOp, Expression}
};

pub fn parse_level_3_binary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((parse_star, parse_slash, parse_percent)).parse_next(input)
}

pub fn level_3_operation_creator(lhs: Expression, rhs: Expression, op: String) -> Option<BinaryOp> {
    match op.as_str() {
        "*" => Some(BinaryOp::Multiplication(Arc::new(lhs), Arc::new(rhs))),
        "/" => Some(BinaryOp::Division(Arc::new(lhs), Arc::new(rhs))),
        "%" => Some(BinaryOp::Modulus(Arc::new(lhs), Arc::new(rhs))),
        _ => None,
    }
}

create_term_parser!(
    parse_level_3_expression,
    parse_level_3_operation,
    parse_level_3_binary_operator,
    parse_factor,
    level_3_operation_creator
);
