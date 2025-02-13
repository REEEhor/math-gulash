use super::canonical_term::*;
use super::number_fraction::*;
use super::Expr;
use super::*;

impl Expr {
    pub fn simplify_mult_div(self: Expr) -> SimplifyResult {
        let mut top = CanonicalMultTerm::new();
        let mut bottom = CanonicalMultTerm::new();
        let success = match self {
            Expr::Multiplication(ref exprs) => exprs
                .iter()
                .all(|expr| canonize_mult_div(expr, &mut top, &mut bottom)),
            Expr::Division { ref lhs, ref rhs } => {
                let success1 = canonize_mult_div(&*lhs, &mut top, &mut bottom);
                let success2 = canonize_mult_div(&*rhs, &mut bottom, &mut top);
                success1 && success2
            }
            expr => return expr.simplify_inner_exprs(&Expr::simplify_mult_div),
        };

        if !success {
            return SimplifyResult::new(self, false);
        }

        let mut result_vars = VarExpMap::new();
        for symbol in 'a'..='z' {
            result_vars[symbol] = top.vars[symbol] - bottom.vars[symbol];
        }

        let (mut top_exprs, mut bottom_exprs) = result_vars.partition_by_exp_sign();

        let number_frac = NumberFraction::new_in_base_form(top.number, bottom.number)
            .unwrap_or_else(|| todo!("Division by zero is not handled yet :("));
        if number_frac.top == 0 {
            return SimplifyResult::new(Expr::zero(), true);
        }

        if number_frac.top != 1 {
            top_exprs.push(Expr::Number(number_frac.top));
        }
        if number_frac.bottom_u32() != 1 {
            bottom_exprs.push(Expr::Number(number_frac.bottom_u32()));
        }

        let result_expr = Expr::mult_div_from_exprs(top_exprs, bottom_exprs)
            .maybe_wrap_in_minus(number_frac.is_negative);

        return SimplifyResult::new(result_expr, true);
    }

    fn _simplify_exp_of_mult(self) -> Expr {
        todo!()
    }

    fn _simplify_number_times_addition(self) -> Expr {
        todo!()
    }
}

fn canonize_mult_div(
    expr: &Expr,
    top: &mut CanonicalMultTerm,
    bottom: &mut CanonicalMultTerm,
) -> bool {
    match expr {
        Expr::Addition(_) => false,
        Expr::Multiplication(exprs) => exprs.iter().all(|e| canonize_mult_div(e, top, bottom)),
        Expr::Division { lhs, rhs } => {
            let success1 = canonize_mult_div(&*lhs, top, bottom);
            let success2 = canonize_mult_div(&*rhs, bottom, top);
            success1 && success2
        }
        Expr::UnaryMinus(inner_expr) => {
            top.number *= -1;
            canonize_mult_div(inner_expr, top, bottom)
        }
        Expr::Exp { base, exp } => false,
        Expr::Variable { symbol } => {
            top.vars[*symbol] += 1;
            true
        }
        Expr::Number(number) => {
            top.number *= *number as i32;
            true
        }
    }
}
