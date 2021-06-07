#[derive(Debug, Clone)]
pub struct NumberLiteral {
    pub value: f64,
}

impl PartialEq for NumberLiteral {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() <= f64::EPSILON
    }
}

impl Eq for NumberLiteral {}

impl NumberLiteral {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[allow(dead_code)]
pub(crate) fn string_literal(value: &str) -> StringLiteral {
    StringLiteral {
        value: value.to_owned(),
    }
}

pub(crate) fn number_literal(value: f64) -> NumberLiteral {
    NumberLiteral { value }
}
