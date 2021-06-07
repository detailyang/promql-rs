use std::time::Duration;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LabelMatcherOp {
    None,
    Equal,
    NotEqual,
    Regexp,
    NotRegexp,
}

impl Default for LabelMatcherOp {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct LabelMatcher {
    pub op: LabelMatcherOp,
    pub name: String,
    pub value: String,
    //
    // fast regex matcher
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Vector {
    pub name: String,
    // pub original_offset: Duration,
    // pub timestamp: u64,
    // pub start_or_end:    // Set when @ is used with start() or end()
    pub label_matchers: Vec<LabelMatcher>,
    pub offset: Option<Duration>,
    pub range: Option<Duration>,
}

pub fn v(name: &str) -> Vector {
    Vector {
        name: name.to_owned(),
        ..Default::default()
    }
}

pub fn v_r(name: &str, range: Duration) -> Vector {
    Vector {
        name: name.to_owned(),
        range: Some(range),
        ..Default::default()
    }
}
