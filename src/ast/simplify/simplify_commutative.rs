use crate::ast::{simplify::number_fraction::NumberFraction, Expr};

use super::SimplifyResult;

impl Expr {
    pub fn simplify_commutative(self) -> SimplifyResult {
        match self {
            Expr::Addition(exprs) => {
                let mut summed_parts = NumberFraction::whole_number(0);
                let mut new_operands = vec![];
                for expr in exprs {
                    match expr.evaluate() {
                        Some(evaluation) => summed_parts += evaluation,
                        None => new_operands.push(expr),
                    }
                }
                if !summed_parts.is_zero() {
                    new_operands.insert(0, Expr::from_number_fraction(summed_parts));
                }
                let new_expr = Expr::checked_add(new_operands);
                SimplifyResult::new(new_expr, true)
            }
            Expr::Multiplication(exprs) => {
                let mut multiplied_parts = NumberFraction::whole_number(1);
                let mut new_operands = vec![];
                for expr in exprs {
                    match expr.evaluate() {
                        Some(evaluation) => multiplied_parts *= evaluation,
                        None => new_operands.push(expr),
                    }
                }
                match multiplied_parts.as_whole_number() {
                    // The multiplied parts multiplied into 0
                    // => The whole expression is zero
                    Some(0) => return SimplifyResult::new(Expr::zero(), true),

                    // The multiplied parts multiplied into 1
                    // => We can just use the `new_operands` as the result
                    // => Don't do anything :)
                    Some(1) => {}

                    Some(_) | None => {
                        new_operands.insert(0, Expr::from_number_fraction(multiplied_parts));
                    }
                }
                let new_expr = Expr::checked_add(new_operands);
                SimplifyResult::new(new_expr, true)
            }
            expr => expr.simplify_inner_exprs(&Expr::simplify_commutative),
        }
    }
}
