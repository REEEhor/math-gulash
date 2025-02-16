use std::{collections::VecDeque, ops::Deref, rc::Rc};
pub mod canonical_term;
pub mod var_exp_map;
pub mod simplify_mult_div;

use crate::{
    expression::{
        error::{EvalError, EvalResult},
        Expr,
    },
    simplification,
};

pub type SimpResult<T> = EvalResult<Option<T>>;

pub trait Simplification {
    fn simplify(&self, expr: &Expr) -> SimpResult<Expr>;

    fn try_simplifying_one_inner_expr(&self, expr: &Expr) -> SimpResult<Expr>
    where
        Self: Sized,
    {
        match expr {
            Expr::Addition(exprs) => {
                Ok(try_simplifying_one_of_exprs(exprs.iter(), self)?.map(Expr::checked_add))
            }

            Expr::Multiplication(exprs) => {
                Ok(try_simplifying_one_of_exprs(exprs.iter(), self)?.map(Expr::checked_mult))
            }

            Expr::Division { lhs, rhs } => {
                if let Some(new_lhs) = self.simplify(lhs)? {
                    return Ok(Some(Expr::Division {
                        lhs: new_lhs.into(),
                        rhs: rhs.clone(),
                    }));
                }
                if let Some(new_rhs) = self.simplify(rhs)? {
                    return Ok(Some(Expr::Division {
                        lhs: lhs.clone(),
                        rhs: new_rhs.into(),
                    }));
                }
                Ok(None)
            }

            Expr::UnaryMinus(expr) => Ok(self
                .simplify(expr)?
                .map(|expr| Expr::UnaryMinus(expr.into()))),

            Expr::Exp { base, exp } => Ok(self.simplify(base)?.map(|expr| Expr::Exp {
                base: expr.into(),
                exp: *exp,
            })),

            // Simple expressions without inner expressions
            Expr::Number(_) => Ok(None),
            Expr::Variable { .. } => Ok(None),
        }
    }

    fn try_simplifying_one_recursive(&self, expr: &Expr) -> SimpResult<Expr>
    where
        Self: Sized,
    {
        if let Some(new_expr) = self.simplify(expr)? {
            return Ok(Some(new_expr));
        }

        self.try_simplifying_one_inner_expr(expr)
    }
}

struct RecursiveSimplification<Simp: Simplification>(Simp);

pub fn try_simplifying_one_of_exprs<
    'a,
    Simp: Simplification,
    ExprIter: Iterator<Item = &'a Expr>,
>(
    exprs: ExprIter,
    simp: &Simp,
) -> SimpResult<VecDeque<Expr>> {
    let mut result_exprs = VecDeque::new();
    let mut simplification_happened = false;

    for expr in exprs {
        let new_expr = match (simplification_happened, simp.simplify(expr)?) {
            (false, Some(simplified_expr)) => {
                simplification_happened = true;
                simplified_expr
            }
            _ => expr.clone(),
        };
        result_exprs.push_back(new_expr);
    }

    Ok(simplification_happened.then_some(result_exprs))
}
