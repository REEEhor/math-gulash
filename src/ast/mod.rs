use std::fmt::{self, write};
use std::rc::Rc;
pub mod canonical;
pub mod display;
pub mod precedence;

use precedence::*;

pub type Digit = u32;

#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    Addition(Vec<Expr>),
    Multiplication(Vec<Expr>),
    Division { lhs: Rc<Expr>, rhs: Rc<Expr> },
    UnaryMinus(Rc<Expr>),
    Number(u32),
    Variable { symbol: char },
    Exp { base: Rc<Expr>, exp: Rc<Expr> },
}

impl Expr {
    pub fn zero() -> Expr {
        Expr::Number(0)
    }

    pub fn maybe_wrap_in_minus(self, should_be_minus: bool) -> Expr {
        if should_be_minus {
            Expr::UnaryMinus(self.into())
        } else {
            self
        }
    }

    pub fn signed_number(number: i32) -> Expr {
        if number.is_negative() {
            Expr::UnaryMinus(Expr::Number(number.unsigned_abs()).into())
        } else {
            Expr::Number(number.unsigned_abs())
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            Expr::Addition(_) => ADDITION,
            Expr::Multiplication(_) => MULTIPLICATION,
            Expr::Division { lhs: _, rhs: _ } => DIVISION,
            Expr::UnaryMinus(_) => UNARY_MINUS,
            Expr::Exp { base: _, exp: _ } => EXPONENTIATION,
            Expr::Number(_) => NUMBER,
            Expr::Variable { symbol: _ } => VARIABLE,
        }
    }
}
