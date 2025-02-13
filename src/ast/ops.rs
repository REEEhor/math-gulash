use std::{ops::Mul, rc::Rc};

use super::Expr;

fn a() {
    let e1 = Expr::Number(1);
    let e2 = Expr::Number(2);
    let res = e1.pow(e2);
}

impl Expr {
    pub fn pow(self, exp: Self) -> Self {
        Expr::Exp {
            base: self.into(),
            exp: exp.into(),
        }
    }
}

impl Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Self::Output {
        use Expr::Multiplication as Mult;
        match (self, rhs) {
            (Mult(mut lhs_exprs), Mult(rhs_exprs)) => {
                lhs_exprs.extend(rhs_exprs);
                Mult(lhs_exprs)
            }
            (Mult(mut lhs_exprs), rhs) => {
                lhs_exprs.push(rhs);
                Mult(lhs_exprs)
            }
            (lhs, Mult(mut rhs_exprs)) => {
                rhs_exprs.push(lhs);
                let last_idx = rhs_exprs.len() - 1;
                rhs_exprs.swap(0, last_idx);
                Mult(rhs_exprs)
            }
            (lhs, rhs) => Expr::mult(lhs, rhs),
        }
    }
}
