// this is actually ternary but who cares

use std::sync::Arc;

use winnow::PResult;

use crate::{
    parser::{
        expression::parse_expression,
        trivial_tokens::{parse_colon, parse_question_mark},
        whitespace::parse_whitespace,
        Stream,
    },
    types::expression::{Expression, TernaryOp},
};

use super::level_12::parse_level_12_expression;

pub fn parse_level_13_ternary_expression<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    let condition = parse_level_12_expression(input)?;

    let question_mark = parse_question_mark(input);

    if question_mark.is_ok() {
        let then_expr = parse_expression(input)?;

        parse_colon(input)?;

        let else_expr = parse_expression(input)?;

        Ok(Expression::TernaryOp(TernaryOp {
            condition: Arc::new(condition),
            then_expr: Arc::new(then_expr),
            else_expr: Arc::new(else_expr),
        }))
    } else {
        Ok(condition)
    }
}
