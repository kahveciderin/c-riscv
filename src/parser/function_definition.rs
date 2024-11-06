use winnow::PResult;

use crate::{
    types::{
        datatype::Datatype,
        declaration::Declarator,
        function_definition::{FunctionArgument, FunctionDefinition},
    },
    utils::random_name::unique_identifier,
};

use super::{
    declaration::{parse_declarator, parse_primitive_datatype},
    scope::parse_scope,
    whitespace::parse_whitespace,
    ParserSymbol, ParserVariable, Stream,
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

pub fn parse_function_definition(input: &mut Stream) -> PResult<FunctionDefinition> {
    parse_whitespace(input)?;

    let base_type = parse_primitive_datatype(input)?;

    let declarator = parse_declarator(input, base_type)?;

    if let Datatype::Function {
        return_type,
        ref arguments,
    } = declarator.datatype
    {
        let name = declarator.name;

        if input.state.static_symbols.iter().any(|x| x.name == name) {
            return Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new(),
            ));
        }

        if arguments.iter().any(|x| x.name.len() == 0) {
            return Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new(),
            ));
        }

        if duplicate_checker(arguments.iter().map(|x| x.name.to_string())) {
            panic!("Duplicate argument names in function definition");
        }

        input.state.start_function_scope(
            name.to_string(),
            arguments.to_vec(),
            return_type.as_ref().clone(),
        );

        let function_arguments = arguments
            .iter()
            .map(|a| FunctionArgument {
                name: a.name.to_string(),
                unique_name: unique_identifier(Some(&a.name), None),
                datatype: a.datatype.as_ref().clone(),
            })
            .collect::<Vec<_>>();

        function_arguments.iter().for_each(|arg| {
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
            return_type: return_type.as_ref().clone(),
            arguments: function_arguments,
            name,
            body,
            scope_state: input.state.function_scope.clone(),
        })
    } else {
        Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ))
    }
}
