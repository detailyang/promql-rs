use crate::ast::selector::{LabelMatcher, LabelMatcherOp, Vector};
use crate::parser::error::{Error, ParserError};
use crate::parser::literal::parse_string_literal;
use crate::parser::{parse_label_name, parse_metric_name, ws};
use log::debug;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while};
use nom::character::is_alphanumeric;
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, tuple};
use nom::Err::Failure;
use nom::IResult;
use std::time::Duration;

pub(crate) fn parse_vector(input: &str) -> IResult<&str, Vector, Error<&str>> {
    tuple((
        parse_instant_vec,
        opt(delimited(tag("["), parse_duration, tag("]"))),
        opt(preceded(ws(tag_no_case("offset")), parse_duration)),
    ))(input)
    .map(|(input, (mut v, range, offset))| {
        v.range = range;
        v.offset = offset;
        (input, v)
    })
}

fn parse_instant_vec(input: &str) -> IResult<&str, Vector, Error<&str>> {
    debug!("parse_instant_vec: {}", input);
    map(
        tuple((opt(ws(parse_metric_name)), opt(ws(parse_label_matchers)))),
        |(id, matchers)| Vector {
            name: id.map(|s| s.to_owned()).unwrap_or_else(|| "".to_string()),
            label_matchers: matchers.unwrap_or_else(Vec::new),
            ..Default::default()
        },
    )(input)
}

pub fn parse_label_matchers(input: &str) -> IResult<&str, Vec<LabelMatcher>, Error<&str>> {
    delimited(
        ws(tag("{")),
        separated_list0(tag(","), parse_label_matcher),
        ws(tag("}")),
    )(input)
    .map(|(input, a)| (input, a))
}

pub fn parse_label_matcher(input: &str) -> IResult<&str, LabelMatcher, Error<&str>> {
    tuple((
        ws(parse_label_name),
        parse_label_matcher_op,
        parse_string_literal,
    ))(input)
    .map(|(input, (a, b, c))| {
        (
            input,
            LabelMatcher {
                op: b,
                name: a.to_owned(),
                value: c.value,
            },
        )
    })
}

pub(crate) fn parse_label_matcher_op(input: &str) -> IResult<&str, LabelMatcherOp, Error<&str>> {
    alt((ws(tag("!~")), ws(tag("!=")), ws(tag("=~")), ws(tag("="))))(input).map(|(input, op)| {
        match op.to_lowercase().as_str() {
            "=" => (input, LabelMatcherOp::Equal),
            "!=" => (input, LabelMatcherOp::NotEqual),
            "=~" => (input, LabelMatcherOp::Regexp),
            "!~" => (input, LabelMatcherOp::NotRegexp),
            _ => panic!("never happened"),
        }
    })
}

pub(crate) fn parse_duration(input: &str) -> IResult<&str, Duration, Error<&str>> {
    match take_while(|x: char| is_alphanumeric(x as u8))(input) {
        Ok((input, d)) => match humantime::parse_duration(d) {
            Ok(d) => Ok((input, d)),
            Err(e) => Err(Failure(Error::Parser(ParserError::InvalidDuration(e)))),
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::selector::LabelMatcherOp::Equal;

    #[test]
    fn test_parse_r() {
        assert_eq!(
            parse_vector("nodejs_http_requests[5m]"),
            Ok((
                "",
                Vector {
                    name: "nodejs_http_requests".to_owned(),
                    range: Some(Duration::from_secs(5 * 60)),
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            parse_vector("foo { }"),
            Ok((
                "",
                Vector {
                    name: "foo".to_owned(),
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn test_parse_v() {
        assert_eq!(
            parse_instant_vec(" sdk"),
            Ok((
                "",
                Vector {
                    name: "sdk".to_owned(),
                    ..Default::default()
                }
            ))
        );
        assert_eq!(
            parse_vector("nodejs_http_requests"),
            Ok((
                "",
                Vector {
                    name: "nodejs_http_requests".to_owned(),
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            parse_vector("{a=\"1\", b=\"2\"}"),
            Ok((
                "",
                Vector {
                    name: "".to_owned(),
                    label_matchers: vec![
                        LabelMatcher {
                            op: Equal,
                            name: "a".to_owned(),
                            value: "1".to_owned()
                        },
                        LabelMatcher {
                            op: Equal,
                            name: "b".to_owned(),
                            value: "2".to_owned()
                        },
                    ],
                    ..Default::default()
                }
            ))
        );

        assert_eq!(
            parse_vector("nodejs_http_requests{a=\"1\"}"),
            Ok((
                "",
                Vector {
                    name: "nodejs_http_requests".to_owned(),
                    label_matchers: vec![LabelMatcher {
                        op: Equal,
                        name: "a".to_owned(),
                        value: "1".to_owned()
                    }],
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("5m"), Ok(("", Duration::from_secs(60 * 5))))
    }

    #[test]
    fn test_parse_label_matchers() {
        assert_eq!(
            parse_label_matchers(r#" { job=~".*" } "#),
            Ok((
                "",
                vec![LabelMatcher {
                    op: LabelMatcherOp::Regexp,
                    name: "job".to_owned(),
                    value: ".*".to_owned(),
                },]
            ))
        );

        assert_eq!(
            parse_label_matchers(r#"{job=~".*",method="get"}"#),
            Ok((
                "",
                vec![
                    LabelMatcher {
                        op: LabelMatcherOp::Regexp,
                        name: "job".to_owned(),
                        value: ".*".to_owned(),
                    },
                    LabelMatcher {
                        op: LabelMatcherOp::Equal,
                        name: "method".to_owned(),
                        value: "get".to_owned(),
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_parse_label_matcher() {
        assert_eq!(
            parse_label_matcher(r#"a = "1""#),
            Ok((
                "",
                LabelMatcher {
                    op: LabelMatcherOp::Equal,
                    name: "a".to_owned(),
                    value: "1".to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_label_matcher(r#" a != "1" "#),
            Ok((
                "",
                LabelMatcher {
                    op: LabelMatcherOp::NotEqual,
                    name: "a".to_owned(),
                    value: "1".to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_label_matcher(r#" a =~ "1" "#),
            Ok((
                "",
                LabelMatcher {
                    op: LabelMatcherOp::Regexp,
                    name: "a".to_owned(),
                    value: "1".to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_label_matcher(r#" a !~ "1" "#),
            Ok((
                "",
                LabelMatcher {
                    op: LabelMatcherOp::NotRegexp,
                    name: "a".to_owned(),
                    value: "1".to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_label_matcher(r#" __name__=~"job:.*" "#),
            Ok((
                "",
                LabelMatcher {
                    op: LabelMatcherOp::Regexp,
                    name: "__name__".to_owned(),
                    value: "job:.*".to_owned(),
                }
            ))
        );
    }
}
