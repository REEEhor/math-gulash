use super::Expr;

pub fn vexp(base: char, exp: i32) -> Expr {
    Expr::Exp {
        base: Expr::Variable { symbol: base }.into(),
        exp,
    }
}

impl Expr {
    pub fn pow(self, exp: i32) -> Expr {
        Expr::Exp {
            base: self.into(),
            exp,
        }
    }
}

pub fn var(symbol: char) -> Expr {
    Expr::Variable { symbol }
}

pub fn mult(exprs: &[Expr]) -> Expr {
    Expr::Multiplication(exprs.to_vec().into())
}

pub fn add(exprs: &[Expr]) -> Expr {
    Expr::Addition(exprs.to_vec().into())
}

pub fn num(number: i32) -> Expr {
    Expr::signed_number(number)
}

pub fn div(top: Expr, bottom: Expr) -> Expr {
    Expr::Division {
        lhs: top.into(),
        rhs: bottom.into(),
    }
}

pub fn neg(inner: Expr) -> Expr {
    Expr::UnaryMinus(inner.into())
}
