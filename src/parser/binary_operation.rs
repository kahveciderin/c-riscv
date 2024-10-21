use winnow::PResult;

use crate::types::expression::Expression;

use super::Stream;

mod level_10;
mod level_11;
mod level_12;
mod level_13;
mod level_14;
mod level_3;
mod level_4;
mod level_5;
mod level_6;
mod level_7;
mod level_8;
mod level_9;

#[derive(Debug)]
pub struct HalfBinaryOp {
    rhs: Expression,
    op: String,
}

pub fn parse_binary_operation<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    level_14::parse_level_14_expression(input)
}

#[macro_export]
macro_rules! create_term_parser {
    ($parser_name:tt, $half_parser_name:tt, $operator_parser:expr, $lower_level_term_parser:expr, $operation_creator:expr) => {
        fn $half_parser_name<'s>(
            input: &mut Stream<'s>,
        ) -> PResult<crate::parser::binary_operation::HalfBinaryOp> {
            parse_whitespace(input)?;

            let op = $operator_parser(input)?.to_string();
            let rhs = $lower_level_term_parser(input)?;
            Ok(crate::parser::binary_operation::HalfBinaryOp { rhs, op })
        }

        pub fn $parser_name<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
            parse_whitespace(input)?;

            let mut lhs = $lower_level_term_parser(input)?;

            let half_binary_expressions: Vec<crate::parser::binary_operation::HalfBinaryOp> =
                winnow::combinator::repeat_till(
                    0..,
                    $half_parser_name,
                    winnow::combinator::not($operator_parser),
                )
                .map(|v| v.0)
                .parse_next(input)?;

            for half_binary_expression in half_binary_expressions {
                let op =
                    $operation_creator(lhs, half_binary_expression.rhs, half_binary_expression.op);

                if let Some(op) = op {
                    lhs = Expression::BinaryOp(op);
                } else {
                    return Err(winnow::error::ErrMode::Backtrack(
                        winnow::error::ContextError::new(),
                    ));
                }
            }

            Ok(lhs)
        }
    };
}
