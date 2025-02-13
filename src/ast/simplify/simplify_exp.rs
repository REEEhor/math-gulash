use super::{Expr, SimplifyResult};

impl Expr {
    pub fn simplify_exp(self) -> SimplifyResult {
        match self {
            Expr::Exp { base, exp } => simplify_exp_inner(&*base, &*exp),
            expr => expr.simplify_inner_exprs(&Expr::simplify_exp),
        }
    }
}

fn simplify_exp_inner(base: &Expr, exponent: &Expr) -> SimplifyResult {
    if let &Expr::Number(0) = exponent {
        if let Expr::Number(0) = base {
            todo!("Handling 0^0 not supported yet :)")
        }
        return SimplifyResult {
            new_expr: Expr::one(),
            got_simplified: true,
        };
    }

    if let &Expr::Number(1) = exponent {
        return SimplifyResult {
            new_expr: base.clone().into(),
            got_simplified: true,
        };
    }

    match base {
        Expr::Multiplication(exprs) => {
            let new_exprs = exprs
                .iter()
                .cloned()
                .map(|expr| expr.pow(exponent.clone()))
                .collect();
            SimplifyResult::new(Expr::Multiplication(new_exprs), true)
        }
        Expr::Division { lhs, rhs } => {
            let new_lhs = Expr::clone(lhs).pow(exponent.clone()).into();
            let new_rhs = Expr::clone(rhs).pow(exponent.clone()).into();
            SimplifyResult::new(
                Expr::Division {
                    lhs: new_lhs,
                    rhs: new_rhs,
                },
                true,
            )
        }
        Expr::Exp { base, exp } => {
            let new_exp = Expr::clone(exp) * exponent.clone();
            SimplifyResult::new(
                Expr::Exp {
                    base: base.clone(),
                    exp: new_exp.into(),
                },
                true,
            )
        }

        base => {
            let SimplifyResult {
                new_expr: new_base,
                got_simplified,
            } = base.clone().simplify_inner_exprs(&Expr::simplify_exp);

            SimplifyResult::new(
                Expr::Exp {
                    base: new_base.into(),
                    exp: exponent.clone().into(),
                },
                got_simplified,
            )
        }
    }
}
