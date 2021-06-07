use crate::ast::{BinaryExpr, Expr, FunCall, NumberLiteral, StringLiteral, SubqueryExpr, Vector};

pub trait Visitor {
    type Err;

    fn visit_expr(&mut self, ast: &Expr) -> Result<(), Self::Err> {
        match ast {
            Expr::BinaryExpr(e) => self.visit_binary_expr(e),
            Expr::FunCallExpr(e) => self.visit_funcall_expr(e),
            Expr::VectorExpr(e) => self.visit_vector_expr(e),
            Expr::NumberLiteralExpr(e) => self.visit_number_literal(e),
            Expr::StringLiteralExpr(e) => self.visit_string_literal(e),
            Expr::SubQueryExpr(e) => self.visit_subquery_expr(e),
            Expr::NegationExpr(e) => self.visit_negation_expr(e),
        }
    }

    fn visit_binary_expr(&mut self, ast: &BinaryExpr) -> Result<(), Self::Err> {
        let _ = ast;
        Ok(())
    }

    fn visit_funcall_expr(&mut self, ast: &FunCall) -> Result<(), Self::Err> {
        ast.args.iter().try_for_each(|arg| self.visit_expr(arg))?;
        Ok(())
    }

    fn visit_vector_expr(&mut self, ast: &Vector) -> Result<(), Self::Err> {
        let _ = ast;
        Ok(())
    }

    fn visit_number_literal(&mut self, ast: &NumberLiteral) -> Result<(), Self::Err> {
        let _ = ast;
        Ok(())
    }

    fn visit_string_literal(&mut self, ast: &StringLiteral) -> Result<(), Self::Err> {
        let _ = ast;
        Ok(())
    }

    fn visit_subquery_expr(&mut self, ast: &SubqueryExpr) -> Result<(), Self::Err> {
        let _ = ast;
        Ok(())
    }

    fn visit_negation_expr(&mut self, ast: &Expr) -> Result<(), Self::Err> {
        let _ = ast;
        Ok(())
    }
}

pub fn visit<V: Visitor>(ast: &Expr, v: &mut V) -> Result<(), V::Err> {
    v.visit_expr(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_expr;

    #[test]
    fn test_vector() {
        struct MyVisitor {
            visit_vector: u32,
        }

        impl Visitor for MyVisitor {
            type Err = ();

            fn visit_vector_expr(&mut self, ast: &Vector) -> Result<(), Self::Err> {
                println!("ast: {:?}", ast);
                self.visit_vector += 1;
                Ok(())
            }
        }

        let (_, expr) = parse_expr("alias(sum(a) by (b), \"c\")").unwrap();
        let mut v = MyVisitor { visit_vector: 0 };
        visit(&expr, &mut v).unwrap();
        assert_eq!(v.visit_vector, 1);
    }
}
