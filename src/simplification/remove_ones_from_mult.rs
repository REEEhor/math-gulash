use std::collections::VecDeque;

use crate::expression::Expr;

use super::{SimpResult, Simplification};

pub struct RemoveOnesFromMult;

impl Simplification for RemoveOnesFromMult {
    fn simplify(&self, expr: &Expr) -> SimpResult<Expr> {
        if let &Expr::Multiplication(original_exprs) = &expr {
            let new_exprs: VecDeque<Expr> = original_exprs
                .iter()
                .filter(|expr| expr.as_number() != Some(1))
                .cloned()
                .collect();
            let is_shorter = new_exprs.len() < original_exprs.len();
            return Ok(is_shorter.then(|| Expr::checked_mult(new_exprs)));
        }

        return Ok(None);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        expression::test_helpers::*,
        simplification::{remove_ones_from_mult::RemoveOnesFromMult, Simplification},
    };

    #[test]
    fn test_no_ones() {
        let expr = mult(&[var('a'), var('b'), num(6), num(-4)]);
        let actual = RemoveOnesFromMult.simplify(&expr);
        assert_eq!(actual, Ok(None));
    }

    #[test]
    fn test_some_ones() {
        let expr = mult(&[num(1), var('a'), var('b'), num(1), num(-4)]);
        let actual = RemoveOnesFromMult.simplify(&expr);
        //
        let expected = mult(&[var('a'), var('b'), num(-4)]);
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_only_ones() {
        let expr = mult(&[num(1), num(1), num(1)]);
        let actual = RemoveOnesFromMult.simplify(&expr);
        //
        let expected = num(1);
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_ones_with_other_expr() {
        let inner = div(vexp('x', -5), num(21));
        let expr = mult(&[num(1), num(1), inner.clone(), num(1)]);
        let actual = RemoveOnesFromMult.simplify(&expr);
        //
        let expected = inner;
        assert_eq!(actual, Ok(Some(expected)));
    }
}
