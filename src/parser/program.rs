use winnow::{combinator, PResult, Parser};

use crate::types::program::{Program, ProgramStatement};

use super::{
    function_definition::{parse_function_declaration, parse_function_definition},
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_program<'s>(input: &mut Stream<'s>) -> PResult<Program<'s>> {
    parse_whitespace(input)?;

    let functions = combinator::repeat_till(
        0..,
        combinator::alt((
            parse_function_definition.map(ProgramStatement::FunctionDefinition),
            parse_function_declaration.map(ProgramStatement::FunctionDeclaration),
        )),
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
