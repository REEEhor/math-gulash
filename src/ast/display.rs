use std::fmt::{self};

use crate::ast::precedence;

use super::{precedence::Precedence, Expr};

fn write_addition(f: &mut fmt::Formatter, exprs: &Vec<Expr>) -> fmt::Result {
    for (idx, mut expr) in exprs.iter().enumerate() {
        if idx != 0 {
            if let Expr::UnaryMinus(inner_expr) = expr {
                expr = inner_expr;
                write!(f, " - ")?;
            } else {
                write!(f, " + ")?;
            }
        }

        let should_print_parenthesis = precedence::ADDITION.is_before(expr.precedence());

        if should_print_parenthesis {
            write!(f, "({expr})")?;
        } else {
            write!(f, "{expr}")?;
        }
    }
    Ok(())
}

fn write_multiplication(f: &mut fmt::Formatter, exprs: &Vec<Expr>) -> fmt::Result {
    for (idx, expr) in exprs.iter().enumerate() {
        if idx != 0 {
            write!(f, "·")?;
        }

        let should_print_parenthesis = precedence::MULTIPLICATION.is_before(expr.precedence());

        if should_print_parenthesis {
            write!(f, "({expr})")?;
        } else {
            write!(f, "{expr}")?;
        }
    }
    Ok(())
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Addition(exprs) => write_addition(f, exprs),
            Expr::Multiplication(exprs) => write_multiplication(f, exprs),
            Expr::Division { lhs, rhs } => {
                let should_print_parenthesis = precedence::DIVISION.is_before(lhs.precedence());
                if should_print_parenthesis {
                    write!(f, "({lhs})")?;
                } else {
                    write!(f, "{lhs}")?;
                }

                let should_print_parenthesis = precedence::DIVISION.is_before(rhs.precedence());
                if should_print_parenthesis {
                    write!(f, "/({rhs})")?;
                } else {
                    write!(f, "/{rhs}")?;
                }
                Ok(())
            }
            Expr::UnaryMinus(expr) => {
                let should_print_parenthesis = precedence::UNARY_MINUS.is_before(expr.precedence());
                if should_print_parenthesis {
                    write!(f, "-({expr})")
                } else {
                    write!(f, "-{expr}")
                }
            }
            Expr::Exp { base, exp } => {
                let should_print_parenthesis =
                    precedence::EXPONENTIATION.is_before(base.precedence());
                if should_print_parenthesis {
                    write!(f, "({base})")?;
                } else {
                    write!(f, "{base}")?;
                }

                let should_print_parenthesis =
                    precedence::EXPONENTIATION.is_before(exp.precedence());
                if should_print_parenthesis {
                    write!(f, "^({exp})")?;
                } else {
                    write!(f, "^{exp}")?;
                }
                Ok(())
            }
            Expr::Number(num) => write!(f, "{num}"),
            Expr::Variable { symbol } => write!(f, "{symbol}"),
        }
    }
}
