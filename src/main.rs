pub mod excercise {
    pub mod division;
}
mod ast;
mod input;

use core::{hash, num};
use std::{
    collections::{BTreeMap, HashSet},
    f64::consts::FRAC_2_PI,
    net::ToSocketAddrs,
    num::{NonZero, NonZeroU32},
    ops::Rem,
    rc::Rc,
};

use crate::ast::simplify::canonical_term::VarExpMap;
use ast::{
    simplify::number_fraction::{self, NumberFraction},
    *,
};
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
    fn generate_random(
        rnd: &mut StdRng,
        no_vars_in_denominator_prob: f64,
        number_part_prob: f64,
        separate_number_fraction_prob: f64,
        number_frac_prob: f64,
        _max_vars: u32,
        non_simplified_prob: f64,
    ) -> Self {
        let symbols: &[char] = &[random_range_filter(rnd, 'a'..='z', |chr| *chr != 'l')];
        let random_pick = rnd.random_bool(0.5_f64);

        let mut random_term = |must_contain_var: bool| {
            random_mult_div_term(
                rnd,
                symbols,
                no_vars_in_denominator_prob,
                number_part_prob,
                separate_number_fraction_prob,
                number_frac_prob,
                must_contain_var,
                NonZeroU32::new(20).unwrap(),
            )
        };

        let a = random_term(random_pick);
        let b = random_term(!random_pick);
        let a2 = square_random(rnd, a.clone(), non_simplified_prob);
        let b2 = square_random(rnd, b.clone(), non_simplified_prob);

        Self { a, b, a2, b2 }
    }
}

fn square_random(rnd: &mut StdRng, expr: Expr, non_simplified_prob: f64) -> Expr {
    let mut squared_expr = expr.pow(Expr::Number(2));
    let simplified_prob = 1_f64 - non_simplified_prob;
    if rnd.random_bool(simplified_prob) {
        squared_expr = squared_expr.simplify_exp().new_expr;
        //
        squared_expr = squared_expr.simplify_commutative().new_expr;
        squared_expr = squared_expr.simplify_by_evaluation().new_expr;
        squared_expr = squared_expr.simplify_mult_div().new_expr;
    }

    for _ in 0..6 {
        squared_expr = squared_expr.simplify_by_evaluation().new_expr;
        squared_expr = squared_expr.simplify_mult_div().new_expr;
        squared_expr = squared_expr.simplify_commutative().new_expr;
    }
    squared_expr
}

fn random_symbols(rnd: &mut StdRng, max_vars: u32) -> HashSet<char> {
    let mut result_set = HashSet::new();
    let vars_count = rnd.random_range(0..=max_vars);
    for _ in 0..vars_count {
        while {
            let random_symbol = random_range_filter(rnd, 'a'..='z', |chr| *chr != 'l');
            !result_set.insert(random_symbol)
        } {}
    }

    return result_set;
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

fn random_pick<T: Copy>(rnd: &mut StdRng, source: &[T]) -> T {
    let random_idx = rnd.random_range(0..source.len());
    source[random_idx]
}

fn random_mult_div_term(
    rnd: &mut StdRng,
    available_vars: &[char],
    no_vars_in_denominator_prob: f64,
    number_part_prob: f64,
    separate_number_fraction_prob: f64,
    number_frac_prob: f64,
    must_contain_variable: bool,
    max_exp: NonZeroU32,
) -> Expr {
    // Figure out the ranges for the exponents of the individual variables
    let max_exp = max_exp.get() as i32;
    let no_vars_in_denominator = rnd.random_bool(no_vars_in_denominator_prob);
    let min_exp = if no_vars_in_denominator { 1 } else { -max_exp };

    // Generate variables to a certain power
    let variable_count = rnd.random_range((must_contain_variable as usize)..=available_vars.len());
    let mut var_exps = VarExpMap::new();
    for _ in 0..variable_count {
        let random_var = random_pick(rnd, available_vars);
        var_exps[random_var] = random_range_filter(rnd, min_exp..=max_exp, |x| *x != 0 && *x != 1);
    }
    let (mut top_exprs, mut bottom_exprs) = var_exps.partition_by_exp_sign();

    // Add the number if it should have been generated
    let wanted_number_part = rnd.random_bool(number_part_prob);
    let should_include_number = wanted_number_part || variable_count == 0;
    if should_include_number {
        let number_part_should_be_fraction = rnd.random_bool(number_frac_prob);
        let number_fraction = if number_part_should_be_fraction {
            random_non_integer_fraction(rnd)
        } else {
            NumberFraction::whole_number(rnd.random_range(2..=20))
        };

        // Check if the number part should be separate.
        // Meaning the expression is in the form:
        //   (fraction or number)  *  (fraction of variables to some power)
        let number_part_wanted_to_be_separate = rnd.random_bool(separate_number_fraction_prob);
        if number_part_wanted_to_be_separate {
            let variables_part = Expr::mult_div_from_exprs(top_exprs, bottom_exprs);
            let number_part = Expr::from_number_fraction(number_fraction.abs());
            return Expr::mult(number_part, variables_part);
        }

        // The number part should not be separate.
        // => Lets add it to the top and bottom part of the fraction.
        let add_number = |number: u32, exprs: &mut Vec<Expr>| {
            if number != 1 {
                exprs.insert(0, Expr::Number(number));
            }
        };
        add_number(number_fraction.top, &mut top_exprs);
        add_number(number_fraction.bottom_u32(), &mut bottom_exprs);
    }

    Expr::mult_div_from_exprs(top_exprs, bottom_exprs)
}

fn random_non_integer_fraction(rnd: &mut StdRng) -> NumberFraction {
    let top: i32 = rnd.random_range(1..=19);
    let bottom: i32 = rnd.random_range((top + 1)..=20);
    NumberFraction::new_in_base_form(top, bottom).expect("Bottom cannot be zero")
}

fn random_number_fraction(rnd: &mut StdRng) -> NumberFraction {
    let top: i32 = rnd.random_range(1..=20);
    let bottom: i32 = rnd.random_range(1..=20);
    NumberFraction::new_in_base_form(top, bottom).expect("Bottom cannot be zero")
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
    for idx in 0.. {
        println!("\n\n\n");
        println!("======================== {idx} ========================");
        let ass = ExtractDiffSquares::generate_random(
            &mut rnd, 0.6__f64, 0.8__f64, 0.4__f64, 0.4__f64, 1, 0.05__f64,
        );
        do_diff_squares(ass);
    }
}
