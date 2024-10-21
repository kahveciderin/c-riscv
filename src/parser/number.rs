use winnow::{token, PResult, Parser};

use super::{whitespace::parse_whitespace, Stream};

pub fn parse_number(input: &mut Stream) -> PResult<i32> {
    parse_whitespace(input)?;

    // todo: add support for negative numbers, floats, other bases etc
    token::take_while(1.., '0'..='9')
        .parse_next(input)
        .map(|s| s.parse().unwrap())
}
