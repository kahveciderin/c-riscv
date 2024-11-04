use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    parser::trivial_tokens::parse_comma,
    types::{
        datatype::Datatype,
        function_definition::{
            FunctionArgument, FunctionArgumentOptionalName, FunctionDeclaration, FunctionDefinition,
        },
    },
    utils::random_name::unique_identifier,
};

use super::{
    datatype::parse_datatype,
    identifier::parse_identifier,
    scope::parse_scope,
    trivial_tokens::{parse_close_paren, parse_open_paren, parse_semicolon, parse_void},
    whitespace::parse_whitespace,
    ParserStaticSymbol, ParserSymbol, ParserVariable, Stream,
};

fn duplicate_checker(input: impl Iterator<Item = String>) -> bool {
    let mut seen = std::collections::HashSet::new();
    for x in input {
        if !seen.insert(x) {
            return true;
        }
    }
    false
}

pub fn parse_function_argument(input: &mut Stream<'_>) -> PResult<FunctionArgument> {
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
pub fn parse_function_argument_with_optional_name(
    input: &mut Stream<'_>,
) -> PResult<FunctionArgumentOptionalName> {
    parse_whitespace(input)?;
    let datatype = parse_datatype(input)?;
    let identifier =
        combinator::opt(parse_identifier.map(|op| op.to_string())).parse_next(input)?;

    Ok(FunctionArgumentOptionalName {
        datatype,
        name: identifier,
    })
}

pub fn parse_function_definition<'s>(input: &mut Stream<'s>) -> PResult<FunctionDefinition<'s>> {
    parse_whitespace(input)?;

    let return_type = parse_datatype(input)?;
    let name = parse_identifier(input)?;

    parse_open_paren(input)?;

    let arguments: Vec<_> = combinator::alt((
        parse_void.map(|_| vec![]),
        combinator::separated(0.., parse_function_argument, parse_comma),
    ))
    .parse_next(input)?;
    parse_close_paren(input)?;

    if input
        .state
        .static_symbols
        .iter()
        .any(|x| x.name == name)
    {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }

    if duplicate_checker(arguments.iter().map(|x| x.name.to_string())) {
        panic!("Duplicate argument names in function definition");
    }

    input.state.start_function_scope(
        name.to_string(),
        arguments.iter().map(|x| x.datatype.clone()).collect(),
        return_type.clone(),
    );

    arguments.iter().for_each(|arg| {
        input
            .state
            .add_argument(ParserSymbol::Argument(ParserVariable {
                name: arg.name.to_string(),
                unique_name: arg.unique_name.to_string(),
                datatype: arg.datatype.clone(),
            }));
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

pub fn parse_function_declaration(input: &mut Stream<'_>) -> PResult<FunctionDeclaration> {
    parse_whitespace(input)?;

    let return_type = parse_datatype(input)?;
    let name = parse_identifier(input)?;

    parse_open_paren(input)?;

    let arguments: Vec<_> = combinator::alt((
        parse_void.map(|_| vec![]),
        combinator::separated(0.., parse_function_argument_with_optional_name, parse_comma),
    ))
    .parse_next(input)?;

    parse_close_paren(input)?;

    parse_semicolon(input)?;

    if duplicate_checker(arguments.iter().filter_map(|x| x.name.clone())) {
        panic!("Duplicate argument names in function declaration");
    }

    if let Some(function) = input.state.static_symbols.iter().find(|x| x.name == name) {
        if let Datatype::FunctionPointer {
            return_type: r,
            arguments: a,
        } = &function.datatype
        {
            if *r.as_ref() != return_type.clone()
                || *a
                    != arguments
                        .iter()
                        .map(|x| x.datatype.clone())
                        .collect::<Vec<_>>()
            {
                panic!(
                    "Function {} already declared with different signature",
                    name
                );
            }
        }
    }

    input.state.add_static_symbol(ParserStaticSymbol {
        name: name.to_string(),
        datatype: Datatype::FunctionPointer {
            return_type: Arc::new(return_type.clone()),
            arguments: arguments.iter().map(|x| x.datatype.clone()).collect(),
        },
    });

    Ok(FunctionDeclaration {
        return_type,
        arguments,
        name: name.to_string(),
    })
}
