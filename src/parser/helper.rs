use crate::parser::error::Error;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::recognize;
use nom::error::ParseError;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};

// > The metric name … must match the regex [a-zA-Z_:][a-zA-Z0-9_:]*.
pub fn parse_metric_name(input: &str) -> IResult<&str, &str, Error<&str>> {
    recognize(tuple((
        alt((alpha1, is_a("_:"))),
        many0(alt((alphanumeric1, is_a("_:")))),
    )))(input)
}

// > Label names … must match the regex [a-zA-Z_][a-zA-Z0-9_]*. Label names beginning with __ are reserved for internal use.
pub fn parse_label_name(input: &str) -> IResult<&str, &str, Error<&str>> {
    recognize(tuple((
        alt((alpha1, is_a("_"))),
        many0(alt((alphanumeric1, is_a("_")))),
    )))(input)
}

pub fn ws<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
    f: F,
) -> impl Parser<&'a str, O, E> {
    delimited(multispace0, f, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label() {
        assert_eq!(parse_label_name("instance)"), Ok((")", "instance")));
        assert_eq!(parse_label_name("a"), Ok(("", "a",)));
        assert_eq!(parse_label_name("abcd"), Ok(("", "abcd",)));
        assert_eq!(parse_label_name("__a__"), Ok(("", "__a__",)));
        assert_eq!(parse_label_name("__name__"), Ok(("", "__name__")));
        assert_eq!(parse_label_name("job="), Ok(("=", "job")));
    }

    #[test]
    fn test_parse_metric_name() {
        assert_eq!(parse_metric_name("a1234"), Ok(("", "a1234",)));

        assert_eq!(
            parse_metric_name("method_code:http_errors:rate5m"),
            Ok(("", "method_code:http_errors:rate5m",))
        );

        assert_eq!(parse_metric_name("__1__"), Ok(("", "__1__",)));
    }
}
