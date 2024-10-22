use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    parser::{identifier::parse_identifier, trivial_tokens::parse_comma},
    types::expression::{Call, Expression, UnaryOp},
};

use super::{
    binary_operation::parse_binary_operation,
    number::parse_number,
    trivial_tokens::{
        parse_bang, parse_close_paren, parse_double_minus, parse_double_plus, parse_minus,
        parse_open_paren, parse_plus, parse_tilda,
    },
    whitespace::parse_whitespace,
    ParserSymbol, Stream,
};

pub fn parse_expression<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_binary_operation.parse_next(input)
}

pub fn parse_postfix_operator<'s>(input: &'s mut Stream) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((parse_double_plus, parse_double_minus, parse_open_paren)).parse_next(input)
}

pub fn parse_term<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    let expression = combinator::alt((
        parse_variable_expression,
        parse_number_expression,
        parse_paren_expression,
    ))
    .parse_next(input)?;

    let postfix_operator = parse_postfix_operator(input);

    println!("postfix operator {postfix_operator:?}");

    if let Ok(postfix) = postfix_operator {
        return match postfix {
            "++" => Ok(Expression::UnaryOp(UnaryOp::PostfixIncrement(Arc::new(
                expression,
            )))),
            "--" => Ok(Expression::UnaryOp(UnaryOp::PostfixDecrement(Arc::new(
                expression,
            )))),
            "(" => {
                let arguments = combinator::separated(0.., parse_expression, parse_comma)
                    .parse_next(input)?;

                parse_close_paren(input)?;

                Ok(Expression::Call(Call {
                    expression: Arc::new(expression),
                    arguments,
                }))
            }
            _ => Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new(),
            )),
        };
    }

    Ok(expression)
}

pub fn parse_factor<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    combinator::alt((parse_term, parse_unary_expression)).parse_next(input)
}

pub fn parse_variable_expression<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    let identifier = identifier.to_string();

    let symbol = input.state.get_symbol(&identifier);

    if let Some(symbol) = symbol {
        match symbol {
            ParserSymbol::Function(function) => Ok(Expression::FunctionSymbol(function.name.clone())),
            ParserSymbol::Variable(variable) => {
                Ok(Expression::Variable(variable.unique_name.clone()))
            }
        }
    } else {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }
}

pub fn parse_paren_expression<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    let expression = combinator::seq!(parse_open_paren, parse_expression, parse_close_paren)
        .map(|(_, expression, _)| expression)
        .parse_next(input)?;

    Ok(expression)
}

pub fn parse_optional_expression<'s>(input: &mut Stream<'s>) -> PResult<Option<Expression>> {
    parse_whitespace(input)?;

    combinator::opt(parse_expression).parse_next(input)
}

pub fn parse_number_expression<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_number.map(Expression::Number).parse_next(input)
}

pub fn parse_unary_operator<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_plus,
        parse_minus,
        parse_tilda,
        parse_bang,
        parse_double_plus,
        parse_double_minus,
    ))
    .parse_next(input)
}

pub fn parse_unary_plus_operation<'s>(input: &mut Stream<'s>) -> PResult<UnaryOp> {
    parse_whitespace(input)?;

    parse_factor
        .map(|v| UnaryOp::Plus(Arc::new(v)))
        .parse_next(input)
}

pub fn parse_unary_negation_operation<'s>(input: &mut Stream<'s>) -> PResult<UnaryOp> {
    parse_whitespace(input)?;

    parse_factor
        .map(|v| UnaryOp::Negation(Arc::new(v)))
        .parse_next(input)
}

pub fn parse_unary_bitwise_not_operation<'s>(input: &mut Stream<'s>) -> PResult<UnaryOp> {
    parse_whitespace(input)?;

    parse_factor
        .map(|v| UnaryOp::BitwiseNot(Arc::new(v)))
        .parse_next(input)
}

pub fn parse_unary_logical_not_operation<'s>(input: &mut Stream<'s>) -> PResult<UnaryOp> {
    parse_whitespace(input)?;

    parse_factor
        .map(|v| UnaryOp::LogicalNot(Arc::new(v)))
        .parse_next(input)
}

pub fn parse_prefix_increment_operation<'s>(input: &mut Stream<'s>) -> PResult<UnaryOp> {
    parse_whitespace(input)?;

    parse_factor
        .map(|v| UnaryOp::PrefixIncrement(Arc::new(v)))
        .parse_next(input)
}
pub fn parse_prefix_decrement_operation<'s>(input: &mut Stream<'s>) -> PResult<UnaryOp> {
    parse_whitespace(input)?;

    parse_factor
        .map(|v| UnaryOp::PrefixDecrement(Arc::new(v)))
        .parse_next(input)
}

pub fn parse_unary_operation<'s>(input: &mut Stream<'s>) -> PResult<UnaryOp> {
    parse_whitespace(input)?;

    combinator::dispatch! {parse_unary_operator;
        "+" => parse_unary_plus_operation,
        "-" => parse_unary_negation_operation,
        "~" => parse_unary_bitwise_not_operation,
        "!" => parse_unary_logical_not_operation,
        "++" => parse_prefix_increment_operation,
        "--" => parse_prefix_decrement_operation,
        _ => combinator::fail
    }
    .parse_next(input)
}

pub fn parse_unary_expression<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_unary_operation
        .map(Expression::UnaryOp)
        .parse_next(input)
}
