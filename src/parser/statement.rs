use std::sync::Arc;

use winnow::{combinator, error, PResult, Parser};

use crate::types::statement::{
    ForInit, ForStatement, IfStatement, JumpStatement, Statement, WhileStatement,
};

use super::{
    declaration::parse_declaration,
    expression::{parse_expression, parse_optional_expression},
    identifier::parse_identifier,
    scope::parse_scope,
    trivial_tokens::{parse_close_paren, parse_open_paren, parse_semicolon},
    whitespace::parse_whitespace,
    Stream,
};

pub fn parse_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;
    combinator::alt((
        parse_jump_statement,
        parse_if_statement,
        parse_while_statement,
        parse_for_statement,
        parse_expression_statement,
        parse_scope_statement,
        parse_null_statement,
    ))
    .parse_next(input)
}

pub fn parse_scope_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Scope { scope: parse_scope }).parse_next(input)
}

pub fn parse_null_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Null{
        _: parse_semicolon,
    })
    .parse_next(input)
}

pub fn parse_expression_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Expression{
        expression: parse_expression,
        _: parse_semicolon,
    })
    .parse_next(input)
}

pub fn parse_jump_statement<'s>(input: &mut Stream<'s>) -> PResult<Statement> {
    parse_whitespace(input)?;

    combinator::seq!(Statement::Jump {
        statement: combinator::alt((parse_break_jump, parse_return_jump, parse_continue_jump))
    })
    .parse_next(input)
}

pub fn parse_break_jump<'s>(input: &mut Stream<'s>) -> PResult<JumpStatement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "break" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    parse_semicolon(input)?;

    if let Some(_loop) = input.state.get_loop() {
        return Ok(JumpStatement::Break {
            id: _loop.id.clone(),
        });
    } else {
        todo!("Error: break statement outside of loop");
    }
}

pub fn parse_return_jump<'s>(input: &mut Stream<'s>) -> PResult<JumpStatement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "return" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    combinator::seq!(JumpStatement::Return{
            expression: parse_optional_expression,
            _: parse_semicolon,
    })
    .parse_next(input)
}

pub fn parse_continue_jump<'s>(input: &mut Stream<'s>) -> PResult<JumpStatement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "continue" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    parse_semicolon(input)?;

    if let Some(_loop) = input.state.get_loop() {
        return Ok(JumpStatement::Continue {
            id: _loop.id.clone(),
        });
    } else {
        todo!("Error: continue statement outside of loop");
    }
}

pub fn parse_else(input: &mut Stream) -> PResult<Statement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "else" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    parse_statement(input)
}

pub fn parse_if_statement(input: &mut Stream) -> PResult<Statement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "if" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    combinator::seq!(IfStatement {
        condition: parse_expression,
        then_block: parse_statement.map(Arc::new),
        else_block: combinator::opt(parse_else.map(Arc::new)),
    })
    .map(|s| Statement::If { statement: s })
    .parse_next(input)
}

pub fn parse_while_statement(input: &mut Stream) -> PResult<Statement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "while" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    parse_open_paren(input)?;

    let condition = parse_expression(input)?;

    parse_close_paren(input)?;

    let id = input.state.push_loop("while".to_string());

    let block = parse_statement(input).map(Arc::new)?;

    let while_statement = WhileStatement {
        condition,
        block,
        id,
    };

    Ok(Statement::While {
        statement: while_statement,
    })
}

pub fn parse_for_init(input: &mut Stream) -> PResult<ForInit> {
    parse_whitespace(input)?;

    combinator::alt((
        parse_declaration.map(ForInit::Declaration),
        parse_expression.map(ForInit::Expression),
    ))
    .parse_next(input)
}

pub fn parse_for_statement(input: &mut Stream) -> PResult<Statement> {
    parse_whitespace(input)?;

    let identifier = parse_identifier(input)?;
    if identifier != "for" {
        return Err(error::ErrMode::Backtrack(error::ContextError::new()));
    }

    input.state.push_scope();

    parse_open_paren(input)?;

    let init = combinator::opt(parse_for_init).parse_next(input)?;

    println!("{:?}, {}", init, input);

    parse_semicolon(input)?;

    let condition = combinator::opt(parse_expression).parse_next(input)?;

    parse_semicolon(input)?;

    let update = combinator::opt(parse_expression).parse_next(input)?;

    parse_close_paren(input)?;

    let id = input.state.push_loop("for".to_string());

    let block = parse_statement(input)?;

    input.state.pop_loop();
    input.state.pop_scope();

    Ok(Statement::For {
        statement: ForStatement {
            init,
            condition,
            increment: update,
            block: Arc::new(block),
            id,
        },
    })
}
