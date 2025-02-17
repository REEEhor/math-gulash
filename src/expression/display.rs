use colored::Color;
use std::fmt;

const COLOR_ARRAY: &[Color] = &[
    Color::Cyan,
    Color::Green,
    Color::Red,
    Color::Blue,
    Color::BrightGreen,
    Color::BrightRed,
    // Color::BrightBlue,
];

use crate::expression::precedence;

use super::{precedence::Precedence, Expr};

#[derive(Default)]
struct ColorStack {
    top_idx: i16,
}
pub struct DisplayContext {
    should_use_color: bool,
    color_stack: ColorStack,
}

impl ColorStack {
    pub fn get_new_color(&mut self) -> Color {
        self.top_idx += 1;
        self.top_idx %= COLOR_ARRAY.len() as i16;
        COLOR_ARRAY[self.top_idx as usize]
    }

    pub fn pop_color(&mut self) -> Color {
        self.top_idx -= 1;
        if self.top_idx.is_negative() {
            self.top_idx = COLOR_ARRAY.len() as i16;
        }
        COLOR_ARRAY[self.top_idx as usize]
    }

    pub fn current_color(&self) -> Color {
        COLOR_ARRAY[self.top_idx as usize]
    }
}

pub struct ExprDisplay<'a> {
    expr: &'a Expr,
    context: DisplayContext,
}

impl<'a> ExprDisplay<'a> {
    pub fn new(expr: &'a Expr, context: DisplayContext) -> Self {
        Self { expr, context }
    }
}

impl Expr {
    pub fn disp<'a>(&'a self) -> ExprDisplay<'a> {
        ExprDisplay::new(
            self,
            DisplayContext {
                should_use_color: false,
                color_stack: Default::default(),
            },
        )
    }

    pub fn disp_with_color<'a>(&'a self) -> ExprDisplay<'a> {
        ExprDisplay::new(
            self,
            DisplayContext {
                should_use_color: true,
                color_stack: Default::default(),
            },
        )
    }
}

fn to_super(number: i32) -> String {
    let number_str = format!("{number}");
    number_str
        .chars()
        .map(|chr| match chr {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '-' => '⁻',
            _ => unreachable!(
            "The number `{number}` contains the char `{chr}` which cannot be converted to exponent"
        ),
        })
        .collect()
}

impl<'a> fmt::Display for ExprDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expr = self.expr;
        let use_color = self.context.should_use_color;
        if use_color {
            todo!("Implement printing with colors :)");
        }

        match expr {
            Expr::Addition(exprs) => write_addition(f, exprs.iter()),
            Expr::Multiplication(exprs) => write_multiplication(f, exprs.iter()),
            Expr::Division { lhs, rhs } => {
                let should_print_parenthesis = precedence::DIVISION.is_before(lhs.precedence());
                if should_print_parenthesis {
                    write!(f, "({})", lhs.disp())?;
                } else {
                    write!(f, "{}", lhs.disp())?;
                }

                let should_print_parenthesis = precedence::DIVISION.is_before(rhs.precedence());
                if should_print_parenthesis {
                    write!(f, "/({})", rhs.disp())?;
                } else {
                    write!(f, "/{}", rhs.disp())?;
                }
                Ok(())
            }
            Expr::UnaryMinus(expr) => {
                let should_print_parenthesis = precedence::UNARY_MINUS.is_before(expr.precedence());
                if should_print_parenthesis {
                    write!(f, "-({})", expr.disp())
                } else {
                    write!(f, "-{}", expr.disp())
                }
            }
            Expr::Exp { base, exp } => {
                let should_print_parenthesis =
                    precedence::EXPONENTIATION.is_before(base.precedence());
                if should_print_parenthesis {
                    write!(f, "({})", base.disp())?;
                } else {
                    write!(f, "{}", base.disp())?;
                }
                write!(f, "{}", to_super(*exp))?;
                Ok(())
            }
            Expr::Number(num) => write!(f, "{num}"),
            Expr::Variable { symbol } => write!(f, "{symbol}"),
        }
    }
}

fn write_addition<'a, ExprIter: Iterator<Item = &'a Expr>>(
    f: &mut fmt::Formatter,
    exprs: ExprIter,
) -> fmt::Result {
    for (idx, mut expr) in exprs.enumerate() {
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
            write!(f, "({})", expr.disp())?;
        } else {
            write!(f, "{}", expr.disp())?;
        }
    }
    Ok(())
}

fn write_multiplication<'a, ExprIter: Iterator<Item = &'a Expr>>(
    f: &mut fmt::Formatter,
    exprs: ExprIter,
) -> fmt::Result {
    for (idx, expr) in exprs.enumerate() {
        // if idx != 0 {
        //     write!(f, "·")?;
        // }
        let should_print_parenthesis = precedence::MULTIPLICATION.is_before(expr.precedence());

        if should_print_parenthesis {
            write!(f, "({})", expr.disp())?;
        } else {
            write!(f, "{}", expr.disp())?;
        }
    }
    Ok(())
}
