#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AggregationModifierAction {
    Without,
    By,
}

impl Default for AggregationModifierAction {
    fn default() -> Self {
        Self::Without
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AggregationModifier {
    pub action: AggregationModifierAction,
    pub labels: Vec<String>,
}
