use crate::ast;
use crate::ast::aggregator::{AggregationModifier, AggregationModifierAction};
use crate::ast::funcall::FunCall;
use crate::ast::Expr;
use crate::parser::error::Error;
use crate::parser::group::parse_grouping_labels;
use crate::parser::literal::parse_string_literal;
use crate::parser::{parse_expr, parse_metric_name, ws};
use log::debug;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};

pub fn parse_function_call(input: &str) -> IResult<&str, FunCall, Error<&str>> {
    tuple((
        parse_metric_name,
        alt((
            map(
                tuple((parse_function_call_body, opt(parse_aggregation_modifier))),
                |(a, b)| (a, b),
            ),
            map(
                tuple((opt(parse_aggregation_modifier), parse_function_call_body)),
                |(a, b)| (b, a),
            ),
        )),
    ))(input)
    .map(|(input, (name, (args, aggregation)))| {
        (
            input,
            FunCall {
                name: name.to_owned(),
                args,
                aggregation,
            },
        )
    })
}

pub fn parse_aggregation_modifier(input: &str) -> IResult<&str, AggregationModifier, Error<&str>> {
    debug!("parse_aggregation_modifier: {}", input);
    tuple((
        alt((ws(tag_no_case("by")), ws(tag_no_case("without")))),
        ws(parse_grouping_labels),
    ))(input)
    .map(|(input, (b, c))| {
        let action = match b {
            "by" => AggregationModifierAction::By,
            "without" => AggregationModifierAction::Without,
            _ => {
                panic!("never happened")
            }
        };
        (input, AggregationModifier { action, labels: c })
    })
}

fn parse_function_call_body(input: &str) -> IResult<&str, Vec<Expr>, Error<&str>> {
    debug!("parse_function_call_body: {}", input);
    context(
        "function_call_body",
        map(
            delimited(ws(tag("(")), parse_function_call_args, ws(tag(")"))),
            |a| a,
        ),
    )(input)
    .map(|(input, a)| (input, a))
}

fn parse_function_call_args(input: &str) -> IResult<&str, Vec<Expr>, Error<&str>> {
    debug!("parse_function_call_args: {}", input);
    separated_list0(
        tag(","),
        alt((
            parse_string_literal.map(|s| Expr::StringLiteralExpr(Box::new(s))),
            parse_expr,
        )),
    )(input)
    .map(|(input, a)| {
        let mut args: Vec<Expr> = Vec::new();
        a.into_iter().for_each(|v| match v {
            Expr::VectorExpr(v) => {
                if v.name.is_empty() && v.label_matchers.is_empty() {
                    return;
                }
                args.push(ast::Expr::VectorExpr(v));
            }
            _ => {
                args.push(v);
            }
        });
        (input, args)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::literal::StringLiteral;
    use crate::ast::op::BinaryOp;
    use crate::ast::selector::{v, v_r, LabelMatcher, LabelMatcherOp, Vector};
    use crate::ast::{binary_expr, fun_call, fun_call_expr, number_literal_expr, vector_expr};
    use std::time::Duration;

    #[test]
    fn test_parse_aggregation_modifier() {
        assert_eq!(
            parse_aggregation_modifier("by (instance)"),
            Ok((
                "",
                AggregationModifier {
                    action: AggregationModifierAction::By,
                    labels: vec!["instance".to_owned()]
                }
            ))
        );
    }

    #[test]
    fn test_parse_function_call_body2() {
        assert_eq!(
            parse_function_call_body("(rate(whatever [5m]) > 0, 0.2)"),
            Ok((
                "",
                vec![
                    binary_expr(
                        BinaryOp::GreaterThan(false, None),
                        fun_call_expr(fun_call(
                            "rate",
                            vec![vector_expr(v_r("whatever", Duration::from_secs(300))),]
                        )),
                        number_literal_expr(0.0),
                    ),
                    number_literal_expr(0.2),
                ],
            ))
        )
    }

    #[test]
    fn test_parse_function_multi_args() {
        assert_eq!(
            parse_function_call("quux(xyzzy, plough)"),
            Ok((
                "",
                FunCall {
                    name: "quux".to_owned(),
                    args: vec![vector_expr(v("xyzzy")), vector_expr(v("plough")),],
                    ..Default::default()
                }
            ))
        )
    }

    #[test]
    fn test_parse_function() {
        assert_eq!(
            parse_function_call(
                r#"label_replace(up{job="api-server",service="a:c"},   'foo' , "$1", "service", "(.*):.*")"#,
            ),
            Ok((
                "",
                FunCall {
                    name: "label_replace".to_owned(),
                    args: vec![
                        Expr::VectorExpr(Box::new(Vector {
                            name: "up".to_owned(),
                            label_matchers: vec![
                                LabelMatcher {
                                    op: LabelMatcherOp::Equal,
                                    name: "job".to_owned(),
                                    value: "api-server".to_owned()
                                },
                                LabelMatcher {
                                    op: LabelMatcherOp::Equal,
                                    name: "service".to_owned(),
                                    value: "a:c".to_owned()
                                }
                            ],
                            ..Default::default()
                        })),
                        Expr::StringLiteralExpr(Box::new(StringLiteral {
                            value: "foo".to_owned()
                        })),
                        Expr::StringLiteralExpr(Box::new(StringLiteral {
                            value: "$1".to_owned()
                        })),
                        Expr::StringLiteralExpr(Box::new(StringLiteral {
                            value: "service".to_owned()
                        })),
                        Expr::StringLiteralExpr(Box::new(StringLiteral {
                            value: "(.*):.*".to_owned()
                        }))
                    ],
                    ..Default::default()
                }
            ))
        );
    }

    #[test]
    fn test_parse_function_call() {
        assert_eq!(
            parse_function_call("rate(whatever[5m])"),
            Ok((
                "",
                FunCall {
                    name: "rate".to_owned(),
                    args: vec![Expr::VectorExpr(Box::new(Vector {
                        name: "whatever".to_owned(),
                        range: Some(Duration::from_secs(300)),
                        ..Default::default()
                    }))],
                    ..Default::default()
                }
            ))
        )
    }

    #[test]
    fn test_parse_function_call_body() {
        assert_eq!(
            parse_function_call_body("( a )"),
            Ok((
                "",
                vec![Expr::VectorExpr(Box::new(Vector {
                    name: "a".to_owned(),
                    ..Default::default()
                }))]
            ))
        );
    }

    #[test]
    fn test_parse_function_call_args() {
        assert_eq!(
            parse_function_call_args("nodejs_http_requests"),
            Ok((
                "",
                vec![Expr::VectorExpr(Box::new(Vector {
                    name: "nodejs_http_requests".to_owned(),
                    ..Default::default()
                }))]
            ))
        );
    }

    #[test]
    fn test_parse_func_by() {
        assert_eq!(
            parse_function_call("sum by (instance)(a)"),
            Ok((
                "",
                FunCall {
                    name: "sum".to_owned(),
                    args: vec![vector_expr(v("a"))],
                    aggregation: Some(AggregationModifier {
                        action: AggregationModifierAction::By,
                        labels: vec!["instance".to_owned()],
                    })
                }
            ))
        );
    }
}
