use std::{collections::VecDeque, ops::Deref, rc::Rc};
pub mod canonical_term;
pub mod simplify_exp;
pub mod simplify_mult_div;
pub mod var_exp_map;

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

    fn simplify_once_recursive(&self, expr: &Expr) -> SimpResult<Expr>
    where
        Self: Sized,
    {
        let initial_simplification = self.simplify(expr)?;
        if let Some(simplified_expr) = initial_simplification {
            return Ok(Some(simplified_expr));
        }

        match expr {
            Expr::Addition(exprs) => {
                Ok(try_simplifying_one_of_exprs(exprs.iter(), self)?.map(Expr::checked_add))
            }

            Expr::Multiplication(exprs) => {
                Ok(try_simplifying_one_of_exprs(exprs.iter(), self)?.map(Expr::checked_mult))
            }

            Expr::Division { lhs, rhs } => {
                if let Some(new_lhs) = self.simplify_once_recursive(lhs)? {
                    return Ok(Some(Expr::Division {
                        lhs: new_lhs.into(),
                        rhs: rhs.clone(),
                    }));
                }
                if let Some(new_rhs) = self.simplify_once_recursive(rhs)? {
                    return Ok(Some(Expr::Division {
                        lhs: lhs.clone(),
                        rhs: new_rhs.into(),
                    }));
                }
                Ok(None)
            }

            Expr::UnaryMinus(expr) => Ok(self
                .simplify_once_recursive(expr)?
                .map(|expr| Expr::UnaryMinus(expr.into()))),

            Expr::Exp { base, exp } => {
                Ok(self.simplify_once_recursive(base)?.map(|expr| Expr::Exp {
                    base: expr.into(),
                    exp: *exp,
                }))
            }

            // Simple expressions without inner expressions
            Expr::Number(_) => Ok(None),
            Expr::Variable { .. } => Ok(None),
        }
    }
}

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
        let new_expr = if simplification_happened {
            expr.clone()
        } else {
            simp.simplify_once_recursive(expr)?
                .unwrap_or_else(|| expr.clone())
        };
        result_exprs.push_back(new_expr);
    }

    Ok(simplification_happened.then_some(result_exprs))
}
