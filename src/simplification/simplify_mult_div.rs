use std::{collections::VecDeque, iter};

use crate::{
    expression::{
        error::{EvalError, EvalResult},
        number_fraction::NumberFraction,
        Expr,
    },
    simplification::{
        canonical_term::{self, CanonicalMultTerm},
        var_exp_map::VarExpMap,
    },
};

use super::{SimpResult, Simplification};

pub struct SimplifyMultDiv;

impl Simplification for SimplifyMultDiv {
    fn simplify(&self, expr: &Expr) -> SimpResult<Expr> {
        let mut term = CanonicalMultTerm::new();
        let mut non_simplified_top_exprs = Vec::new();
        let mut non_simplified_bottom_exprs = Vec::new();
        match expr {
            Expr::Multiplication(exprs) => {
                simplify_mult(
                    exprs.iter(),
                    &mut term,
                    1,
                    false,
                    &mut non_simplified_top_exprs,
                    &mut non_simplified_bottom_exprs,
                )?;
            }
            Expr::Division { lhs, rhs } => {
                simplify_mult(
                    iter::once(&**lhs),
                    &mut term,
                    1,
                    false,
                    &mut non_simplified_top_exprs,
                    &mut non_simplified_bottom_exprs,
                )?;
                simplify_mult(
                    iter::once(&**rhs),
                    &mut term,
                    -1,
                    true,
                    &mut non_simplified_bottom_exprs,
                    &mut non_simplified_top_exprs,
                )?;
            }
            _ => return Ok(None),
        }

        let (top_vars, bottom_vars) = term.vars.partition_by_exp_sign();
        let (mut top_vars, mut bottom_vars): (VecDeque<_>, VecDeque<_>) =
            (top_vars.into(), bottom_vars.into());
        //
        top_vars.append(&mut non_simplified_top_exprs.into());
        bottom_vars.append(&mut non_simplified_bottom_exprs.into());

        let result = if bottom_vars.is_empty() {
            // The result will be in the form:
            //   number-or-fraction  *  var² * var⁶ * var⁷ * var⁶ * ...
            let number_part_abs = (Expr::from_number_fraction(term.number_part.abs()));
            match number_part_abs {
                Expr::Number(1) => {}
                Expr::Number(0) => return Ok(Some(Expr::zero())),
                expression => top_vars.push_front(expression),
            }
            Expr::checked_mult(top_vars)
        } else {
            // The result will be in the form:
            //   (number * var² * var⁶) / (number * var⁷ * var⁶)
            let top_number = term.number_part.top;
            let bottom_number = term.number_part.bottom;
            match top_number {
                1 => {}
                0 => return Ok(Some(Expr::zero())),
                top_number => top_vars.push_front(Expr::Number(top_number)),
            }
            if bottom_number.get() != 1 {
                bottom_vars.push_front(Expr::Number(bottom_number.get()));
            }
            Expr::Division {
                lhs: Expr::checked_mult(top_vars).into(),
                rhs: Expr::checked_mult(bottom_vars).into(),
            }
        }
        .maybe_wrap_in_minus(term.number_part.is_negative);

        return Ok((result != *expr).then_some(result));
    }
}

