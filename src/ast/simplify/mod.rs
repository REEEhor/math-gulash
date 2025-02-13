pub mod canonical_term;
pub mod number_fraction;
pub mod simplify_mult_div;
pub mod simplify_exp;
pub mod evaluate;
pub mod simplify_commutative;
use super::super::Expr;

pub struct SimplifyResult {
    pub new_expr: Expr,
    pub got_simplified: bool,
}

impl SimplifyResult {
    fn new(expr: Expr, got_simplified: bool) -> Self {
        Self {
            new_expr: expr,
            got_simplified,
        }
    }
}

pub type SimplifyFunc = dyn Fn(Expr) -> SimplifyResult;

#[derive(Clone, Copy)]
struct Simplification<'a> {
    f: &'a SimplifyFunc,
    got_simplified: bool,
}

impl<'a> Simplification<'a> {
    fn new(f: &'a SimplifyFunc) -> Self {
        Self {
            f,
            got_simplified: false,
        }
    }
    fn simplify(&mut self, expr: Expr) -> Expr {
        let res = (self.f)(expr);
        self.got_simplified |= res.got_simplified;
        res.new_expr
    }
}

impl Expr {
    fn simplify_inner_exprs(self, f: &SimplifyFunc) -> SimplifyResult {
        let mut simp = Simplification::new(f);
        let new_expr = match self {
            Expr::Addition(exprs) => {
                Expr::Addition(exprs.into_iter().map(|e| simp.simplify(e)).collect())
            }
            Expr::Multiplication(exprs) => {
                Expr::Multiplication(exprs.into_iter().map(|e| simp.simplify(e)).collect())
            }
            Expr::Division { lhs, rhs } => Expr::Division {
                lhs: simp.simplify(Expr::clone(&lhs)).into(),
                rhs: simp.simplify(Expr::clone(&rhs)).into(),
            },
            Expr::Exp { base, exp } => Expr::Exp {
                base: simp.simplify(Expr::clone(&base)).into(),
                exp: simp.simplify(Expr::clone(&exp)).into(),
            },
            Expr::UnaryMinus(expr) => Expr::UnaryMinus(simp.simplify(Expr::clone(&expr)).into()),

            simple_expr => simple_expr,
        };

        SimplifyResult::new(new_expr, simp.got_simplified)
    }
}
