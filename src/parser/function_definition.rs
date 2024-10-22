use winnow::{combinator, PResult, Parser};

use crate::{
    types::function_definition::{FunctionArgument, FunctionDefinition},
    utils::random_name::unique_identifier,
};

use super::{
    datatype::{self, parse_datatype},
    identifier::parse_identifier,
    scope::parse_scope,
    trivial_tokens::{parse_close_paren, parse_open_paren},
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_function_argument<'s>(input: &mut Stream<'s>) -> PResult<FunctionArgument> {
    parse_whitespace(input)?;
    let datatype = parse_datatype(input)?;
    let identifier = parse_identifier(input)?;
    let unique_name = unique_identifier(Some(identifier), None);

    Ok(FunctionArgument {
        datatype,
        name: identifier.to_string(),
        unique_name,
    })
}

pub fn parse_function_definition<'s>(input: &mut Stream<'s>) -> PResult<FunctionDefinition<'s>> {
    parse_whitespace(input)?;

    let return_type = parse_datatype(input)?;
    let name = parse_identifier(input)?;

    parse_open_paren(input)?;

    let arguments: Vec<FunctionArgument> =
        combinator::repeat_till(0.., parse_function_argument, parse_close_paren)
            .map(|v| v.0)
            .parse_next(input)?;

    input.state.start_function_scope(name.to_string());

    arguments.iter().for_each(|arg| {
        input.state.add_raw_name_variable(
            arg.name.to_string(),
            arg.unique_name.to_string(),
            arg.datatype.clone(),
        );
    });

    let body = parse_scope(input)?;

    Ok(FunctionDefinition {
        return_type,
        arguments,
        name,
        body,
        scope_state: input.state.function_scope.clone(),
    })
}
