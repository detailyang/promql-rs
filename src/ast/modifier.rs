/// Vector matching operator modifier (`on (…)`/`ignoring (…)`).
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BinaryModifier {
    /// Action applied to a list of vectors; whether `on (…)` or `ignored(…)` is used after the operator.
    pub action: BinaryModifierAction,
    /// Set of labels to apply `action` to.
    pub labels: Vec<String>,
    /// Additional grouping modifier, if any.
    pub group: Option<BinaryModifierGroup>,
}

pub fn binary_modifier(action: BinaryModifierAction, labels: Vec<String>) -> BinaryModifier {
    BinaryModifier {
        action,
        labels,
        ..Default::default()
    }
}

pub fn binary_group_modifier(
    action: BinaryModifierAction,
    labels: Vec<String>,
    group: BinaryModifierGroup,
) -> BinaryModifier {
    BinaryModifier {
        action,
        labels,
        group: Some(group),
    }
}

pub fn binary_modifier_group_right(labels: Vec<String>) -> BinaryModifierGroup {
    BinaryModifierGroup {
        side: BinaryModifierGroupSide::Right,
        labels,
    }
}

pub fn binary_modifier_group_left(labels: Vec<String>) -> BinaryModifierGroup {
    BinaryModifierGroup {
        side: BinaryModifierGroupSide::Left,
        labels,
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinaryModifierAction {
    On,
    Ignore,
}

impl Default for BinaryModifierAction {
    fn default() -> Self {
        Self::On
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinaryModifierGroupSide {
    Left,
    Right,
}

impl Default for BinaryModifierGroupSide {
    fn default() -> Self {
        Self::Left
    }
}

/// Vector grouping operator modifier (`group_left(…)`/`group_right(…)`).
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BinaryModifierGroup {
    pub side: BinaryModifierGroupSide,
    pub labels: Vec<String>,
}
