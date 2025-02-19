use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{self, write};
use std::num::TryFromIntError;
use std::rc::Rc;
pub mod display;
pub mod error;
pub mod number_fraction;
pub mod ops;
pub mod precedence;
pub mod test_helpers;

use crate::expression::number_fraction::NumberFraction;
use precedence::*;

pub type Digit = u32;

type ExprRef = Rc<Expr>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    Addition(VecDeque<Expr>),
    Multiplication(VecDeque<Expr>),
    Division { lhs: ExprRef, rhs: ExprRef },
    UnaryMinus(ExprRef),
    Number(u32),
    Variable { symbol: char },
    Exp { base: ExprRef, exp: i32 },
}

impl Expr {
    pub const fn zero() -> Expr {
        Expr::Number(0)
    }

    pub const fn one() -> Expr {
        Expr::Number(1)
    }

    pub fn from_number_fraction(fraction: NumberFraction) -> Self {
        if let Some(integer) = fraction.as_whole_number() {
            return Self::signed_number(integer);
        }
        Expr::Division {
            lhs: Expr::Number(fraction.top).into(),
            rhs: Expr::Number(fraction.bottom_u32()).into(),
        }
        .maybe_wrap_in_minus(fraction.is_negative)
    }

    pub fn mult_div_from_exprs(top_exprs: VecDeque<Expr>, bottom_exprs: VecDeque<Expr>) -> Expr {
        match (top_exprs.is_empty(), bottom_exprs.is_empty()) {
            (_, true) /* denominator is one       */ => Expr::checked_mult(top_exprs),
            (true, false) /* nominator canceled out   */ => Expr::Division {
                lhs: Expr::Number(1).into(),
                rhs: Expr::checked_mult(bottom_exprs).into(),
            },
            (false, false) /* both parts are non empty */ => Expr::Division {
                lhs: Expr::checked_mult(top_exprs).into(),
                rhs: Expr::checked_mult(bottom_exprs).into(),
            },
        }
    }

    pub fn checked_mult(exprs: VecDeque<Expr>) -> Expr {
        let mut exprs = exprs;
        match exprs.len() {
            0 => Expr::Number(1),
            1 => exprs
                .pop_front()
                .expect("The lenght is manually checked that it is 1"),
            _ => Expr::Multiplication(exprs),
        }
    }

    pub fn checked_add(exprs: VecDeque<Expr>) -> Expr {
        let mut exprs = exprs;
        match exprs.len() {
            0 => Expr::Number(0).into(),
            1 => exprs
                .pop_front()
                .expect("The lenght is manually checked that it is 1"),
            _ => Expr::Addition(exprs),
        }
    }

    pub fn as_number(&self) -> Option<i32> {
        match self {
            Expr::UnaryMinus(expr) => {
                if let Expr::Number(num) = &**expr {
                    Some(-(*num as i32))
                } else {
                    None
                }
            }
            Expr::Number(num) => Some(*num as i32),
            _ => None,
        }
    }

    pub fn mult(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Multiplication(VecDeque::from([lhs, rhs]))
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
