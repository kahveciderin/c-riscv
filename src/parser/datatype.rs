use winnow::{combinator, PResult, Parser};

use crate::types::datatype::Datatype;

use super::{identifier::parse_identifier, whitespace::parse_whitespace, Stream};

pub fn parse_datatype(input: &mut Stream) -> PResult<Datatype> {
    parse_whitespace(input)?;

    combinator::alt((parse_primitive_datatype,)).parse_next(input)
}

fn parse_primitive_datatype(input: &mut Stream) -> PResult<Datatype> {
    parse_whitespace(input)?;

    let datatype = parse_identifier(input)?;

    match datatype {
        "int" => Ok(Datatype::Int),
        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        )),
    }
}
