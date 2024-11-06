use std::sync::Arc;

use winnow::{combinator, PResult, Parser};

use crate::{
    parser::ParserSymbol,
    types::{
        datatype::{Argument, Datatype},
        declaration::{Declaration, Declarator},
        expression::Expression,
    },
};

use super::{
    expression::parse_expression,
    trivial_tokens::{parse_close_paren, parse_equals, parse_open_paren, parse_star, parse_void},
    whitespace::parse_whitespace,
    Stream,
};
use crate::parser::identifier::parse_identifier;

pub fn parse_primitive_datatype(input: &mut Stream) -> PResult<Datatype> {
    parse_whitespace(input)?;

    let datatype = parse_identifier(input)?;

    match datatype {
        "int" => Ok(Datatype::Int),
        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        )),
    }
}

fn parse_declaration_value(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_equals(input)?;

    parse_expression(input)
}

#[derive(Debug, Clone)]
struct Param {
    datatype: Datatype,
    declarator: InnerDeclarator,
}

#[derive(Debug, Clone)]
enum InnerDeclarator {
    Abstract,
    Identifier(String),
    Pointer(Arc<InnerDeclarator>),
    Function {
        params: Vec<Param>,
        declarator: Arc<InnerDeclarator>,
    },
}

fn parse_paren_declarator(input: &mut Stream) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    parse_open_paren(input)?;

    let declarator = parse_inner_declarator(input)?;

    if let InnerDeclarator::Abstract = declarator {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }

    parse_close_paren(input)?;

    Ok(declarator)
}

fn parse_simple_declarator<'s>(input: &mut Stream<'s>) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_identifier.map(|i| InnerDeclarator::Identifier(i.to_string())),
        parse_paren_declarator,
    ))
    .parse_next(input)
}

fn parse_param(input: &mut Stream) -> PResult<Param> {
    parse_whitespace(input)?;

    combinator::seq!(Param {
        datatype: parse_primitive_datatype,
        declarator: combinator::alt((parse_inner_declarator, parse_inner_abstract_declarator)),
    })
    .parse_next(input)
}

fn parse_param_list(input: &mut Stream) -> PResult<Vec<Param>> {
    parse_whitespace(input)?;

    parse_open_paren(input)?;

    combinator::alt((
        combinator::repeat_till(0.., parse_param, parse_close_paren).map(|v| v.0),
        combinator::terminated(parse_void, parse_close_paren).map(|_| Vec::new()),
    ))
    .parse_next(input)
}

fn parse_function_declarator<'s>(input: &mut Stream<'s>) -> PResult<InnerDeclarator> {
    let declarator = parse_simple_declarator(input)?;

    if let InnerDeclarator::Abstract = declarator {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }

    let params = parse_param_list(input)?;

    Ok(InnerDeclarator::Function {
        declarator: Arc::new(declarator),
        params,
    })
}

fn parse_direct_declarator<'s>(input: &mut Stream<'s>) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    combinator::alt((parse_function_declarator, parse_simple_declarator)).parse_next(input)
}

fn parse_pointer_declarator(input: &mut Stream) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    parse_star(input)?;

    parse_inner_declarator
        .map(Arc::new)
        .map(InnerDeclarator::Pointer)
        .parse_next(input)
}

fn parse_abstract_direct_declarator(input: &mut Stream) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    parse_open_paren(input)?;

    let inner_declarator = parse_inner_abstract_declarator(input)?;

    parse_close_paren(input)?;

    Ok(inner_declarator)
}

fn parse_abstract_pointer_declarator(input: &mut Stream) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    parse_star(input)?;

    let inner_declarator = combinator::opt(parse_inner_abstract_declarator).parse_next(input)?;

    if let Some(inner_declarator) = inner_declarator {
        Ok(InnerDeclarator::Pointer(Arc::new(inner_declarator)))
    } else {
        Ok(InnerDeclarator::Pointer(Arc::new(
            InnerDeclarator::Abstract,
        )))
    }
}

fn parse_abstract_any_declarator(input: &mut Stream) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    Ok(InnerDeclarator::Abstract)
}

fn parse_inner_abstract_declarator(input: &mut Stream) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    println!("input: {:?}", input.input);

    combinator::alt((
        parse_abstract_direct_declarator,
        parse_abstract_pointer_declarator,
        parse_abstract_any_declarator,
    ))
    .parse_next(input)
}

fn parse_inner_declarator(input: &mut Stream) -> PResult<InnerDeclarator> {
    parse_whitespace(input)?;

    let declarator =
        combinator::alt((parse_pointer_declarator, parse_direct_declarator)).parse_next(input)?;

    if let InnerDeclarator::Abstract = declarator {
        return Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ));
    }

    Ok(declarator)
}

pub fn parse_declarator<'s>(input: &mut Stream<'s>, base_type: Datatype) -> PResult<Declarator> {
    parse_whitespace(input)?;

    let declarator = parse_inner_declarator(input)?;

    println!("Inner declarator: {:?}", declarator);

    let declarator = process_declarator(declarator, base_type.clone(), false);

    if let Some(declarator) = declarator {
        Ok(declarator)
    } else {
        Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new(),
        ))
    }
}

fn process_declarator(
    declarator: InnerDeclarator,
    base_type: Datatype,
    abstract_allowed: bool,
) -> Option<Declarator> {
    match declarator {
        InnerDeclarator::Identifier(name) => Some(Declarator {
            name,
            datatype: base_type,
        }),
        InnerDeclarator::Pointer(d) => process_declarator(
            d.as_ref().clone(),
            Datatype::Pointer {
                inner: Arc::new(base_type),
            },
            abstract_allowed,
        ),
        InnerDeclarator::Function { params, declarator } => {
            let params = params
                .into_iter()
                .map(|p| process_declarator(p.declarator, p.datatype, true))
                .collect::<Vec<_>>();

            if params.iter().any(Option::is_none) {
                return None;
            }

            let params = params.into_iter().map(Option::unwrap).collect::<Vec<_>>();

            let derived_type = Datatype::Function {
                return_type: Arc::new(base_type),
                arguments: params
                    .iter()
                    .map(|p| Argument {
                        name: p.name.clone(),
                        datatype: Arc::new(p.datatype.clone()),
                    })
                    .collect(),
            };

            println!("Derived type: {:?}", derived_type);
            process_declarator(declarator.as_ref().clone(), derived_type, abstract_allowed)
        }
        InnerDeclarator::Abstract => {
            println!("Abstract: {:?}", abstract_allowed);
            if abstract_allowed {
                Some(Declarator {
                    name: String::new(),
                    datatype: base_type,
                })
            } else {
                None
            }
        }
    }
}

pub fn parse_declaration(input: &mut Stream) -> PResult<Declaration> {
    parse_whitespace(input)?;

    let base_type = parse_primitive_datatype(input)?;

    let declarator = parse_declarator(input, base_type)?;

    println!("Variable: {:?}", declarator);

    let variable = input
        .state
        .add_variable(declarator.name.clone(), declarator.datatype.clone());

    let value = combinator::opt(parse_declaration_value).parse_next(input)?;

    let name = match variable {
        ParserSymbol::Variable(var) => var.unique_name,

        // these two should be unreachable, but they are here just in case
        ParserSymbol::Argument(var) => var.unique_name,
        ParserSymbol::Function(fun) => fun.name,
    };

    Ok(Declaration {
        datatype: declarator.datatype.clone(),
        name,
        value,
    })
}
