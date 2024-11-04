use winnow::{combinator, PResult, Parser};

use crate::{
    parser::ParserSymbol,
    types::{declaration::Declaration, expression::Expression},
};

use super::{
    datatype::parse_datatype, expression::parse_expression, trivial_tokens::parse_equals,
    whitespace::parse_whitespace, Stream,
};
use crate::parser::identifier::parse_identifier;

fn parse_declaration_value(input: &mut Stream) -> PResult<Expression> {
    parse_whitespace(input)?;

    parse_equals(input)?;

    parse_expression(input)
}

pub fn parse_declaration(input: &mut Stream) -> PResult<Declaration> {
    parse_whitespace(input)?;

    let data_type = parse_datatype(input)?;

    let identifier = parse_identifier(input)?.to_string();

    println!("Variable: {:?}", identifier);
    let variable = input
        .state
        .add_variable(identifier.clone(), data_type.clone());

    let value = combinator::opt(parse_declaration_value).parse_next(input)?;

    let name = match variable {
        ParserSymbol::Variable(var) => var.unique_name,

        // these two should be unreachable, but they are here just in case
        ParserSymbol::Argument(var) => var.unique_name,
        ParserSymbol::Function(fun) => fun.name,
    };

    Ok(Declaration {
        data_type,
        name,
        value,
    })
}
