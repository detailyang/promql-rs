use crate::ast::modifier::{
    BinaryModifier, BinaryModifierAction, BinaryModifierGroup, BinaryModifierGroupSide,
};
use crate::parser::error::Error;
use crate::parser::group::parse_grouping_labels;
use crate::parser::ws;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;

pub(crate) fn parse_group_modifiers(input: &str) -> IResult<&str, BinaryModifier, Error<&str>> {
    tuple((
        parse_on_or_ignoring,
        opt(tuple((
            ws(alt((tag_no_case("group_left"), tag_no_case("group_right")))),
            parse_grouping_labels,
        ))),
    ))(input)
    .map(|(input, (mut a, b))| match b {
        Some((group, labels)) => {
            let group = match group {
                "group_left" => BinaryModifierGroup {
                    side: BinaryModifierGroupSide::Left,
                    labels,
                },
                "group_right" => BinaryModifierGroup {
                    side: BinaryModifierGroupSide::Right,
                    labels,
                },
                _ => panic!("never happened"),
            };
            a.group = Some(group);
            (input, a)
        }
        None => (input, a),
    })
}

fn parse_on_or_ignoring(input: &str) -> IResult<&str, BinaryModifier, Error<&str>> {
    alt((
        map(
            tuple((ws(tag_no_case("ignoring")), ws(parse_grouping_labels))),
            |(_, b)| BinaryModifier {
                action: BinaryModifierAction::Ignore,
                labels: b,
                ..Default::default()
            },
        ),
        map(
            tuple((ws(tag_no_case("on")), ws(parse_grouping_labels))),
            |(_, b)| BinaryModifier {
                action: BinaryModifierAction::On,
                labels: b,
                ..Default::default()
            },
        ),
    ))(input)
}

pub fn parse_binary_modifier(
    input: &str,
) -> IResult<&str, (bool, Option<BinaryModifier>), Error<&str>> {
    tuple((opt(ws(tag_no_case("bool"))), opt(parse_group_modifiers)))(input)
        .map(|(input, (a, b))| (input, (a.is_some(), b)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::modifier::{
        binary_group_modifier, binary_modifier, binary_modifier_group_left,
    };

    #[test]
    fn test_parse_binary_modifier() {
        assert_eq!(
            parse_binary_modifier("ignoring(a, b, c) group_left(d, e, g)"),
            Ok((
                "",
                (
                    false,
                    Some(binary_group_modifier(
                        BinaryModifierAction::Ignore,
                        vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
                        binary_modifier_group_left(vec![
                            "d".to_owned(),
                            "e".to_owned(),
                            "g".to_owned(),
                        ])
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_parse_on_or_ignoring() {
        assert_eq!(
            parse_on_or_ignoring("ignoring (a, b, c)"),
            Ok((
                "",
                binary_modifier(
                    BinaryModifierAction::Ignore,
                    vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
                )
            ))
        );

        assert_eq!(
            parse_on_or_ignoring("ON (a, b, c)"),
            Ok((
                "",
                binary_modifier(
                    BinaryModifierAction::On,
                    vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
                )
            ))
        );
    }
}
