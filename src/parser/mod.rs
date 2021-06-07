use crate::ast::{Expr, SubqueryExpr};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded, tuple};
use nom::{IResult, Parser};

pub use binary::*;
pub use error::*;
pub use func::*;
pub use group::*;
pub use helper::*;
pub use literal::*;
pub use modifier::*;
pub use op::*;
pub use vector::*;

mod binary;
mod error;
mod func;
mod group;
mod helper;
mod literal;
mod modifier;
mod op;
mod vector;

pub fn parse_expr(input: &str) -> IResult<&str, Expr, Error<&str>> {
    tuple((
        parse_or,
        opt(tuple((
            ws(tag("[")),
            parse_duration,
            ws(tag(":")),
            opt(parse_duration),
            ws(tag("]")),
        ))),
    ))(input)
    .map(|(input, (expr, b))| match b {
        None => (input, expr),
        Some((_, range, _, resolution, _)) => (
            input,
            Expr::SubQueryExpr(Box::new(SubqueryExpr {
                expr,
                range: Some(range),
                resolution,
            })),
        ),
    })
}

pub(crate) fn parse_atom(input: &str) -> IResult<&str, Expr, Error<&str>> {
    alt((
        delimited(tag("("), parse_expr, tag(")")),
        ws(parse_number_literal).map(|e| Expr::NumberLiteralExpr(Box::new(e))),
        preceded(tag("+"), parse_atom),
        map(preceded(tag("-"), parse_atom), |e| {
            Expr::NegationExpr(Box::new(e))
        }),
        parse_function_call.map(|e| Expr::FunCallExpr(Box::new(e))),
        parse_vector.map(|e| Expr::VectorExpr(Box::new(e))),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::ast::aggregator::{AggregationModifier, AggregationModifierAction};
    use crate::ast::op::BinaryOp;
    use crate::ast::selector::LabelMatcherOp::Equal;
    use crate::ast::selector::{v_r, LabelMatcher};
    use crate::ast::{
        binary_expr, fun_call, fun_call_agg, fun_call_expr, number_literal_expr, subquery_expr,
        vector, vector_expr, vector_labels, Vector,
    };
    use crate::parse_expr;
    use crate::parser::parse_vector;
    use std::time::Duration;

    #[test]
    fn test_parse_offset_duration() {
        assert_eq!(
            parse_vector("a{}[1m] offset 10m"),
            Ok((
                "",
                Vector {
                    name: "a".to_owned(),
                    label_matchers: vec![],
                    offset: Some(Duration::from_secs(60 * 10)),
                    range: Some(Duration::from_secs(60)),
                }
            ))
        );
    }

    #[test]
    fn test_parse_paren() {
        assert_eq!(
            parse_expr("(1+2)*3"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::Mul(None),
                    binary_expr(
                        BinaryOp::Add(None),
                        number_literal_expr(1.0),
                        number_literal_expr(2.0)
                    ),
                    number_literal_expr(3.0),
                )
            )),
        );
    }

    #[test]
    fn test_comple_expr() {
        assert_eq!(
            parse_expr(
                "sum(1 - something_used{env=\"production\"} / something_total) by (instance)"
            ),
            Ok((
                "",
                fun_call_expr(fun_call_agg(
                    "sum",
                    vec![binary_expr(
                        BinaryOp::Sub(None),
                        number_literal_expr(1.0),
                        binary_expr(
                            BinaryOp::Div(None),
                            vector_expr(vector_labels(
                                "something_used",
                                vec![LabelMatcher {
                                    op: Equal,
                                    name: "env".to_owned(),
                                    value: "production".to_owned()
                                }]
                            )),
                            vector_expr(vector("something_total"))
                        )
                    )],
                    Some(AggregationModifier {
                        action: AggregationModifierAction::By,
                        labels: vec!["instance".to_owned()]
                    })
                )),
            ))
        )
    }

    #[test]
    fn test_parse_subquery() {
        assert_eq!(
            parse_expr("min_over_time(rate(http_requests_total[5m])[30m:1m])"),
            Ok((
                "",
                fun_call_expr(fun_call(
                    "min_over_time",
                    vec![subquery_expr(
                        fun_call_expr(fun_call(
                            "rate",
                            vec![vector_expr(v_r(
                                "http_requests_total",
                                Duration::from_secs(300)
                            ))],
                        )),
                        Some(Duration::from_secs(30 * 60)),
                        Some(Duration::from_secs(60)),
                    )]
                )),
            ))
        );
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(
            parse_expr("foo() + bar(baz) + quux(xyzzy, plough)"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::Add(None),
                    binary_expr(
                        BinaryOp::Add(None),
                        fun_call_expr(fun_call("foo", vec![])),
                        fun_call_expr(fun_call("bar", vec![vector_expr(vector("baz"))]))
                    ),
                    fun_call_expr(fun_call(
                        "quux",
                        vec![vector_expr(vector("xyzzy")), vector_expr(vector("plough")),]
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_parse_expr2() {
        assert_eq!(
            parse_expr("round(rate(whatever [5m]) > 0, 0.2)"),
            Ok((
                "",
                fun_call_expr(fun_call(
                    "round",
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
                    ]
                )),
            ))
        );
    }

    #[test]
    fn test_parse_expr3() {
        // "sum by (bar) (foo) * count without (bar) (foo)"
    }

    #[test]
    fn test_parse_binary_complex_compare_expr() {
        assert_eq!(
            parse_expr("foo > bar != 0 and 15.5 < xyzzy"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::And(None),
                    binary_expr(
                        BinaryOp::NotEqual(false, None),
                        binary_expr(
                            BinaryOp::GreaterThan(false, None),
                            vector_expr(vector("foo")),
                            vector_expr(vector("bar")),
                        ),
                        number_literal_expr(0.0),
                    ),
                    binary_expr(
                        BinaryOp::LessThan(false, None),
                        number_literal_expr(15.5),
                        vector_expr(vector("xyzzy"))
                    )
                ),
            ))
        );
    }
}
