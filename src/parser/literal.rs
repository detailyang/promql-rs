use crate::ast::literal::{NumberLiteral, StringLiteral};
use crate::parser::error::Error;
use crate::parser::ws;
use log::debug;
use nom::branch::alt;
use nom::bytes::complete::{escaped, is_not, tag};
use nom::character::complete::one_of;
use nom::number::complete::double;
use nom::sequence::delimited;
use nom::IResult;

pub fn parse_number_literal(input: &str) -> IResult<&str, NumberLiteral, Error<&str>> {
    debug!("parse_number_literal: {}", input);
    double(input).map(|(input, a)| (input, NumberLiteral { value: a }))
}

pub fn parse_string_literal(input: &str) -> IResult<&str, StringLiteral, Error<&str>> {
    debug!("parse_string_literal: {}", input);

    alt((
        delimited(
            ws(tag("\"")),
            escaped(is_not("\"\\"), '\\', one_of("\"\\")),
            ws(tag("\"")),
        ),
        delimited(
            ws(tag("'")),
            escaped(is_not("'\\"), '\\', one_of("'\\")),
            ws(tag("'")),
        ),
        delimited(
            ws(tag("`")),
            escaped(is_not("`\\"), '\\', one_of("`\\")),
            ws(tag("`")),
        ),
    ))(input)
    .map(|(input, s)| {
        (
            input,
            StringLiteral {
                value: s.to_owned(),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::literal::string_literal;

    #[test]
    fn test_parse_number_literal() {
        assert_eq!(
            parse_number_literal("1.23"),
            Ok(("", NumberLiteral { value: 1.23 }))
        );

        assert_eq!(
            parse_number_literal("-1.23"),
            Ok(("", NumberLiteral { value: -1.23 }))
        );
    }

    #[test]
    fn test_parse_string_literal() {
        assert_eq!(
            parse_string_literal(r#" `abcd` "#),
            Ok(("", string_literal("abcd"),))
        );

        assert_eq!(
            parse_string_literal(r#" `111\`222` "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"111\`222"#.to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_string_literal(r#" `1` "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"1"#.to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_string_literal(r#" '111\'222' "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"111\'222"#.to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_string_literal(r#" '1' "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"1"#.to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_string_literal(r#" "1" "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"1"#.to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_string_literal(r#" "abcd" "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"abcd"#.to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_string_literal(r#" "ab cd _ 123" "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"ab cd _ 123"#.to_owned(),
                }
            ))
        );

        assert_eq!(
            parse_string_literal(r#" "111\"222" "#),
            Ok((
                "",
                StringLiteral {
                    value: r#"111\"222"#.to_owned(),
                }
            ))
        );
    }
}
