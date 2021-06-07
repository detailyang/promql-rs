use crate::ast::{LabelMatcher, Vector};
use std::time::Duration;

pub fn vector_labels(name: &str, label_matchers: Vec<LabelMatcher>) -> Vector {
    Vector {
        name: name.to_owned(),
        label_matchers,
        ..Default::default()
    }
}

pub fn vector_labels_range(
    name: &str,
    label_matchers: Vec<LabelMatcher>,
    range: Option<Duration>,
) -> Vector {
    Vector {
        name: name.to_owned(),
        label_matchers,
        range,
        offset: None,
    }
}

pub fn vector(name: &str) -> Vector {
    Vector {
        name: name.to_owned(),
        ..Default::default()
    }
}
