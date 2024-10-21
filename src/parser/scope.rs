use winnow::{combinator, PResult, Parser};

use crate::types::scope::{Scope, ScopeItem};

use super::{
    declaration::parse_declaration,
    statement::parse_statement,
    trivial_tokens::{parse_close_scope, parse_open_scope, parse_semicolon},
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_statement_scope_item<'s>(input: &mut Stream<'s>) -> PResult<ScopeItem> {
    parse_whitespace(input)?;

    parse_statement.map(ScopeItem::Statement).parse_next(input)
}

pub fn parse_declaration_scope_item<'s>(input: &mut Stream<'s>) -> PResult<ScopeItem> {
    parse_whitespace(input)?;

    combinator::terminated(parse_declaration, parse_semicolon)
        .map(ScopeItem::Declaration)
        .parse_next(input)
}

pub fn parse_scope_item<'s>(input: &mut Stream<'s>) -> PResult<ScopeItem> {
    parse_whitespace(input)?;

    combinator::alt((parse_declaration_scope_item, parse_statement_scope_item)).parse_next(input)
}

pub fn parse_scope<'s>(input: &mut Stream<'s>) -> PResult<Scope> {
    parse_whitespace(input)?;

    input.state.push_scope();

    parse_open_scope.parse_next(input)?;

    let items = combinator::repeat_till(0.., parse_scope_item, parse_close_scope)
        .map(|v| v.0)
        .parse_next(input)?;

    input.state.pop_scope();

    let scope = Scope { items };

    Ok(scope)
}
