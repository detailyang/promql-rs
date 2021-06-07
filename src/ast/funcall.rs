use crate::ast::aggregator::AggregationModifier;
use crate::ast::Expr;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct FunCall {
    pub name: String,
    pub args: Vec<Expr>,
    pub aggregation: Option<AggregationModifier>,
}

#[allow(dead_code)]
pub(crate) fn fun_call(name: &str, args: Vec<Expr>) -> FunCall {
    FunCall {
        name: name.to_owned(),
        args,
        ..Default::default()
    }
}

#[allow(dead_code)]
pub(crate) fn fun_call_agg(
    name: &str,
    args: Vec<Expr>,
    aggregation: Option<AggregationModifier>,
) -> FunCall {
    FunCall {
        name: name.to_owned(),
        args,
        aggregation,
    }
}
