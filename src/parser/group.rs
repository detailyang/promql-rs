use crate::parser::error::Error;
use crate::parser::{parse_label_name, ws};
use log::debug;
use nom::bytes::complete::tag;
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

// grouping_labels : LEFT_PAREN grouping_label_list RIGHT_PAREN
// | LEFT_PAREN grouping_label_list COMMA RIGHT_PAREN
// | LEFT_PAREN RIGHT_PAREN
// ;
// grouping_label_list:
// grouping_label_list COMMA grouping_label
// | grouping_label
// ;
//
// grouping_label  : maybe_label
// ;
pub fn parse_grouping_labels(input: &str) -> IResult<&str, Vec<String>, Error<&str>> {
    debug!("parse_grouping_labels: {}", input);
    delimited(tag("("), parse_grouping_label_list, tag(")"))(input)
}

pub fn parse_grouping_label_list(input: &str) -> IResult<&str, Vec<String>, Error<&str>> {
    debug!("parse_grouping_label_list: {}", input);
    separated_list0(tag(","), ws(parse_grouping_label))(input).map(|(input, a)| {
        debug!("input:{} a:{:?}", input, a);
        (
            input,
            a.into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )
    })
}

pub fn parse_grouping_label(input: &str) -> IResult<&str, &str, Error<&str>> {
    debug!("parse_grouping_label: {}", input);
    context("grouping_label", parse_label_name)(input).map(|(input, a)| (input, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grouping_labels() {
        assert_eq!(
            parse_grouping_labels("(abc)"),
            Ok(("", vec!["abc".to_owned()]))
        );
        assert_eq!(
            parse_grouping_labels("(instance)"),
            Ok(("", vec!["instance".to_owned()]))
        );
        assert_eq!(
            parse_grouping_labels("(a,b,c)"),
            Ok(("", vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]))
        );
        assert_eq!(parse_grouping_labels("()"), Ok(("", vec![])));
        assert_eq!(parse_grouping_labels("(a)"), Ok(("", vec!["a".to_owned()])));
        assert_eq!(
            parse_grouping_labels("(a,b)"),
            Ok(("", vec!["a".to_owned(), "b".to_owned()]))
        );
    }
}
