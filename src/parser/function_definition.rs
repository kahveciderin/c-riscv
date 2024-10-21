use winnow::{combinator, PResult, Parser};

use crate::types::function_definition::FunctionDefinition;

use super::{
    datatype::parse_datatype,
    identifier::parse_identifier,
    scope::parse_scope,
    trivial_tokens::{parse_close_paren, parse_open_paren},
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_function_definition<'s>(input: &mut Stream<'s>) -> PResult<FunctionDefinition<'s>> {
    parse_whitespace(input)?;

    input.state.start_function_scope();

    let return_type = parse_datatype(input)?;
    let name = parse_identifier(input)?;

    parse_open_paren(input)?;

    parse_close_paren(input)?;

    let body = parse_scope(input)?;

    Ok(FunctionDefinition {
        return_type,
        name,
        body,
        scope_state: input.state.function_scope.clone(),
    })
}
