#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueType {
    None,
    Vector,
    Scalar,
    Matrix,
    String,
}

impl Default for ValueType {
    fn default() -> Self {
        Self::None
    }
}
