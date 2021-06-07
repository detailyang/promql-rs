use std::time::Duration;

pub use aggregator::*;
pub use binary::*;
pub use funcall::*;
pub use literal::*;
pub use modifier::*;
pub use op::*;
pub use selector::*;
pub use subquery::*;
pub use value::*;
pub use vector::*;

pub mod aggregator;
pub mod binary;
pub mod funcall;
pub mod literal;
pub mod modifier;
pub mod op;
pub mod selector;
pub mod subquery;
pub mod value;
pub mod vector;

#[derive(Debug, Clone)]
pub enum Node {
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    BinaryExpr(Box<BinaryExpr>),
    FunCallExpr(Box<FunCall>),
    VectorExpr(Box<Vector>),
    NumberLiteralExpr(Box<NumberLiteral>),
    StringLiteralExpr(Box<StringLiteral>),
    SubQueryExpr(Box<SubqueryExpr>),
    NegationExpr(Box<Expr>),
}

impl Default for Expr {
    fn default() -> Self {
        Self::NumberLiteralExpr(Box::new(NumberLiteral::new(0.0)))
    }
}

pub fn subquery_expr(expr: Expr, range: Option<Duration>, resolution: Option<Duration>) -> Expr {
    Expr::SubQueryExpr(Box::new(SubqueryExpr {
        expr,
        range,
        resolution,
    }))
}

pub fn fun_call_expr(func: FunCall) -> Expr {
    Expr::FunCallExpr(Box::new(func))
}

pub fn binary_expr(op: BinaryOp, lhs: Expr, rhs: Expr) -> Expr {
    Expr::BinaryExpr(Box::new(BinaryExpr { op, lhs, rhs }))
}

pub fn number_literal_expr(val: f64) -> Expr {
    Expr::NumberLiteralExpr(Box::new(number_literal(val)))
}

pub fn vector_expr(v: Vector) -> Expr {
    Expr::VectorExpr(Box::new(v))
}
