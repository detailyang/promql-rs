use crate::ast::op::BinaryOp;
use crate::parser::error::Error;
use crate::parser::modifier::parse_binary_modifier;
use crate::parser::ws;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

pub(crate) fn parse_binary_op_power(input: &str) -> IResult<&str, BinaryOp, Error<&str>> {
    tuple((ws(tag("^")), opt(parse_binary_modifier)))(input).map(|(input, (a, b))| match b {
        Some((_c, d)) => (
            input,
            match a {
                "^" => BinaryOp::Power(d),
                _ => panic!("never happened"),
            },
        ),
        None => (
            input,
            match a {
                "^" => BinaryOp::Power(None),
                _ => panic!("never happened"),
            },
        ),
    })
}

pub(crate) fn parse_binary_op_mul_div_mod(input: &str) -> IResult<&str, BinaryOp, Error<&str>> {
    tuple((
        ws(alt((tag("*"), tag("/"), tag("%")))),
        opt(parse_binary_modifier),
    ))(input)
    .map(|(input, (a, b))| match b {
        Some((_c, d)) => (
            input,
            match a {
                "*" => BinaryOp::Mul(d),
                "/" => BinaryOp::Div(d),
                "%" => BinaryOp::Mod(d),
                _ => panic!("never happened"),
            },
        ),
        None => (
            input,
            match a {
                "*" => BinaryOp::Mul(None),
                "/" => BinaryOp::Div(None),
                "%" => BinaryOp::Mod(None),
                _ => panic!("never happened"),
            },
        ),
    })
}

pub(crate) fn parse_binary_op_plus_minus(input: &str) -> IResult<&str, BinaryOp, Error<&str>> {
    tuple((ws(alt((tag("+"), tag("-")))), opt(parse_binary_modifier)))(input).map(
        |(input, (a, b))| match b {
            Some((_, d)) => (
                input,
                match a {
                    "+" => BinaryOp::Add(d),
                    "-" => BinaryOp::Sub(d),
                    _ => panic!("never happened"),
                },
            ),
            None => (
                input,
                match a {
                    "+" => BinaryOp::Add(None),
                    "-" => BinaryOp::Sub(None),
                    _ => panic!("never happened"),
                },
            ),
        },
    )
}

pub(crate) fn parse_binary_op_compare(input: &str) -> IResult<&str, BinaryOp, Error<&str>> {
    tuple((
        ws(alt((
            tag(">="),
            tag("<="),
            tag("=="),
            tag("!="),
            tag(">"),
            tag("<"),
        ))),
        opt(parse_binary_modifier),
    ))(input)
    .map(|(input, (a, b))| match b {
        Some((c, d)) => (
            input,
            match a {
                ">=" => BinaryOp::GreaterEqual(c, d),
                "<=" => BinaryOp::LessEqual(c, d),
                "!=" => BinaryOp::NotEqual(c, d),
                "==" => BinaryOp::Equal(c, d),
                ">" => BinaryOp::GreaterThan(c, d),
                "<" => BinaryOp::LessThan(c, d),
                _ => panic!("never happened"),
            },
        ),
        None => (
            input,
            match a {
                ">=" => BinaryOp::GreaterEqual(false, None),
                "<=" => BinaryOp::LessEqual(false, None),
                "!=" => BinaryOp::NotEqual(false, None),
                "==" => BinaryOp::Equal(false, None),
                ">" => BinaryOp::GreaterThan(false, None),
                "<" => BinaryOp::LessThan(false, None),
                _ => panic!("never happened"),
            },
        ),
    })
}

pub fn parse_binary_op_and_unless(input: &str) -> IResult<&str, BinaryOp, Error<&str>> {
    tuple((
        ws(alt((tag("and"), tag("less")))),
        opt(parse_binary_modifier),
    ))(input)
    .map(|(input, (a, b))| match b {
        Some((_c, d)) => (
            input,
            match a {
                "and" => BinaryOp::And(d),
                "unless" => BinaryOp::Unless(d),
                _ => panic!("never happened"),
            },
        ),
        None => (
            input,
            match a {
                "and" => BinaryOp::And(None),
                "unless" => BinaryOp::Unless(None),
                _ => panic!("never happened"),
            },
        ),
    })
}

pub(crate) fn parse_binary_op_or(input: &str) -> IResult<&str, BinaryOp, Error<&str>> {
    tuple((ws(tag("or")), opt(parse_binary_modifier)))(input).map(|(input, (a, b))| match b {
        Some((_c, d)) => (
            input,
            match a {
                "or" => BinaryOp::Or(d),
                _ => panic!("never happened"),
            },
        ),
        None => (
            input,
            match a {
                "or" => BinaryOp::Or(None),
                _ => panic!("never happened"),
            },
        ),
    })
}
