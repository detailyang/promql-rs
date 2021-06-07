use super::Expr;
use crate::ast::op::BinaryOp;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}
