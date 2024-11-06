use winnow::{combinator, PResult, Parser};

use crate::types::program::{Program, ProgramStatement};

use super::{function_definition::parse_function_definition, whitespace::parse_whitespace, Stream};

pub fn parse_program(input: &mut Stream) -> PResult<Program> {
    parse_whitespace(input)?;

    let functions = combinator::repeat_till(
        0..,
        combinator::alt((parse_function_definition.map(ProgramStatement::FunctionDefinition),)),
        combinator::eof,
    )
    .map(|v| v.0)
    .parse_next(input)?;

    parse_whitespace(input)?;

    if input.len() > 0 {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }

    Ok(Program { functions })
}
