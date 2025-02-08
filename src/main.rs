pub mod excercise {
    pub mod division;
}
mod ast;
mod input;

use core::num;
use std::{collections::BTreeMap, net::ToSocketAddrs, ops::Rem, rc::Rc};

use ast::*;
use input::{get_input, get_number, get_number_in_range, wait_for_enter};
use rand::{
    distr::uniform::{SampleRange, SampleUniform},
    rngs::StdRng,
    Rng, SeedableRng,
};

/*
    a^2 - b^2 = (a - b) * (a + b)
*/
struct ExtractDiffSquares {
    a: Expr,
    b: Expr,
    a2: Expr,
    b2: Expr,
}

impl ExtractDiffSquares {
    fn generate_random(rnd: &mut StdRng, max_vars: u32, simple_chance: f64) -> Self {
        let a = generate_term(rnd, max_vars);
        let b = generate_term(rnd, max_vars);
        let a2 = a.pow_random(rnd, 2, simple_chance);
        let b2 = b.pow_random(rnd, 2, simple_chance);
        Self { a, b, a2, b2 }
    }
}

fn random_range_filter<T: SampleUniform, R: SampleRange<T> + Clone, F: Fn(&T) -> bool>(
    rnd: &mut StdRng,
    range: R,
    is_ok_pred: F,
) -> T {
    loop {
        let value = rnd.random_range(range.clone());
        if is_ok_pred(&value) {
            return value;
        }
    }
}

fn generate_term(rnd: &mut StdRng, max_vars: u32) -> Expr {
    let variable_count = rnd.random_range(0..=max_vars);
    let mut var_exp_dict = BTreeMap::<char, u32>::new();
    for _ in 0..variable_count {
        let new_var = random_range_filter(rnd, 'a'..='z', |chr| *chr != 'l');
        let exp = rnd.random_range(1..=20);
        var_exp_dict.insert(new_var, exp);
    }

    let mut variables = vec![Expr::Number(rnd.random_range(1..=20))];
    variables.extend(var_exp_dict.iter().map(|(var, exp)| Expr::Exp {
        base: Expr::Variable { symbol: *var }.into(),
        exp: Expr::Number(*exp).into(),
    }));

    if variable_count == 0 {
        variables[0].clone()
    } else {
        Expr::Multiplication(variables)
    }
}

impl Expr {
    fn pow(&self, exponent: i32) -> Expr {
        match self {
            Expr::Multiplication(exprs) => {
                let new_exprs = exprs.iter().map(|e| e.pow(exponent)).collect();
                Expr::Multiplication(new_exprs)
            }
            Expr::Division { lhs, rhs } => Expr::Division {
                lhs: lhs.pow(exponent).into(),
                rhs: rhs.pow(exponent).into(),
            },
            Expr::Variable { symbol: _ } => Expr::Exp {
                base: self.clone().into(),
                exp: Expr::signed_number(exponent).into(),
            },
            Expr::Exp { base, exp } => {
                let new_exp = exp.multiply_by_number(exponent);
                match new_exp {
                    Expr::Number(1) => (**base).clone(),
                    Expr::Number(0) => Expr::Number(1), // WARNING: this could be undefined 0^0
                    _ => Expr::Exp {
                        base: base.clone(),
                        exp: new_exp.into(),
                    },
                }
            }
            Expr::Number(0 | 1) => self.clone(),
            Expr::Number(number) => {
                let abs_exp = exponent.unsigned_abs();
                if exponent.is_negative() {
                    Expr::Division {
                        lhs: Expr::Number(1).into(),
                        rhs: Expr::Number(number.pow(abs_exp)).into(),
                    }
                } else {
                    Expr::Number(number.pow(abs_exp))
                }
            }
            Expr::Addition(_) => unimplemented!("Neumím umocnit sčítání"),
            Expr::UnaryMinus(_) => unimplemented!("Neumím umocnit sčítání"),
            // _ => unimplemented!("Neumím umocnit binární operace \"{self}\" :)"),
        }
    }