fn simplify_mult<'a, ExprIter: Iterator<Item = &'a Expr>>(
    exprs: ExprIter,
    term: &mut CanonicalMultTerm,
    sign: i32,
    is_in_denominator: bool,
    remainin_top_exprs: &mut Vec<Expr>,
    remainin_bottom_exprs: &mut Vec<Expr>,
) -> EvalResult<()> {
    for expr in exprs {
        match expr {
            Expr::Addition(exprs) => remainin_top_exprs.push(expr.clone()),
            Expr::Multiplication(exprs) => simplify_mult(
                exprs.iter(),
                term,
                sign,
                is_in_denominator,
                remainin_top_exprs,
                remainin_bottom_exprs,
            )?,
            Expr::Division { lhs, rhs } => {
                simplify_mult(
                    iter::once(&**lhs),
                    term,
                    sign,
                    is_in_denominator,
                    remainin_top_exprs,
                    remainin_bottom_exprs,
                )?;
                simplify_mult(
                    iter::once(&**rhs),
                    term,
                    -sign,
                    true,
                    remainin_bottom_exprs,
                    remainin_top_exprs,
                )?;
            }
            Expr::Exp { base, exp } => match &**base {
                Expr::Variable { symbol } => term.vars[*symbol] += sign * *exp,
                _ => remainin_top_exprs.push(expr.clone()),
            },
            Expr::UnaryMinus(expr_inner) => {
                term.number_part = term.number_part.with_flipped_sign(); // Flip the sign
                simplify_mult(
                    iter::once(&**expr_inner),
                    term,
                    sign,
                    is_in_denominator,
                    remainin_top_exprs,
                    remainin_bottom_exprs,
                )?;
            }
            Expr::Variable { symbol } => term.vars[*symbol] += sign * 1,
            Expr::Number(integer) => {
                if *integer == 0 && is_in_denominator {
                    return Err(EvalError::DivisionByZero);
                }

                let int_as_fraction = NumberFraction::whole_number(*integer as i32);
                match sign {
                    1 => term.number_part *= int_as_fraction,
                    -1 => term.number_part = (term.number_part / int_as_fraction)?,
                    _ => {
                        unreachable!("The `sign` was expected to be `-1` or `1` but is `{integer}`")
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        expression::{self, error::EvalError, test_helpers::*, Expr},
        simplification::Simplification,
    };

    use super::SimplifyMultDiv;

    #[test]
    fn var_exp_combination_1() {
        let vars = [vexp('a', 3), vexp('a', 2)].into();
        let result = SimplifyMultDiv.simplify(&Expr::Multiplication(vars));
        assert_eq!(result, Ok(Some(vexp('a', 5))));
    }

    #[test]
    fn var_exp_combination_2() {
        let vars = [vexp('a', -3), vexp('a', 4)].into();
        let result = SimplifyMultDiv.simplify(&Expr::Multiplication(vars));
        assert_eq!(result, Ok(Some(var('a'))));
    }

    #[test]
    fn var_exp_combination_3() {
        let vars = [vexp('a', -3), vexp('b', 5), vexp('a', 4)].into();
        let result = SimplifyMultDiv.simplify(&Expr::Multiplication(vars));
        //
        let expected = mult(&[var('a'), vexp('b', 5)]);
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn var_exp_combination_4() {
        let vars = [vexp('a', -3), vexp('b', 5), vexp('a', 3), vexp('b', -5)].into();
        let result = SimplifyMultDiv.simplify(&Expr::Multiplication(vars));
        //
        let expected = Expr::one();
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn var_exp_combination_5() {
        let vars = [
            var('a'),
            vexp('b', 5),
            vexp('a', 3),
            vexp('b', -5),
            vexp('c', 10),
            var('d'),
            var('d'),
            var('d'),
            vexp('c', 3),
            var('c'),
            var('d'),
        ];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        let expected = mult(&[vexp('a', 4), vexp('c', 14), vexp('d', 4)]);
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_just_numbers_1() {
        let exprs = [num(30), num(20)];
        let result = SimplifyMultDiv.simplify(&mult(&exprs));
        //
        let expected = Expr::Number(600);
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_just_numbers_2() {
        let exprs = [num(-30), num(40)];
        let result = SimplifyMultDiv.simplify(&mult(&exprs));
        //
        let expected = Expr::UnaryMinus(Expr::Number(1200).into());
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_var_division_1() {
        let vars = [vexp('a', -3)];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        let expected = div(num(1), vexp('a', 3));
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_var_division_2() {
        let vars = [var('a'), var('b'), vexp('a', -3)];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        let expected = div(var('b'), vexp('a', 2));
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_div_by_zero_detection_1() {
        let vars = [var('a'), var('b'), vexp('a', -3), div(num(2), num(0))];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        assert_eq!(result, Err(EvalError::DivisionByZero));
    }

    #[test]
    fn test_div_by_zero_detection_2() {
        let expr = div(num(1), num(0));
        let result = SimplifyMultDiv.simplify(&expr);
        //
        assert_eq!(result, Err(EvalError::DivisionByZero));
    }

    #[test]
    fn test_div_by_zero_detection_3() {
        let vars = [div(num(1), div(num(1), num(0)))];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        assert_eq!(result, Err(EvalError::DivisionByZero));
    }

    #[test]
    fn test_number_and_vars_1() {
        let vars = [
            num(3),
            var('a'),
            var('b'),
            vexp('a', -3),
            vexp('a', 2),
            div(num(2), num(3)),
        ];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        let expected = mult(&[num(2), var('b')]);
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_number_and_vars_2() {
        let vars = [
            num(4),
            neg(vexp('d', 2)),
            var('a'),
            var('b'),
            vexp('d', -5),
            div(num(-2), num(-3)),
        ];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        let expected = neg(div(
            mult(&[num(8), var('a'), var('b')]),
            mult(&[num(3), vexp('d', 3)]),
        ));
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_number_and_vars_3() {
        let vars = [
            num(3),
            neg(neg(vexp('d', 2))),
            var('a'),
            num(2),
            div(num(1), vexp('a', -3)),
            var('b'),
            num(7),
            num(1),
            div(num(1), num(1)),
            num(1),
            div(num(1), num(1)),
            num(1),
            div(vexp('c', -6), num(6)),
        ];
        let result = SimplifyMultDiv.simplify(&mult(&vars));
        //
        let expected = div(
            mult(&[num(7), vexp('a', 4), var('b'), vexp('d', 2)]),
            vexp('c', 6),
        );
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_division_in_division() {
        let expr = div(num(1), div(num(1), div(num(2), vexp('x', 5))));
        let result = SimplifyMultDiv.simplify(&expr);
        //
        let expected = div(num(2), vexp('x', 5));
        assert_eq!(result, Ok(Some(expected)));
    }

    #[test]
    fn test_propagating_addition() {
        let expr = div(
            mult(&[var('a'), add(&[num(5), num(6)])]),
            mult(&[add(&[num(1), num(2), num(3)]), var('a')]),
        );
        let result = SimplifyMultDiv.simplify(&expr);
        //
        let expected = div(add(&[num(5), num(6)]), add(&[num(1), num(2), num(3)]));
        assert_eq!(result, Ok(Some(expected)));
    }
}
