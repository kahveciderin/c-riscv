use std::sync::Arc;

use winnow::{combinator, error, PResult, Parser};

use crate::types::statement::{IfStatement, JumpStatement, Statement};

use super::{
    expression::{parse_expression, parse_optional_expression},
    identifier::parse_identifier,
    scope::parse_scope,
    trivial_tokens::parse_semicolon,
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;
    combinator::alt((
        parse_jump_statement,
        parse_if_statement,
        parse_expression_statement,
        parse_scope_statement,
        parse_null_statement,
    ))
    .parse_next(input)
}

pub fn parse_scope_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Scope { scope: parse_scope }).parse_next(input)
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

pub fn parse_else(input: &mut Stream) -> PResult<Statement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "else" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    parse_statement(input)
}

pub fn parse_if_statement(input: &mut Stream) -> PResult<Statement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "if" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    combinator::seq!(IfStatement {
        condition: parse_expression,
        then_block: parse_statement.map(Arc::new),
        else_block: combinator::opt(parse_else.map(Arc::new)),
    })
    .map(|s| Statement::If { statement: s })
    .parse_next(input)
}
