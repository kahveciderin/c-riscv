use winnow::{combinator, PResult, Parser};

use crate::types::scope::{Label, Scope, ScopeItem};

use super::{
    declaration::parse_declaration,
    expression::{fold::Fold, parse_expression},
    identifier::parse_identifier,
    statement::parse_statement,
    trivial_tokens::{parse_close_scope, parse_colon, parse_open_scope, parse_semicolon},
    whitespace::parse_whitespace,
    Case, Stream,
};

pub fn parse_statement_scope_item(input: &mut Stream<'_>) -> PResult<ScopeItem> {
    parse_whitespace(input)?;

    parse_statement.map(ScopeItem::Statement).parse_next(input)
}

pub fn parse_declaration_scope_item(input: &mut Stream<'_>) -> PResult<ScopeItem> {
    parse_whitespace(input)?;

    combinator::terminated(parse_declaration, parse_semicolon)
        .map(ScopeItem::Declaration)
        .parse_next(input)
}

pub fn parse_scope_item(input: &mut Stream<'_>) -> PResult<ScopeItem> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_declaration_scope_item,
        parse_statement_scope_item,
        parse_label,
    ))
    .parse_next(input)
}

pub fn parse_scope(input: &mut Stream<'_>) -> PResult<Scope> {
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

pub fn parse_label(input: &mut Stream) -> PResult<ScopeItem> {
    parse_whitespace(input)?;

    let name = parse_identifier(input)?;

    let label = if name == "default" {
        parse_colon(input)?;
        let id = input
            .state
            .get_switch()
            .unwrap_or_else(|| panic!("Default label outside of switch"))
            .id
            .clone();

        input.state.push_case_to_switch(Case::Default);

        Label::Default { id }
    } else if name == "case" {
        let value = parse_expression(input)?;
        parse_colon(input)?;
        let id = input
            .state
            .get_switch()
            .unwrap_or_else(|| panic!("Case label outside of switch"))
            .id
            .clone();

        let value = value.fold();

        if let Some(value) = value {
            input.state.push_case_to_switch(Case::Case(value));
            Label::Case { id, value }
        } else {
            panic!("Case label must be a constant expression");
        }
    } else {
        parse_colon(input)?;
        Label::Named(name.to_string())
    };

    Ok(ScopeItem::Label(label))
}
