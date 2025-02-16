use std::{collections::VecDeque, ops::Mul, rc::Rc};

use super::{Expr, ExprRef};

pub fn exp(base: Rc<Expr>, exponent: i32) -> Expr {
    Expr::Exp {
        base,
        exp: exponent,
    }
}

pub fn one_over(expr: Rc<Expr>) -> Expr {
    Expr::Division {
        lhs: Expr::one().into(),
        rhs: expr.clone(),
    }
}

fn mul(lhs: Expr, rhs: Expr) -> Expr {
    match (lhs, rhs) {
        (Expr::Multiplication(mut lhs_exprs), Expr::Multiplication(mut rhs_exprs)) => {
            lhs_exprs.append(&mut rhs_exprs);
            Expr::checked_mult(lhs_exprs)
        }
        (Expr::Multiplication(mut lhs_exprs), rhs) => {
            lhs_exprs.push_back(rhs);
            Expr::checked_mult(lhs_exprs)
        }
        (lhs, Expr::Multiplication(mut rhs_exprs)) => {
            rhs_exprs.push_front(lhs);
            Expr::checked_mult(rhs_exprs)
        }
        (lhs, rhs) => Expr::Multiplication(VecDeque::from([lhs, rhs])),
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn test_mul_two_numbers() {
        let lhs = Expr::Number(2);
        let rhs = Expr::Number(3);
        let result = mul(lhs, rhs);
        assert_eq!(
            result,
            Expr::Multiplication(VecDeque::from([Expr::Number(2), Expr::Number(3)]))
        );
    }

    #[test]
    fn test_with_empty_multiplication() {
        let lhs = Expr::Number(2);
        let rhs = Expr::Multiplication(VecDeque::new());
        let result = mul(lhs, rhs);
        assert_eq!(result, Expr::Number(2));
    }

    #[test]
    fn test_with_empty_rhs_multiplication() {
        let lhs = Expr::Multiplication(VecDeque::new());
        let rhs = Expr::Number(2);
        let result = mul(lhs, rhs);
        assert_eq!(result, Expr::Number(2));
    }

    #[test]
    fn test_with_two_empty_multiplications() {
        let lhs = Expr::Multiplication(VecDeque::new());
        let rhs = Expr::Multiplication(VecDeque::new());
        let result = mul(lhs, rhs);
        assert_eq!(result, Expr::Number(1));
    }

    #[test]
    fn test_mul_with_multiplication_expr() {
        let lhs = Expr::Multiplication(VecDeque::from([Expr::Number(2), Expr::Number(3)]));
        let rhs = Expr::Number(4);
        let result = mul(lhs, rhs);
        assert_eq!(
            result,
            Expr::Multiplication(VecDeque::from([
                Expr::Number(2),
                Expr::Number(3),
                Expr::Number(4)
            ]))
        );
    }

    #[test]
    fn test_mul_with_rhs_multiplication_expr() {
        let lhs = Expr::Number(4);
        let rhs = Expr::Multiplication(VecDeque::from([Expr::Number(2), Expr::Number(3)]));
        let result = mul(lhs, rhs);
        assert_eq!(
            result,
            Expr::Multiplication(VecDeque::from([
                Expr::Number(4),
                Expr::Number(2),
                Expr::Number(3)
            ]))
        );
    }

    #[test]
    fn test_mul_two_multiplication_exprs() {
        let lhs = Expr::Multiplication(VecDeque::from([Expr::Number(2), Expr::Number(3)]));
        let rhs = Expr::Multiplication(VecDeque::from([Expr::Number(4), Expr::Number(5)]));
        let result = mul(lhs, rhs);
        assert_eq!(
            result,
            Expr::Multiplication(VecDeque::from([
                Expr::Number(2),
                Expr::Number(3),
                Expr::Number(4),
                Expr::Number(5)
            ]))
        )
    }
}
