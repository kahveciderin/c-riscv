use winnow::{combinator, PResult, Parser};

use crate::types::{declaration::Declaration, expression::Expression};

use super::{
    datatype::parse_datatype, expression::parse_expression, trivial_tokens::{parse_equals, parse_semicolon}, whitespace::parse_whitespace, Stream
};
use crate::parser::identifier::parse_identifier;

fn parse_declaration_value<'s>(input: &mut Stream<'s>) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_equals(input)?;

    parse_expression(input)
}

pub fn parse_declaration<'s>(input: &mut Stream<'s>) -> PResult<Declaration> {
    parse_whitespace(input)?;

    let data_type = parse_datatype(input)?;

    let identifier = parse_identifier(input)?.to_string();

    let variable = input.state.add_variable(identifier.clone(), data_type.clone());

    let value = combinator::opt(parse_declaration_value).parse_next(input)?;

    parse_semicolon(input)?;

    Ok(Declaration {
        data_type,
        name: variable.unique_name.clone(),
        value,
    })
}
