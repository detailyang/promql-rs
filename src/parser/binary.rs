use crate::ast::binary::BinaryExpr;
use crate::ast::Expr;
use crate::parser::error::Error;
use crate::parser::op::{
    parse_binary_op_and_unless, parse_binary_op_compare, parse_binary_op_mul_div_mod,
    parse_binary_op_or, parse_binary_op_plus_minus, parse_binary_op_power,
};
use crate::parser::{parse_atom, ws};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

fn parse_power(input: &str) -> IResult<&str, Expr, Error<&str>> {
    tuple((
        parse_atom,
        opt(tuple((ws(parse_binary_op_power), parse_power))),
    ))(input)
    .map(|(input, (a, b))| match b {
        None => (input, a),
        Some((op, b)) => (
            input,
            Expr::BinaryExpr(Box::new(BinaryExpr { op, lhs: a, rhs: b })),
        ),
    })
}

fn parse_mul_div_mod(input: &str) -> IResult<&str, Expr, Error<&str>> {
    tuple((
        parse_power,
        many0(tuple((ws(parse_binary_op_mul_div_mod), parse_power))),
    ))(input)
    .map(|(input, (a, b))| {
        if b.is_empty() {
            return (input, a);
        }

        let mut a = a;
        for (op, c) in b.into_iter() {
            a = Expr::BinaryExpr(Box::new(BinaryExpr { op, lhs: a, rhs: c }))
        }

        (input, a)
    })
}

fn parse_plus_minus(input: &str) -> IResult<&str, Expr, Error<&str>> {
    tuple((
        parse_mul_div_mod,
        many0(tuple((ws(parse_binary_op_plus_minus), parse_mul_div_mod))),
    ))(input)
    .map(|(input, (a, b))| {
        if b.is_empty() {
            return (input, a);
        }

        let mut a = a;
        for (op, c) in b.into_iter() {
            a = Expr::BinaryExpr(Box::new(BinaryExpr { op, lhs: a, rhs: c }))
        }

        (input, a)
    })
}

pub fn parse_compare(input: &str) -> IResult<&str, Expr, Error<&str>> {
    tuple((
        parse_plus_minus,
        many0(tuple((ws(parse_binary_op_compare), parse_plus_minus))),
    ))(input)
    .map(|(input, (a, b))| {
        if b.is_empty() {
            return (input, a);
        }

        let mut a = a;
        for (op, c) in b.into_iter() {
            a = Expr::BinaryExpr(Box::new(BinaryExpr { op, lhs: a, rhs: c }))
        }
        (input, a)
    })
}

fn parse_and_unless(input: &str) -> IResult<&str, Expr, Error<&str>> {
    tuple((
        parse_compare,
        many0(tuple((ws(parse_binary_op_and_unless), parse_compare))),
    ))(input)
    .map(|(input, (a, b))| {
        if b.is_empty() {
            return (input, a);
        }

        let mut a = a;
        for (op, c) in b.into_iter() {
            a = Expr::BinaryExpr(Box::new(BinaryExpr { op, lhs: a, rhs: c }))
        }
        (input, a)
    })
}

pub fn parse_or(input: &str) -> IResult<&str, Expr, Error<&str>> {
    tuple((
        parse_and_unless,
        many0(tuple((ws(parse_binary_op_or), parse_and_unless))),
    ))(input)
    .map(|(input, (a, b))| {
        if b.is_empty() {
            return (input, a);
        }

        let mut a = a;
        for (op, c) in b.into_iter() {
            a = Expr::BinaryExpr(Box::new(BinaryExpr { op, lhs: a, rhs: c }))
        }
        (input, a)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::binary::BinaryExpr;
    use crate::ast::op::BinaryOp;
    use crate::ast::selector::Vector;
    use crate::ast::{binary_expr, vector, vector_expr, Expr};

    #[test]
    fn test_parse_or() {
        assert_eq!(
            parse_or("a or b"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::Or(None),
                    vector_expr(vector("a")),
                    vector_expr(vector("b")),
                )
            )),
        );
    }

    #[test]
    fn test_parse_and_unless() {
        assert_eq!(
            parse_and_unless("a and b"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::And(None),
                    vector_expr(vector("a")),
                    vector_expr(vector("b")),
                )
            )),
        );

        assert_eq!(
            parse_and_unless("a and b and c"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::And(None),
                    binary_expr(
                        BinaryOp::And(None),
                        vector_expr(vector("a")),
                        vector_expr(vector("b")),
                    ),
                    vector_expr(vector("c")),
                )
            )),
        );
    }

    #[test]
    fn test_parse_compare() {
        assert_eq!(
            parse_compare("a > b"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::GreaterThan(false, None),
                    vector_expr(vector("a")),
                    vector_expr(vector("b")),
                )
            )),
        );

        assert_eq!(
            parse_compare("a > b > c"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::GreaterThan(false, None),
                    binary_expr(
                        BinaryOp::GreaterThan(false, None),
                        vector_expr(vector("a")),
                        vector_expr(vector("b")),
                    ),
                    vector_expr(vector("c")),
                )
            )),
        );
    }

    #[test]
    fn test_parse_plus_minus() {
        assert_eq!(
            parse_plus_minus("a + b"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::Add(None),
                    vector_expr(vector("a")),
                    vector_expr(vector("b")),
                )
            )),
        );
        assert_eq!(
            parse_plus_minus("a + b * c"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::Add(None),
                    vector_expr(vector("a")),
                    binary_expr(
                        BinaryOp::Mul(None),
                        vector_expr(vector("b")),
                        vector_expr(vector("c")),
                    ),
                )
            )),
        );
    }

    #[test]
    fn test_parse_mul_div_mod() {
        assert_eq!(
            parse_mul_div_mod("a / b"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::Div(None),
                    vector_expr(vector("a")),
                    vector_expr(vector("b")),
                )
            )),
        );

        assert_eq!(
            parse_mul_div_mod("a * b / c"),
            Ok((
                "",
                binary_expr(
                    BinaryOp::Div(None),
                    binary_expr(
                        BinaryOp::Mul(None),
                        vector_expr(vector("a")),
                        vector_expr(vector("b")),
                    ),
                    vector_expr(vector("c")),
                )
            )),
        );
    }

    #[test]
    fn test_parse_power() {
        assert_eq!(
            parse_power("a ^ b"),
            Ok((
                "",
                Expr::BinaryExpr(Box::new(BinaryExpr {
                    op: BinaryOp::Power(None),
                    lhs: Expr::VectorExpr(Box::new(Vector {
                        name: "a".to_owned(),
                        ..Default::default()
                    })),
                    rhs: Expr::VectorExpr(Box::new(Vector {
                        name: "b".to_owned(),
                        ..Default::default()
                    })),
                    ..Default::default()
                }))
            ))
        );
    }
}
