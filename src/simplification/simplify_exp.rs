use std::{mem::swap, num::NonZero, rc::Rc};

use crate::expression::{
    error::EvalError,
    ops::{exp, one_over},
    Expr,
};

use super::{SimpResult, Simplification};

pub struct SimplifyExponentiation;

impl Simplification for SimplifyExponentiation {
    fn simplify(&self, expr: &Expr) -> SimpResult<Expr> {
        match expr {
            Expr::Exp { base, exp } => simplify_inner(base.clone(), *exp),
            _ => Ok(None),
        }
    }
}

fn simplify_inner(base: Rc<Expr>, exponent: i32) -> SimpResult<Expr> {
    if exponent == 0 {
        return match base.as_number() {
            Some(0) => Err(EvalError::ZeroToZero),
            _ => Ok(Some(Expr::one())),
        };
    }
    if exponent == 1 {
        return Ok(Some(Expr::clone(&*base)));
    }

    match &*base {
        Expr::Addition(exprs) => {
            // TODO: check for negative exponent
            todo!("Implement the formula (a +- b)^2")
        }
        Expr::Multiplication(exprs) => {
            let new_exprs = exprs
                .iter()
                .map(|expr| exp(expr.clone().into(), exponent))
                .collect();
            Ok(Some(Expr::Multiplication(new_exprs)))
        }
        Expr::Division { lhs, rhs } => {
            let abs_exponent = exponent.abs();
            let mut new_lhs = exp(lhs.clone(), abs_exponent).into();
            let mut new_rhs = exp(rhs.clone(), abs_exponent).into();
            if exponent.is_negative() {
                swap(&mut new_lhs, &mut new_rhs);
            }
            Ok(Some(Expr::Division {
                lhs: new_lhs,
                rhs: new_rhs,
            }))
        }
        Expr::UnaryMinus(expr) => {
            // Check if the exponent is even => we can omit the negative sign
            let remove_minus = exponent.rem_euclid(2) == 0;
            Ok(Some(
                exp(expr.clone(), exponent).maybe_wrap_in_minus(!remove_minus),
            ))
        }
        Expr::Exp { base, exp } => Ok(Some(Expr::Exp {
            base: base.clone(),
            exp: *exp * exponent,
        })),
        Expr::Variable { symbol } => {
            let exponent_abs = exponent.abs();
            Ok(Some(if exponent.is_negative() {
                one_over(exp(base.clone(), exponent_abs).into())
            } else {
                exp(base.clone(), exponent_abs).into()
            }))
        }
        Expr::Number(num) => {
            let exponent_abs = exponent.abs();
            let base = num.pow(exponent.unsigned_abs());
            let base = Expr::Number(base);
            Ok(Some(if exponent.is_negative() {
                one_over(base.into()).into()
            } else {
                base
            }))
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        expression::{self, error::EvalError, ops::exp, test_helpers::*, Expr},
        simplification::Simplification,
    };

    use super::SimplifyExponentiation;

    #[test]
    fn test_number() {
        let expr = num(4).pow(3);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = num(64);
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_negative_number_in_base() {
        let expr = num(-2).pow(3);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = neg(num(2).pow(3));
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_number_in_base_with_negative_exponent() {
        let expr = num(4).pow(-8);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = div(num(1), num(4_i32.pow(8)));
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_flipping_of_fractions() {
        let expr = div(num(2), add(&[var('a'), num(9)])).pow(-99);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = div(add(&[var('a'), num(9)]).pow(99), num(2).pow(99));
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_exp_of_multiplication() {
        let expr = mult(&[num(10), num(20), var('x'), num(-30)]).pow(101);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = mult(&[
            num(10).pow(101),
            num(20).pow(101),
            var('x').pow(101),
            num(-30).pow(101),
        ]);
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_exp_of_fraction() {
        let expr = div(num(4), num(5)).pow(97);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = div(num(4).pow(97), num(5).pow(97));
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_exp_of_unary_minus_1() {
        let inner = add(&[var('c'), vexp('x', 101)]);
        //
        let expr = neg(inner.clone()).pow(5);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = neg(inner.clone().pow(5));
        assert_eq!(actual, Ok(Some(expected)));
    }

    #[test]
    fn test_exp_of_unary_minus_2() {
        let inner = add(&[var('c'), vexp('x', 101)]);
        //
        let expr = neg(inner.clone()).pow(4);
        let actual = SimplifyExponentiation.simplify(&expr);
        //
        let expected = inner.pow(4);
        assert_eq!(actual, Ok(Some(expected)));
    }
}
