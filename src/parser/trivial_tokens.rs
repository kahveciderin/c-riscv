use winnow::{combinator, PResult, Parser};

use super::{whitespace::parse_whitespace, Stream};

pub fn parse_multiple_chars<'s>(input: &mut Stream<'s>, mut s: &str) -> PResult<&'s str> {
    parse_whitespace(input)?;
    s.parse_next(input)
}

pub fn parse_multiple_chars_not_followed_by<'s>(
    input: &mut Stream<'s>,
    s: &str,
    n: &[&str],
) -> PResult<&'s str> {
    parse_whitespace(input)?;

    for not in n.iter() {
        combinator::not(combinator::terminated(s, *not)).parse_next(input)?;
    }

    parse_multiple_chars(input, s)
}

pub fn parse_open_paren<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "(")
}
pub fn parse_close_paren<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ")")
}

pub fn parse_open_scope<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "{")
}
pub fn parse_close_scope<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "}")
}

pub fn parse_semicolon<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ";")
}

pub fn parse_comma<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ",")
}

pub fn parse_plus<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "+", &["+", "="])
}

pub fn parse_minus<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "-", &["-", "="])
}

pub fn parse_tilda<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "~")
}

pub fn parse_bang<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "!", &["="])
}

pub fn parse_star<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "*", &["="])
}

pub fn parse_slash<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "/", &["="])
}

pub fn parse_percent<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "%", &["="])
}

pub fn parse_pipe<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "|", &["|", "="])
}

pub fn parse_caret<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "^", &["="])
}

pub fn parse_ampersand<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "&", &["&", "="])
}

pub fn parse_double_pipe<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "||")
}

pub fn parse_double_ampersand<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "&&")
}

pub fn parse_double_greater_than<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, ">>", &["="])
}

pub fn parse_double_less_than<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "<<", &["="])
}

pub fn parse_greater_than<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, ">", &[">", "="])
}

pub fn parse_less_than<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "<", &[">", "="])
}

pub fn parse_greater_than_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ">=")
}

pub fn parse_less_than_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "<=")
}

pub fn parse_double_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "==")
}

pub fn parse_bang_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "!=")
}

pub fn parse_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "=", &["="])
}

pub fn parse_plus_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "+=")
}

pub fn parse_minus_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "-=")
}
pub fn parse_star_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "*=")
}
pub fn parse_slash_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "/=")
}
pub fn parse_percent_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "%=")
}
pub fn parse_double_greater_than_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ">>=")
}
pub fn parse_double_less_than_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "<<=")
}
pub fn parse_ampersand_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "&=")
}
pub fn parse_caret_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "^=")
}
pub fn parse_pipe_equals<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "|=")
}

pub fn parse_question_mark<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "?")
}

pub fn parse_colon<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, ":")
}

pub fn parse_double_plus<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "++")
}

pub fn parse_double_minus<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "--")
}

pub fn parse_void<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars(input, "void")
}

pub fn parse_pointer_ampersand<'s>(input: &mut Stream<'s>) -> PResult<&'s str> {
    parse_multiple_chars_not_followed_by(input, "&", &[])
}
