use winnow::{combinator, token, PResult, Parser};

use super::{whitespace::parse_whitespace, Stream};

pub fn parse_hex_number(input: &mut Stream) -> PResult<i32> {
    parse_whitespace(input)?;

    "0x".parse_next(input)?;

    token::take_while(2.., |c: char| c.is_ascii_hexdigit())
        .parse_next(input)
        .map(|s| i32::from_str_radix(s, 16).unwrap())
}

pub fn parse_binary_number(input: &mut Stream) -> PResult<i32> {
    parse_whitespace(input)?;

    "0b".parse_next(input)?;

    token::take_while(2.., |c: char| c == '0' || c == '1')
        .parse_next(input)
        .map(|s| i32::from_str_radix(s, 2).unwrap())
}

pub fn parse_octal_number(input: &mut Stream) -> PResult<i32> {
    parse_whitespace(input)?;

    "0o".parse_next(input)?;

    token::take_while(2.., |c: char| c.is_ascii_digit())
        .parse_next(input)
        .map(|s| i32::from_str_radix(s, 8).unwrap())
}

pub fn parse_decimal_number(input: &mut Stream) -> PResult<i32> {
    parse_whitespace(input)?;

    token::take_while(1.., '0'..='9')
        .parse_next(input)
        .map(|s| s.parse().unwrap())
}

pub fn parse_number(input: &mut Stream) -> PResult<i32> {
    parse_whitespace(input)?;

    // todo: add support for negative numbers, floats etc
    combinator::alt((
        parse_hex_number,
        parse_binary_number,
        parse_octal_number,
        parse_decimal_number,
    ))
    .parse_next(input)
}
