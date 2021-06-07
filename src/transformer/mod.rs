use crate::ast::{BinaryExpr, Expr, FunCall, NumberLiteral, StringLiteral, SubqueryExpr, Vector};

pub trait Transformer {
    type Err;

    fn transform_expr(&mut self, ast: &Expr) -> Result<Expr, Self::Err> {
        match ast {
            Expr::BinaryExpr(e) => self.transform_binary_expr(e),
            Expr::FunCallExpr(e) => self.transform_funcall_expr(e),
            Expr::VectorExpr(e) => self.transform_vector_expr(e),
            Expr::NumberLiteralExpr(e) => self.transform_number_literal(e),
            Expr::StringLiteralExpr(e) => self.transform_string_literal(e),
            Expr::SubQueryExpr(e) => self.transform_subquery_expr(e),
            Expr::NegationExpr(e) => self.transform_negation_expr(e),
        }
    }

    fn transform_binary_expr(&mut self, ast: &BinaryExpr) -> Result<Expr, Self::Err> {
        Ok(Expr::BinaryExpr(Box::new(ast.clone())))
    }

    fn transform_funcall_expr(&mut self, ast: &FunCall) -> Result<Expr, Self::Err> {
        Ok(Expr::FunCallExpr(Box::new(ast.clone())))
    }

    fn transform_vector_expr(&mut self, ast: &Vector) -> Result<Expr, Self::Err> {
        Ok(Expr::VectorExpr(Box::new(ast.clone())))
    }

    fn transform_number_literal(&mut self, ast: &NumberLiteral) -> Result<Expr, Self::Err> {
        Ok(Expr::NumberLiteralExpr(Box::new(ast.clone())))
    }

    fn transform_string_literal(&mut self, ast: &StringLiteral) -> Result<Expr, Self::Err> {
        Ok(Expr::StringLiteralExpr(Box::new(ast.clone())))
    }

    fn transform_subquery_expr(&mut self, ast: &SubqueryExpr) -> Result<Expr, Self::Err> {
        Ok(Expr::SubQueryExpr(Box::new(ast.clone())))
    }

    fn transform_negation_expr(&mut self, ast: &Expr) -> Result<Expr, Self::Err> {
        Ok(Expr::NegationExpr(Box::new(ast.clone())))
    }
}

pub fn transform<T: Transformer>(ast: &Expr, t: &mut T) -> Result<Expr, T::Err> {
    t.transform_expr(ast)
}