    fn multiply_by_number(&self, multiplier: i32) -> Expr {
        match self {
            Expr::Addition(exprs) => {
                let new_exprs = exprs
                    .iter()
                    .map(|e| e.multiply_by_number(multiplier))
                    .collect();
                Expr::Addition(new_exprs)
            }
            Expr::Multiplication(exprs) => {
                // Find any "number" or "-number" at some index
                let multiplied_expr: Option<(usize, Expr)> =
                    exprs
                        .iter()
                        .enumerate()
                        .find_map(|(idx, expr)| -> Option<(usize, Expr)> {
                            match expr {
                                Expr::UnaryMinus(inner) => {
                                    if let Expr::Number(_) = **inner {
                                        (idx, expr.multiply_by_number(multiplier)).into()
                                    } else {
                                        None
                                    }
                                }
                                Expr::Number(_) => {
                                    (idx, expr.multiply_by_number(multiplier)).into()
                                }
                                _ => None,
                            }
                        });

                let mut exprs_copy = exprs.clone();
                if let Some((idx, expr)) = multiplied_expr {
                    exprs_copy[idx] = expr;
                }

                Expr::Multiplication(exprs_copy)
            }
            Expr::Division { lhs, rhs } => Expr::Division {
                lhs: lhs.multiply_by_number(multiplier).into(),
                rhs: rhs.clone(),
            },
            Expr::UnaryMinus(expr) => expr.multiply_by_number(-multiplier),
            Expr::Number(num) => Expr::signed_number((*num as i32) * multiplier),
            expr => {
                let abs_multiplier = multiplier.unsigned_abs();
                let inner_expr = match abs_multiplier {
                    0 => return Expr::zero(),
                    1 => expr.clone(),
                    _ => Expr::Multiplication(vec![Expr::Number(abs_multiplier), expr.clone()]),
                };

                inner_expr.maybe_wrap_in_minus(multiplier.is_negative())
            }
        }
    }

    fn pow_simple(&self, exponent: i32) -> Expr {
        Expr::Exp {
            base: self.clone().into(),
            exp: Expr::signed_number(exponent).into(),
        }
    }

    fn pow_random(&self, rnd: &mut StdRng, exponent: i32, simple_chance: f64) -> Expr {
        if rnd.random_bool(simple_chance) {
            self.pow_simple(exponent)
        } else {
            self.pow(exponent)
        }
    }
}

fn do_diff_squares(assignment: ExtractDiffSquares) {
    let ExtractDiffSquares { a, b, a2, b2 } = assignment;
    println!("Rozložte na součin podle vzorce:");
    println!("{a2} - {b2}");
    println!();

    wait_for_enter("Pro zobrazení řešení dej enter");

    println!("Tohle je řešení:");
    println!("= ({a} - {b}) * ({a} + {b})");
}

// struct SquareAss {
//     a: Expr,
//     b: Expr,
//     a2: Expr,
//     b2: Expr,
//     two_ab: Expr,
// }

// impl SquareAss {
//     fn generate_random(rnd: &mut StdRng, max_vars: u32, simple_chance: f64) -> Self {
//         let a = generate_term(rnd, max_vars);
//         let b = generate_term(rnd, max_vars);
//         let a2 = a.pow_random(rnd, 2, simple_chance);
//         let b2 = b.pow_random(rnd, 2, simple_chance);
//         let two_ab = {
//             match (a,b) {
//                 (Expr::BinaryOp(binary_op), Expr::BinaryOp(binary_op)) => todo!(),
//                 (Expr::BinaryOp(binary_op), Expr::AtomExp(atom_with_exp)) => todo!(),
//                 (Expr::AtomExp(atom_with_exp), Expr::BinaryOp(binary_op)) => todo!(),
//                 (Expr::AtomExp(atom_with_exp), Expr::AtomExp(atom_with_exp)) => todo!(),
//             };

//         };

//         Self {
//             a: todo!(),
//             b: todo!(),
//             a2: todo!(),
//             b2: todo!(),
//             two_ab: todo!(),
//         }
//     }
// }

// fn do_square()

fn main() {
    let seed: u64 = get_number("Zadej seed");
    let mut rnd = StdRng::seed_from_u64(seed);
    for _ in 0.. {
        println!("\n\n\n");
        println!("========================");
        let ass = ExtractDiffSquares::generate_random(&mut rnd, 4, 0.09f64);
        do_diff_squares(ass);
    }
}
