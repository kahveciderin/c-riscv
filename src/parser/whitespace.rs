use winnow::{token, PResult, Parser};

use super::Stream;

pub fn parse_whitespace(input: &mut Stream) -> PResult<()> {
    token::take_while(0.., (' ', '\n', '\t', '\r'))
        .void()
        .parse_next(input)
}
