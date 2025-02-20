use std::{collections::VecDeque, ops::Deref, rc::Rc};
pub mod canonical_term;
pub mod remove_ones_from_mult;
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

    fn simplify_recursive(&self, expr: &Expr) -> SimpResult<Expr>
    where
        Self: Sized,
    {
        let mut simplified_expr = match self.simplify_once_recursive(expr)? {
            Some(simplified_expr) => simplified_expr,
            None => return Ok(None),
        };

        while let Some(new_simplified_expr) = self.simplify_once_recursive(&simplified_expr)? {
            simplified_expr = new_simplified_expr;
        }

        Ok(Some(simplified_expr))
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
                .inspect(|_simp_expr| simplification_happened = true)
                .unwrap_or_else(|| expr.clone())
        };
        result_exprs.push_back(new_expr);
    }

    Ok(simplification_happened.then_some(result_exprs))
}

#[cfg(test)]
mod test {
    use std::{
        collections::{HashMap, HashSet},
        convert::Infallible,
    };

    use crate::expression::{error::EvalError, test_helpers::*, Expr};

    use super::{SimpResult, Simplification};

    struct SimpMock<F: Fn(&Expr) -> SimpResult<Expr>> {
        simplify_func: F,
    }

    fn simpl<F: Fn(&Expr) -> SimpResult<Expr>>(f: F) -> SimpMock<F> {
        SimpMock { simplify_func: f }
    }

    impl<F: Fn(&Expr) -> SimpResult<Expr>> Simplification for SimpMock<F> {
        fn simplify(&self, expr: &Expr) -> SimpResult<Expr> {
            (self.simplify_func)(expr)
        }
    }

    fn rename_simp(
        rename: &[(char, char)],
        fail: &[char],
    ) -> SimpMock<impl Fn(&Expr) -> SimpResult<Expr>> {
        let rename_map = HashMap::<char, char>::from_iter(rename.iter().copied());
        let fail_set = HashSet::<char>::from_iter(fail.iter().copied());
        simpl(move |expr| match expr {
            Expr::Variable { symbol } if fail_set.contains(symbol) => {
                Err(EvalError::TestError(symbol.to_string()))
            }
            Expr::Variable { symbol } if rename_map.contains_key(symbol) => {
                Ok(Some(Expr::Variable {
                    symbol: *rename_map.get(symbol).unwrap(),
                }))
            }
            _ => Ok(None),
        })
    }

    fn err(symbol: char) -> SimpResult<Expr> {
        Err(EvalError::TestError(symbol.to_string()))
    }

    #[test]
    fn test_simplify_once_recursive_base_case() {
        let s = rename_simp(&[('x', 'a')], &[]);
        let expr = Expr::Variable { symbol: 'x' };
        let actual = s.simplify_once_recursive(&expr);
        //
        let expected = Expr::Variable { symbol: 'a' };
        assert_eq!(actual, Ok(Some(expected)))
    }

    #[test]
    fn test_simplify_once_recursive_propagating_erorrs() {
        let s = rename_simp(&[], &['x']);
        let expr = var('x');
        let actual = s.simplify_once_recursive(&expr);
        //
        assert_eq!(actual, err('x'));
    }

    fn rename_simp_1() -> SimpMock<impl Fn(&Expr) -> SimpResult<Expr>> {
        rename_simp(&[('a', 'A'), ('b', 'B'), ('c', 'C')], &['x'])
    }

    #[test]
    fn test_simplify_once_recursive_one_deep_recursion() {
        let s = rename_simp_1();
        //
        let expr = add(&[var('b'), var('a'), var('c')]);
        let actual = s.simplify_once_recursive(&expr);
        let expected = add(&[var('B'), var('a'), var('c')]);
        assert_eq!(actual, Ok(Some(expected)));
        //
        let expr = actual.unwrap().unwrap();
        let actual = s.simplify_once_recursive(&expr);
        let expected = add(&[var('B'), var('A'), var('c')]);
        assert_eq!(actual, Ok(Some(expected)));
        //
        let expr = actual.unwrap().unwrap();
        let actual = s.simplify_once_recursive(&expr);
        let expected = add(&[var('B'), var('A'), var('C')]);
        assert_eq!(actual, Ok(Some(expected)));
        //
        let expr = actual.unwrap().unwrap();
        let actual = s.simplify_once_recursive(&expr);
        assert_eq!(actual, Ok(None));
    }
}
