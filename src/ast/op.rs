use crate::ast::modifier::BinaryModifier;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    Add(Option<BinaryModifier>),
    Sub(Option<BinaryModifier>),
    Mul(Option<BinaryModifier>),
    Div(Option<BinaryModifier>),
    Mod(Option<BinaryModifier>),
    Power(Option<BinaryModifier>),

    Equal(bool, Option<BinaryModifier>),
    NotEqual(bool, Option<BinaryModifier>),
    GreaterThan(bool, Option<BinaryModifier>),
    LessThan(bool, Option<BinaryModifier>),
    GreaterEqual(bool, Option<BinaryModifier>),
    LessEqual(bool, Option<BinaryModifier>),

    And(Option<BinaryModifier>),
    Or(Option<BinaryModifier>),
    Unless(Option<BinaryModifier>),
}

impl Default for BinaryOp {
    fn default() -> Self {
        Self::Add(None)
    }
}
