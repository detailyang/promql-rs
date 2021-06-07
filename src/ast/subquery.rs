use crate::ast::Expr;
use std::time::Duration;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct SubqueryExpr {
    pub expr: Expr,
    pub range: Option<Duration>,
    pub resolution: Option<Duration>,
}
