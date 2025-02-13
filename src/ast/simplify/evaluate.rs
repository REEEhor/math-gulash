use super::{number_fraction::NumberFraction, Expr, SimplifyResult};

impl Expr {
    pub fn evaluate(&self) -> Option<NumberFraction> {
        match self {
            Expr::Number(number) => Some(NumberFraction::whole_number(*number as i32)),

            Expr::Variable { symbol: _ } => None,

            Expr::UnaryMinus(expr) => expr.evaluate().map(|fraction| fraction.with_flipped_sign()),

            Expr::Addition(exprs) => {
                exprs
                    .iter()
                    .try_fold(NumberFraction::whole_number(0), |acc, expr| {
                        let current = expr.evaluate()?;
                        Some(acc + current)
                    })
            }

            Expr::Multiplication(exprs) => {
                exprs
                    .iter()
                    .try_fold(NumberFraction::whole_number(1), |acc, expr| {
                        let current = expr.evaluate()?;
                        Some(acc * current)
                    })
            }

            Expr::Division { lhs, rhs } => {
                let lhs = lhs.evaluate()?;
                let rhs = rhs.evaluate()?;
                lhs / rhs
            }

            Expr::Exp { base, exp } => {
                let base = base.evaluate()?;
                let exp = exp.evaluate()?;
                base.pow(exp)
            }
        }
    }

    pub fn simplify_by_evaluation(self) -> SimplifyResult {
        match self.evaluate() {
            Some(fraction) => SimplifyResult::new(Expr::from_number_fraction(fraction), true),
            None => self.simplify_inner_exprs(&Expr::simplify_by_evaluation),
        }
    }
}
