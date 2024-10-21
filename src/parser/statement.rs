use winnow::{combinator, error, PResult, Parser};

use crate::types::statement::{JumpStatement, Statement};

use super::{
    expression::{parse_expression, parse_optional_expression},
    identifier::parse_identifier,
    trivial_tokens::parse_semicolon,
    whitespace::parse_whitespace, Stream,
};

pub fn parse_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;
    combinator::alt((
        parse_jump_statement,
        parse_expression_statement,
        parse_null_statement,
    ))
    .parse_next(input)
}

pub fn parse_null_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Null{
        _: parse_semicolon,
    })
    .parse_next(input)
}

pub fn parse_expression_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Expression{
        expression: parse_expression,
        _: parse_semicolon,
    })
    .parse_next(input)
}

pub fn parse_jump_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Jump {
        statement: parse_return_jump
    })
    .parse_next(input)
}

pub fn parse_return_jump<'s>(input: &mut Stream<'s>) -> PResult<JumpStatement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "return" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    combinator::seq!(JumpStatement::Return{
            expression: parse_optional_expression,
            _: parse_semicolon,
    })
    .parse_next(input)
}
