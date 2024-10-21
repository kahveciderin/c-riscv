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

    combinator::seq!(FunctionDefinition {
        return_type: parse_datatype,
        name: parse_identifier,
        _: parse_open_paren,
        // arguments: separated(parse_identifier, parse_comma),
        _: parse_close_paren,
        body: parse_scope,
    })
    .parse_next(input)
}
